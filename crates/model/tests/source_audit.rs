use serde_json::json;
use vesta_model::source_audit::{self as sa, AuditSource};
use vesta_model::*;
use vesta_source_derive::source_model;
fn project() -> ProjectContent {
    ProjectContent {
        scene_id: "1".repeat(32).into(),
        phases: vec![Phase {
            id: "2".repeat(32).into(),
            label: "phase".into(),
            cell: Cell3 {
                length_a: [4.; 3],
                angles_deg: [90.; 3],
            },
            sites: vec![Site {
                id: "3".repeat(32).into(),
                label: "C1".into(),
                element: "C".into(),
                frac: [0.25, 0., 0.5],
                occupancy: 1.,
                u_iso_a2: 0.,
            }],
            symmetry_op_id: "4".repeat(32).into(),
            boundary_min: [-0.5, 0., 0.],
            boundary_max: [1.5, 2., 1.],
        }],
        camera: Camera {
            rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            projection: "parallel".into(),
            zoom: 1.,
        },
        native_camera: None,
        hidden: vec![ImageKey {
            phase: "2".repeat(32).into(),
            site: "3".repeat(32).into(),
            op: "4".repeat(32).into(),
            shift: [-1, 0, 1],
        }],
        comment: "1".repeat(32),
        resources: vec![],
        legacy_source: None,
    }
}
#[test]
fn generated_identity_graph_rebinds_hidden_but_preserves_literal_text() {
    let mut p = project();
    p.validate().unwrap();
    let old = p.clone();
    let map = p.fresh_ids(&mut Ids::new([55; 16])).unwrap();
    assert_eq!(map.len(), 4);
    assert_ne!(p.scene_id, old.scene_id);
    assert_eq!(p.comment, old.comment);
    assert_eq!(p.hidden[0].phase, p.phases[0].id);
    assert_eq!(p.hidden[0].site, p.phases[0].sites[0].id);
    assert_eq!(p.hidden[0].op, p.phases[0].symmetry_op_id);
    assert_eq!(p.phases[0].boundary_min, [-0.5, 0., 0.]);
}
source_model! {pub struct Extended {pub new_persistent:String,pub absent:Option<f64>,pub empty:Vec<i32>,pub f32_value:f32}}
source_model! {#[source(tag="kind")] pub enum Choice{Idle,Values{value:Option<f32>}}}
#[test]
fn generated_members_null_empty_tag_and_bits_are_in_the_ledger() {
    let v = Extended {
        new_persistent: "new member".into(),
        absent: None,
        empty: vec![],
        f32_value: -0.0,
    };
    let n = v.audit().unwrap();
    let w = n.wire();
    assert_eq!(w["members"]["newPersistent"]["value"], "new member");
    assert_eq!(w["members"]["absent"]["kind"], "Null");
    assert_eq!(w["members"]["empty"]["items"], json!([]));
    assert_eq!(w["members"]["f32Value"]["ieeeHexBE"], "80000000");
    let l = n.ledger(&json!({"documentId":"a".repeat(32),"epoch":"b".repeat(32),"revision":"1"}));
    assert_eq!(l.as_array().unwrap().len(), 5);
    assert_eq!(
        Choice::Values { value: None }.audit().unwrap().wire()["members"]["kind"]["value"],
        "Values"
    );
}
#[test]
fn duplicate_cross_kind_and_dangling_references_refuse() {
    let mut p = project();
    p.hidden[0].site = "9".repeat(32).into();
    assert!(p.validate().is_err());
    let mut p = project();
    p.phases[0].id = p.scene_id.to_string().into();
    assert!(p.validate().is_err());
}
#[test]
fn resource_ids_are_distinct_from_physical_digests_and_all_fresh() {
    let mut p = project();
    let bytes = b"same".to_vec();
    let hash = digest(&bytes);
    p.resources = vec![
        PersistentResource {
            id: "5".repeat(32).into(),
            logical_key: "source/one".into(),
            sha256: hash.clone(),
            byte_length: "4".into(),
            media_type: "application/octet-stream".into(),
        },
        PersistentResource {
            id: "6".repeat(32).into(),
            logical_key: "source/two".into(),
            sha256: hash.clone(),
            byte_length: "4".into(),
            media_type: "application/octet-stream".into(),
        },
    ];
    let old = p.resources.clone();
    let mut n = p.audit().unwrap();
    let table = p
        .resources
        .iter()
        .map(|r| (r.id.to_string(), (hash.clone(), bytes.clone())))
        .collect();
    n.bind_resources(&table).unwrap();
    assert_eq!(n.definitions().unwrap().len(), 6);
    p.fresh_ids(&mut Ids::new([57; 16])).unwrap();
    assert_ne!(p.resources[0].id, p.resources[1].id);
    assert_ne!(p.resources[0].id, old[0].id);
    assert_eq!(p.resources[0].sha256, hash);
    let _ = sa::Role::Alias;
}

#[test]
fn tagged_audit_payload_retains_raw_bits_and_rebinds_only_references() {
    use vesta_model::source_audit::{AuditSource, Node, Remap, Role};
    let old = "11111111111111111111111111111111";
    let new = "22222222222222222222222222222222";
    let input = serde_json::json!({
        "kind":"TaggedVariant","tag":"KnownObservation1","payload":{"kind":"Map","members":{
            "literal":{"kind":"Utf8","value":old},
            "measure":{"kind":"F32Bits","ieeeHexBE":"7fc01234"},
            "target":{"kind":"EntityRef","objectKind":"Site","objectId":old}
        }}
    });
    let mut node = Node::from_wire(&input).unwrap();
    assert_eq!(node.wire(), input);
    let audit = node.audit().unwrap();
    assert_eq!(audit.roles(), vec![(
        "/payload/members/target/objectId".into(),"Site".into(),old.into(),Role::Reference
    )]);
    node.rebind(&Remap::from([(("Site".into(), old.into()), new.into())])).unwrap();
    let mut expected = input;
    expected["payload"]["members"]["target"]["objectId"] = new.into();
    assert_eq!(node.wire(), expected);
}
