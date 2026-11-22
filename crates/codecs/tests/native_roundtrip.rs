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
