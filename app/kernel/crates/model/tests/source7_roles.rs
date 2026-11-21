use serde_json::{Value, json};
use std::collections::BTreeMap;
use vesta_model::{
    Ids, digest,
    source_audit::{AuditSource, Role},
    source7::SourceDocument,
};

// Authored independently from the source schema and product output. This
// fixture tests typed preservation, including an unqualified opaque closure.
fn rich() -> Value {
    serde_json::from_str(include_str!("fixtures/source7/rich-persistence.valid.json")).unwrap()
}

#[test]
fn generated_role_inventory_preserves_same_literal_and_fresh_aliases() {
    let mut input = rich();
    input["sections"][0]["fieldIds"] = json!([input["fields"][0]["id"].clone()]);
    assert_eq!(input.as_object().unwrap().len(), 22);
    let mut source = SourceDocument::decode(input.clone()).unwrap();
    let old = source.audit().unwrap();
    assert_eq!(old.data().unwrap(), input);
    let roles: BTreeMap<_, _> = old
        .roles()
        .into_iter()
        .map(|(path, kind, id, role)| (path, (kind, id, role)))
        .collect();
    let mesh = input["meshes"][0]["id"].as_str().unwrap();
    assert_eq!(
        roles["/meshes/0/id"],
        ("Mesh".into(), mesh.into(), Role::Definition)
    );
    assert_eq!(
        roles["/meshes/0/value/objectId"],
        ("Mesh".into(), mesh.into(), Role::Alias)
    );
    assert_eq!(
        roles["/scenes/0/nodes/3/sourceObjectId"],
        ("Mesh".into(), mesh.into(), Role::Reference)
    );
    assert!(!roles.contains_key("/metadata/0/tree/members/literalIdRetained/value"));
    assert_eq!(
        roles["/metadata/0/tree/members/typedRemapMesh/objectId"],
        ("Mesh".into(), mesh.into(), Role::Reference)
    );
    assert_eq!(roles["/shapes/0/value/halfspaces/0/id"].0, "ShapeHalfspace");
    assert_eq!(
        roles["/planes/1/value/source/points/0/pointId"].0,
        "MeasurementPoint"
    );
    assert_eq!(roles["/sections/0/fieldIds/0"].0, "Field");
    let first_bytes = &roles["/fields/0/metadata/0/tree/payload/members/bytes/resourceId"];
    let second_bytes = &roles["/fields/1/metadata/0/tree/payload/members/bytes/resourceId"];
    assert_eq!(first_bytes.0, "Resource");
    assert_eq!(second_bytes.0, "Resource");
    assert_ne!(first_bytes.1, second_bytes.1);

    let remap = old.fresh_map(&mut Ids::new([4; 16])).unwrap();
    source.rebind(&remap).unwrap();
    let output = source.to_value().unwrap();
    let new_mesh = remap.get(&("Mesh".into(), mesh.into())).unwrap();
    assert_eq!(output["meshes"][0]["id"], json!(new_mesh));
    assert_eq!(output["meshes"][0]["value"]["objectId"], json!(new_mesh));
    assert_eq!(
        output["scenes"][0]["nodes"][3]["sourceObjectId"],
        json!(new_mesh)
    );
    assert_eq!(
        output["metadata"][0]["tree"]["members"]["typedRemapMesh"]["objectId"],
        json!(new_mesh)
    );
    assert_eq!(
        output["metadata"][0]["tree"]["members"]["literalIdRetained"]["value"],
        json!(mesh)
    );
    assert_eq!(
        output["sections"][0]["nativeRecord"],
        input["sections"][0]["nativeRecord"]
    );
    assert_eq!(
        output["resources"][0]["sha256"],
        output["resources"][1]["sha256"]
    );
    assert_ne!(output["resources"][0]["id"], output["resources"][1]["id"]);
}

#[test]
fn discriminator_and_scope_are_required_independently_of_reference_existence() {
    let mut wrong_mesh_alias = rich();
    wrong_mesh_alias["meshes"][0]["value"]["objectId"] =
        wrong_mesh_alias["materials"][0]["id"].clone();
    assert!(SourceDocument::decode(wrong_mesh_alias).is_err());

    let mut wrong_bond_owner = rich();
    wrong_bond_owner["bondRules"][1]["sgProjectionRuleIds"] =
        json!([wrong_bond_owner["bondRules"][1]["id"].clone()]);
    assert!(SourceDocument::decode(wrong_bond_owner).is_err());

    let mut wrong_field = rich();
    wrong_field["fields"][0]["metadata"][0]["tree"]["payload"]["members"]["owner"] =
        wrong_field["fields"][1]["metadata"][0]["tree"]["payload"]["members"]["owner"].clone();
    assert!(SourceDocument::decode(wrong_field).is_err());

    let mut unknown_tag = rich();
    unknown_tag["metadata"][0]["tree"] =
        json!({"kind":"TaggedVariant","tag":"UndeclaredCurrentShape","payload":{"kind":"Null"}});
    assert!(SourceDocument::decode(unknown_tag).is_err());

    let mut unknown_field = rich();
    unknown_field["meshes"][0]["value"]["missingFromType"] = json!(1);
    assert!(SourceDocument::decode(unknown_field).is_err());
}

#[test]
fn fresh_rebind_is_reversible_for_full_tree_and_rejects_incomplete_or_colliding_maps() {
    let input = rich();
    let mut source = SourceDocument::decode(input.clone()).unwrap();
    let audit = source.audit().unwrap();
    let map = audit.fresh_map(&mut Ids::new([64; 16])).unwrap();
    let mut keys = map.keys();
    let first = keys.next().unwrap();
    let second = keys.next().unwrap();

    let mut missing = map.clone();
    missing.remove(first);
    let mut collision = map.clone();
    collision.insert(first.clone(), map[second].clone());
    let mut retired = map.clone();
    retired.insert(first.clone(), second.1.clone());
    for invalid in [missing, collision, retired] {
        assert!(source.rebind(&invalid).is_err());
        assert_eq!(source.to_value().unwrap(), input);
    }

    source.rebind(&map).unwrap();
    let rebound_audit = source.audit().unwrap();
    let rebound = source.to_value().unwrap();
    assert_eq!(rebound_audit.data().unwrap(), rebound);
    let inverse = map.into_iter()
        .map(|((kind, old), new)| ((kind, new), old))
        .collect();
    source.rebind(&inverse).unwrap();
    assert_eq!(source.to_value().unwrap(), input);
    assert_eq!(source.audit().unwrap().roles(), audit.roles());
}

#[test]
fn source_entry_order_requires_every_model_entity_exactly_once() {
    let mut reordered = rich();
    reordered["sourceEntryOrder"].as_array_mut().unwrap().reverse();
    let source = SourceDocument::decode(reordered.clone()).unwrap();
    assert_eq!(source.to_value().unwrap()["sourceEntryOrder"], reordered["sourceEntryOrder"]);

    let mut missing = reordered.clone();
    missing["sourceEntryOrder"].as_array_mut().unwrap().pop();
    let error = SourceDocument::decode(missing).unwrap_err();
    assert!(error.message.contains("sourceEntryOrder omits a model entity"));

    let repeated = reordered["sourceEntryOrder"][0].clone();
    reordered["sourceEntryOrder"][1] = repeated;
    assert!(SourceDocument::decode(reordered).is_err());
}

#[test]
fn cartesian_covariance_rebinds_its_selected_point_and_rejects_unrelated_id() {
    let mut input = rich();
    let point = input["planes"][1]["value"]["source"]["points"][0]["pointId"]
        .as_str()
        .unwrap()
        .to_owned();
    input["phaseGraph"]["phases"][0]["structure"]["covariance"]["parameters"][0] =
        json!({"kind":"site-cartesian","axis":0,"pointId":point});
    let mut source = SourceDocument::decode(input.clone()).unwrap();
    let remap = source
        .audit()
        .unwrap()
        .fresh_map(&mut Ids::new([5; 16]))
        .unwrap();
    source.rebind(&remap).unwrap();
    let output = source.to_value().unwrap();
    let actual_point = &output["planes"][1]["value"]["source"]["points"][0]["pointId"];
    let actual_parameter =
        &output["phaseGraph"]["phases"][0]["structure"]["covariance"]["parameters"][0]["pointId"];
    assert_eq!(actual_parameter, actual_point);
    assert_ne!(actual_point, &json!(point));
    input["phaseGraph"]["phases"][0]["structure"]["covariance"]["parameters"][0]["pointId"] =
        input["meshes"][0]["id"].clone();
    let error = SourceDocument::decode(input).unwrap_err();
    assert!(error.to_string().contains("owning phase"));
}

#[test]
fn nullable_phase_graph_and_raw_subspan_are_preserved_without_synthetic_phase() {
    let raw = b"prefix:source-only:suffix";
    let id = "00000000000000000000000000000003";
    let sha = digest(raw);
    let span = digest(b"source-only");
    let mut input = rich();
    for (key, value) in input.as_object_mut().unwrap() {
        if value.is_array() {
            *value = json!([]);
        } else if key == "phaseGraph" {
            *value = Value::Null;
        }
    }
    input["resources"] = json!([{
        "id":id,"sha256":sha,"byteLength":raw.len().to_string(),
        "storageKind":"owned","sourceDescriptorDigest":"a".repeat(64)
    }]);
    input["metadata"] = json!([{
        "owner":{"kind":"document","id": input["identity"]["documentId"].clone()},
        "producerId":"test","producerVersion":"1","formatRef":"raw","dialect":"authored",
        "tree":{"kind":"F32Bits","ieeeHexBE":"80000000"},
        "spans":[{"sourceResourceId":id,"byteOffset":"7","byteLength":"11","sha256":span}]
    }]);
    let typed = SourceDocument::decode(input.clone()).unwrap();
    let resources = BTreeMap::from([(id.into(), (sha, raw.to_vec()))]);
    typed.validate_resources(&resources).unwrap();
    assert_eq!(typed.to_value().unwrap(), input);

    input["metadata"][0]["spans"][0]["byteOffset"] = json!("6");
    let moved = SourceDocument::decode(input).unwrap();
    assert!(moved.validate_resources(&resources).is_err());
}

#[test]
fn bound_quantity_uses_exact_owned_definition_scope_and_evidence_bytes() {
    use vesta_model::source_audit::Node;

    let mut input = rich();
    let molecule = input["molecules"][0].clone();
    let entry = input["sourceEntryOrder"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["kind"] == "molecule")
        .unwrap()
        .clone();
    for (key, value) in input.as_object_mut().unwrap() {
        if value.is_array() {
            *value = json!([]);
        } else if key == "phaseGraph" {
            *value = Value::Null;
        }
    }
    input["molecules"] = json!([molecule]);
    input["sourceEntryOrder"] = json!([entry]);

    let definition_id = "00000000000000000000000000000041";
    let scope_id = "00000000000000000000000000000042";
    let evidence_id = "00000000000000000000000000000043";
    let scope = b"independently addressed scope input".to_vec();
    let evidence = b"head precision-record end".to_vec();
    let scope_sha = digest(&scope);
    let evidence_sha = digest(&evidence);
    let definition = json!({
        "schemaVersion":"FC-source-descriptor-2","producerId":"authored","producerVersion":"1",
        "dialect":"native","scopeInputDigest":scope_sha,"fieldBindings":[],"unknownSourceFields":[],
        "evidence":[{
            "evidenceResource":{"resourceId":"old-captured-evidence-alias","sha256":evidence_sha,
                "byteLength":evidence.len().to_string()},
            "byteOffset":"5","byteLength":"16","sourceSpanDigest":digest(b"precision-record"),
            "sourceVersion":"1"
        }]
    });
    let definition = serde_json::to_vec(&definition).unwrap();
    let definition_sha = digest(&definition);
    let entries = [
        (definition_id, definition_sha.clone(), definition),
        (scope_id, scope_sha, scope),
        (evidence_id, evidence_sha, evidence),
    ];
    let reference =
        |id: &str, kind: &str| json!({"kind":"EntityRef","objectKind":kind,"objectId":id});
    let literal = |text: &str| json!({"kind":"Utf8","value":text});
    let data_resource = |(id, sha, bytes): &(&str, String, Vec<u8>)| {
        json!({"kind":"Map","members":{
            "resourceId": reference(id, "resource"),
            "sha256": literal(sha),
            "byteLength": literal(&bytes.len().to_string())
        }})
    };
    input["resources"] = Value::Array(
        entries
            .iter()
            .map(|(id, sha, bytes)| {
                json!({
                    "id":id,"sha256":sha,"byteLength":bytes.len().to_string(),
                    "storageKind":"owned","sourceDescriptorDigest":"a".repeat(64)
                })
            })
            .collect(),
    );
    let atom = &mut input["molecules"][0]["frames"][0]["atoms"][0];
    let owner = atom["id"].as_str().unwrap().to_owned();
    let pointer = "/molecules/0/frames/0/atoms/0/partialCharges/0/sourceDescriptorDigest";
    let schema_pin = "f79e7d7146cd6326c39c70dd4acb3b55f6cb6c712cdedab7d2eb3b339ead84d2";
    let rule_pin = "972df77f2885256d3b460c440aba6804befe02c947bc79460207678598e6cf40";
    let tag = json!({"kind":"TaggedVariant","tag":"BoundDescriptorContext1","payload":{
        "kind":"Map","members":{
            "schemaVersion":literal("BoundDescriptorContext-proposal-1"),
            "owner":{"kind":"Map","members":{"kind":literal("atom"),"id":reference(&owner,"atom")}},
            "sourceTypedPointer":literal(pointer),"scopeInput":data_resource(&entries[1]),
            "definition":data_resource(&entries[0]),"descriptorSchemaDigest":literal(schema_pin),
            "referenceRuleDigest":literal(rule_pin),"bindings":{"kind":"Array","items":[{
                "kind":"Map","members":{
                    "jsonPointer":literal("/evidence/0/evidenceResource"),
                    "capturedAlias":literal("old-captured-evidence-alias"),
                    "target":data_resource(&entries[2])
                }
            }]}
        }}
    });
    assert_eq!(Node::from_wire(&tag).unwrap().wire(), tag);
    atom["partialCharges"][0]["sourceDescriptorDigest"] = json!(definition_sha);
    atom["metadata"] = json!([{
        "owner":{"kind":"atom","id":owner},"producerId":"authored","producerVersion":"1",
        "formatRef":"SourceDescriptor2","dialect":"owner-scoped",
        "tree":tag,"spans":[]
    }]);
    let resources = entries
        .into_iter()
        .map(|(id, sha, bytes)| (id.into(), (sha, bytes)))
        .collect();
    let source = SourceDocument::decode(input.clone()).unwrap();
    source.validate_resources(&resources).unwrap();

    let prefix = format!("descriptor-data/{}", digest(pointer.as_bytes()));
    let target = digest(b"/evidence/0/evidenceResource");
    let mut installed = [
        (definition_id, format!("{prefix}/definition")),
        (scope_id, format!("{prefix}/scope")),
        (evidence_id, format!("{prefix}/target/{target}")),
    ].map(|(id, logical_key)| vesta_model::PersistentResource {
        id: id.to_owned().into(),
        logical_key,
        sha256: resources[id].0.clone(),
        byte_length: resources[id].1.len().to_string(),
        media_type: "application/octet-stream".into(),
    });
    source.validate_installed_resources(&resources, &installed).unwrap();
    let decoy = "00000000000000000000000000000044";
    let mut with_decoy = resources.clone();
    with_decoy.insert(decoy.into(), resources[evidence_id].clone());
    installed[2].id = decoy.to_owned().into();
    source.validate_resources(&with_decoy).unwrap();
    assert!(source.validate_installed_resources(&with_decoy, &installed).unwrap_err()
        .message.contains("descriptor logical resource differs"));

    let bad_owner = "00000000000000000000000000000002";
    let tree = &mut input["molecules"][0]["frames"][0]["atoms"][0]["metadata"][0]["tree"];
    tree["payload"]["members"]["bindings"]["items"][0]["members"]["capturedAlias"] =
        literal(bad_owner);
    let wrong = SourceDocument::decode(input).unwrap();
    assert!(wrong.validate_resources(&resources).is_err());
}

#[test]
fn nonperiodic_bond_endpoints_belong_to_the_owning_frame() {
    let mut input = rich();
    let same_frame = "00000000000000000000000000000041";
    let separate_frame = "00000000000000000000000000000042";
    let mut atom = input["molecules"][0]["frames"][0]["atoms"][0].clone();
    let first = atom["id"].clone();
    atom["id"] = json!(same_frame);
    input["molecules"][0]["frames"][0]["atoms"]
        .as_array_mut()
        .unwrap()
        .push(atom.clone());
    atom["id"] = json!(separate_frame);
    input["molecules"][0]["frames"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id":"00000000000000000000000000000043","sourceModelCode":"2","atoms":[atom],
            "bonds":[],"basisCenters":[],"metadata":[]
        }));
    input["molecules"][0]["frames"][0]["bonds"] = json!([{
        "id":"00000000000000000000000000000044","atomIds":[first,same_frame],
        "order":"single","stereo":{"kind":"Null"},
        "sourceMultiplicity":{"kind":"U64","value":"2"},"metadata":[]
    }]);
    let typed = SourceDocument::decode(input.clone()).unwrap();
    assert_eq!(typed.to_value().unwrap(), input);

    input["molecules"][0]["frames"][0]["bonds"][0]["atomIds"][1] = json!(separate_frame);
    assert!(
        SourceDocument::decode(input)
            .unwrap_err()
            .to_string()
            .contains("molecule frame")
    );
}
