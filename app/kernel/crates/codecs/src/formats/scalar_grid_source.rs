//! Selected EB reader and release-owned Descriptor2 capture. The independently
//! retained raw grid and materialized scalar buffer have no physical projection.
use super::{full_source7, native_vesta_v2::source_only_capture};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::source_audit::{Node, Role, canonical};
use vesta_model::source7::{SourceDocument, native_grid_data_use};
use vesta_model::{Capture, Error, Ids, PersistentResource, RestoredProject, Result, digest};

const FIXTURE: &str = "7aaf51bc1048bbc2ade1d3a5b1516127b188cd8eaeda85b98647f0702e93063f";
const PRODUCER: &str = "draft.vend-native-stored-grid";
const FORMAT: &str = "VM18-I33";
const NATIVE_KIND: &str = "vend-native-stored-grid";
const TAG: &str = "NativeScalarGridSource1";
const FRAME: &str = include_str!("../../contracts/native-scalar-grid-frame.schema.json");
const CONTRACT: &str = include_str!("../../contracts/native-scalar-grid-source.contract.json");
const WIRE: &str = include_str!("../../contracts/native-wien-eb-wire.contract.json");
const DESCRIPTOR: &str =
    include_str!("../../../model/src/source7/frozen/source-descriptor.schema.json");
const BINDING: &str =
    include_str!("../../../model/src/source7/frozen/bound-descriptor-context.contract.json");

pub(super) fn rule_digest() -> String {
    digest(&canonical(&json!([
        PRODUCER, FIXTURE, FRAME, CONTRACT, WIRE
    ])))
}

pub fn is_selected_eb(bytes: &[u8]) -> bool {
    bytes.len() == 316 && digest(bytes) == FIXTURE
}

fn invalid(message: &str) -> Error {
    Error::new("E_REOPEN_COVERAGE", message)
}

fn number(raw: Option<&str>) -> Result<f32> {
    raw.and_then(|word| word.parse::<f32>().ok())
        .filter(|n| n.is_finite())
        .ok_or_else(|| invalid("invalid finite EB scalar"))
}

fn triple<'a>(words: &mut impl Iterator<Item = &'a str>) -> Result<[f32; 3]> {
    Ok([
        number(words.next())?,
        number(words.next())?,
        number(words.next())?,
    ])
}

struct Recipe {
    frame: Value,
    data: Vec<u8>,
    field_evidence: usize,
}

fn read(raw: &[u8]) -> Result<Recipe> {
    if !is_selected_eb(raw) {
        return Err(invalid(
            "EB source outside the selected stored-scalar reader rule",
        ));
    }
    let text = std::str::from_utf8(raw).map_err(|_| invalid("EB source encoding"))?;
    let mut words = text.split_ascii_whitespace();
    let marker = words
        .next()
        .filter(|m| matches!(*m, "cell" | "free"))
        .ok_or_else(|| invalid("EB marker"))?;
    let first_lengths = triple(&mut words)?;
    let angles = triple(&mut words)?;
    let mut shape = [0usize; 3];
    for axis in &mut shape {
        *axis = words
            .next()
            .and_then(|word| word.parse::<usize>().ok())
            .filter(|&n| n >= 2)
            .ok_or_else(|| invalid("EB shape"))?;
    }
    let repeated = triple(&mut words)?;
    let size = shape
        .into_iter()
        .try_fold(1usize, usize::checked_mul)
        .filter(|&n| n <= 100_000)
        .ok_or_else(|| invalid("EB scalar count limit"))?;
    let [nx, ny, nz] = shape;
    let mut data = vec![0u8; size * 4];
    for x in 0..nx {
        for y in 0..ny {
            for z in 0..nz {
                let at = 4 * (x + nx * (y + ny * z));
                data[at..at + 4].copy_from_slice(&number(words.next())?.to_le_bytes());
            }
        }
    }
    if words.next().is_some() {
        return Err(invalid("extra EB scalar records"));
    }
    let header = text.split_inclusive('\n').take(4).map(str::len).sum();
    let bits = |values: [f32; 3]| values.map(|value| format!("{:08x}", value.to_bits()));
    let display_lengths =
        repeated.map(|n| (f64::from(n) * f64::from_bits(0x3fe0ef050a9737de)) as f32);
    let display_angles = if marker == "free" { [90.; 3] } else { angles };
    Ok(Recipe {
        data,
        field_evidence: header,
        frame: json!({
            "schemaVersion":"FC-native-scalar-grid-frame-1","scopeInputDigest":digest(raw),
            "pointShape":shape,"axisOrder":"x-fastest","sourceMarker":marker,
            "firstLengthF32Bits":bits(first_lengths),"repeatedLengthF32Bits":bits(repeated),
            "sourceAngleF32Bits":bits(angles),"nativeDisplayLengthF32Bits":bits(display_lengths),
            "nativeDisplayAngleF32Bits":bits(display_angles),"nativeDisplayDivisions":shape.map(|n| n-1),
            "physicalCoordinateContext":"source-unresolved","sourceEndpointPeriodicity":"source-unresolved",
            "displayCoordinateRole":"numerical-native-display-only","ruleId":PRODUCER,"ruleVersion":"1"
        }),
    })
}

const FRAME_POINTER: &str = "/fieldBindings/0/frameResource";
const HEADER_POINTER: &str = "/fieldBindings/0/evidence/0/evidenceResource";
const RAW_POINTER: &str = "/evidence/0/evidenceResource";

fn target_slot(pointer: &str) -> String {
    format!("target/{}", digest(pointer.as_bytes()))
}

fn slots(raw: &[u8], recipe: &Recipe) -> BTreeMap<String, Vec<u8>> {
    let frame = canonical(&recipe.frame);
    let frame_alias = "reader-frame:stored-grid";
    let header_alias = "reader-evidence:header";
    let raw_alias = "reader-evidence:complete-grid";
    let reference = |id: &str, bytes: &[u8]| json!({"resourceId":id,"sha256":digest(bytes),"byteLength":bytes.len().to_string()});
    let evidence = |alias: &str, length: usize| {
        json!({
            "evidenceResource":reference(alias, raw), "byteOffset":"0","byteLength":length.to_string(),
            "sourceSpanDigest":digest(&raw[..length]),"sourceVersion":"selected-EB-wire/1"
        })
    };
    let definition = json!({
        "schemaVersion":"FC-source-descriptor-2","producerId":PRODUCER,"producerVersion":"1",
        "dialect":"selected-EB-wire/1","scopeInputDigest":digest(raw),"unknownSourceFields":[],
        "fieldBindings":[{
            "sourcePointer":"/dataResource","physicalKind":"uninterpreted-stored-scalar",
            "unit":"source-unresolved","basis":"source-unresolved",
            "frameResource":reference(frame_alias, &frame),
            "conversionRuleId":PRODUCER,"conversionRuleVersion":"1",
            "evidence":[evidence(header_alias, recipe.field_evidence)]
        }],
        "evidence":[evidence(raw_alias, raw.len())]
    });
    BTreeMap::from([
        ("scope".into(), raw.to_vec()),
        ("definition".into(), canonical(&definition)),
        (target_slot(FRAME_POINTER), frame),
        (target_slot(HEADER_POINTER), raw.to_vec()),
        (target_slot(RAW_POINTER), raw.to_vec()),
        ("data".into(), recipe.data.clone()),
    ])
}

fn slot_key(pointer: &str, slot: &str) -> String {
    let prefix = digest(pointer.as_bytes());
    if slot == "data" {
        format!("native-scalar-grid/{prefix}/data")
    } else {
        format!("descriptor-data/{prefix}/{slot}")
    }
}

fn raw_ref(r: &PersistentResource) -> Node {
    Node::Resource {
        id: r.id.to_string(),
        sha: r.sha256.clone(),
        len: r.byte_length.clone(),
        role: Role::Reference,
    }
}

fn scalar_node(value: &Value) -> Result<Node> {
    if let Some(text) = value.as_str() {
        Ok(Node::Text(text.into()))
    } else if let Some(n) = value.as_u64() {
        Ok(Node::U64(n))
    } else if let Some(list) = value.as_array() {
        Ok(Node::Array(
            list.iter().map(scalar_node).collect::<Result<_>>()?,
        ))
    } else {
        Err(invalid("unexpected registered grid literal"))
    }
}

fn source_entity(
    id: &str,
    pointer: &str,
    recipe: &Recipe,
    resources: &BTreeMap<String, &PersistentResource>,
) -> Result<Value> {
    let scope = resources["scope"];
    let data = resources["data"];
    let definition = resources["definition"];
    let frame = recipe
        .frame
        .as_object()
        .ok_or_else(|| invalid("grid frame shape"))?;
    let mut payload: BTreeMap<_, _> = frame
        .iter()
        .filter(|(key, _)| *key != "schemaVersion")
        .map(|(key, value)| Ok((key.clone(), scalar_node(value)?)))
        .collect::<Result<_>>()?;
    for (key, literal) in [
        ("schemaVersion", "FC-native-scalar-grid-source-1"),
        ("sourceFormatRef", FORMAT),
        ("datumRole", "uninterpreted-stored-scalar"),
        ("physicalEnergyContext", "source-unresolved"),
        ("sourceBandIdentity", "source-unresolved"),
        ("dataEncoding", "ieee754-little-endian-f32"),
    ] {
        payload.insert(key.into(), Node::Text(literal.into()));
    }
    payload.insert("sourceResource".into(), raw_ref(scope));
    payload.insert("dataResource".into(), raw_ref(data));
    let tree = Node::TaggedVariant {
        tag: TAG.into(),
        payload: Box::new(Node::Map(payload)),
    };
    let text = |value: &str| json!({"kind":"Utf8","value":value});
    let map = |members: Value| json!({"kind":"Map","members":members});
    let entity = |kind: &str, id: &str| json!({"kind":"EntityRef","objectKind":kind,"objectId":id});
    let reference = |r: &PersistentResource| {
        map(json!({
            "resourceId":entity("resource",r.id.as_str()),
            "sha256":text(&r.sha256),"byteLength":text(&r.byte_length)
        }))
    };
    let bindings: Vec<_> = [
        (RAW_POINTER, "reader-evidence:complete-grid"),
        (HEADER_POINTER, "reader-evidence:header"),
        (FRAME_POINTER, "reader-frame:stored-grid"),
    ]
    .into_iter()
    .map(|(pointer, alias)| {
        map(json!({
            "jsonPointer":text(pointer),"capturedAlias":text(alias),
            "target":reference(resources[&target_slot(pointer)])
        }))
    })
    .collect();
    Ok(json!({
        "id":id,"nativeKind":NATIVE_KIND,"sourceTree":tree.wire(),
        "sourceRecords":[{"sourceResourceId":scope.id,"byteOffset":"0","byteLength":scope.byte_length,"sha256":scope.sha256}],
        "dependencyIds":[],"scientificProjection":"not-qualified",
        "metadata":[{
            "owner":{"kind":"source-entity","id":id},"producerId":PRODUCER,
            "producerVersion":"1","formatRef":FORMAT,"dialect":"selected-EB-wire/1","spans":[],
            "tree":{"kind":"TaggedVariant","tag":"BoundDescriptorContext1","payload":map(json!({
                "schemaVersion":text("BoundDescriptorContext-proposal-1"),
                "owner":map(json!({"kind":text("source-entity"),"id":entity("source-entity",id)})),
                "sourceTypedPointer":text(pointer),"definition":reference(definition),"scopeInput":reference(scope),
                "descriptorSchemaDigest":text(&digest(DESCRIPTOR.as_bytes())),
                "referenceRuleDigest":text(&digest(BINDING.as_bytes())),
                "bindings":{"kind":"Array","items":bindings}
            }))}
        }]
    }))
}

pub fn open_selected_eb(raw: &[u8], mut ids: Ids) -> Result<RestoredProject> {
    let recipe = read(raw)?;
    let pointer = native_grid_data_use(0);
    let mut capture = source_only_capture(&mut ids);
    let owned: Vec<_> = slots(raw, &recipe)
        .into_iter()
        .map(|(slot, bytes)| {
            let sha = digest(&bytes);
            let resource = PersistentResource {
                id: ids.next_id().into(),
                logical_key: slot_key(&pointer, &slot),
                byte_length: bytes.len().to_string(),
                sha256: sha.clone(),
                media_type: "application/octet-stream".into(),
            };
            capture.resources.entry(sha).or_insert(bytes);
            (slot, resource)
        })
        .collect();
    capture.content.resources = owned.iter().map(|(_, resource)| resource.clone()).collect();
    let resources: BTreeMap<_, _> = owned
        .iter()
        .map(|(slot, resource)| (slot.clone(), resource))
        .collect();
    let id = ids.next_id();
    let mut value = full_source7::source_only_base(&capture);
    value["sourceEntities"] = json!([source_entity(&id, &pointer, &recipe, &resources)?]);
    value["sourceEntryOrder"] = json!([{"kind":"source-entity","id":id}]);
    let dataid = resources["data"].id.as_str();
    let data_row = value["resources"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|row| row["id"] == dataid)
        .expect("installed data");
    data_row["sourceDescriptorDigest"] = json!(resources["definition"].sha256);
    verify(&capture, &value)?;
    capture.source = Some(SourceDocument::decode(value)?);
    RestoredProject::new(capture, ids)
}

/// Rebuild the independent reader expectation from the installed logical
/// resource table, then compare all three nested target leaves and their bytes.
pub(super) fn verify(capture: &Capture, value: &Value) -> Result<BTreeSet<String>> {
    let mut slots_seen = BTreeSet::new();
    let resources: BTreeMap<_, _> = capture
        .content
        .resources
        .iter()
        .map(|r| (r.logical_key.as_str(), r))
        .collect();
    let read_slot = |pointer: &str, slot: &str| -> Result<(&PersistentResource, &[u8])> {
        let key = slot_key(pointer, slot);
        let resource = *resources
            .get(key.as_str())
            .ok_or_else(|| invalid("missing independent reader resource use"))?;
        let raw = capture
            .resources
            .get(&resource.sha256)
            .filter(|bytes| {
                resource.byte_length == bytes.len().to_string() && digest(bytes) == resource.sha256
            })
            .ok_or_else(|| invalid("reader resource bytes differ"))?;
        Ok((resource, raw))
    };
    for (i, row) in value["sourceEntities"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        if row["nativeKind"] != NATIVE_KIND && row["sourceTree"]["tag"] != TAG {
            continue;
        }
        let id = row["id"]
            .as_str()
            .ok_or_else(|| invalid("grid source owner identity"))?;
        let pointer = native_grid_data_use(i);
        let (_, raw) = read_slot(&pointer, "scope")?;
        // The selected EB grammar and independently reconstructed closed frame
        // are terminal rules. JSON aliases in Descriptor2 do not select either.
        let recipe = read(raw)?;
        let mut expected = BTreeMap::new();
        for (slot, bytes) in slots(raw, &recipe) {
            let (resource, actual) = read_slot(&pointer, &slot)?;
            if actual != bytes.as_slice() {
                return Err(invalid("independent reader target content differs"));
            }
            slots_seen.insert(resource.logical_key.clone());
            expected.insert(slot, resource);
        }
        let data = expected["data"];
        let data_rows = value["resources"].as_array().into_iter().flatten();
        if !data_rows.clone().any(|r| {
            r["id"] == data.id.as_str()
                && r["sourceDescriptorDigest"] == expected["definition"].sha256
        }) || source_entity(id, &pointer, &recipe, &expected)? != *row
        {
            return Err(invalid(
                "grid owner, descriptor or typed resource binding differs",
            ));
        }
    }
    Ok(slots_seen)
}
