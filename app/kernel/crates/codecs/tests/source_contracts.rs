use serde_json::{Value, json};
use vesta_codecs::*;
use vesta_commands::{EngineV2, Limits};
use vesta_model::source_audit::AuditSource;
use vesta_model::*;
fn engine(label: &str) -> EngineV2 {
    EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        label,
        Ids::new([42; 16]),
        Limits::default(),
    )
    .unwrap()
}
fn source(bundle: &SaveBundle) -> Value {
    serde_json::from_slice(
        bundle
            .dependencies
            .iter()
            .find(|(p, _)| p.ends_with(".source.json"))
            .unwrap()
            .1,
    )
    .unwrap()
}
#[test]
fn logical_resources_share_blob_not_id() {
    let mut e = engine("logical");
    e.attach_resource("source/one", b"same bytes".to_vec())
        .unwrap();
    e.attach_resource("source/two", b"same bytes".to_vec())
        .unwrap();
    let b = encode(&e.capture().unwrap(), "logical.vesta").unwrap();
    let s = source(&b);
    let r = s["resources"].as_array().unwrap();
    assert_eq!(r.len(), 2);
    assert_ne!(
        r[0]["capturedResourceId"], r[1]["capturedResourceId"],
        "distinct logical resources must never share an identity"
    );
    assert_eq!(r[0]["sha256"], r[1]["sha256"]);
}
#[test]
fn source_and_binding_descriptor_digest_equal() {
    let mut e = engine("descriptor");
    e.attach_resource("source/one", b"same bytes".to_vec())
        .unwrap();
    let s = source(&encode(&e.capture().unwrap(), "descriptor.vesta").unwrap());
    let typed = &s["tree"]["members"]["resources"]["items"][0]["members"]["sourceDescriptorDigest"]
        ["value"];
    assert_eq!(
        typed, &s["resources"][0]["sourceDescriptorDigest"],
        "same source descriptor must have one canonical digest"
    );
}
#[test]
fn reserved_user_line_refused_before_encode() {
    let e = engine("#DRAFT_SOURCE_MANIFEST_V1 user text");
    assert!(
        encode(&e.capture().unwrap(), "reserved.vesta").is_err(),
        "do not emit an artifact rejected by our reserved-line grammar"
    );
    let mut c = engine("ordinary").capture().unwrap();
    c.content.comment = " #DRAFT_SOURCE_MANIFEST_NEW bad".into();
    c.state_digest = c.content.digest();
    assert!(encode(&c, "reserved.vesta").is_err());
    let _ = json!(null);
}

#[test]
fn generated_ledger_covers_source_and_resource_owners() {
    let mut e = engine("audit");
    e.attach_resource("source/one", b"same bytes".to_vec())
        .unwrap();
    e.attach_resource("source/two", b"same bytes".to_vec())
        .unwrap();
    let c = e.capture().unwrap();
    let evidence = audit_evidence(&c).unwrap();
    let ledger = evidence["ledger"].as_array().unwrap();
    for r in &c.content.resources {
        assert!(
            ledger
                .iter()
                .any(|row| row["ownerKind"] == "Resource" && row["ownerId"] == r.id.as_str())
        );
    }
    assert!(ledger.iter().any(|r| r["relativePath"] == "/schemaVersion"));
    let mut source = evidence["source7"].clone();
    source["identity"] = Value::Null;
    assert_eq!(
        vesta_model::digest(&vesta_model::source_audit::canonical(&source)),
        evidence["source7"]["identity"]["sourceRootDigest"]
    );
    if let Ok(p) = std::env::var("VESTA_AUDIT_EVIDENCE") {
        std::fs::write(p, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
    }
}

#[test]
fn fresh_lifetime_never_reuses_retired_ids_with_repeated_allocator_seed() {
    let e = engine("fresh");
    let old = e.capture().unwrap();
    let b = encode(&old, "fresh.vesta").unwrap();
    let restored = decode(&b.primary, &b.dependencies, Ids::new([42; 16])).unwrap();
    let mut old_ids: std::collections::BTreeSet<_> = old
        .content
        .audit()
        .unwrap()
        .definitions()
        .unwrap()
        .keys()
        .map(|(_, id)| id.clone())
        .collect();
    old_ids.insert(old.document.document_id);
    old_ids.insert(old.document.epoch);
    for (_, id) in restored
        .content
        .audit()
        .unwrap()
        .definitions()
        .unwrap()
        .keys()
    {
        assert!(!old_ids.contains(id));
    }
    assert!(!old_ids.contains(&restored.document.document_id));
    assert!(!old_ids.contains(&restored.document.epoch));
}
