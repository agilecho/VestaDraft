use serde_json::json;
use vesta_codecs::*;
use vesta_commands::{EngineV2, Limits};
use vesta_model::*;
#[test]
fn full_current_fields_resources_and_new_lifetime() {
    let mut e = EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "原生",
        Ids::new([2; 16]),
        Limits::default(),
    )
    .unwrap();
    let p = e.content().phases[0].id.clone();
    let at = e.document();
    e.execute(&at,&json!({"command":"site.add","params":{"phaseId":p,"label":"C1","species":{"element":"C"},"frac":[0.25,0,0.5],"occupancy":1,"uIsoA2":0}})).unwrap();
    e.attach_resource(
        "original/source",
        b"independent immutable source bytes".to_vec(),
    )
    .unwrap();
    let source = e.capture().unwrap();
    let b = encode(&source, "晶体.vesta").unwrap();
    let source_bytes = b
        .dependencies
        .iter()
        .find(|(p, _)| p.ends_with(".source.json"))
        .unwrap()
        .1;
    let saved: serde_json::Value = serde_json::from_slice(source_bytes).unwrap();
    let frozen = include_bytes!("../contracts/codec-source-document.schema.json");
    assert_eq!(saved["sourceDTOSchemaDigest"], digest(frozen));
    let schema: serde_json::Value = serde_json::from_slice(frozen).unwrap();
    let roots = saved["tree"]["members"].as_object().unwrap();
    assert_eq!(roots.len(), schema["required"].as_array().unwrap().len());
    for root in schema["required"].as_array().unwrap() {
        assert!(roots.contains_key(root.as_str().unwrap()));
    }
    assert_eq!(
        roots["schemaVersion"]["value"],
        "CodecSourceDocument-proposal-7"
    );
    assert!(
        std::str::from_utf8(&b.primary)
            .unwrap()
            .starts_with("#VESTA_FORMAT_VERSION 3.5.4\n#DRAFT_SOURCE_MANIFEST_V1 ")
    );
    assert!(!b.dependencies.is_empty());
    let fresh = decode(&b.primary, &b.dependencies, Ids::new([3; 16])).unwrap();
    assert_ne!(fresh.content.phases[0].id, source.content.phases[0].id);
    assert_eq!(fresh.content.phases[0].sites[0].frac, [0.25, 0., 0.5]);
    assert_eq!(fresh.resources, source.resources);
    assert_ne!(
        fresh.content.resources[0].id,
        source.content.resources[0].id
    );
    assert_eq!(
        fresh.content.resources[0].sha256,
        source.content.resources[0].sha256
    );
    let mut broken = b.dependencies.clone();
    let k = broken.keys().next().unwrap().clone();
    broken.remove(&k);
    assert!(decode(&b.primary, &broken, Ids::new([4; 16])).is_err());
}

#[test]
fn literal_identity_text_and_two_logical_resource_owners_survive() {
    let mut e = EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "typed source",
        Ids::new([8; 16]),
        Limits::default(),
    )
    .unwrap();
    e.attach_resource("source/one", b"same physical bytes".to_vec())
        .unwrap();
    e.attach_resource("source/two", b"same physical bytes".to_vec())
        .unwrap();
    let mut source = e.capture().unwrap();
    source.content.comment = source.content.scene_id.to_string();
    source.state_digest = source.content.digest();
    let b = encode(&source, "owners.vesta").unwrap();
    let loaded = decode(&b.primary, &b.dependencies, Ids::new([9; 16])).unwrap();
    assert_eq!(loaded.content.comment, source.content.scene_id.to_string());
    assert_ne!(loaded.content.scene_id, source.content.scene_id);
    for (i, r) in loaded.content.resources.iter().enumerate() {
        assert_ne!(r.id, source.content.resources[i].id);
        assert_eq!(r.sha256, source.content.resources[i].sha256);
        assert_eq!(r.logical_key, source.content.resources[i].logical_key);
    }
    assert_eq!(loaded.content.resources.len(), 2);
    assert_eq!(loaded.resources.len(), 1);
}

// A hostile file author can recompute every public hash. Reject semantic/type loss too.
fn rewrite_source(bundle: &SaveBundle, edit: impl FnOnce(&mut serde_json::Value)) -> SaveBundle {
    let mut b = bundle.clone();
    let old_source = b
        .dependencies
        .keys()
        .find(|p| p.ends_with(".source.json"))
        .unwrap()
        .clone();
    let old_manifest = b
        .dependencies
        .keys()
        .find(|p| p.contains("/manifests/"))
        .unwrap()
        .clone();
    let mut s: serde_json::Value =
        serde_json::from_slice(&b.dependencies.remove(&old_source).unwrap()).unwrap();
    edit(&mut s);
    let source = serde_json::to_vec(&s).unwrap();
    let source_hash = digest(&source);
    let root = old_source.split("/blobs/").next().unwrap();
    let new_source = format!("{root}/blobs/{source_hash}.source.json");
    let mut lines = String::from_utf8(b.primary.clone())
        .unwrap()
        .split('\n')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    lines[1] = format!("#DRAFT_SOURCE_MANIFEST_V1 {source_hash}");
    b.primary = lines.join("\n").into_bytes();
    let primary_hash = digest(&b.primary);
    let mut m: serde_json::Value =
        serde_json::from_slice(&b.dependencies.remove(&old_manifest).unwrap()).unwrap();
    m["primarySha256"] = json!(primary_hash);
    m["sourceDocumentRelativePath"] = json!(new_source);
    m["sourceDocumentSha256"] = json!(source_hash);
    m["sourceDocumentByteLength"] = json!(source.len().to_string());
    m["dependencies"]
        .as_object_mut()
        .unwrap()
        .remove(&old_source);
    m["dependencies"][&new_source] = json!({"relativePath":new_source,"sha256":source_hash,"byteLength":source.len().to_string(),"role":"context-resource"});
    b.dependencies.insert(new_source, source);
    b.dependencies.insert(
        format!("{root}/manifests/{primary_hash}.json"),
        serde_json::to_vec(&m).unwrap(),
    );
    b
}
#[test]
fn self_consistent_hashes_do_not_admit_missing_roots_or_wrong_active_state() {
    let mut e = EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "closed source",
        Ids::new([10; 16]),
        Limits::default(),
    )
    .unwrap();
    let b = encode(&e.capture().unwrap(), "schema.vesta").unwrap();
    let missing = rewrite_source(&b, |s| {
        s["tree"]["members"]
            .as_object_mut()
            .unwrap()
            .remove("molecules");
    });
    assert!(decode(&missing.primary, &missing.dependencies, Ids::new([11; 16])).is_err());
    let altered = rewrite_source(&b, |s| {
        s["tree"]["members"]["phaseGraph"]["members"]["phases"]["items"][0]["members"]["structure"]
            ["members"]["cell"]["members"]["lengthA"]["items"][0] =
            json!({"kind":"F64Bits","ieeeHexBE":format!("{:016x}",9f64.to_bits())});
    });
    assert!(decode(&altered.primary, &altered.dependencies, Ids::new([12; 16])).is_err());
    let unknown = rewrite_source(&b, |s| {
        s["tree"]["members"]["metadata"]["items"][0]["members"]["tree"]["members"]["members"]["members"]
            ["futurePersistent"] =
            json!({"kind":"Map","members":{"kind":{"kind":"Utf8","value":"Null"}}});
    });
    assert!(decode(&unknown.primary, &unknown.dependencies, Ids::new([13; 16])).is_err());

    e.attach_resource("authored-chart-lookalike", b"unqualified chart".to_vec())
        .unwrap();
    let bundle = encode(&e.capture().unwrap(), "projection.vesta").unwrap();
    let mut candidate = decode(&bundle.primary, &bundle.dependencies, Ids::new([14; 16]))
        .unwrap()
        .capture;
    let mut source = candidate.source.as_ref().unwrap().to_value().unwrap();
    let phase = &candidate.content.phases[0];
    let resource = &candidate.content.resources[0];
    let projection = json!({"kind":"TaggedVariant","tag":"GSASSpinProjection1","payload":{
        "kind":"Map","members":{
            "schemaVersion":{"kind":"Utf8","value":"FC-GSAS-spin-projection-1"},
            "captureRole":{"kind":"Utf8","value":"actual-current-qualified-spin-projection"},
            "owner":{"kind":"EntityRef","objectKind":"phase","objectId":phase.id},
            "chartResource":{"kind":"BytesRef","resourceId":resource.id,
                "sha256":resource.sha256,"byteLength":resource.byte_length},
            "sourceFlagF32Bits":{"kind":"Array","items":vec![json!({"kind":"Utf8","value":"3f800000"});12]},
            "operationAssignments":{"kind":"Array","items":[{"kind":"Map","members":{
                "operation":{"kind":"EntityRef","objectKind":"symmetry-operation","objectId":phase.symmetry_op_id},
                "timeSign":{"kind":"I64","value":"1"}
            }}]},
            "ruleId":{"kind":"Utf8","value":"draft.gsas-original-r1253-spin-chart"},
            "ruleVersion":{"kind":"Utf8","value":"1"}
        }
    }});
    let owner = source["identity"]["documentId"].clone();
    source["metadata"].as_array_mut().unwrap().push(json!({
        "owner":{"kind":"document","id":owner},"producerId":"authored-test","producerVersion":"1",
        "formatRef":"VM18-I14","dialect":"projection-lookalike","spans":[],
        "tree":{"kind":"Map","members":{"nestedProjection":projection}}
    }));
    candidate.source = Some(source7::SourceDocument::decode(source).unwrap());
    candidate.state_digest = Capture::digest_state(&candidate.content, candidate.source.as_ref()).unwrap();
    let error = encode(&candidate, "projection.vesta").unwrap_err();
    assert_eq!(error.code, "E_REOPEN_COVERAGE");
    assert_eq!(error.message, "GSAS current projection requires a qualified original chart provider");
}

#[test]
fn envelope_schema_rejects_self_consistent_bad_structure_and_extra_dependency() {
    let e = EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "schema negatives",
        Ids::new([44; 16]),
        Limits::default(),
    )
    .unwrap();
    let b = encode(&e.capture().unwrap(), "negative.vesta").unwrap();
    for pointer in ["not/a/pointer", "/~2"] {
        let bad = rewrite_source(&b, |s| {
            s["identitySlots"][0]["typedSourcePointer"] = json!(pointer)
        });
        assert!(decode(&bad.primary, &bad.dependencies, Ids::new([45; 16])).is_err());
    }
    let mut extra = b.clone();
    extra.dependencies.insert(
        "negative.vesta.resources/blobs/unreferenced.raw.bin".into(),
        b"unreferenced".to_vec(),
    );
    assert!(decode(&extra.primary, &extra.dependencies, Ids::new([46; 16])).is_err());
    let unknown = rewrite_source(&b, |s| {
        s["sourceTypeDigest"] = json!("0".repeat(64));
        s["sourceRuleSetDigest"] = json!("0".repeat(64));
    });
    assert!(decode(&unknown.primary, &unknown.dependencies, Ids::new([47; 16])).is_err());
}

#[test]
fn native_manifest_roles_must_match_resource_uses_even_with_exact_bytes() {
    let mut engine = EngineV2::new(
        Cell3 { length_a: [4.; 3], angles_deg: [90.; 3] },
        "dependency roles",
        Ids::new([40; 16]),
        Limits::default(),
    ).unwrap();
    engine.attach_resource("original/input", b"source evidence".to_vec()).unwrap();
    let bundle = encode(&engine.capture().unwrap(), "roles.vesta").unwrap();
    let manifest_path = format!("roles.vesta.resources/manifests/{}.json", digest(&bundle.primary));
    let manifest: serde_json::Value = serde_json::from_slice(&bundle.dependencies[&manifest_path]).unwrap();
    let source_path = manifest["sourceDocumentRelativePath"].as_str().unwrap();
    let raw_path = manifest["dependencies"].as_object().unwrap().keys()
        .find(|path| path.ends_with(".raw.bin")).unwrap();
    for (path, role) in [
        (source_path, "source-resource"),
        (raw_path.as_str(), "context-resource"),
        (raw_path.as_str(), "native-projection-resource"),
    ] {
        let mut changed = manifest.clone();
        changed["dependencies"][path]["role"] = json!(role);
        let mut deps = bundle.dependencies.clone();
        deps.insert(manifest_path.clone(), serde_json::to_vec(&changed).unwrap());
        let err = decode(&bundle.primary, &deps, Ids::new([41; 16])).err().unwrap();
        assert!(err.message.contains("artifact dependency role differs"), "{err:?}");
    }
}

#[test]
fn archived_early_core_migrates_under_its_exact_version_and_reopens() {
    const PRIMARY: &[u8] = include_bytes!("fixtures/early-core/first.vesta");
    const SOURCE: &[u8] = include_bytes!("fixtures/early-core/first.vesta.resources/blobs/9a0fff86e35822d518be469abe739406f3176828b17e6526411e6d33b7040db5.source.json");
    const MANIFEST: &[u8] = include_bytes!("fixtures/early-core/first.vesta.resources/manifests/1da6b9c2248cd6815e50ea3248d2004eb0a0a39ce434ce114636c60d00776727.json");
    let manifest: serde_json::Value = serde_json::from_slice(MANIFEST).unwrap();
    let source_path = manifest["sourceDocumentRelativePath"].as_str().unwrap();
    let manifest_path = format!("first.vesta.resources/manifests/{}.json", digest(PRIMARY));
    let deps = std::collections::BTreeMap::from([
        (source_path.into(), SOURCE.to_vec()), (manifest_path.clone(), MANIFEST.to_vec()),
    ]);
    let original: serde_json::Value = serde_json::from_slice(SOURCE).unwrap();
    let loaded = decode(PRIMARY, &deps, Ids::new([42; 16])).unwrap();
    assert_eq!(loaded.content.phases[0].cell.length_a, [6., 4., 4.]);
    assert_eq!(loaded.content.phases[0].sites[0].frac, [0.25, 0., 0.5]);
    assert_eq!(loaded.content.phases[0].label, "晶体首链");
    assert_ne!(loaded.document.document_id.as_str(), original["capturedSnapshot"]["documentId"].as_str().unwrap());
    assert_eq!(loaded.resources[&digest(SOURCE)], SOURCE);
    let bundle = encode(&loaded.capture, "migrated.vesta").unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([43; 16])).unwrap();
    assert_eq!(reopened.content.phases[0].cell, loaded.content.phases[0].cell);
    assert_eq!(reopened.resources, loaded.resources);
    assert_ne!(reopened.content.phases[0].id, loaded.content.phases[0].id);
    let mut wrong = manifest.clone();
    wrong["sourceDTOSchemaDigest"] = json!("0".repeat(64));
    let mut bad = deps.clone();
    bad.insert(manifest_path.clone(), serde_json::to_vec(&wrong).unwrap());
    assert!(decode(PRIMARY, &bad, Ids::new([44; 16])).is_err());
    let mut role = manifest.clone();
    role["dependencies"][source_path]["role"] = json!("source-resource");
    let mut bad = deps.clone();
    bad.insert(manifest_path, serde_json::to_vec(&role).unwrap());
    assert!(decode(PRIMARY, &bad, Ids::new([45; 16])).is_err());
    let mut missing = deps;
    missing.remove(source_path);
    assert!(decode(PRIMARY, &missing, Ids::new([46; 16])).is_err());
}

#[test]
fn retired_full_keeps_opaque_history_while_rebinding_version_and_fresh_owner() {
    use std::{collections::BTreeMap, path::Path};

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/retired-full/feff");
    let primary = std::fs::read(root.join("feff.vesta")).unwrap();
    let manifest_name = format!("feff.vesta.resources/manifests/{}.json", digest(&primary));
    let manifest_bytes = std::fs::read(root.join(&manifest_name)).unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
    let mut dependencies = BTreeMap::from([(manifest_name, manifest_bytes)]);
    for relative in manifest["dependencies"].as_object().unwrap().keys() {
        dependencies.insert(relative.clone(), std::fs::read(root.join(relative)).unwrap());
    }
    let bundle = SaveBundle { primary, dependencies };

    let wrong_footprint = rewrite_source(&bundle, |saved| {
        saved["tree"]["members"]["opaqueBlocks"]["items"][0]["members"]["editDependencies"]
            ["members"]["baseline"]["members"]["resourceGraphDigest"]["value"] =
            json!("0".repeat(64));
    });
    assert_eq!(
        decode(&wrong_footprint.primary, &wrong_footprint.dependencies, Ids::new([197;16]))
            .unwrap_err().code,
        "E_UNSAFE_SOURCE_REWRITE"
    );

    let migrated = decode(&bundle.primary, &bundle.dependencies, Ids::new([198;16])).unwrap();
    let after = encode_verified(&migrated.capture, "new-feff.vesta", Ids::new([199;16])).unwrap();
    let reopened = decode(&after.primary, &after.dependencies, Ids::new([200;16])).unwrap();
    let read = |capture: &Capture| capture.source.as_ref().unwrap().to_value().unwrap();
    let old_lifetime = read(&migrated.capture);
    let new_lifetime = read(&reopened.capture);
    for source in [&old_lifetime, &new_lifetime] {
        assert_eq!(source["scenes"][0]["render"], json!({"kind":"Null"}));
    }
    let before_tail = &old_lifetime["opaqueBlocks"][0];
    let tail = &new_lifetime["opaqueBlocks"][0];
    let baseline = &tail["editDependencies"]["baseline"];
    assert_ne!(before_tail["id"], tail["id"]);
    assert_eq!(before_tail["editDependencies"]["baseline"], *baseline);
    assert_eq!(baseline["sourceTypeDigest"], new_lifetime["identity"]["sourceTypeDigest"]);
    assert_eq!(baseline["policyDigest"], new_lifetime["identity"]["policyDigest"]);
    assert_ne!(baseline["policyDigest"], manifest["sourceRuleSetDigest"]);
    assert_eq!(baseline["persistentSourceDigest"],
        "f663f1c4ca1e35885705dbd6b525b3b08f854c3d25da3645a8d1661ed4ea627a");
    assert_eq!(baseline["referenceNamespaceDigest"],
        "d507d7e3220550c2bb20e838bdbb74493c830d184d5f59d47b5134244c0a22c2");
    assert_eq!(baseline["resourceGraphDigest"],
        "2110f5e84c45688f0c8c6d6f780c4f1493793a38e2b034a6bb5932d707db10ec");
    let extent = &tail["extent"];
    let owner = reopened.content.resources.iter().find(|res|
        extent["sourceResourceId"].as_str() == Some(res.id.as_str())).unwrap();
    assert_ne!(before_tail["extent"]["sourceResourceId"], extent["sourceResourceId"]);
    assert_eq!(owner.sha256, "2ede634fa6a972344eec197aaeecd79497146d3110abcbc87039f1763d1bc09e");
    assert_eq!(&reopened.resources[&owner.sha256][146..171], b"load after-child-end.inc\n");
    assert_eq!(extent["sha256"], digest(b"load after-child-end.inc\n"));
}
