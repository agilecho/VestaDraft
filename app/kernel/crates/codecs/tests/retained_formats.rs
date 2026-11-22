use serde_json::Value;
use std::path::PathBuf;
use vesta_codecs::formats::fdat_source::{FdatLayout, open_selected_fdat};
use vesta_codecs::{decode, encode_verified};
use vesta_model::{Capture, Ids};

fn fixtures(path: &str) -> Vec<u8> {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap()
        .to_owned();
    std::fs::read(repository.join(path)).unwrap()
}

fn save_and_reopen(capture: &Capture, name: &str) -> Capture {
    let bundle = encode_verified(capture, name, Ids::new([177; 16])).unwrap();
    decode(&bundle.primary, &bundle.dependencies, Ids::new([178; 16]))
        .unwrap()
        .capture
}

fn tagged(capture: &Capture, tag: &str) -> Value {
    let value = capture.source.as_ref().unwrap().to_value().unwrap();
    value["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["tree"]["tag"] == tag)
        .unwrap()
        .clone()
}

#[test]
fn fdat_aabhtz_real_nonempty_table_preserves_both_observation_modes() {
    let raw = fixtures(
        "output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/FDAT/C68H23O68N68Cl99.fdt",
    );
    for (layout, expected) in [
        (FdatLayout::Uninterpreted, "source-uninterpreted"),
        (FdatLayout::Published, "source-parsed"),
    ] {
        let in_memory = open_selected_fdat(&raw, layout, Ids::new([176; 16]))
            .unwrap()
            .capture;
        let after = save_and_reopen(&in_memory, "fdat.vesta");
        assert!(after.content.phases.is_empty());
        assert_ne!(
            after.content.resources[0].id,
            in_memory.content.resources[0].id
        );
        assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
        let row = tagged(&after, "FCON1");
        let span = &row["spans"][1];
        let at: usize = span["byteOffset"].as_str().unwrap().parse().unwrap();
        let len: usize = span["byteLength"].as_str().unwrap().parse().unwrap();
        assert_eq!(&raw[at..at + len], &raw[raw.len() - 78..raw.len() - 2]);
        let body = &row["tree"]["payload"]["members"];
        assert_eq!(body["NAT"]["value"], "35");
        assert_eq!(body["NCON"]["value"], "37");
        let obs = &body["observation"]["members"];
        assert_eq!(obs["kind"]["value"], expected);
        if matches!(layout, FdatLayout::Published) {
            assert_eq!(obs["sourceIntegerTokens"]["items"][0]["value"], " 3");
            let pairs = obs["directedSourcePairs"]["items"].as_array().unwrap();
            assert_eq!(pairs.last().unwrap()["items"][0]["value"], "15");
            assert_eq!(pairs.last().unwrap()["items"][1]["value"], "16");
        }
    }
}

#[test]
fn mxd_file06_all_groups_reopen_from_exact_report_bytes() {
    let raw = fixtures("output/vesta-p2-03/20261121-c8-retained-formats/fixtures/ortho.file06");
    let open = vesta_codecs::formats::mxd_report::open(&raw, Ids::new([179; 16])).unwrap();
    let after = save_and_reopen(&open.capture, "mxd.vesta");
    let source = after.source.unwrap().to_value().unwrap();
    let reports = source["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["tree"]["tag"] == "MXDReportBlockObservations1")
        .collect::<Vec<_>>();
    assert_eq!(reports.len(), 2);
    let groups = &reports[0]["tree"]["payload"]["members"]["sites"]["items"];
    assert_eq!(groups.as_array().unwrap().len(), 5);
    assert_eq!(groups[0]["members"]["sourceLabel4"]["value"], "ZN  ");
    assert_eq!(
        groups[0]["members"]["axes"]["items"][0]["members"]["meanToken"]["value"],
        "-1.2345"
    );
    assert_eq!(groups[4]["members"]["sourceOrdinal"]["value"], "4");
    assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
    assert!(after.content.phases.is_empty());
    let inspected = vesta_codecs::formats::mxd_report::inspect(&after).unwrap().unwrap();
    assert_eq!(inspected["reports"][0]["groups"][1]["firstSiteOrdinal"], 4);
    let span = &inspected["reports"][0]["groups"][1]["axes"][2]["span"];
    let start: usize = span["byteOffset"].as_str().unwrap().parse().unwrap();
    let length: usize = span["byteLength"].as_str().unwrap().parse().unwrap();
    assert_eq!(span["sourceResourceId"], after.content.resources[0].id.as_str());
    assert_eq!(vesta_model::digest(&raw[start..start + length]), span["sha256"]);
    assert_eq!(inspected["reports"][0]["observation"]["sites"][4]["axes"][2]["axis"], "z");
    let mut misplaced = open.capture.clone();
    let mut doc = misplaced.source.as_ref().unwrap().to_value().unwrap();
    let row = &mut doc["metadata"][0];
    row["tree"] = serde_json::json!({"kind":"Map","members":{"nested":row["tree"]}});
    row["dialect"] = serde_json::json!("unqualified-placement-control");
    misplaced.source = Some(vesta_model::source7::SourceDocument::decode(doc).unwrap());
    misplaced.state_digest = Capture::digest_state(&misplaced.content, misplaced.source.as_ref()).unwrap();
    let wrong = encode_verified(&misplaced, "wrong-mxd-location.vesta", Ids::new([175; 16])).unwrap_err();
    assert_eq!(wrong.message, "MXD COORDN observations must be top-level owned report blocks");
    let mut copied_resource = open.capture.clone();
    let mut doc = copied_resource.source.as_ref().unwrap().to_value().unwrap();
    let mut other = copied_resource.content.resources[0].clone();
    other.id = Ids::new([165; 16]).next_id().into();
    other.logical_key = "test/duplicate-report".into();
    let mut typed_resource = doc["resources"][0].clone();
    typed_resource["id"] = serde_json::json!(other.id);
    doc["resources"].as_array_mut().unwrap().push(typed_resource);
    doc["metadata"][0]["tree"]["payload"]["members"]["producerContext"]["members"]["originalReport"]
        ["resourceId"] = serde_json::json!(other.id);
    copied_resource.content.resources.push(other);
    copied_resource.source = Some(vesta_model::source7::SourceDocument::decode(doc).unwrap());
    copied_resource.state_digest =
        Capture::digest_state(&copied_resource.content, copied_resource.source.as_ref()).unwrap();
    let wrong = encode_verified(&copied_resource, "wrong-mxd-resource.vesta", Ids::new([166; 16])).unwrap_err();
    assert_eq!(wrong.message, "MXD COORDN source blocks or spans differ");
    let mut corrupt = raw;
    let at = corrupt.windows(7).position(|w| w == b"-1.2345").unwrap();
    corrupt[at] = b'*';
    assert!(vesta_codecs::formats::mxd_report::open(&corrupt, Ids::new([180; 16])).is_err());
}

#[test]
fn vics_retains_exact_legacy_archive_and_installs_current_display() {
    use vesta_codecs::formats::live_state::{
        NativeStateInput, inspect_native_state, open_native_state,
    };
    let input = fixtures("output/vesta-p2-03/20261121-c8-retained-formats/fixtures/legacy.vstate");
    let state = NativeStateInput::parse(&input).unwrap();
    let raw = fixtures(
        "output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/inputs/author-VICS-CaTiO3.vcs",
    );
    let c = open_native_state(state, None, |_| Ok(raw.clone()), Ids::new([181; 16]))
        .unwrap()
        .capture;
    let c = save_and_reopen(&c, "vics.vesta");
    let observed = inspect_native_state(&c).unwrap().unwrap();
    let display = &observed["legacyDisplays"][0];
    assert_eq!(display["sceneSlots"]["0x270"], 50);
    assert_eq!(
        display["sceneSlots"]["0x3fc:3"],
        serde_json::json!([0, 77, 0])
    );
    assert_eq!(
        display["lights"][0]["rgba"][2],
        serde_json::json!([255, 255, 255, 255])
    );
    assert_eq!(display["sceneSlots"]["0x278"]["ieeeHexBE"], "80000000");
    assert_eq!(c.resources[&c.content.resources[0].sha256], raw);
    let source = c.source.as_ref().unwrap().to_value().unwrap();
    let history = source["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["dialect"] == "vics-legacy-archive-1")
        .unwrap();
    let last = history["spans"].as_array().unwrap().last().unwrap();
    let at: usize = last["byteOffset"].as_str().unwrap().parse().unwrap();
    assert!(raw[at..].starts_with(b"DELOB\r\n"));
}

#[test]
fn vics_explicit_current_words_and_old_source_view_survive_separately() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{
        NativeStateInput, inspect_native_state, open_legacy_vics, open_native_state,
    };
    let raw = fixtures(
        "output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs",
    );
    let historical = open_legacy_vics(&raw, Ids::new([171; 16])).unwrap().capture;
    let archive = inspect_native_state(&historical).unwrap().unwrap();
    assert_eq!(archive["legacySourceViews"][0]["settings4"][2]["ieeeHexBE"], "c00b851f");
    assert_eq!(archive["legacyViews"], json!([]));

    let mut manifest: Value = serde_json::from_slice(&fixtures(
        "output/vesta-p2-03/20261121-c8-retained-formats/fixtures/legacy.vstate",
    )).unwrap();
    manifest["legacyDisplay"]["currentValues"] = json!(true);
    manifest["legacyDisplay"]["sceneSlots"]["0x270"] = json!(91);
    let make = |wire: &Value| NativeStateInput::parse(&serde_json::to_vec(wire).unwrap()).unwrap();
    assert!(open_native_state(make(&manifest), None, |_| Ok(raw.clone()), Ids::new([172; 16])).is_err());
    manifest["legacyView"] = json!({"matrixBits":std::array::from_fn::<_, 16, _>(|i|
        if i % 5 == 0 { "3ff0000000000000" } else { "0000000000000000" }),
        "settingsBits":["80000000","00000000","00000000","3fa00000"]});
    let first = open_native_state(make(&manifest), None, |_| Ok(raw.clone()), Ids::new([173;16])).unwrap().capture;
    let after = save_and_reopen(&first, "separate-legacy-view.vesta");
    let observed = inspect_native_state(&after).unwrap().unwrap();
    assert_eq!(observed["legacySourceViews"][0]["settings4"][2]["ieeeHexBE"], "c00b851f");
    assert_eq!(observed["legacyViews"][0]["settings4"][0]["ieeeHexBE"], "80000000");
    assert_eq!(observed["legacyViews"][0]["settings4"][3]["ieeeHexBE"], "3fa00000");
    assert_eq!(observed["legacyDisplays"][0]["sceneSlots"]["0x270"], 91);
    assert_eq!(observed["legacyViews"][0]["scene"]["objectId"], after.content.scene_id.as_str());
    assert_ne!(observed["legacyViews"][0]["scene"]["objectId"], first.content.scene_id.as_str());
    let mut wrong = after;
    let mut doc = wrong.source.as_ref().unwrap().to_value().unwrap();
    doc["scenes"][0]["cameraRaw"] = json!({"kind":"Null"});
    wrong.source = Some(vesta_model::source7::SourceDocument::decode(doc).unwrap());
    wrong.state_digest = Capture::digest_state(&wrong.content, wrong.source.as_ref()).unwrap();
    let error = encode_verified(&wrong, "missing-current-view.vesta", Ids::new([174;16])).unwrap_err();
    assert_eq!(error.message, "VICS current display has no matching current scene view");
}

#[test]
fn nvis_native_phase_preserves_original_lists_and_independent_current_atom_flags() {
    use vesta_codecs::formats::{
        live_state::{NativeStateInput, inspect_native_state, open_native_state},
        resource_graph::{CaptureLimits, GraphProfile, discover_graph_with_profile},
    };
    use vesta_codecs::open_selected_orffe;
    let root = "app/kernel/crates/codecs/tests/fixtures/r80";
    let graph = discover_graph_with_profile(
        "orffe-reopen.vesta",
        GraphProfile::NativeVesta,
        |relative, _| Ok(Some(fixtures(&format!("{root}/{relative}")))),
        CaptureLimits::default(),
    )
    .unwrap();
    let base = open_selected_orffe(&graph, Ids::new([182; 16]))
        .unwrap()
        .capture;
    let spec = NativeStateInput::parse(
        br#"{"schemaVersion":"VestaDraft-native-current-state-1","grids":[],
        "nativeFlags":{"labeledSiteIndices":[0],"disabledSiteIndices":[1]}}"#,
    )
    .unwrap();
    let c = open_native_state(
        spec,
        Some(base),
        |_| panic!("NVIS original is the verified native graph"),
        Ids::new([183; 16]),
    )
    .unwrap()
    .capture;
    let c = save_and_reopen(&c, "flags.vesta");
    assert_eq!(c.content.hidden.len(), 1);
    let core = &c.content.phases[0];
    assert_eq!(c.content.hidden[0].site, core.sites[1].id);
    let source = tagged(&c, "NVIS1");
    assert_eq!(source["spans"].as_array().unwrap().len(), 5);
    let now = inspect_native_state(&c).unwrap().unwrap();
    let flags = &now["nativeFlags"][0];
    assert_eq!(
        flags["currentFlagSets"][0]["resolution"]["selections"][0]["target"]["image"]["site"]["objectId"],
        core.sites[0].id.as_str()
    );
    assert_eq!(flags["observations"][0]["sourceTokens"][0], "-1");
}

#[test]
fn nvis_reopens_nonempty_face_bond_and_polyhedron_selections() {
    use vesta_codecs::formats::{
        live_state::{NativeStateInput, inspect_native_state, open_native_state},
        resource_graph::{CaptureLimits, GraphProfile, discover_graph_with_profile},
    };
    let root = "app/kernel/crates/codecs/tests/fixtures/r80";
    let graph = discover_graph_with_profile(
        "orffe-reopen.vesta",
        GraphProfile::NativeVesta,
        |relative, _| Ok(Some(fixtures(&format!("{root}/{relative}")))),
        CaptureLimits::default(),
    )
    .unwrap();
    let base = vesta_codecs::open_selected_orffe(&graph, Ids::new([184; 16]))
        .unwrap()
        .capture;
    let input = NativeStateInput::parse(
        br#"{
        "schemaVersion":"VestaDraft-native-current-state-1","grids":[],
        "nativeFlags":{"labeledSiteIndices":[0],"disabledSiteIndices":[1],
            "selectedObjects":{"morphologyBoxA":[[-2,-2,-2],[2,2,2]],"labeledFaceIndices":[5],
                               "disabledBondIndices":[0,5],"disabledPolyhedronIndices":[0]}}
    }"#,
    )
    .unwrap();
    let capture = open_native_state(
        input,
        Some(base),
        |_| panic!("owned input"),
        Ids::new([185; 16]),
    )
    .unwrap()
    .capture;
    let before = inspect_native_state(&capture).unwrap().unwrap();
    let reopened = save_and_reopen(&capture, "object-flags.vesta");
    let after = inspect_native_state(&reopened).unwrap().unwrap();
    let slots = &after["nativeFlags"][0]["currentFlagSets"];
    let candidate = slots[1]["resolution"]["selections"][0]["target"].clone();
    assert_eq!(candidate["kind"], "morphology-face");
    let doc = reopened.source.as_ref().unwrap().to_value().unwrap();
    assert_eq!(
        candidate["halfspaceId"]["objectId"],
        doc["shapes"][0]["value"]["halfspaces"][5]["id"]
    );
    assert_ne!(
        before["nativeFlags"][0]["currentFlagSets"][1]["resolution"]["selections"][0]["target"]["shapeId"],
        candidate["shapeId"]
    );
    let bonds = slots[3]["resolution"]["selections"].as_array().unwrap();
    assert_eq!(bonds.len(), 2);
    for row in bonds {
        assert_eq!(
            row["target"]["ruleId"]["objectId"],
            doc["bondRules"][0]["id"]
        );
    }
    let poly = &slots[4]["resolution"]["selections"][0]["target"];
    assert_eq!(
        poly["boundaryRuleId"]["objectId"],
        doc["boundaries"][0]["id"]
    );
    assert_eq!(poly["vertices"].as_array().unwrap().len(), 6);
    assert_eq!(
        slots[4]["resolution"]["selections"][0]["applicability"],
        "active"
    );
    use serde_json::json;
    use vesta_commands::{EngineV2, Limits};
    use vesta_model::RestoredProject;
    let transition = |capture: Capture, min, max| {
        let project = RestoredProject::new(capture, Ids::new([167; 16])).unwrap();
        let mut engine = EngineV2::loaded(project, Ids::new([168; 16]), Limits::default()).unwrap();
        let phase = engine.content().phases[0].id.clone();
        engine.execute(&engine.document(), &json!({"command":"boundary.set",
            "params":{"phaseId":phase,"min":min,"max":max}})).unwrap();
        save_and_reopen(&engine.capture().unwrap(), "nvis-boundary-edit.vesta")
    };
    let shrunk = transition(reopened, [0.4, 0., 0.], [1.; 3]);
    let checked = inspect_native_state(&shrunk).unwrap().unwrap();
    let sets = &checked["nativeFlags"][0]["currentFlagSets"];
    assert_eq!(sets[3]["resolution"]["selections"][0]["applicability"], "latent");
    assert_eq!(sets[3]["resolution"]["selections"][1]["applicability"], "active");
    assert_eq!(sets[4]["resolution"]["selections"][0]["applicability"], "latent");
    let shrunk_source = shrunk.source.as_ref().unwrap().to_value().unwrap();
    assert_eq!(shrunk_source["boundaries"][0]["value"]["minimumFractional"], json!([0.4, 0., 0.]));
    assert_eq!(sets[4]["resolution"]["selections"][0]["target"]["boundaryRuleId"]["objectId"],
        shrunk_source["boundaries"][0]["id"]);
    let expanded = transition(shrunk, [0.; 3], [1.; 3]);
    let checked = inspect_native_state(&expanded).unwrap().unwrap();
    let sets = &checked["nativeFlags"][0]["currentFlagSets"];
    assert_eq!(sets[3]["resolution"]["selections"][0]["applicability"], "active");
    assert_eq!(sets[4]["resolution"]["selections"][0]["applicability"], "active");
}

#[test]
fn vics_original_source_builds_a_historical_phase_and_bound_observations() {
    use vesta_codecs::formats::live_state::open_legacy_vics;
    let raw = fixtures(
        "output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/inputs/author-VICS-CaTiO3.vcs",
    );
    let project = open_legacy_vics(&raw, Ids::new([186; 16])).unwrap().capture;
    let reopened = save_and_reopen(&project, "original-vics.vesta");
    let doc = reopened.source.as_ref().unwrap().to_value().unwrap();
    let original = project.source.as_ref().unwrap().to_value().unwrap();
    let phase = &doc["phaseGraph"]["phases"][0];
    assert_ne!(phase["id"], original["phaseGraph"]["root"]);
    assert_eq!(phase["structure"]["sites"].as_array().unwrap().len(), 4);
    assert_eq!(
        phase["structure"]["group"]["operators"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    assert_eq!(doc["boundaries"][0]["owner"]["id"], phase["id"]);
    let archive = doc["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["dialect"] == "vics-historical-phase-1")
        .unwrap();
    assert_eq!(archive["owner"]["id"], phase["id"]);
    assert_eq!(
        archive["tree"]["members"]["originalBondRecipes"]["items"][0]["members"]["fieldCount"]["value"],
        "7"
    );
    assert!(reopened.content.phases.is_empty());
    assert_eq!(
        reopened.resources[&reopened.content.resources[0].sha256],
        raw
    );
}

#[test]
fn published_mxd_reports_without_coordn_do_not_acquire_report_blocks() {
    for file in ["l-j-nims.file06", "brucite-nims.file06"] {
        let raw = fixtures(&format!(
            "output/vesta-p2-03/20261121-c8-historical-and-public/public-inputs/{file}"
        ));
        let err = vesta_codecs::formats::mxd_report::open(&raw, Ids::new([187; 16])).unwrap_err();
        assert_eq!(err.code, "E_REOPEN_COVERAGE");
        assert!(format!("{err:?}").contains("no selected MXD COORDN block"));
    }
}

#[test]
fn nvis_nonperiodic_targets_stay_in_their_actual_frame_after_reopen() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{
        NativeStateInput, inspect_native_state, open_native_state,
    };
    let mut base = vesta_commands::EngineV2::new(
        vesta_model::Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "NVIS phase",
        Ids::new([188; 16]),
        vesta_commands::Limits::default(),
    )
    .unwrap()
    .capture()
    .unwrap();
    let seed = encode_verified(&base, "molecule-seed.vesta", Ids::new([189; 16])).unwrap();
    base = decode(&seed.primary, &seed.dependencies, Ids::new([190; 16]))
        .unwrap()
        .capture;
    let mut source = base.source.as_ref().unwrap().to_value().unwrap();
    let fixture:Value=serde_json::from_slice(&fixtures(
        "output/vesta-p2-03/20261121-c8-closure-execution/inputs/source7/rich-persistence.valid.json")).unwrap();
    let mut molecule = fixture["molecules"][0].clone();
    let frame = &mut molecule["frames"][0];
    let mut other = frame["atoms"][0].clone();
    frame["atoms"][0]["partialCharges"] = json!([]);
    other["partialCharges"] = json!([]);
    let id = "00000000000000000000000000000041";
    other["id"] = json!(id);
    other["cartesianA"] = json!([1.25, 0.0, 0.0]);
    let first = frame["atoms"][0]["id"].clone();
    frame["atoms"].as_array_mut().unwrap().push(other);
    frame["bonds"] = json!([{"id":"00000000000000000000000000000044",
        "atomIds":[first,id],"order":"single","stereo":{"kind":"Null"},
        "sourceMultiplicity":{"kind":"Null"},"metadata":[]}]);
    source["sourceEntryOrder"]
        .as_array_mut()
        .unwrap()
        .push(json!({"kind":"molecule","id":molecule["id"]}));
    source["molecules"] = json!([molecule]);
    base.source = Some(vesta_model::source7::SourceDocument::decode(source).unwrap());
    let state=NativeStateInput::parse(br#"{"schemaVersion":"VestaDraft-native-current-state-1","grids":[],
        "moleculeFlags":[{"moleculeIndex":0,"labeledAtoms":[[0,1]],"disabledAtoms":[],"disabledBonds":[[0,0]]}]}"#).unwrap();
    let live = open_native_state(
        state,
        Some(base),
        |_| panic!("no extra inputs"),
        Ids::new([191; 16]),
    )
    .unwrap()
    .capture;
    let result = save_and_reopen(&live, "new-molecule-flags.vesta");
    let image = inspect_native_state(&result).unwrap().unwrap();
    let doc = result.source.as_ref().unwrap().to_value().unwrap();
    let frame = &doc["molecules"][0]["frames"][0];
    let flags = &image["nativeFlags"][0]["currentFlagSets"];
    let atom = &flags[0]["resolution"]["selections"][0]["target"];
    let bond = &flags[3]["resolution"]["selections"][0]["target"];
    assert_eq!(atom["frameId"]["objectId"], frame["id"]);
    assert_eq!(atom["atomId"]["objectId"], frame["atoms"][1]["id"]);
    assert_eq!(bond["frameId"]["objectId"], frame["id"]);
    assert_eq!(bond["bondId"]["objectId"], frame["bonds"][0]["id"]);
}

fn dialect_row(capture: &Capture, dialect: &str) -> Value {
    let source = capture.source.as_ref().unwrap().to_value().unwrap();
    source["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["dialect"] == dialect)
        .unwrap()
        .clone()
}

#[test]
fn gsas_fixed_column_history_reopens_without_promoting_magnetic_ops() {
    use vesta_codecs::formats::gsas_exp;
    // Contract-shaped synthetic magnetic rows are a wire control only.
    let raw = fixtures("output/vesta-p2-03/20261121-c8-frozen-format-static/gsas-wire-control.exp");
    let opened = gsas_exp::open(&raw, Ids::new([191; 16])).unwrap().capture;
    let after = save_and_reopen(&opened, "gsas-source.vesta");
    let row = dialect_row(&after, "gsas-exp-observations-1");
    let body = &row["tree"]["members"];
    let phases = body["phases"]["items"].as_array().unwrap();
    assert_eq!(phases.len(), 2);
    assert_eq!(phases[0]["members"]["phaseKind"]["value"], "1");
    assert_eq!(phases[1]["members"]["phaseNumber"]["value"], "3");
    let words = &phases[1]["members"]["spinFlipWords"]["items"];
    assert_eq!(words.as_array().unwrap().len(), 12);
    assert_eq!(words[1]["value"], " -1.");
    assert_eq!(words[2]["value"], "  0.");
    assert_eq!(
        phases[1]["members"]["atoms"]["items"][0]["members"]["momentTokens"]["items"][0]["value"],
        "   3.50000"
    );
    assert!(after.content.phases.is_empty());
    assert_eq!(
        after.source.as_ref().unwrap().to_value().unwrap()["phaseGraph"],
        Value::Null
    );
    assert_ne!(
        opened.content.resources[0].id,
        after.content.resources[0].id
    );
    assert_eq!(
        body["rawPrimary"]["resourceId"],
        after.content.resources[0].id.as_str()
    );
    assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
}

#[test]
fn gsas_rejects_missing_wrong_phase_duplicate_and_changed_persisted_words() {
    use serde_json::json;
    use vesta_codecs::formats::gsas_exp;
    use vesta_model::source7::SourceDocument;
    let raw = fixtures("output/vesta-p2-03/20261121-c8-frozen-format-static/gsas-wire-control.exp");
    let text = std::str::from_utf8(&raw).unwrap();
    let spin = text
        .lines()
        .find(|row| row.starts_with("CRS3  SPNFLP"))
        .unwrap();
    let moment = text
        .lines()
        .find(|row| row.starts_with("CRS3  AT  1M"))
        .unwrap();
    for broken in [
        text.replace(spin, ""),
        text.replace("CRS3  SPNFLP", "CRS1  SPNFLP"),
        text.replace(spin, &format!("{spin}\n{spin}")),
        text.replace(moment, ""),
        text.replace("CRS3  AT  1M", "CRS3  AT  2M"),
    ] {
        assert_eq!(
            gsas_exp::open(broken.as_bytes(), Ids::new([192; 16]))
                .unwrap_err()
                .code,
            "E_REOPEN_COVERAGE"
        );
    }
    let mut unqualified = gsas_exp::open(&raw, Ids::new([196; 16])).unwrap().capture;
    let mut source = unqualified.source.as_ref().unwrap().to_value().unwrap();
    let original = fixtures("app/kernel/crates/model/tests/fixtures/source7/rich-persistence.valid.json");
    let current: Value = serde_json::from_slice(&original).unwrap();
    let mut phase = current["phaseGraph"]["phases"][0].clone();
    phase["fieldIds"] = json!([]);
    phase["shapeId"] = Value::Null;
    source["phaseGraph"] = json!({"root":phase["id"],"phases":[phase]});
    source["sourceEntryOrder"] = json!([{"kind":"phase","id":source["phaseGraph"]["root"]}]);
    let mut magnetic_group = source.clone();
    magnetic_group["phaseGraph"]["phases"][0]["structure"]["group"]["kind"] = json!("magnetic");
    source["vectors"] = current["vectors"].clone();
    for promoted in [magnetic_group, source] {
        unqualified.source = Some(SourceDocument::decode(promoted).unwrap());
        let rejected = encode_verified(
            &unqualified, "unqualified-gsas-group.vesta", Ids::new([197; 16]),
        ).unwrap_err();
        assert_eq!(rejected.code, "E_REOPEN_COVERAGE");
        assert_eq!(rejected.message, "GSAS current magnetic state requires a qualified original chart provider");
    }

    let mut capture = gsas_exp::open(&raw, Ids::new([193; 16])).unwrap().capture;
    let mut source = capture.source.as_ref().unwrap().to_value().unwrap();
    source["metadata"][0]["tree"]["members"]["phases"]["items"][1]["members"]["spinFlipWords"]["items"]
        [1]["value"] = Value::from("  1.");
    capture.source = Some(SourceDocument::decode(source).unwrap());
    assert!(encode_verified(&capture, "bad-gsas.vesta", Ids::new([194; 16])).is_err());
}

#[test]
fn vics_original_thermal_rows_follow_serial_and_label_without_a_current_adp() {
    use vesta_codecs::formats::live_state::open_legacy_vics;
    use vesta_model::source7::SourceDocument;
    for (path, count) in [
        (
            "output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/inputs/author-VICS-CaTiO3.vcs",
            4,
        ),
        (
            "output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs",
            0,
        ),
    ] {
        let raw = fixtures(path);
        let source = open_legacy_vics(&raw, Ids::new([195; 16])).unwrap().capture;
        let after = save_and_reopen(&source, "legacy-thermal.vesta");
        let row = dialect_row(&after, "vics-historical-phase-1");
        let records = row["tree"]["members"]["thermalSourceRows"]["items"]
            .as_array()
            .unwrap();
        assert_eq!(records.len(), count);
        if count == 4 {
            let site = &records[3]["members"];
            assert_eq!(site["sourceSerial"]["value"], "4");
            assert_eq!(site["sourceLabel"]["value"], "O2");
            assert_eq!(site["theriToken"]["value"], "1.000");
            assert_eq!(site["thermTokens"]["items"][5]["value"], "-0.00100");

            for (field, value) in [("theriToken", "2.000"), ("sourceLabel", "O1")] {
                let mut altered = after.clone();
                let mut typed = altered.source.as_ref().unwrap().to_value().unwrap();
                let history = typed["metadata"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|row| row["dialect"] == "vics-historical-phase-1")
                    .unwrap();
                history["tree"]["members"]["thermalSourceRows"]["items"][3]["members"][field]["value"] =
                    Value::from(value);
                altered.source = Some(SourceDocument::decode(typed).unwrap());
                let error =
                    encode_verified(&altered, "vics-altered-thermal.vesta", Ids::new([196; 16]))
                        .unwrap_err();
                assert_eq!(error.code, "E_REOPEN_COVERAGE");
                assert_eq!(
                    error.message,
                    "restored VICS historical phase, owners or source bytes differ"
                );
            }
        }
        assert!(after.content.phases.is_empty());
        assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
        let mut other = raw;
        other[0] = b'X';
        assert!(open_legacy_vics(&other, Ids::new([196; 16])).is_err());
    }
}

#[test]
fn anatase_preserves_source_flag_ordinals_and_rejects_false_current_resolution() {
    use vesta_codecs::formats::live_state::open_historical_anatase;
    use vesta_model::source7::SourceDocument;
    let raw = fixtures(
        "output/vesta-study/20261108-113622/s05/official-examples/VESTA-Examples/VESTA/Composite_data/Anatase.vesta",
    );
    let source = open_historical_anatase(&raw, Ids::new([197; 16]))
        .unwrap()
        .capture;
    let mut after = save_and_reopen(&source, "anatase-source.vesta");
    let row = dialect_row(&after, "vesta-anatase-source-flags-1");
    let entries = &row["tree"]["members"]["sourceLists"]["items"];
    assert_eq!(
        row["tree"]["members"]["sourceFormatHeader"]["value"],
        "#VESTA_FORMAT_VERSION 2.90.1"
    );
    assert_eq!(row["spans"].as_array().unwrap().len(), 8);
    assert_eq!(entries.as_array().unwrap().len(), 5);
    assert_eq!(
        entries[1]["members"]["sourceTokens"]["items"][1]["value"],
        "8"
    );
    assert_eq!(entries[1]["members"]["resolution"]["value"], "source-only");
    assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
    assert!(after.content.phases.is_empty() && after.content.hidden.is_empty());
    let mut doc = after.source.as_ref().unwrap().to_value().unwrap();
    let row = doc["metadata"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|row| row["dialect"] == "vesta-anatase-source-flags-1")
        .unwrap();
    row["tree"]["members"]["sourceLists"]["items"][1]["members"]["resolution"]["value"] =
        Value::from("qualified");
    after.source = Some(SourceDocument::decode(doc).unwrap());
    assert!(encode_verified(&after, "false-current-flags.vesta", Ids::new([198; 16])).is_err());
}

#[test]
fn mxd_tricl_crlf_roundtrip_and_rejects_conflicting_or_malformed_header() {
    use vesta_codecs::formats::mxd_report;
    let raw = fixtures("output/vesta-p2-03/20261121-c8-retained-formats/fixtures/tricl.file06");
    let before = mxd_report::open(&raw, Ids::new([199; 16])).unwrap().capture;
    let after = save_and_reopen(&before, "mxd-tricl.vesta");
    let row = tagged(&after, "MXDReportBlockObservations1");
    assert_eq!(
        row["tree"]["payload"]["members"]["producerContext"]["members"]["wireDialect"]["value"],
        "tricl-20251119"
    );
    assert_eq!(after.resources[&after.content.resources[0].sha256], raw);
    let text = std::str::from_utf8(&raw).unwrap();
    let no_tail_padding = text.replace("101.00000    \r\n", "101.00000\r\n");
    assert_ne!(no_tail_padding, text);
    mxd_report::open(no_tail_padding.as_bytes(), Ids::new([200; 16])).unwrap();

    let (header, body) = text.split_once("\r\n\r\n").unwrap();
    let mut axes = body.split_inclusive("\r\n");
    let x = axes.next().unwrap();
    let y = axes.next().unwrap();
    let z = axes.next().unwrap().strip_suffix("\r\n").unwrap();
    assert!(x.starts_with("    ZN  "));
    assert_eq!((x.len(), y.len(), z.len()), (130, 130, 128));
    let without_z = format!("{header}\r\n\r\n{x}{y}");
    let short_z = format!("{without_z}{}\r\n", &z[..z.len() - 1]);
    let following_groups = &body[x.len() + y.len() + z.len() + "\r\n".len()..];
    let next_x = following_groups.split_once("\r\n").unwrap().0;
    assert!(next_x.starts_with("    N1  "));
    assert_eq!(next_x.len(), 32);
    // Keep the next group so the missing middle row reaches the width check.
    let missing_z_before_next_group = format!("{without_z}{following_groups}");
    for (truncated, expected_message) in [
        (without_z, "truncated MXD COORDN axes"),
        (
            short_z,
            "MXD COORDN group requires three equal fixed-width axes",
        ),
        (
            missing_z_before_next_group,
            "MXD COORDN group requires three equal fixed-width axes",
        ),
    ] {
        let error = mxd_report::open(truncated.as_bytes(), Ids::new([200; 16])).unwrap_err();
        assert_eq!(error.code, "E_REOPEN_COVERAGE");
        assert_eq!(error.message, expected_message);
    }

    for bad in [
        text.replace("operations=  1", "operations= ***"),
        format!("I  F90 for all platform  Version 2025-11-20-00  I\r\n{text}"),
        text.replace("2.0000(X 1)", "2.0000(X**)"),
    ] {
        assert_eq!(
            mxd_report::open(bad.as_bytes(), Ids::new([200; 16]))
                .unwrap_err()
                .code,
            "E_REOPEN_COVERAGE"
        );
    }
}
