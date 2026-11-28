use serde_json::Value;
use std::path::PathBuf;
use vesta_codecs::formats::fdat_source::{FdatLayout, open_selected_fdat};
use vesta_codecs::{decode, encode_verified};
use vesta_model::{Capture, Ids};

fn fixtures(path: &str) -> Vec<u8> {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
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
    let source = after.source.as_ref().unwrap().to_value().unwrap();
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
fn actual_mxd_ortho_file06_roundtrips_every_site_and_axis() {
    check_actual_mxd_file06(
        include_bytes!("fixtures/mxd/ortho-20251120-xd32.file06"),
        "27f690917a2e16ae380e695a12aaaeff2d04469d3efadc2879b42f9a6334743a",
        "ba74cde127477789dc9b796a8928175cec9593eee6a01b452141c7e9a30d518f",
        "ortho-20251120",
        Some("2025-11-20-00"),
    );
}

#[test]
fn actual_mxd_tricl_file06_preserves_signed_cell_words_and_all_axes() {
    let raw = include_bytes!("fixtures/mxd/tricl-20251119-xd32.file06");
    check_actual_mxd_file06(
        raw,
        "4f1e33174781c14f574c3eacbbab49e88e102603c8e3a440811ffc533d47a821",
        "4d645badbfd87bbc1d54fe03d950bebe545fb7eba73e91c237cf847b576c40bb",
        "tricl-20251119",
        None,
    );
    let text = std::str::from_utf8(raw).unwrap();
    let mixed_words = text.replace(" -0.02618", " -2.02618");
    assert_ne!(mixed_words, text);
    let error = vesta_codecs::formats::mxd_report::open(mixed_words.as_bytes(), Ids::new([202; 16]))
        .unwrap_err();
    assert_eq!(error.message, "MXD COORDN triclinic signed cell word outside cosine range");
}

fn check_actual_mxd_file06(
    raw: &[u8],
    original_sha: &str,
    token_sha: &str,
    dialect: &str,
    exact_printed_release: Option<&str>,
) {
    use vesta_codecs::formats::mxd_report::{inspect, open};
    use vesta_model::digest;

    assert_eq!(digest(raw), original_sha);
    let original = open(raw, Ids::new([201; 16])).unwrap();
    let after = save_and_reopen(&original.capture, "actual-xd32.vesta");
    assert!(after.content.phases.is_empty());
    assert_ne!(after.content.resources[0].id, original.content.resources[0].id);
    let resource = &after.content.resources[0];
    assert_eq!(after.resources[original_sha].as_slice(), raw);
    assert_eq!(resource.sha256, original_sha);

    let readback = inspect(&after).unwrap().unwrap();
    let [report] = readback["reports"].as_array().unwrap().as_slice()
        else { panic!("expected the single actual COORDN block") };
    let observed = &report["observation"];
    assert_eq!(observed["unit"], "1");
    assert_eq!(observed["producerContext"]["wireDialect"], dialect);
    assert_eq!(observed["producerContext"]["reportedRelease"].as_str(), exact_printed_release);
    assert_eq!(observed["producerContext"]["originalReport"], resource.id.as_str());
    if dialect == "tricl-20251119" {
        let heading = observed["rawHeading"].as_str().unwrap();
        assert!(heading.ends_with("  0.00873     -0.02618      0.05234"));
    }
    let sites = observed["sites"].as_array().unwrap();
    assert_eq!(sites.len(), 32);
    let mut exact_tokens = Vec::new();
    for (index, site) in sites.iter().enumerate() {
        assert_eq!(site["sourceOrdinal"], index as u64);
        exact_tokens.extend(site["sourceLabel4"].as_str().unwrap().as_bytes());
        let axes = site["axes"].as_array().unwrap();
        assert_eq!(axes.len(), 3);
        for axis in axes {
            for key in ["axis", "meanToken", "statisticToken", "referenceToken"] {
                exact_tokens.extend(axis[key].as_str().unwrap().as_bytes());
            }
        }
    }
    // The independent reader computed this over the real output, before product verification.
    assert_eq!(digest(&exact_tokens), token_sha);
    let groups = report["groups"].as_array().unwrap();
    assert_eq!(groups.len(), 8);
    for (index, group) in groups.iter().enumerate() {
        assert_eq!(group["firstSiteOrdinal"], (4 * index) as u64);
        assert_eq!(group["siteCount"], 4);
        for axis in group["axes"].as_array().unwrap() {
            let span = &axis["span"];
            let start: usize = span["byteOffset"].as_str().unwrap().parse().unwrap();
            let length: usize = span["byteLength"].as_str().unwrap().parse().unwrap();
            assert_eq!(span["sourceResourceId"], resource.id.as_str());
            assert_eq!(digest(&raw[start..start + length]), span["sha256"]);
        }
    }
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
    let root = "crates/codecs/tests/fixtures/r80";
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
    let root = "crates/codecs/tests/fixtures/r80";
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
    let original = fixtures("crates/model/tests/fixtures/source7/rich-persistence.valid.json");
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

// Authored Engine controls for FC's native SourceEntity fallback, not vendor captures.
fn vics_fallback_control() -> (Capture, Value, Vec<u8>) {
    use serde_json::json;
    use vesta_codecs::formats::live_state::open_legacy_vics;
    let raw = fixtures("output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs");
    let mut capture = open_legacy_vics(&raw, Ids::new([183; 16])).unwrap().capture;
    let mut source = capture.source.as_ref().unwrap().to_value().unwrap();
    let phase = source["phaseGraph"]["phases"][0]["id"].clone();
    let f = json!({"kind":"F32Bits","ieeeHexBE":"80000000"});
    let b = json!({"bounds6":vec![f.clone();6]});
    let current = [
        ("atom-type", "atomTypes", json!({"speciesHex":"4375","radius":f,"rgba":[1,2,3,4]})),
        ("phase-boundary", "phaseBoundaries", b.clone()),
        ("bond-rule", "bondRules", json!({"selectorAHex":"4375","selectorBHex":"4f",
            "minimum":f,"maximum":f,"searchMode":-2,"boundaryMode":3,"fullFlags":65535,"style":255,
            "radius":f,"lineWidth":f,"rgba":[4,3,2,1]})),
        ("first-shape", "shapeAlpha", json!({"alpha":17})),
        ("polyhedron", "polyhedronAlpha", json!({"alpha":19})),
    ];
    let mut manifest: Value = serde_json::from_slice(&fixtures(
        "output/vesta-p2-03/20261121-c8-retained-formats/fixtures/legacy.vstate")).unwrap();
    manifest["legacyDisplay"]["currentValues"] = json!(true);
    manifest["legacyView"] = json!({"matrixBits":vec!["8000000000000000";16],
        "settingsBits":vec!["80000000";4]});
    for (n, (role, field, value)) in current.into_iter().enumerate() {
        let id = format!("{:032x}", 900 + n);
        let (kind, owner) = if role == "atom-type" {
            ("scene", source["scenes"][0]["id"].clone())
        } else { ("phase", phase.clone()) };
        // Authored current-control bytes are their own provenance, not an old
        // vendor observation or a padded empty source-record population.
        let control = format!("Authored Engine native {role} controlWord=65535\n").into_bytes();
        let resource = vesta_model::PersistentResource {
            id: format!("{:032x}", 1000+n).into(),
            logical_key: format!("test/native-display/{role}"),
            sha256: vesta_model::digest(&control),
            byte_length: control.len().to_string(),
            media_type: "application/octet-stream".into(),
        };
        let descriptor = vesta_model::digest(&vesta_model::source_audit::canonical(&json!({
            "logicalKey":resource.logical_key,"sha256":resource.sha256,
            "byteLength":resource.byte_length,"mediaType":resource.media_type})));
        capture.resources.insert(resource.sha256.clone(), control);
        capture.content.resources.push(resource.clone());
        source["resources"].as_array_mut().unwrap().push(json!({"id":resource.id,
            "sha256":resource.sha256,"byteLength":resource.byte_length,"storageKind":"owned",
            "sourceDescriptorDigest":descriptor}));
        // The full native tree remains a real independent source definition.
        source["sourceEntities"].as_array_mut().unwrap().push(json!({"id":id,
            "nativeKind":format!("vics-native-display/{role}"),
            "sourceTree":{"kind":"Map","members":{
                "owner":{"kind":"EntityRef","objectKind":kind,"objectId":owner},
                "nativeState":{"kind":"Map","members":{"controlWord":{"kind":"U64","value":"65535"}}}}},
            "sourceRecords":[{"sourceResourceId":resource.id,"byteOffset":"0",
                "byteLength":resource.byte_length,"sha256":resource.sha256}],
            "dependencyIds":[],"metadata":[],"scientificProjection":"not-qualified"}));
        source["sourceEntryOrder"].as_array_mut().unwrap().push(json!({"kind":"source-entity","id":id}));
        manifest["legacyDisplay"][field] = if role == "phase-boundary" { json!([b,value]) } else {json!([value])};
    }
    capture.source = Some(vesta_model::source7::SourceDocument::decode(source).unwrap());
    capture.state_digest = Capture::digest_state(&capture.content, capture.source.as_ref()).unwrap();
    (capture, manifest, raw)
}

#[test]
fn vics_native_subobjects_use_distinct_source_entity_targets_and_fresh_owners() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{NativeStateInput, inspect_native_state, open_native_state};
    let (base, manifest, raw) = vics_fallback_control();
    let old_ids = base.source.as_ref().unwrap().to_value().unwrap()["sourceEntities"].as_array().unwrap()
        .iter().map(|row| row["id"].clone()).collect::<Vec<_>>();
    let input = NativeStateInput::parse(&serde_json::to_vec(&manifest).unwrap()).unwrap();
    let capture = open_native_state(input, Some(base), |_| Ok(raw.clone()), Ids::new([184;16])).unwrap().capture;
    let after = save_and_reopen(&capture, "vics-native-subobjects.vesta");
    let source = after.source.as_ref().unwrap().to_value().unwrap();
    let observed = inspect_native_state(&after).unwrap().unwrap();
    let display = &observed["legacyDisplays"][0];
    for (i, field) in ["atomTypes","phaseBoundaries","bondRules","shapeAlpha","polyhedronAlpha"].iter().enumerate() {
        let row = display[field].as_array().unwrap().last().unwrap();
        assert_eq!(row["target"]["objectKind"], "source-entity");
        assert_eq!(row["target"]["objectId"], source["sourceEntities"][i]["id"]);
        assert_ne!(row["target"]["objectId"], old_ids[i]);
        if i != 0 { assert_eq!(row["owner"]["objectId"], source["phaseGraph"]["phases"][0]["id"]); }
    }
    // Independently read each actual retained control span after fresh remap.
    for row in source["sourceEntities"].as_array().unwrap() {
        let role = row["nativeKind"].as_str().unwrap().strip_prefix("vics-native-display/").unwrap();
        let expected = format!("Authored Engine native {role} controlWord=65535\n").into_bytes();
        let span = &row["sourceRecords"][0];
        let resource = after.content.resources.iter().find(|r|
            Some(r.id.as_str()) == span["sourceResourceId"].as_str()).unwrap();
        assert_eq!(after.resources[&resource.sha256], expected);
        assert_eq!(span["sha256"], vesta_model::digest(&expected));
        assert_eq!(span["byteOffset"], "0");
        assert_eq!(span["byteLength"], expected.len().to_string());
    }
    assert_eq!(display["bondRules"][0]["fullFlags"], 65535);
    assert_eq!(display["bondRules"][0]["style"], 255);
    assert_eq!(display["atomTypes"][0]["radius"], json!({"kind":"F32Bits","ieeeHexBE":"80000000"}));
    assert_eq!(source["shapes"], json!([]));
    assert_eq!(source["meshes"], json!([]));
    assert_eq!(source["sourceEntities"][1]["sourceTree"]["members"]["owner"]["objectId"],
        source["phaseGraph"]["phases"][0]["id"]);
    let mut wrong = after;
    let mut bad = source;
    bad["scenes"][0]["render"]["members"]["nativeLegacyDisplayCurrent"]["payload"]["members"]
        ["bondRules"]["items"][0]["members"]["target"]["objectId"] = bad["sourceEntities"][0]["id"].clone();
    wrong.source = Some(vesta_model::source7::SourceDocument::decode(bad).unwrap());
    wrong.state_digest = Capture::digest_state(&wrong.content, wrong.source.as_ref()).unwrap();
    assert!(encode_verified(&wrong, "unrelated-native-target.vesta", Ids::new([186;16])).is_err());
}

#[test]
fn vics_polyp_replay_keeps_reader_alpha_separate_from_current_styles() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{NativeStateInput, inspect_native_state, open_native_state};

    let (base, mut manifest, raw) = vics_fallback_control();
    let base_source = base.source.as_ref().unwrap().to_value().unwrap();
    let owner = &base_source["phaseGraph"]["phases"][0]["id"];
    let display = &mut manifest["legacyDisplay"];
    display["currentValues"] = json!(false);
    display["atomTypes"][0]["polyhedronColor"] = json!({"rgb":[7,8,9],"alpha":11});
    let replay = |spec: &Value, seed| {
        let input = NativeStateInput::parse(&serde_json::to_vec(spec).unwrap()).unwrap();
        open_native_state(input, Some(base.clone()), |_| Ok(raw.clone()), Ids::new([seed;16]))
            .unwrap().capture
    };
    let capture = replay(&manifest, 190);
    let readback = save_and_reopen(&capture, "polyp-scene-style.vesta");
    let seen = inspect_native_state(&readback).unwrap().unwrap();
    let current = &seen["legacyDisplays"][0];
    // The YBa reader multiplies 0.700 by 255 in f32 and stores 179.
    // A later save can observe a different Element in the current Scene.
    assert_eq!(current["sceneSlots"]["0x2b0:4"], json!([179,179,179,179]));
    assert_eq!(current["captureRole"], "legacy-reader-replay");
    assert_eq!(current["legacyReaderElementAlpha"], 179);
    assert_eq!(current["atomTypes"][0]["polyhedronColor"], json!({"rgb":[7,8,9],"alpha":11}));
    assert_eq!(current["atomTypes"][0]["rgba"], json!([1,2,3,4]));
    assert_eq!(current["polyhedronAlpha"][0]["alpha"], 19);
    let restored = readback.source.as_ref().unwrap().to_value().unwrap();
    let restored_owner = &restored["phaseGraph"]["phases"][0]["id"];
    assert_eq!(&current["polyhedronAlpha"][0]["owner"]["objectId"], restored_owner);
    assert_ne!(restored_owner, owner);
    assert_eq!(current["polyhedronAlphaOrigin"], "supplied-current");

    manifest["legacyDisplay"]["currentValues"] = json!(true);
    let capture = replay(&manifest, 191);
    let seen = inspect_native_state(&capture).unwrap().unwrap();
    assert_eq!(seen["legacyDisplays"][0]["captureRole"], "actual-current-native-display");
    assert!(seen["legacyDisplays"][0].get("legacyReaderElementAlpha").is_none());
    assert_eq!(seen["legacyDisplays"][0]["atomTypes"][0]["polyhedronColor"],
        json!({"rgb":[7,8,9],"alpha":11}));

    // The reader's write supplies no missing final Element channels.
    manifest["legacyDisplay"]["currentValues"] = json!(false);
    manifest["legacyDisplay"]["atomTypes"][0].as_object_mut().unwrap().remove("polyhedronColor");
    let capture = replay(&manifest, 194);
    let seen = inspect_native_state(&capture).unwrap().unwrap();
    assert_eq!(seen["legacyDisplays"][0]["legacyReaderElementAlpha"], 179);
    assert!(seen["legacyDisplays"][0]["atomTypes"][0].get("polyhedronColor").is_none());
}

#[test]
fn vics_yba_preserves_the_four_saved_element_styles_and_separate_reader_alpha() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{
        NativeStateInput, inspect_native_state, open_legacy_vics, open_native_state,
    };
    use vesta_model::{PersistentResource, digest, source7::SourceDocument};

    const ELEMENT_BYTES: usize = 0x98;
    let words = include_bytes!("fixtures/native-scenes/yba-native-element-words.bin");
    let words_sha = "01352b575c53e2192aa7e5938521864888c8199b7178c96f8451dda7d9648fc9";
    assert_eq!(digest(words), words_sha);
    let (reader_elements, saved_elements) = words.split_at(4 * ELEMENT_BYTES);
    assert!(reader_elements.chunks_exact(ELEMENT_BYTES).all(|row| row[0x8f] == 179));
    assert!(saved_elements.chunks_exact(ELEMENT_BYTES).all(|row| row[0x8f] == 204));
    let raw = fixtures("output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs");
    let mut ids = Ids::new([211; 16]);
    let mut base = open_legacy_vics(&raw, ids.fork()).unwrap().capture;
    let mut source = base.source.as_ref().unwrap().to_value().unwrap();
    let scene = &base.content.scene_id;
    let resource = PersistentResource {
        id: ids.next_id().into(),
        logical_key: "yba/native-scene-element-memory".into(),
        sha256: words_sha.into(),
        byte_length: words.len().to_string(),
        media_type: "application/octet-stream".into(),
    };
    let descriptor = digest(&vesta_model::source_audit::canonical(&json!({
        "logicalKey":resource.logical_key, "sha256":resource.sha256,
        "byteLength":resource.byte_length, "mediaType":resource.media_type,
    })));
    source["resources"].as_array_mut().unwrap().push(json!({
        "id":resource.id, "sha256":resource.sha256, "byteLength":resource.byte_length,
        "storageKind":"owned", "sourceDescriptorDigest":descriptor,
    }));
    let mut styles = Vec::new();
    for (ordinal, bytes) in saved_elements.chunks_exact(ELEMENT_BYTES).enumerate() {
        let id = ids.next_id();
        let start = (4 + ordinal) * ELEMENT_BYTES;
        let symbol = bytes[0x14..0x1c].split(|byte| *byte == 0).next().unwrap();
        let species_hex = symbol.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
        let radius = u32::from_le_bytes(bytes[0x1c..0x20].try_into().unwrap());
        let (rgba, pcolor) = (&bytes[0x88..0x8c], &bytes[0x8c..0x90]);
        styles.push(json!({
            "speciesHex":species_hex, "radius":{"kind":"F32Bits","ieeeHexBE":format!("{radius:08x}")},
            "rgba":rgba, "polyhedronColor":{"rgb":&pcolor[..3],"alpha":pcolor[3]},
        }));
        source["sourceEntities"].as_array_mut().unwrap().push(json!({
            "id":id, "nativeKind":"vics-native-display/atom-type",
            "sourceTree":{"kind":"Map","members":{
                "owner":{"kind":"EntityRef","objectKind":"scene","objectId":scene},
                "nativeState":{"kind":"Map","members":{
                    "ordinal":{"kind":"U64","value":ordinal.to_string()},
                    "elementPColorAlpha":{"kind":"U64","value":pcolor[3].to_string()},
                }}
            }},
            "sourceRecords":[{"sourceResourceId":resource.id,"byteOffset":start.to_string(),
                "byteLength":ELEMENT_BYTES.to_string(),"sha256":digest(bytes)}],
            "dependencyIds":[], "metadata":[], "scientificProjection":"not-qualified",
        }));
        source["sourceEntryOrder"].as_array_mut().unwrap()
            .push(json!({"kind":"source-entity","id":id}));
    }
    base.content.resources.push(resource);
    base.resources.insert(words_sha.into(), words.to_vec());
    base.source = Some(SourceDocument::decode(source).unwrap());
    base.state_digest = Capture::digest_state(&base.content, base.source.as_ref()).unwrap();

    let witness: Value = serde_json::from_slice(include_bytes!("fixtures/native-scenes/yba-globals.json")).unwrap();
    assert_eq!(witness["sourceSHA256"], digest(&raw));
    let saved = &witness["snapshots"][1];
    let mut manifest: Value = serde_json::from_slice(&fixtures(
        "output/vesta-p2-03/20261121-c8-retained-formats/fixtures/legacy.vstate",
    )).unwrap();
    let display = &mut manifest["legacyDisplay"];
    display["sceneSlots"] = saved["sceneSlots"].clone();
    display["lights"] = saved["lights"].clone();
    display["atomTypes"] = json!(styles);
    let bounds = [0.0_f32, 2.0, 0.0, 1.0, -0.5, 1.0]
        .map(|value| json!({"kind": "F32Bits", "ieeeHexBE": format!("{:08x}", value.to_bits())}));
    display["phaseBoundaries"] = json!([{"bounds6": bounds}]);
    let words_at = |field: &str| saved["view"][field].as_array().unwrap().iter()
        .map(|value| value["ieeeHexBE"].clone()).collect::<Vec<_>>();
    manifest["legacyView"] = json!({"matrixBits":words_at("matrix16"),"settingsBits":words_at("settings4")});

    for current in [true, false] {
        manifest["legacyDisplay"]["currentValues"] = json!(current);
        let input = NativeStateInput::parse(&serde_json::to_vec(&manifest).unwrap()).unwrap();
        let first = open_native_state(input, Some(base.clone()), |_| Ok(raw.clone()), ids.fork()).unwrap().capture;
        let reopened = save_and_reopen(&first, if current { "yba-current.vesta" } else { "yba-reader.vesta" });
        let doc = reopened.source.as_ref().unwrap().to_value().unwrap();
        let seen = inspect_native_state(&reopened).unwrap().unwrap();
        let display = &seen["legacyDisplays"][0];
        assert_eq!(display["legacyReaderElementAlpha"].as_u64(), (!current).then_some(179));
        assert_eq!(display["atomTypes"].as_array().unwrap().len(), 4);
        let original_doc = first.source.as_ref().unwrap().to_value().unwrap();
        for (index, expected) in styles.iter().enumerate() {
            let row = &display["atomTypes"][index];
            for field in ["speciesHex", "radius", "rgba", "polyhedronColor"] {
                assert_eq!(row[field], expected[field]);
            }
            let definition = &doc["sourceEntities"][index];
            assert_eq!(row["target"]["objectId"], definition["id"]);
            assert_ne!(definition["id"], original_doc["sourceEntities"][index]["id"]);
            assert_eq!(definition["sourceTree"]["members"]["owner"]["objectId"], display["scene"]["objectId"]);
            let span = &definition["sourceRecords"][0];
            let offset = (4 + index) * ELEMENT_BYTES;
            assert_eq!(span["byteOffset"], offset.to_string());
            assert_eq!(span["sha256"], digest(&words[offset..offset + ELEMENT_BYTES]));
        }
        assert_eq!(reopened.resources[words_sha].as_slice(), &words[..]);
    }
}

#[test]
fn vics_archived_full_migration_preserves_saved_display_words_without_replay() {
    use serde_json::json;
    use std::collections::BTreeMap;
    use vesta_codecs::formats::live_state::inspect_native_state;

    let root = "crates/codecs/tests/fixtures/native-scenes/legacy-full";
    let primary = fixtures(&format!("{root}/first.vesta"));
    let manifest = format!("first.vesta.resources/manifests/{}.json", vesta_model::digest(&primary));
    let manifest_bytes = fixtures(&format!("{root}/{manifest}"));
    let spec: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    let mut dependencies = BTreeMap::from([(manifest.clone(), manifest_bytes)]);
    for name in spec["dependencies"].as_object().unwrap().keys() {
        dependencies.insert(name.clone(), fixtures(&format!("{root}/{name}")));
    }
    let witness: Value = serde_json::from_slice(&fixtures(
        "crates/codecs/tests/fixtures/native-scenes/yba-globals.json")).unwrap();
    let snapshot = &witness["snapshots"][0];
    let migrated = decode(&primary, &dependencies, Ids::new([192;16])).unwrap();
    let again = save_and_reopen(&migrated.capture, "vics-migrated-full.vesta");
    assert_ne!(again.document.document_id, migrated.document.document_id);
    for capture in [&migrated.capture, &again] {
        let readback = inspect_native_state(capture).unwrap().unwrap();
        let display = &readback["legacyDisplays"][0];
        assert_eq!(display["sceneSlots"], snapshot["sceneSlots"]);
        assert_eq!(display["lights"], snapshot["lights"]);
        assert_eq!(display["scene"]["objectId"], capture.content.scene_id.as_str());
        assert_eq!(display["atomTypes"], json!([]));
        assert_eq!(display["polyhedronAlpha"], json!([]));
        assert_eq!(display["polyhedronAlphaOrigin"], "legacy-current-1");
    }
    let current = encode_verified(&again, "vics-current.vesta", Ids::new([195;16])).unwrap();
    let new_manifest: Value = serde_json::from_slice(
        &current.dependencies[&format!("vics-current.vesta.resources/manifests/{}.json",
            vesta_model::digest(&current.primary))],
    ).unwrap();
    assert_ne!(spec["sourceRuleSetDigest"], new_manifest["sourceRuleSetDigest"]);
    let mut bad_manifest = spec;
    bad_manifest["sourceRuleSetDigest"] = new_manifest["sourceRuleSetDigest"].clone();
    let mut mixed = dependencies.clone();
    mixed.insert(manifest, serde_json::to_vec(&bad_manifest).unwrap());
    assert!(decode(&primary, &mixed, Ids::new([196;16])).is_err());

    let blob = dependencies.keys().find(|path| path.ends_with(".raw.bin")).unwrap().clone();
    dependencies.get_mut(&blob).unwrap()[0] ^= 1;
    assert!(decode(&primary, &dependencies, Ids::new([193;16])).is_err());
}

#[test]
fn vics_native_subobjects_reject_missing_population_wrong_owner_and_reserved_role() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{NativeStateInput, open_native_state};
    for variant in 0..4 {
        let (mut base, mut manifest, raw) = vics_fallback_control();
        let mut source = base.source.as_ref().unwrap().to_value().unwrap();
        match variant {
            0 => { manifest["legacyDisplay"]["polyhedronAlpha"] = json!([]); },
            1 => { source["sourceEntities"][1]["sourceTree"]["members"]["owner"] = json!({"kind":"EntityRef",
                "objectKind":"scene","objectId":source["scenes"][0]["id"]}); },
            2 => { source["sourceEntities"][2]["nativeKind"] = json!("vics-native-display/unknown"); },
            _ => { source["sourceEntities"][3]["sourceTree"]["members"]["nativeState"] = json!({"kind":"Null"}); },
        }
        base.source = Some(vesta_model::source7::SourceDocument::decode(source).unwrap());
        base.state_digest = Capture::digest_state(&base.content, base.source.as_ref()).unwrap();
        let input = NativeStateInput::parse(&serde_json::to_vec(&manifest).unwrap()).unwrap();
        assert!(open_native_state(input, Some(base), |_| Ok(raw.clone()), Ids::new([185;16])).is_err(), "case {variant}");
    }
}

#[test]
fn vics_recorded_read_and_saved_scene_globals_remain_distinct_on_reopen() {
    use serde_json::json;
    use vesta_codecs::formats::live_state::{NativeStateInput, inspect_native_state, open_native_state};
    let witness: Value = serde_json::from_slice(&fixtures(
        "crates/codecs/tests/fixtures/native-scenes/yba-globals.json")).unwrap();
    let raw = fixtures("output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs");
    assert_eq!(vesta_model::digest(&raw), witness["sourceSHA256"]);
    for (i, snapshot) in witness["snapshots"].as_array().unwrap().iter().enumerate() {
        let memory = fixtures(&format!("crates/codecs/tests/fixtures/native-scenes/{}", snapshot["rawFile"].as_str().unwrap()));
        assert_eq!(vesta_model::digest(&memory), snapshot["rawSHA256"]);
        assert_eq!(snapshot["sceneSlots"]["0x230:3"], if i == 0 {json!([1,123,0])} else {json!([0,0,0])});
        assert_eq!(snapshot["sceneSlots"]["0x5b0"], if i == 0 {json!(39850946)} else {json!(37753794)});
        let mut manifest: Value = serde_json::from_slice(&fixtures(
            "output/vesta-p2-03/20261121-c8-retained-formats/fixtures/legacy.vstate")).unwrap();
        manifest["legacyDisplay"]["currentValues"] = json!(true);
        manifest["legacyDisplay"]["sceneSlots"] = snapshot["sceneSlots"].clone();
        manifest["legacyDisplay"]["lights"] = snapshot["lights"].clone();
        manifest["legacyView"] = json!({
            "matrixBits":snapshot["view"]["matrix16"].as_array().unwrap().iter().map(|w|w["ieeeHexBE"].clone()).collect::<Vec<_>>(),
            "settingsBits":snapshot["view"]["settings4"].as_array().unwrap().iter().map(|w|w["ieeeHexBE"].clone()).collect::<Vec<_>>()});
        // This engineering working state installs actual recorded global words.
        // Empty Draft populations do not claim a complete historic native import.
        let input = NativeStateInput::parse(&serde_json::to_vec(&manifest).unwrap()).unwrap();
        let capture = open_native_state(input,None, |_| Ok(raw.clone()),Ids::new([187+i as u8;16])).unwrap().capture;
        let after = save_and_reopen(&capture,&format!("native-global-{i}.vesta"));
        let current = inspect_native_state(&after).unwrap().unwrap();
        assert_eq!(current["legacyDisplays"][0]["sceneSlots"], snapshot["sceneSlots"]);
        assert_eq!(current["legacyDisplays"][0]["lights"], snapshot["lights"]);
        assert_eq!(current["legacyViews"][0]["matrix16"], snapshot["view"]["matrix16"]);
        assert_eq!(current["legacyViews"][0]["settings4"], snapshot["view"]["settings4"]);
        assert_eq!(after.resources[&after.content.resources[0].sha256],raw);
    }
}
