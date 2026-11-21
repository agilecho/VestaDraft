//! Strict, read-only Core1 reader. No export path and no fresh identity allocator.
use super::legacy_types::*;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
const HEADER: &str = "#VESTA_FORMAT_VERSION 3.5.4";
const MARKER: &str = "#DRAFT_SOURCE_MANIFEST_V1 ";
const MAX: usize = 16 * 1024 * 1024;
#[path = "native_core1_source7.rs"]
mod current_source7;
const CORE_SCHEMA: &str = include_str!("../../legacy-core1-content.schema.json");
fn error(s: &str) -> Error {
    Error::new("E_REFERENCE_FAILURE", s)
}
fn ser(v: &Value) -> Result<Vec<u8>> {
    serde_json::to_vec(v).map_err(|e| Error::new("E_SCHEMA", e.to_string()))
}
pub(super) fn type_digest() -> String {
    current_source7::type_digest()
}
fn schema_digest() -> String {
    digest(current_source7::SOURCE_SCHEMA.as_bytes())
}
pub(super) fn rule_digest() -> String {
    current_source7::rule_digest()
}
fn leaf_ok(s: &str) -> bool {
    !s.is_empty()
        && s != "."
        && s != ".."
        && s.len() < 240
        && !s.contains(['/', '\\', '\0', '\n', '\r'])
}
fn pointer(p: &str, key: &str) -> String {
    format!("{p}/{}", key.replace('~', "~0").replace('/', "~1"))
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Slot {
    kind: String,
    key: String,
}
fn core_slots(
    c: &ProjectContent,
) -> Result<(BTreeMap<String, Slot>, BTreeMap<String, Slot>, Vec<Value>)> {
    let mut own = BTreeMap::new();
    let mut refs = BTreeMap::new();
    let mut objects = vec![];
    let mut byid = BTreeMap::new();
    let mut add = |p: String, id: &str, kind: &str, key: String| {
        own.insert(
            p,
            Slot {
                kind: kind.into(),
                key: key.clone(),
            },
        );
        byid.insert(
            id.to_string(),
            Slot {
                kind: kind.into(),
                key: key.clone(),
            },
        );
        objects.push(json!({"key":key,"objectKind":kind,"captureObjectId":id}));
    };
    add("/sceneId".into(), &c.scene_id, "Scene", "scene".into());
    for (i, p) in c.phases.iter().enumerate() {
        add(
            format!("/phases/{i}/id"),
            &p.id,
            "Phase",
            format!("phase/{i}"),
        );
        add(
            format!("/phases/{i}/symmetryOpId"),
            &p.symmetry_op_id,
            "SymmetryOp",
            format!("op/{i}"),
        );
        for (j, s) in p.sites.iter().enumerate() {
            add(
                format!("/phases/{i}/sites/{j}/id"),
                &s.id,
                "Site",
                format!("site/{i}/{j}"),
            );
        }
    }
    for (i, k) in c.hidden.iter().enumerate() {
        for (field, id) in [("phase", &k.phase), ("site", &k.site), ("op", &k.op)] {
            refs.insert(
                format!("/hidden/{i}/{field}"),
                byid.get(id)
                    .cloned()
                    .ok_or_else(|| error("hidden reference missing"))?,
            );
        }
    }
    Ok((own, refs, objects))
}
fn stored(
    v: &Value,
    path: &str,
    slots: &BTreeMap<String, Slot>,
    resource_slots: &BTreeMap<String, Value>,
) -> Result<Value> {
    if let Some(s) = slots.get(path) {
        return Ok(json!({"kind":"EntityRef","objectKind":s.kind,"objectKey":s.key}));
    }
    if let Some(r) = resource_slots.get(path) {
        return Ok(r.clone());
    }
    Ok(match v {
        Value::Null => json!({"kind":"Null"}),
        Value::Bool(v) => json!({"kind":"Bool","value":v}),
        Value::String(v) => json!({"kind":"Utf8","value":v}),
        Value::Number(v) => {
            if let Some(n) = v.as_u64() {
                if !v.is_f64() {
                    return Ok(json!({"kind":"U64","value":n.to_string()}));
                }
            }
            if let Some(n) = v.as_i64() {
                if !v.is_f64() {
                    return Ok(json!({"kind":"I64","value":n.to_string()}));
                }
            }
            let n = v.as_f64().ok_or_else(|| error("number representation"))?;
            json!({"kind":"F64Bits","ieeeHexBE":format!("{:016x}",n.to_bits())})
        }
        Value::Array(a) => {
            json!({"kind":"Array","items":a.iter().enumerate().map(|(i,v)|stored(v,&format!("{path}/{i}"),slots,resource_slots)).collect::<Result<Vec<_>>>()?})
        }
        Value::Object(o) => {
            let m = o
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        stored(v, &pointer(path, k), slots, resource_slots)?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>>>()?;
            json!({"kind":"Map","members":m})
        }
    })
}
fn node_pointer(p: &str, value: &Value) -> Result<String> {
    let mut out = String::new();
    let mut cur = value;
    for part in p.split('/').skip(1) {
        if cur.is_array() {
            out.push_str("/items/");
            out.push_str(part);
            cur = cur
                .get(part.parse::<usize>().map_err(|_| error("slot index"))?)
                .ok_or_else(|| error("slot array"))?;
        } else {
            out.push_str("/members/");
            out.push_str(part);
            let key = part.replace("~1", "/").replace("~0", "~");
            cur = cur.get(&key).ok_or_else(|| error("slot map"))?;
        }
    }
    Ok(out)
}
fn slot_rows(slots: &BTreeMap<String, Slot>, value: &Value) -> Result<Vec<Value>> {
    slots.iter().map(|(p,s)|Ok(json!({"typedSourcePointer":p,"storedNodePointer":node_pointer(p,value)?,"objectKind":s.kind,"objectKey":s.key}))).collect()
}
fn projection(c: &ProjectContent, stamp: &str) -> Result<Vec<u8>> {
    use std::fmt::Write;
    c.validate()?;
    let p = &c.phases[0];
    let mut s = format!(
        "{HEADER}\n{MARKER}{stamp}\n\nCRYSTAL\n\nTITLE\n{}\n\nGROUP\n1 1 P 1\nSYMOP\n0 0 0 1 0 0 0 1 0 0 0 1 1\n -1 -1 -1 0 0 0 0 0 0 0 0 0\nCELLP\n",
        p.label
    );
    let n: Vec<_> = p
        .cell
        .length_a
        .iter()
        .chain(p.cell.angles_deg.iter())
        .map(|v| format!("{v:.6}"))
        .collect();
    s.push_str(&n.join(" "));
    s.push_str("\n0 0 0 0 0 0\nSTRUC\n");
    for (i, a) in p.sites.iter().enumerate() {
        if a.label.len() > 10 || a.label.chars().any(char::is_whitespace) {
            return Err(Error::new(
                "E_FORMAT_LOSS",
                "native atom labels require 1..10 non-space UTF-8 bytes",
            ));
        }
        writeln!(
            s,
            "{} {} {} {:.4} {:.6} {:.6} {:.6} 1a 1\n0 0 0 0",
            i + 1,
            a.element,
            a.label,
            a.occupancy,
            a.frac[0],
            a.frac[1],
            a.frac[2]
        )
        .map_err(|_| error("native string"))?;
    }
    s.push_str("0 0 0 0 0 0 0\nTHERI 1\n");
    for (i, a) in p.sites.iter().enumerate() {
        writeln!(s, "{} {} {:.6}", i + 1, a.label, a.u_iso_a2)
            .map_err(|_| error("native string"))?;
    }
    s.push_str("0 0 0\nBOUND\n");
    for i in 0..3 {
        write!(s, "{:.6} {:.6} ", p.boundary_min[i], p.boundary_max[i])
            .map_err(|_| error("native string"))?;
    }
    s.push_str("\n0 0 0 0 0\nSCENE\n");
    for i in 0..3 {
        writeln!(
            s,
            "{:.6} {:.6} {:.6} 0",
            c.camera.rotation[i][0], c.camera.rotation[i][1], c.camera.rotation[i][2]
        )
        .map_err(|_| error("native string"))?;
    }
    writeln!(
        s,
        "0 0 0 1\n0 0\n0\n{:.6}\nSTYLE\nDISPF 0\nMODEL 0 1 0\nPROJT {} 1\nCOMMENT\n{}",
        c.camera.zoom,
        if c.camera.projection == "parallel" {
            0
        } else {
            1
        },
        c.comment
    )
    .map_err(|_| error("native string"))?;
    Ok(s.into_bytes())
}
fn safe_path(path: &str) -> bool {
    let Some((root, tail)) = path.split_once('/') else {
        return false;
    };
    leaf_ok(root) && root.ends_with(".resources") && safe_key(tail)
}
fn restore_node(
    v: &Value,
    objects: &BTreeMap<String, (String, String)>,
    resources: &BTreeMap<String, (String, Vec<u8>)>,
    depth: usize,
) -> Result<Value> {
    if depth > 64 {
        return Err(error("stored tree depth"));
    }
    let kind = v["kind"].as_str().ok_or_else(|| error("stored node tag"))?;
    let o = v.as_object().ok_or_else(|| error("stored node"))?;
    let expected_keys: &[&str] = match kind {
        "Null" => &["kind"],
        "F64Bits" | "F32Bits" => &["kind", "ieeeHexBE"],
        "Array" => &["kind", "items"],
        "Map" => &["kind", "members"],
        "EntityRef" => &["kind", "objectKind", "objectKey"],
        "BytesRef" => &["kind", "resourceKey", "sha256", "byteLength"],
        "Bool" | "I64" | "U64" | "Utf8" => &["kind", "value"],
        _ => return Err(error("unregistered stored node kind")),
    };
    if o.len() != expected_keys.len() || expected_keys.iter().any(|k| !o.contains_key(*k)) {
        return Err(error("stored node unknown/missing field"));
    }
    Ok(match kind {
        "Null" => Value::Null,
        "Bool" => Value::Bool(v["value"].as_bool().ok_or_else(|| error("bool"))?),
        "Utf8" => json!(v["value"].as_str().ok_or_else(|| error("utf8 value"))?),
        "U64" => json!(vesta_model::json::u64_str(&v["value"])?),
        "I64" => json!(
            v["value"]
                .as_str()
                .and_then(|s| s.parse::<i64>().ok())
                .ok_or_else(|| error("i64"))?
        ),
        "F64Bits" | "F32Bits" => {
            let s = v["ieeeHexBE"].as_str().ok_or_else(|| error("bits"))?;
            if s.len() != if kind == "F64Bits" { 16 } else { 8 }
                || !s
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(error("IEEE encoding"));
            }
            let n = u64::from_str_radix(s, 16).map_err(|_| error("bits"))?;
            let n = if kind == "F64Bits" {
                f64::from_bits(n)
            } else {
                f32::from_bits(n as u32) as f64
            };
            if !n.is_finite() {
                return Err(error("nonfinite stored value"));
            }
            json!(n)
        }
        "Array" => Value::Array(
            v["items"]
                .as_array()
                .ok_or_else(|| error("items"))?
                .iter()
                .map(|v| restore_node(v, objects, resources, depth + 1))
                .collect::<Result<_>>()?,
        ),
        "Map" => {
            let a = v["members"].as_object().ok_or_else(|| error("members"))?;
            Value::Object(
                a.iter()
                    .map(|(k, v)| Ok((k.clone(), restore_node(v, objects, resources, depth + 1)?)))
                    .collect::<Result<_>>()?,
            )
        }
        "EntityRef" => {
            let k = v["objectKey"].as_str().ok_or_else(|| error("object key"))?;
            let (kind, id) = objects
                .get(k)
                .ok_or_else(|| error("missing object definition"))?;
            if v["objectKind"] != *kind {
                return Err(error("object kind mismatch"));
            }
            json!(id)
        }
        "BytesRef" => {
            let k = v["resourceKey"]
                .as_str()
                .ok_or_else(|| error("resource key"))?;
            let (hash, bytes) = resources
                .get(k)
                .ok_or_else(|| error("missing typed bytes"))?;
            if v["sha256"] != *hash || v["byteLength"] != bytes.len().to_string() {
                return Err(error("typed BytesRef binding"));
            }
            json!(hash)
        }
        _ => return Err(error("node")),
    })
}
pub(super) fn decode(primary: &[u8], deps: &BTreeMap<String, Vec<u8>>) -> Result<Capture> {
    let total = deps
        .values()
        .try_fold(primary.len(), |n, b| n.checked_add(b.len()))
        .ok_or_else(|| error("input size overflow"))?;
    if total > MAX {
        return Err(Error::new("E_RESOURCE_LIMIT", "native input budget"));
    }
    let text = std::str::from_utf8(primary).map_err(|_| error("native UTF8"))?;
    let lines: Vec<_> = text.lines().collect();
    if lines.first() != Some(&HEADER) {
        return Err(Error::new(
            "E_FORMAT_UNSUPPORTED",
            "only stamped current 3.5.4 engineering artifacts supported",
        ));
    }
    let records: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, s)| s.trim_start().starts_with("#DRAFT_SOURCE_MANIFEST_"))
        .collect();
    if records.len() != 1 || records[0].0 != 1 || !text.starts_with(&format!("{HEADER}\n")) {
        return Err(error("E_SOURCE_STAMP: moved/duplicate/reserved marker"));
    }
    let stamp = lines[1]
        .strip_prefix(MARKER)
        .ok_or_else(|| error("E_SOURCE_STAMP: malformed namespace"))?;
    if stamp.len() != 64
        || !stamp
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(error("E_SOURCE_STAMP: SHA"));
    }
    let p_hash = digest(primary);
    let suffix = format!("/manifests/{p_hash}.json");
    let ms: Vec<_> = deps.iter().filter(|(p, _)| p.ends_with(&suffix)).collect();
    if ms.len() != 1 {
        return Err(error("exact actual primary manifest required"));
    }
    let (mp, mb) = ms[0];
    if !safe_path(mp) {
        return Err(error("unsafe manifest path"));
    }
    let manifest = vesta_model::json::parse(mb, MAX)?;
    let required = [
        "schemaVersion",
        "primarySha256",
        "sourceDocumentRelativePath",
        "sourceDocumentSha256",
        "sourceDocumentByteLength",
        "sourceDTOSchemaDigest",
        "sourceRuleSetDigest",
        "dependencies",
    ];
    if manifest
        .as_object()
        .is_none_or(|o| o.len() != required.len() || required.iter().any(|k| !o.contains_key(*k)))
        || manifest["schemaVersion"] != "FC-native-source-manifest-2-proposal"
        || manifest["primarySha256"] != p_hash
        || manifest["sourceDocumentSha256"] != stamp
        || manifest["sourceDTOSchemaDigest"] != schema_digest()
        || manifest["sourceRuleSetDigest"] != rule_digest()
    {
        return Err(error("native manifest binding/schema"));
    }
    let entries = manifest["dependencies"]
        .as_object()
        .ok_or_else(|| error("dependency table"))?;
    let mut keys: BTreeSet<_> = entries.keys().cloned().collect();
    keys.insert(mp.clone());
    if keys != deps.keys().cloned().collect() {
        return Err(error("exact artifact file set differs"));
    }
    for (p, desc) in entries {
        if !safe_path(p)
            || desc["relativePath"] != *p
            || desc.as_object().is_none_or(|o| o.len() != 4)
        {
            return Err(error("dependency key/closed descriptor"));
        }
        let b = deps.get(p).ok_or_else(|| error("dependency missing"))?;
        if desc["sha256"] != digest(b)
            || vesta_model::json::u64_str(&desc["byteLength"])? != b.len() as u64
            || ![
                "source-resource",
                "context-resource",
                "native-projection-resource",
            ]
            .contains(&desc["role"].as_str().unwrap_or(""))
        {
            return Err(error("dependency bytes differ"));
        }
    }
    let source_path = manifest["sourceDocumentRelativePath"]
        .as_str()
        .ok_or_else(|| error("source document path"))?;
    let source = deps
        .get(source_path)
        .ok_or_else(|| error("source absent"))?;
    if digest(source) != stamp
        || vesta_model::json::u64_str(&manifest["sourceDocumentByteLength"])? != source.len() as u64
    {
        return Err(error("source bytes mismatch"));
    }
    let root = mp
        .strip_suffix(&suffix)
        .ok_or_else(|| error("manifest root"))?;
    if source_path != format!("{root}/blobs/{stamp}.source.json") {
        return Err(error("source layout"));
    }
    let saved = vesta_model::json::parse(source, MAX)?;
    let required = [
        "schemaVersion",
        "sourceTypeDigest",
        "sourceDTOSchemaDigest",
        "sourceRuleSetDigest",
        "capturedSourceLedgerDigest",
        "capturedSnapshot",
        "objects",
        "identitySlots",
        "referenceSlots",
        "resources",
        "tree",
    ];
    if saved
        .as_object()
        .is_none_or(|o| o.len() != required.len() || required.iter().any(|k| !o.contains_key(*k)))
        || saved["schemaVersion"] != "FC-native-saved-source-1-proposal"
        || saved["sourceTypeDigest"] != type_digest()
        || saved["sourceDTOSchemaDigest"] != schema_digest()
        || saved["sourceRuleSetDigest"] != rule_digest()
    {
        return Err(Error::new(
            "E_FORMAT_UNSUPPORTED",
            "source type/schema/rules not qualified",
        ));
    }
    let mut objects = BTreeMap::new();
    for x in saved["objects"]
        .as_array()
        .ok_or_else(|| error("objects"))?
    {
        let k = x["key"].as_str().ok_or_else(|| error("object key"))?;
        let kind = x["objectKind"]
            .as_str()
            .ok_or_else(|| error("object kind"))?;
        let id = x["captureObjectId"]
            .as_str()
            .ok_or_else(|| error("object id"))?;
        if x.as_object().is_none_or(|o| o.len() != 3)
            || objects
                .insert(k.to_owned(), (kind.to_owned(), id.to_owned()))
                .is_some()
        {
            return Err(error("object definitions"));
        }
    }
    let mut resources = BTreeMap::new();
    let mut allbytes = BTreeMap::new();
    let mut complete_paths = BTreeSet::from([mp.clone(), source_path.to_owned()]);
    for x in saved["resources"]
        .as_array()
        .ok_or_else(|| error("resources"))?
    {
        let key = x["key"].as_str().ok_or_else(|| error("resource key"))?;
        let path = x["relativeBlobPath"]
            .as_str()
            .ok_or_else(|| error("blob path"))?;
        let hash = x["sha256"]
            .as_str()
            .ok_or_else(|| error("resource digest"))?;
        let b = deps.get(path).ok_or_else(|| error("resource missing"))?;
        complete_paths.insert(path.to_owned());
        if x.as_object().is_none_or(|o| o.len() != 7)
            || path != format!("{root}/blobs/{hash}.raw.bin")
            || digest(b) != hash
            || vesta_model::json::u64_str(&x["byteLength"])? != b.len() as u64
            || resources
                .insert(key.into(), (hash.into(), b.clone()))
                .is_some()
        {
            return Err(error("resource binding"));
        }
        allbytes.insert(hash.into(), b.clone());
    }
    if complete_paths != deps.keys().cloned().collect() {
        return Err(error(
            "unreferenced artifact outside complete source closure",
        ));
    }
    let restored = restore_node(&saved["tree"], &objects, &resources, 0)?;
    let captured = &saved["capturedSnapshot"];
    let text = |k: &str| {
        captured[k]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| error("captured snapshot identity"))
    };
    let old_doc = DocumentAt {
        document_id: text("documentId")?,
        epoch: text("epoch")?,
        revision: text("revision")?,
    };
    if !valid_id(&old_doc.document_id)
        || !valid_id(&old_doc.epoch)
        || vesta_model::json::u64_str(&json!(old_doc.revision)).is_err()
    {
        return Err(error("capture lifetime identity"));
    }
    if *captured != restored["identity"] {
        return Err(error("Source7 capture identity differs"));
    }
    let content = current_source7::restore(&restored, &old_doc)?;
    if saved["capturedSourceLedgerDigest"] != content.digest()
        || saved["capturedSnapshot"]["sourceRootDigest"] != content.digest()
    {
        return Err(error("complete source content differs from capture"));
    }
    let (own, refs, expected_objects) = current_source7::roles(&content)?;
    if saved["objects"] != json!(expected_objects)
        || saved["identitySlots"] != json!(slot_rows(&own, &restored)?)
        || saved["referenceSlots"] != json!(slot_rows(&refs, &restored)?)
    {
        return Err(error("typed identity visitor coverage mismatch"));
    }
    let mut combined = own;
    combined.extend(refs);
    let rs = current_source7::resource_slots(&content)?;
    for (i, r) in content.resources.iter().enumerate() {
        let expected_binding = json!({"key":format!("resource/{i}"),"capturedResourceId":r.sha256,"sha256":r.sha256,"byteLength":r.byte_length,"relativeBlobPath":format!("{root}/blobs/{}.raw.bin",r.sha256),"storageFormat":"raw.bin","sourceDescriptorDigest":digest(&ser(&serde_json::to_value(r).map_err(|e|Error::new("E_SCHEMA",e.to_string()))?)?)});
        if saved["resources"].get(i) != Some(&expected_binding) {
            return Err(error("full resource source descriptor differs"));
        }
        let actual = resources
            .get(&format!("resource/{i}"))
            .ok_or_else(|| error("source resource population"))?;
        if actual.0 != r.sha256 || actual.1.len().to_string() != r.byte_length {
            return Err(error("resource semantic binding"));
        }
    }
    if stored(&restored, "", &combined, &rs)? != saved["tree"]
        || projection(&content, stamp)? != primary
    {
        return Err(error("native/source full content comparison differs"));
    }
    if content.resources.len() != resources.len() {
        return Err(error("unexpected source resource"));
    }
    Ok(Capture {
        state_digest: content.digest(),
        content,
        document: old_doc,
        resources: allbytes,
    })
}
