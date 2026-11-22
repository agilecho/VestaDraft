use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::PathBuf};
use vesta_codecs::{decode, encode_verified};
use vesta_commands::{EngineV2, Limits};
use vesta_model::{
    Camera, Capture, DocumentAt, Error, Ids, PersistentResource, Phase, ProjectContent,
    RestoredProject, Site, digest, field::restore_fields, source7::SourceDocument,
};

fn inputs() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.ancestors()
        .nth(2)
        .unwrap()
        .join("output/vesta-p2-03/20261121-c8-closure-execution/inputs")
}

fn source_and_bytes() -> (Value, Vec<u8>) {
    let inputs = inputs();
    let mut source: Value = serde_json::from_slice(
        &fs::read(inputs.join("source7/phase-scene-field-roles.valid.json")).unwrap(),
    )
    .unwrap();
    source["identity"]["revision"] = json!("0");
    let values = fs::read(inputs.join("field/shared-closed.f32le")).unwrap();
    (source, values)
}

fn construct(value: Value, raw: Vec<u8>) -> Capture {
    construct_with_bindings(
        value,
        BTreeMap::from([(digest(&raw), raw)]),
        &["field-data/0/phase/0", "field-data/1/scene/0"],
    )
}

fn construct_with_bindings(
    value: Value,
    blobs: BTreeMap<String, Vec<u8>>,
    logical_uses: &[&str],
) -> Capture {
    let scene = &value["scenes"][0];
    let phase = &value["phaseGraph"]["phases"][0];
    let structure = &phase["structure"];
    let as_id = |v: &Value| v.as_str().unwrap().to_owned();
    let row = &structure["sites"][0];
    let s = Site {
        id: as_id(&row["id"]).into(),
        label: row["label"].as_str().unwrap().into(),
        element: row["species"]["element"].as_str().unwrap().into(),
        frac: serde_json::from_value(row["fractional"].clone()).unwrap(),
        occupancy: row["occupancy"].as_f64().unwrap(),
        u_iso_a2: 0.0,
    };
    let content = ProjectContent {
        native_camera: None,
        phases: vec![Phase {
            id: as_id(&phase["id"]).into(),
            label: phase["label"].as_str().unwrap().into(),
            cell: serde_json::from_value(structure["cell"].clone()).unwrap(),
            sites: vec![s],
            symmetry_op_id: as_id(&structure["group"]["operators"][0]["id"]).into(),
            boundary_min: [0.; 3],
            boundary_max: [1.; 3],
        }],
        scene_id: as_id(&scene["id"]).into(),
        camera: Camera {
            rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            projection: "parallel".into(),
            zoom: 1.,
        },
        hidden: vec![],
        comment: String::new(),
        legacy_source: None,
        resources: value["resources"]
            .as_array()
            .unwrap()
            .iter()
            .zip(logical_uses.iter().copied())
            .map(|(row, name)| PersistentResource {
                id: as_id(&row["id"]).into(),
                logical_key: name.into(),
                sha256: as_id(&row["sha256"]),
                byte_length: as_id(&row["byteLength"]),
                media_type: "application/octet-stream".into(),
            })
            .collect(),
    };
    let document = DocumentAt {
        document_id: as_id(&value["identity"]["documentId"]),
        epoch: as_id(&value["identity"]["epoch"]),
        revision: "0".into(),
    };
    let source = SourceDocument::decode(value).unwrap();
    let state_digest = Capture::digest_state(&content, Some(&source)).unwrap();
    Capture {
        document,
        content,
        resources: blobs,
        state_digest,
        source: Some(source),
    }
}

#[test]
fn rich_supported_source_families_reach_the_writer_and_unknown_opaque_content_refuses() {
    let path = inputs();
    let mut source: Value =
        serde_json::from_slice(&fs::read(path.join("source7/rich-persistence.valid.json")).unwrap()).unwrap();
    source["identity"]["revision"] = json!("0");
    // This fixture tests the qualified structural families. The unrelated authored
    // partial charge has no captured descriptor and gets its own separate test.
    source["molecules"][0]["frames"][0]["atoms"][0]["partialCharges"] = json!([]);
    let files = [
        "field/shared-closed.f32le",
        "source7/original-legacy-section.vesta",
        "source7/authored-reference.raw",
        "source7/authored-opaque.raw",
    ];
    let blobs = files.into_iter().map(|name| {
        let bytes = fs::read(path.join(name)).unwrap();
        (digest(&bytes), bytes)
    }).collect();
    let keys = ["field-data/0/phase/0", "field-data/1/scene/0", "source/legacy-section",
        "source/declared-reference", "source/opaque"];
    let with_unknown = construct_with_bindings(source.clone(), blobs, &keys);
    assert_eq!(encode_verified(&with_unknown, "unknown.vesta", Ids::new([118; 16])).unwrap_err().code,
        "E_UNSAFE_SOURCE_REWRITE");

    // A distinct engineering fixture exercises the closed structured families.
    source["opaqueBlocks"] = json!([]);
    let raw_id = source["resources"].as_array_mut().unwrap().pop().unwrap()["sha256"]
        .as_str().unwrap().to_owned();
    let mut blobs = with_unknown.resources;
    blobs.remove(&raw_id);
    let prepared = construct_with_bindings(source.clone(), blobs, &keys[..4]);
    let engine = EngineV2::loaded(
        RestoredProject::new(prepared, Ids::new([119; 16])).unwrap(),
        Ids::new([120; 16]), Limits::default(),
    ).unwrap();
    let captured = engine.capture().unwrap();
    let bundle = encode_verified(&captured, "rich-source.vesta", Ids::new([121; 16])).unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([122; 16])).unwrap();
    let persisted = reopened.capture.source.unwrap().to_value().unwrap();
    for (name, expected) in source.as_object().unwrap() {
        if let Some(items) = expected.as_array() {
            let actual = persisted[name].as_array().unwrap();
            assert_eq!(actual.len(), items.len() + (name == "metadata") as usize, "{name}");
        }
    }
    assert_eq!(persisted["molecules"][0]["frames"][0]["atoms"][0]["cartesianA"],
        source["molecules"][0]["frames"][0]["atoms"][0]["cartesianA"]);
    let halfspaces = |project: &Value| project["shapes"][0]["value"]["halfspaces"]
        .as_array().unwrap().iter().map(|row| {
            (row["normal"].clone(), row["offsetA"].clone())
        }).collect::<Vec<_>>();
    assert_eq!(halfspaces(&persisted), halfspaces(&source));
    assert_eq!(persisted["cameras"][0]["value"], source["cameras"][0]["value"]);
    assert_ne!(persisted["phaseGraph"]["root"], source["phaseGraph"]["root"]);
    for raw in captured.resources.values() {
        assert!(bundle.dependencies.values().any(|stored| stored == raw));
    }
    write_bundle("VESTA_RICH_ARTIFACT_DIR", "rich-source.vesta", bundle);
}

fn write_bundle(variable: &str, name: &str, bundle: vesta_codecs::SaveBundle) {
    if let Some(path) = std::env::var_os(variable) {
        let root = PathBuf::from(path);
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join(name), bundle.primary).unwrap();
        for (relative, bytes) in bundle.dependencies {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
    }
}

#[test]
fn nonperiodic_molecule_survives_native_save_without_an_invented_phase_or_scene() {
    let mut value: Value = serde_json::from_slice(
        &fs::read(inputs().join("source7/codec-source-document-preservation.valid.json")).unwrap(),
    )
    .unwrap();
    let document = DocumentAt {
        document_id: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        epoch: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
        revision: "0".into(),
    };
    value["identity"]["documentId"] = json!(document.document_id);
    value["identity"]["epoch"] = json!(document.epoch);
    value["identity"]["revision"] = json!(document.revision);
    // The research fixture uses symbolic identities and a partial charge with no raw
    // descriptor. This engineering positive supplies engine IDs and no charge use.
    let molecule_id = "cccccccccccccccccccccccccccccccc";
    value["sourceEntryOrder"][0]["id"] = json!(molecule_id);
    value["molecules"][0]["id"] = json!(molecule_id);
    let frame = &mut value["molecules"][0]["frames"][0];
    frame["id"] = json!("dddddddddddddddddddddddddddddddd");
    frame["atoms"][0]["id"] = json!("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
    frame["atoms"][0]["partialCharges"] = json!([]);
    frame["basisCenters"][0]["id"] = json!("ffffffffffffffffffffffffffffffff");
    let original_molecule = value["molecules"][0].clone();
    let mut content = EngineV2::new(
        vesta_model::Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "private editor seed",
        Ids::new([92; 16]),
        Limits::default(),
    )
    .unwrap()
    .content()
    .clone();
    content.phases.clear();
    let source = SourceDocument::decode(value).unwrap();
    let capture = Capture {
        document,
        state_digest: Capture::digest_state(&content, Some(&source)).unwrap(),
        source: Some(source),
        content,
        resources: BTreeMap::new(),
    };
    let bundle = encode_verified(&capture, "molecule.vesta", Ids::new([93; 16])).unwrap();
    let primary = std::str::from_utf8(&bundle.primary).unwrap();
    assert!(
        !primary
            .lines()
            .any(|line| matches!(line, "CRYSTAL" | "CELLP" | "STRUC"))
    );
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([94; 16])).unwrap();
    assert!(reopened.content.phases.is_empty());
    let fresh = reopened.capture.source.unwrap().to_value().unwrap();
    assert!(fresh["phaseGraph"].is_null());
    assert_eq!(fresh["scenes"], json!([]));
    let molecule = &fresh["molecules"][0];
    assert_ne!(molecule["id"], original_molecule["id"]);
    assert_eq!(molecule["label"], original_molecule["label"]);
    let original_frame = &original_molecule["frames"][0];
    let fresh_frame = &molecule["frames"][0];
    assert_eq!(
        fresh_frame["atoms"][0]["species"],
        original_frame["atoms"][0]["species"]
    );
    assert_eq!(
        fresh_frame["basisCenters"][0]["basisSetup"],
        original_frame["basisCenters"][0]["basisSetup"]
    );
    assert_eq!(fresh["sourceEntryOrder"][0]["id"], molecule["id"]);
}

fn violation(result: vesta_model::Result<impl std::fmt::Debug>) -> Error {
    result.unwrap_err()
}

#[test]
fn engine_installs_two_field_uses_and_restores_them_from_one_blob() {
    let (source, raw) = source_and_bytes();
    let capture = construct(source, raw.clone());
    let first = EngineV2::loaded(
        RestoredProject::new(capture, Ids::new([80; 16])).unwrap(),
        Ids::new([81; 16]),
        Limits::default(),
    )
    .unwrap();
    let fields = first.active_fields();
    assert_eq!(fields.len(), 2);
    assert_ne!(fields[0].owner, fields[1].owner);
    assert_ne!(fields[0].resource_id, fields[1].resource_id);
    assert_eq!(fields[0].descriptor.resource, fields[1].descriptor.resource);
    for field in fields {
        let data = first.field_buffer(&field.field_id).unwrap();
        assert_eq!(data.stored_shape(), [3, 2, 2]);
        for (index, word) in raw.chunks_exact(4).enumerate() {
            let at = [index % 3, (index / 3) % 2, index / 6];
            let expected = u32::from_le_bytes(word.try_into().unwrap());
            assert_eq!(data.sample_bits(at), Some(u64::from(expected)));
        }
    }
    let phase = &first.content().phases[0];
    let exported = first.capture().unwrap();
    assert_eq!(exported.resources.len(), 1);
    let bundle = encode_verified(&exported, "field-initial.vesta", Ids::new([82; 16])).unwrap();
    assert_eq!(
        bundle
            .dependencies
            .values()
            .filter(|blob| **blob == raw)
            .count(),
        1
    );
    let restored = decode(&bundle.primary, &bundle.dependencies, Ids::new([83; 16])).unwrap();
    let mut second = EngineV2::loaded(restored, Ids::new([84; 16]), Limits::default()).unwrap();
    assert_eq!(second.active_fields().len(), 2);
    for (before, after) in fields.iter().zip(second.active_fields()) {
        assert_ne!(before.field_id, after.field_id);
        assert_ne!(before.resource_id, after.resource_id);
        assert_eq!(before.descriptor, after.descriptor);
        assert!(second.field_buffer(&before.field_id).is_err());
        let data = second.field_buffer(&after.field_id).unwrap();
        for (index, word) in raw.chunks_exact(4).enumerate() {
            let at = [index % 3, (index / 3) % 2, index / 6];
            let expected = u32::from_le_bytes(word.try_into().unwrap());
            assert_eq!(data.sample_bits(at), Some(u64::from(expected)));
        }
    }
    let editable = second.content().phases[0].clone();
    assert_ne!(phase.id, editable.id);
    second
        .execute(
            &second.document(),
            &json!({"command":"site.setFractional","params":{
                "phaseId":editable.id,"siteId":editable.sites[0].id,"frac":[0.625,0.25,0.125]
            }}),
        )
        .unwrap();
    let edited = second.capture().unwrap();
    let artifact = encode_verified(&edited, "field-continued.vesta", Ids::new([85; 16])).unwrap();
    let latest = decode(
        &artifact.primary,
        &artifact.dependencies,
        Ids::new([86; 16]),
    )
    .unwrap();
    assert_eq!(latest.content.phases[0].sites[0].frac, [0.625, 0.25, 0.125]);
    write_bundle("VESTA_FIELD_ARTIFACT_DIR", "field-initial.vesta", bundle);
}

#[test]
fn native_reopen_restores_authored_z_fastest_f64_with_its_cartesian_frame() {
    let (mut source, _) = source_and_bytes();
    let raw = fs::read(inputs().join("field/z-fastest.f64le")).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(inputs().join("expected.json")).unwrap()).unwrap();
    let expected = &expected["field"]["zFastestF64"];
    for resource in source["resources"].as_array_mut().unwrap() {
        resource["sha256"] = json!(digest(&raw));
        resource["byteLength"] = json!(raw.len().to_string());
    }
    for field in source["fields"].as_array_mut().unwrap() {
        field["field"] = expected["descriptor"].clone();
        let use_data = &mut field["metadata"][0]["tree"]["payload"]["members"];
        use_data["descriptorDigest"]["value"] = expected["descriptorCanonicalJsonSHA256"].clone();
        use_data["bytes"]["sha256"] = json!(digest(&raw));
        use_data["bytes"]["byteLength"] = json!(raw.len().to_string());
    }
    let bundle =
        encode_verified(&construct(source, raw), "z-axis.vesta", Ids::new([95; 16])).unwrap();
    let loaded = decode(&bundle.primary, &bundle.dependencies, Ids::new([96; 16])).unwrap();
    let engine = EngineV2::loaded(loaded, Ids::new([97; 16]), Limits::default()).unwrap();
    for field in engine.active_fields() {
        let values = engine.field_buffer(&field.field_id).unwrap();
        assert_eq!(values.stored_shape(), [2; 3]);
        assert_eq!(field.descriptor.origin_a, [0.25, 0.5, 0.75]);
        assert_eq!(
            field.descriptor.step_basis_a,
            [[0.5, 0.25, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 2.0]]
        );
        for (i, word) in expected["dataWords"].as_array().unwrap().iter().enumerate() {
            let mut native = [0; 8];
            for (j, byte) in native.iter_mut().enumerate() {
                let word = word.as_str().unwrap();
                *byte = u8::from_str_radix(&word[j * 2..j * 2 + 2], 16).unwrap();
            }
            let coord = [i / 4, (i / 2) % 2, i % 2];
            assert_eq!(values.sample_bits(coord), Some(u64::from_le_bytes(native)));
        }
    }
    write_bundle("VESTA_FIELD_ARTIFACT_DIR", "z-axis.vesta", bundle);
}

#[test]
fn field_missing_registration_and_wrong_semantic_owner_refuse_by_identity() {
    let (source, raw) = source_and_bytes();
    let capture = construct(source.clone(), raw);
    let mut missing = source.clone();
    missing["fields"][0]["metadata"] = json!([]);
    assert_eq!(
        violation(restore_fields(
            &missing,
            &capture.content.resources,
            &capture.resources,
            100_000
        ))
        .code,
        "E_REOPEN_COVERAGE"
    );
    let mut detached = source.clone();
    detached["phaseGraph"]["phases"][0]["fieldIds"] = json!([]);
    let mut repeated = source.clone();
    let field = repeated["fields"][0]["id"].clone();
    repeated["phaseGraph"]["phases"][0]["fieldIds"] = json!([field, field]);
    for invalid in [detached, repeated] {
        assert_eq!(
            violation(restore_fields(
                &invalid,
                &capture.content.resources,
                &capture.resources,
                100_000
            ))
            .code,
            "E_REOPEN_OWNER"
        );
    }
    let mut wrong = source;
    let b = wrong["fields"][1]["metadata"][0]["tree"]["payload"]["members"]["owner"].clone();
    wrong["fields"][0]["metadata"][0]["tree"]["payload"]["members"]["owner"] = b;
    assert_eq!(
        violation(restore_fields(
            &wrong,
            &capture.content.resources,
            &capture.resources,
            100_000
        ))
        .code,
        "E_REOPEN_COVERAGE"
    );
}

#[test]
fn swapping_two_valid_same_sha_uses_cannot_retarget_the_installed_fields() {
    let (mut source, raw) = source_and_bytes();
    let use_path = |field| format!("/fields/{field}/metadata/0/tree/payload/members/bytes");
    let a_path = use_path(0);
    let b_path = use_path(1);
    let a = source.pointer(&a_path).unwrap().clone();
    let b = source.pointer(&b_path).unwrap().clone();
    assert_eq!(a["sha256"], b["sha256"]);
    assert_ne!(a["resourceId"], b["resourceId"]);
    *source.pointer_mut(&a_path).unwrap() = b;
    *source.pointer_mut(&b_path).unwrap() = a;
    let capture = construct(source, raw);
    assert_eq!(
        encode_verified(&capture, "swapped-uses.vesta", Ids::new([105; 16]))
            .unwrap_err()
            .code,
        "E_REOPEN_OWNER"
    );
    let restored = RestoredProject::new(capture, Ids::new([98; 16])).unwrap();
    let failure = EngineV2::loaded(restored, Ids::new([99; 16]), Limits::default())
        .err()
        .expect("same-content resource IDs still represent different installed uses");
    assert_eq!(failure.code, "E_REOPEN_OWNER");
}

#[test]
fn native_writer_rejects_self_consistent_field_descriptors_with_invalid_storage() {
    let (source, raw) = source_and_bytes();
    let cases = [
        ("unit", json!("ePerA3"), "E_FIELD_DESCRIPTOR"),
        ("coreShape", json!([2, 3, 2]), "E_FIELD_STORAGE"),
        ("periodic", json!([false, false, false]), "E_FIELD_CLOSURE"),
    ];
    for (name, replacement, expected) in cases {
        let mut invalid = source.clone();
        let row = &mut invalid["fields"][0];
        row["field"][name] = replacement;
        let descriptor = vesta_model::source_audit::canonical(&row["field"]);
        row["metadata"][0]["tree"]["payload"]["members"]["descriptorDigest"]["value"] =
            json!(digest(&descriptor));
        let capture = construct(invalid, raw.clone());
        assert_eq!(
            encode_verified(&capture, "invalid-field.vesta", Ids::new([106; 16]))
                .unwrap_err()
                .code,
            expected,
            "{name}"
        );
    }
}

#[test]
fn selected_c1_grid_installs_all_512_native_words_and_preserves_the_source_grid() {
    use vesta_codecs::formats::resource_graph::{
        CaptureLimits, GraphProfile, discover_graph_with_profile,
    };
    let study = inputs().ancestors().nth(3).unwrap().join("vesta-study/20261108-113622/s04");
    let primary = "temporary-C1-density.vesta";
    let graph = discover_graph_with_profile(
        primary,
        GraphProfile::NativeVesta,
        |key, _| fs::read(study.join(key)).map(Some).map_err(|e| Error::new("E_IO", e.to_string())),
        CaptureLimits::default(),
    ).unwrap();
    let grid = &graph.files["temporary-C1-density.pgrid"];
    let capture = vesta_codecs::open_selected_density(&graph, Ids::new([100; 16])).unwrap();
    let writer = EngineV2::loaded(capture, Ids::new([101; 16]), Limits::default()).unwrap();
    let bundle = encode_verified(&writer.capture().unwrap(), "density.vesta", Ids::new([102; 16])).unwrap();
    assert!(bundle.dependencies.values().any(|bytes| bytes == grid));
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([103; 16])).unwrap();
    let engine = EngineV2::loaded(reopened, Ids::new([104; 16]), Limits::default()).unwrap();
    let [field] = engine.active_fields() else { panic!("one active field expected") };
    assert_eq!(field.descriptor.kind, vesta_model::field::FieldKind::ElectronDensity);
    assert_eq!(field.descriptor.unit, vesta_model::field::FieldUnit::EPerA3);
    assert_eq!(field.descriptor.core_shape, [8; 3]);
    assert_eq!(field.descriptor.periodic, [true; 3]);
    assert_eq!(field.descriptor.closure, [false; 3]);
    assert_eq!(field.descriptor.step_basis_a, [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]);
    assert_ne!(writer.active_fields()[0].resource_id, field.resource_id);
    let raw = fs::read(inputs().join("field/original-8cube.f32le")).unwrap();
    let data = engine.field_buffer(&field.field_id).unwrap();
    for (index, word) in raw.chunks_exact(4).enumerate() {
        let expected = u32::from_le_bytes(word.try_into().unwrap()) as u64;
        assert_eq!(data.sample_bits([index % 8, index / 8 % 8, index / 64]), Some(expected));
    }
    let mut engine = engine;
    let at = engine.document();
    let phase = engine.content().phases[0].id.clone();
    let failure = engine.execute(&at, &json!({"command":"cell.set","params":{
        "phaseId":phase,"cell":{"lengthA":[5.,4.,4.],"anglesDeg":[90.,90.,90.]}
    }})).unwrap_err();
    assert_eq!(failure.code, "E_REOPEN_OWNER");
    assert_eq!(engine.document(), at);
}
