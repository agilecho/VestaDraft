use std::{fs, path::PathBuf};

use serde_json::{Value, json};
use vesta_codecs::{decode, encode_verified, formats::scalar_grid_source::open_selected_eb};
use vesta_commands::{EngineV2, Limits};
use vesta_model::{Ids, digest, source7::SourceDocument};

fn fixture() -> Vec<u8> {
    let checkout = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fs::read(checkout.ancestors().nth(4).unwrap().join(
        "output/vesta-study/20261108-113622/reviews/20261110-104021-s05-completion/input-sources/VS-0089/producer-layout.eb",
    )).unwrap()
}

fn typed_context(source: &Value) -> &Value {
    &source["sourceEntities"][0]["metadata"][0]["tree"]["payload"]["members"]
}

fn id(resource: &Value) -> &str {
    resource["members"]["resourceId"]["objectId"]
        .as_str()
        .unwrap()
}

#[test]
fn native_eb_reader_binds_definition_frame_and_distinct_evidence_uses_across_save() {
    let raw = fixture();
    let initial = open_selected_eb(&raw, Ids::new([122; 16])).unwrap();
    let engine = EngineV2::loaded(initial, Ids::new([123; 16]), Limits::default()).unwrap();
    assert!(engine.active_fields().is_empty());
    let captured = engine.capture().unwrap();
    let value = captured.source.as_ref().unwrap().to_value().unwrap();
    assert!(value["phaseGraph"].is_null());
    let ctx = typed_context(&value);
    let bindings = ctx["bindings"]["items"].as_array().unwrap();
    assert_eq!(bindings.len(), 3);
    let installed = |resource_id| {
        captured
            .content
            .resources
            .iter()
            .find(|r| r.id.as_str() == resource_id)
            .unwrap()
    };
    let definition = installed(id(&ctx["definition"]));
    let wire: Value = serde_json::from_slice(&captured.resources[&definition.sha256]).unwrap();
    let scope = installed(id(&ctx["scopeInput"]));
    assert_eq!(scope.sha256, digest(&raw));
    let evidence = installed(id(&bindings[0]["members"]["target"]));
    assert_ne!(evidence.id, scope.id);
    assert_eq!(evidence.sha256, scope.sha256);
    assert_eq!(wire["fieldBindings"][0]["unit"], "source-unresolved");
    let bundle = encode_verified(&captured, "eb-source.vesta", Ids::new([124; 16])).unwrap();
    let new = decode(&bundle.primary, &bundle.dependencies, Ids::new([125; 16])).unwrap();
    let value = new.capture.source.as_ref().unwrap().to_value().unwrap();
    let fresh = typed_context(&value);
    assert_ne!(id(&ctx["scopeInput"]), id(&fresh["scopeInput"]));
    assert_ne!(id(&ctx["definition"]), id(&fresh["definition"]));
    let fresh_definition = new
        .content
        .resources
        .iter()
        .find(|r| r.id.as_str() == id(&fresh["definition"]))
        .unwrap();
    assert_eq!(
        &new.resources[&fresh_definition.sha256],
        &captured.resources[&definition.sha256]
    );
    assert_eq!(
        fresh["bindings"]["items"][0]["members"]["capturedAlias"],
        bindings[0]["members"]["capturedAlias"]
    );

    let mut resumed = EngineV2::loaded(new, Ids::new([126; 16]), Limits::default()).unwrap();
    resumed
        .execute(
            &resumed.document(),
            &json!({"command":"view.setCamera","params":{
                "rotation":[[1,0,0],[0,1,0],[0,0,1]],"projection":"parallel","zoom":1.75
            }}),
        )
        .unwrap();
    let bundle = encode_verified(
        &resumed.capture().unwrap(),
        "eb-edited.vesta",
        Ids::new([127; 16]),
    )
    .unwrap();
    let edited = decode(&bundle.primary, &bundle.dependencies, Ids::new([128; 16])).unwrap();
    assert_eq!(edited.content.camera.zoom, 1.75);
}

#[test]
fn native_eb_same_bytes_wrong_logical_evidence_and_missing_target_fail() {
    let raw = fixture();
    let original = open_selected_eb(&raw, Ids::new([129; 16])).unwrap().capture;
    let source = original.source.as_ref().unwrap().to_value().unwrap();
    let mut wrong = source.clone();
    let path = "/sourceEntities/0/metadata/0/tree/payload/members";
    let scope_id = id(&typed_context(&source)["scopeInput"]);
    wrong
        .pointer_mut(&format!(
            "{path}/bindings/items/0/members/target/members/resourceId/objectId"
        ))
        .map(|value| *value = json!(scope_id))
        .unwrap();
    let mut candidate = original.clone();
    candidate.source = Some(SourceDocument::decode(wrong).unwrap());
    assert_eq!(
        encode_verified(&candidate, "swapped.vesta", Ids::new([130; 16]))
            .unwrap_err()
            .code,
        "E_REOPEN_COVERAGE"
    );

    let mut missing = source;
    missing
        .pointer_mut(&format!("{path}/bindings/items"))
        .unwrap()
        .as_array_mut()
        .unwrap()
        .pop();
    let mut candidate = original;
    candidate.source = Some(SourceDocument::decode(missing).unwrap());
    assert_eq!(
        encode_verified(&candidate, "missing.vesta", Ids::new([131; 16]))
            .unwrap_err()
            .code,
        "E_REOPEN_COVERAGE"
    );

    let mut unqualified = raw;
    unqualified.push(b'\n');
    assert!(!vesta_codecs::formats::scalar_grid_source::is_selected_eb(
        &unqualified
    ));
    assert_eq!(
        open_selected_eb(&unqualified, Ids::new([132; 16]))
            .unwrap_err()
            .code,
        "E_REOPEN_COVERAGE"
    );
}
