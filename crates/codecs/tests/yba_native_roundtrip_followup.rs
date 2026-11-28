//! Actual VICS capture round trip through the existing source-state and save APIs.

use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use vesta_codecs::formats::live_state::{
    NativeStateInput, inspect_native_state, open_legacy_vics, open_native_state,
};
use vesta_codecs::{decode, encode_verified};
use vesta_model::source_audit::{Node, canonical};
use vesta_model::source7::SourceDocument;
use vesta_model::{Capture, Ids, PersistentResource, digest};

const RAW: &[u8] = include_bytes!("fixtures/native-scenes/yba-display-capture.bin");
const SPEC: &[u8] = include_bytes!("fixtures/native-scenes/yba-display-capture-compact.json");
const ORIGINAL: &str = "output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0080/YBa2Cu4O8.vcs";
const CAPTURE_KEY: &str = "vics/current-yba-native-spans";
const GRAPH_KIND: &str = "vics-current-coordination-population";
const CAPTURE_DIGEST: &str = "d1495be6240bce306c304b0a892abc44c1c3ca54e7008d393a2d1950c3bb36a6";

fn reference(kind: &str, id: &Value) -> Value {
    json!({"kind": "EntityRef", "objectKind": kind, "objectId": id})
}

fn native_tree(value: &Value) -> Node {
    match value {
        Value::Object(map) if map.contains_key("kind") => Node::from_wire(value).unwrap(),
        Value::Object(map) => Node::Map(
            map.iter()
                .map(|(key, val)| (key.clone(), native_tree(val)))
                .collect(),
        ),
        Value::Array(values) => Node::Array(values.iter().map(native_tree).collect()),
        Value::String(text) => Node::Text(text.clone()),
        Value::Number(number) if number.is_u64() => Node::U64(number.as_u64().unwrap()),
        Value::Number(number) => Node::I64(number.as_i64().unwrap()),
        Value::Bool(value) => Node::Bool(*value),
        Value::Null => Node::Null,
    }
}

fn span_range(row: &Value) -> std::ops::Range<usize> {
    let offset = usize::try_from(row["offset"].as_u64().unwrap()).unwrap();
    let length = usize::try_from(row["length"].as_u64().unwrap()).unwrap();
    offset..offset.checked_add(length).unwrap()
}

fn definition(
    id: &Value,
    native_kind: &str,
    owner: Value,
    state: &Value,
    spans: &Value,
    resource: &PersistentResource,
) -> Value {
    let records = spans
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let range = span_range(row);
            assert_eq!(digest(&RAW[range.clone()]), row["sha256"]);
            json!({"sourceResourceId": resource.id, "byteOffset": range.start.to_string(),
            "byteLength": range.len().to_string(), "sha256": row["sha256"]})
        })
        .collect::<Vec<_>>();
    assert!(!records.is_empty());
    json!({"id": id, "nativeKind": native_kind,
        "sourceTree": native_tree(&json!({"owner": owner, "nativeState": state})).wire(),
        "sourceRecords": records, "dependencyIds": [], "metadata": [],
        "scientificProjection": "not-qualified"})
}

fn add_definition(source: &mut Value, definition: Value) {
    source["sourceEntryOrder"]
        .as_array_mut()
        .unwrap()
        .push(json!({"kind": "source-entity", "id": definition["id"]}));
    source["sourceEntities"]
        .as_array_mut()
        .unwrap()
        .push(definition);
}

fn prepare(base: &mut Capture, spec: &Value, ids: &mut Ids) {
    let mut source = base.source.as_ref().unwrap().to_value().unwrap();
    let phase = &source["phaseGraph"]["phases"][0];
    let phase_owner = reference("phase", &phase["id"]);
    let scene_owner = reference("scene", &source["scenes"][0]["id"]);
    let source_sites: BTreeMap<_, _> = phase["structure"]["sites"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["label"].as_str().unwrap(), r["id"].clone()))
        .collect();
    assert_eq!(source_sites.len(), 8);
    let site_ids = spec["graph"]["sites"]
        .as_array()
        .unwrap()
        .iter()
        .map(|site| source_sites[site["siteLabel"].as_str().unwrap()].clone())
        .collect::<Vec<_>>();

    let resource = PersistentResource {
        id: ids.next_id().into(),
        logical_key: CAPTURE_KEY.into(),
        sha256: digest(RAW),
        byte_length: RAW.len().to_string(),
        media_type: "application/octet-stream".into(),
    };
    assert_eq!(resource.sha256, CAPTURE_DIGEST);
    assert_eq!(spec["packedSpansSHA256"], resource.sha256);
    let descriptor = digest(&canonical(&json!({"logicalKey": resource.logical_key,
        "sha256": resource.sha256, "byteLength": resource.byte_length, "mediaType": resource.media_type})));
    source["resources"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id": resource.id,
        "sha256": resource.sha256, "byteLength": resource.byte_length,
        "storageKind": "owned", "sourceDescriptorDigest": descriptor}));

    let graph_id = json!(ids.next_id());
    let targets = spec["targets"].as_array().unwrap();
    let target_ids = targets
        .iter()
        .map(|_| json!(ids.next_id()))
        .collect::<Vec<_>>();
    assert_eq!(targets.len(), 45);
    let rule_index = targets
        .iter()
        .position(|row| row["kind"] == "bond-rule")
        .unwrap();
    let mut graph = spec["graph"].clone();
    let graph_spans = graph.as_object_mut().unwrap().remove("spans").unwrap();
    graph["bondRule"] = reference("source-entity", &target_ids[rule_index]);
    graph["sourceTraceSHA256"] = spec["sourceTraceSHA256"].clone();
    for (row, site_id) in graph["sites"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .zip(&site_ids)
    {
        row["site"] = reference("site", site_id);
    }
    for (row, id) in targets.iter().zip(&target_ids) {
        let role = row["kind"].as_str().unwrap();
        let owner = if role == "atom-type" {
            &scene_owner
        } else {
            &phase_owner
        };
        let mut state = row["state"].clone();
        if role == "polyhedron" {
            let site = usize::try_from(state["siteOrdinal"].as_u64().unwrap()).unwrap();
            state["centerSite"] = reference("site", &site_ids[site]);
            state["coordinationPopulation"] = reference("source-entity", &graph_id);
        }
        add_definition(
            &mut source,
            definition(
                id,
                &format!("vics-native-display/{role}"),
                owner.clone(),
                &state,
                &row["spans"],
                &resource,
            ),
        );
    }
    add_definition(
        &mut source,
        definition(
            &graph_id,
            GRAPH_KIND,
            phase_owner,
            &graph,
            &graph_spans,
            &resource,
        ),
    );
    base.resources.insert(resource.sha256.clone(), RAW.to_vec());
    base.content.resources.push(resource);
    eprintln!("YBa stage: decode independently captured Source7 definitions");
    base.source = Some(SourceDocument::decode(source).unwrap());
    base.state_digest = Capture::digest_state(&base.content, base.source.as_ref()).unwrap();
}

fn native_state(spec: &Value) -> NativeStateInput {
    let saved = &spec["sceneGlobals"];
    let mut display = spec["observedDisplayWords"].clone();
    display["sceneSlots"] = saved["sceneSlots"].clone();
    display["lights"] = saved["lights"].clone();
    display["legacySource"] = json!("YBa2Cu4O8.vcs");
    display["currentValues"] = json!(true);
    let words = |field: &str| {
        saved["view"][field]
            .as_array()
            .unwrap()
            .iter()
            .map(|word| word["ieeeHexBE"].clone())
            .collect::<Vec<_>>()
    };
    NativeStateInput::parse(
        &serde_json::to_vec(&json!({
            "schemaVersion": "VestaDraft-native-current-state-1", "grids": [],
            "legacyDisplay": display, "legacyView": {"matrixBits": words("matrix16"),
                "settingsBits": words("settings4")},
        }))
        .unwrap(),
    )
    .unwrap()
}

fn write_seed(root: &Path, primary: &[u8], dependencies: &BTreeMap<String, Vec<u8>>) {
    std::fs::create_dir(root).unwrap();
    std::fs::write(root.join("native-yba.vesta"), primary).unwrap();
    for (key, bytes) in dependencies {
        let path = root.join(key);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }
}

#[test]
fn real_yba_capture_keeps_current_population_roles_and_raw_spans() {
    let spec: Value = serde_json::from_slice(SPEC).unwrap();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap();
    let original = std::fs::read(repository.join(ORIGINAL)).unwrap();
    assert_eq!(digest(&original), spec["originalSHA256"]);
    let mut ids = Ids::new([224; 16]);
    eprintln!("YBa stage: open actual legacy source");
    let mut base = open_legacy_vics(&original, ids.fork()).unwrap().capture;
    prepare(&mut base, &spec, &mut ids);
    eprintln!("YBa stage: install actual current native state");
    let capture = open_native_state(
        native_state(&spec),
        Some(base),
        |key| {
            assert_eq!(key, "YBa2Cu4O8.vcs");
            Ok(original.clone())
        },
        ids.fork(),
    )
    .unwrap()
    .capture;
    eprintln!("YBa stage: encode and verify engineering seed");
    let bundle = encode_verified(&capture, "native-yba.vesta", ids.fork()).unwrap();
    if let Some(output) = std::env::var_os("VD_P2_YBA_SEED_ROOT").map(PathBuf::from) {
        write_seed(&output, &bundle.primary, &bundle.dependencies);
    }
    eprintln!("YBa stage: independently reopen typed seed and compare");
    let restored = decode(&bundle.primary, &bundle.dependencies, ids.fork())
        .unwrap()
        .capture;
    let before = capture.source.as_ref().unwrap().to_value().unwrap();
    let after = restored.source.as_ref().unwrap().to_value().unwrap();
    assert!(after["meshes"].as_array().unwrap().is_empty());
    let phase = &after["phaseGraph"]["phases"][0];
    assert_ne!(phase["id"], before["phaseGraph"]["phases"][0]["id"]);
    let old_ids = before["sourceEntities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let entities = after["sourceEntities"].as_array().unwrap();
    assert_eq!(
        entities.len(),
        spec["targets"].as_array().unwrap().len() + 1
    );
    let restored_resource = restored
        .content
        .resources
        .iter()
        .find(|row| row.logical_key == CAPTURE_KEY)
        .unwrap();
    let original_resource = capture
        .content
        .resources
        .iter()
        .find(|row| row.logical_key == CAPTURE_KEY)
        .unwrap();
    assert_ne!(restored_resource.id, original_resource.id);
    assert_eq!(restored.resources[CAPTURE_DIGEST].as_slice(), RAW);
    for item in entities {
        assert!(!old_ids.contains(item["id"].as_str().unwrap()));
        for span in item["sourceRecords"].as_array().unwrap() {
            assert_eq!(
                span["sourceResourceId"],
                serde_json::to_value(&restored_resource.id).unwrap()
            );
            let offset = span["byteOffset"]
                .as_str()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let len = span["byteLength"]
                .as_str()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            assert_eq!(digest(&RAW[offset..offset + len]), span["sha256"]);
        }
    }
    let found = inspect_native_state(&restored).unwrap().unwrap();
    let displayed = &found["legacyDisplays"][0];
    let expected = spec["observedDisplayWords"].as_object().unwrap();
    for (field, rows) in expected {
        let actual = displayed[field].as_array().unwrap();
        let rows = rows.as_array().unwrap();
        assert_eq!(actual.len(), rows.len(), "{field}");
        for (actual, words) in actual.iter().zip(rows) {
            for (key, val) in words.as_object().unwrap() {
                assert_eq!(&actual[key], val, "{field}/{key}");
            }
        }
    }
    let graph = entities
        .iter()
        .find(|row| row["nativeKind"] == GRAPH_KIND)
        .unwrap();
    let polys = displayed["polyhedronAlpha"].as_array().unwrap();
    for (poly, source) in polys.iter().zip(
        entities
            .iter()
            .filter(|r| r["nativeKind"] == "vics-native-display/polyhedron"),
    ) {
        let state = &source["sourceTree"]["members"]["nativeState"]["members"];
        assert_eq!(poly["target"], reference("source-entity", &source["id"]));
        assert_eq!(poly["owner"], reference("phase", &phase["id"]));
        assert_eq!(
            state["coordinationPopulation"],
            reference("source-entity", &graph["id"])
        );
        assert_eq!(state["centerSite"]["objectKind"], "site");
    }
}
