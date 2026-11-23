//! The sole current native writer. Pure bytes; no filesystem fallback or Saved authority.
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::*;
use vesta_model::source7::resource_graph::CAPTURE_MEDIA;
use super::{current_source7, full_source7, graph_provenance};
const HEADER: &str = "#VESTA_FORMAT_VERSION 3.5.4";
const MARKER: &str = "#DRAFT_SOURCE_MANIFEST_V1 ";
const MAX: usize = 16 * 1024 * 1024;
#[path = "native_core1_migration.rs"]
mod native_core1_migration;
#[derive(Clone, Debug)]
pub struct SaveBundle {
    pub primary: Vec<u8>,
    pub dependencies: BTreeMap<String, Vec<u8>>,
}
pub(super) fn error(s: &str) -> Error {
    Error::new("E_REFERENCE_FAILURE", s)
}
fn ser(v: &Value) -> Result<Vec<u8>> {
    serde_json::to_vec(v).map_err(|e| Error::new("E_SCHEMA", e.to_string()))
}
fn type_digest() -> String {
    current_source7::type_digest()
}
fn schema_digest() -> String {
    digest(current_source7::SOURCE_SCHEMA.as_bytes())
}
fn rule_digest() -> String {
    current_source7::rule_digest()
}
fn leaf_ok(s: &str) -> bool {
    !s.is_empty()
        && s != "."
        && s != ".."
        && s.len() < 240
        && !s.contains(['/', '\\', '\0', '\n', '\r'])
}
pub(super) fn pointer(p: &str, key: &str) -> String {
    format!("{p}/{}", key.replace('~', "~0").replace('/', "~1"))
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Slot {
    pub(super) kind: String,
    pub(super) key: String,
}
fn stored_typed(
    n: &vesta_model::source_audit::Node,
    path: &str,
    slots: &BTreeMap<String, Slot>,
    resources: &BTreeMap<String, Value>,
) -> Result<Value> {
    use vesta_model::source_audit::Node;
    if let Some(s) = slots.get(path) {
        return Ok(json!({"kind":"EntityRef","objectKind":s.kind,"objectKey":s.key}));
    }
    if let Some(r) = resources.get(path) {
        return Ok(r.clone());
    }
    Ok(match n {
        Node::Owned { node, .. } => stored_typed(node, path, slots, resources)?,
        Node::Array(a) => {
            json!({"kind":"Array","items":a.iter().enumerate().map(|(i,n)|stored_typed(n,&format!("{path}/{i}"),slots,resources)).collect::<Result<Vec<_>>>()?})
        }
        Node::Map(m) => {
            json!({"kind":"Map","members":m.iter().map(|(k,n)|Ok((k.clone(),stored_typed(n,&pointer(path,k),slots,resources)?))).collect::<Result<BTreeMap<_,_>>>()?})
        }
        Node::Identity { .. } | Node::Resource { .. } => {
            return Err(error("uncovered generated role"));
        }
        _ => n.wire(),
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
    for text in c
        .phases
        .iter()
        .map(|p| p.label.as_str())
        .chain(std::iter::once(c.comment.as_str()))
    {
        if text
            .lines()
            .any(|l| l.trim_start().starts_with("#DRAFT_SOURCE_MANIFEST_"))
        {
            return Err(error(
                "E_SOURCE_STAMP: user text occupies reserved namespace",
            ));
        }
    }
    let Some(p) = c.phases.first() else {
        return Ok(format!("{HEADER}\n{MARKER}{stamp}\n\nCOMMENT\n{}", c.comment).into_bytes());
    };
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
fn capture_for_save(source: &Capture) -> Result<(Value, vesta_model::source_audit::Node)> {
    full_source7::capture(source)
}

/// Open the exact selected R80 original with the already captured file closure.
pub fn open_selected_orffe(archive: &super::resource_graph::GraphArchive, mut ids: Ids) -> Result<RestoredProject> {
    use super::resource_graph::{import_r80_phase, materialize_r80_orffe, r80_resource_key};

    let import = import_r80_phase(archive, &mut ids)?;
    let table_key = r80_resource_key(archive)?;
    let mut capture = graph_import_capture(archive, vec![import.phase().clone()], &mut ids)?;
    let table_resource = capture.content.resources.iter().find(|r| r.logical_key == table_key)
        .ok_or_else(|| error("selected ORFFE table resource missing"))?;
    let table = materialize_r80_orffe(archive, &import, import.phase(), table_resource)?;
    capture.source = Some(full_source7::with_orffe(&capture, &table)?);
    capture_for_save(&capture)?;
    RestoredProject::new(capture, ids)
}

pub fn open_selected_density(
    archive: &super::resource_graph::GraphArchive,
    mut ids: Ids,
) -> Result<RestoredProject> {
    let (phase, descriptor, data) = super::resource_graph::read_c1_density(archive, &mut ids)?;
    let mut capture = graph_import_capture(archive, vec![phase], &mut ids)?;
    let field_id = ids.next_id();
    let resource = PersistentResource {
        id: ids.next_id().into(),
        logical_key: "field-data/0/phase/0".into(),
        sha256: descriptor.resource.digest.clone(),
        byte_length: descriptor.resource.bytes.to_string(),
        media_type: "application/octet-stream".into(),
    };
    capture.resources.insert(resource.sha256.clone(), data.to_vec());
    capture.content.resources.push(resource.clone());
    capture.source = Some(full_source7::with_c1_density(&capture, &field_id, &descriptor, &resource)?);
    capture_for_save(&capture)?;
    RestoredProject::new(capture, ids)
}

pub fn open_selected_rietan(
    archive: &super::resource_graph::GraphArchive,
    mut ids: Ids,
) -> Result<RestoredProject> {
    use super::resource_graph::{read_rietan_phase, rietan_binding};

    let phase = read_rietan_phase(archive, &mut ids)?;
    let mut capture = graph_import_capture(archive, Vec::new(), &mut ids)?;
    let root_index = archive.graph.files.iter().position(|file| file.key == archive.graph.root_key)
        .ok_or_else(|| error("selected RIETAN source root missing"))?;
    let primary = &capture.content.resources[root_index + 1];
    let table = rietan_binding(archive, &phase, &capture.content.resources)?;
    capture.source = Some(full_source7::with_rietan(&capture, phase, table.as_ref(), primary)?);
    capture_for_save(&capture)?;
    RestoredProject::new(capture, ids)
}

fn graph_import_capture(
    archive: &super::resource_graph::GraphArchive,
    phases: Vec<Phase>,
    ids: &mut Ids,
) -> Result<Capture> {
    use vesta_model::source7::resource_graph::GRAPH_GROUP;

    let graph = serde_json::to_value(&archive.graph).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    let graph_bytes = vesta_model::source_audit::canonical(&graph);
    let prefix = format!("{GRAPH_GROUP}/{}", digest(&graph_bytes));
    let mut bindings = Vec::new();
    let mut blobs = BTreeMap::new();
    let mut bind = |key: String, media: &str, raw: &[u8]| {
        let sha = digest(raw);
        bindings.push(PersistentResource {
            id: ids.next_id().into(), logical_key: key, byte_length: raw.len().to_string(),
            sha256: sha.clone(), media_type: media.into(),
        });
        blobs.entry(sha).or_insert_with(|| raw.to_vec());
    };
    bind(format!("{prefix}/capture"), CAPTURE_MEDIA, &graph_bytes);
    for (i, file) in archive.graph.files.iter().enumerate() {
        let raw = archive.files.get(&file.key).ok_or_else(|| error("selected ORFFE graph file missing"))?;
        bind(format!("{prefix}/file/{i}"), "application/octet-stream", raw);
    }
    let mut capture = source_only_capture(ids);
    capture.content.phases = phases;
    capture.content.resources = bindings;
    capture.resources = blobs;
    capture.state_digest = capture.content.digest();
    Ok(capture)
}

pub(super) fn source_only_capture(ids: &mut Ids) -> Capture {
    let content = ProjectContent {
        native_camera: None,
        phases: vec![],
        scene_id: ids.next_id().into(),
        camera: Camera {
            rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            projection: "parallel".into(), zoom: 1.,
        },
        hidden: Vec::new(), comment: String::new(), resources: vec![], legacy_source: None,
    };
    Capture {
        document: DocumentAt { document_id: ids.next_id(), epoch: ids.next_id(), revision: "0".into() },
        state_digest: content.digest(), content, resources: BTreeMap::new(), source: None,
    }
}

/// Verify the produced artifact against the actual source registered before writing it.
/// This checks the in-memory candidate; the engineering host still verifies published bytes.
pub fn encode_verified(source: &Capture, leaf: &str, ids: Ids) -> Result<SaveBundle> {
    let (expected, _) = capture_for_save(source)?;
    let bundle = encode(source, leaf)?;
    decode_checked(&bundle.primary, &bundle.dependencies, ids, Some(&expected))?;
    Ok(bundle)
}

fn resource_descriptor<'a>(value: &'a Value, resource: &PersistentResource) -> Result<&'a str> {
    value["resources"].as_array().and_then(|rows| rows.iter().find(|row| row["id"] == resource.id.as_str()))
        .and_then(|row| row["sourceDescriptorDigest"].as_str())
        .ok_or_else(|| error("typed resource descriptor absent"))
}

pub fn encode(source: &Capture, leaf: &str) -> Result<SaveBundle> {
    source.content.validate()?;
    if !leaf_ok(leaf) {
        return Err(error("unsafe output leaf"));
    }
    if source.state_digest != Capture::digest_state(&source.content, source.source.as_ref())? {
        return Err(error("source capture digest mismatch"));
    }
    let root = format!("{leaf}.resources");
    let mut deps = BTreeMap::new();
    let mut context_paths = BTreeSet::new();
    let (value, node) = capture_for_save(source)?;
    let (own, refs, objects, resource_slots) = current_source7::roles(&node)?;
    let mut all = own.clone();
    all.extend(refs.clone());

    let mut bindings = vec![];
    let expected: BTreeSet<_> = source
        .content
        .resources
        .iter()
        .map(|r| r.sha256.clone())
        .collect();
    if expected != source.resources.keys().cloned().collect() {
        return Err(error("source capture exact resource coverage"));
    }
    for r in &source.content.resources {
        let descriptor = resource_descriptor(&value, r)?;
        let bytes = source
            .resources
            .get(&r.sha256)
            .ok_or_else(|| error("missing actual source bytes"))?;
        if digest(bytes) != r.sha256 || bytes.len().to_string() != r.byte_length {
            return Err(error("source resource binding mismatch"));
        }
        let path = format!("{root}/blobs/{}.raw.bin", r.sha256);
        if r.media_type == CAPTURE_MEDIA {
            context_paths.insert(path.clone());
        }
        deps.insert(path.clone(), bytes.clone());
        let key = format!("resource/{}", r.id);
        bindings.push(json!({"key":key,"capturedResourceId":r.id,"sha256":r.sha256,"byteLength":r.byte_length,"relativeBlobPath":path,"storageFormat":"raw.bin","sourceDescriptorDigest":descriptor}));
    }
    let tree = stored_typed(&node, "", &all, &resource_slots)?;
    let snapshot = &value["identity"];
    let source_rule = &snapshot["policyDigest"];
    let ledger = digest(&vesta_model::source_audit::canonical(&node.ledger(snapshot)));
    let source_doc = json!({"schemaVersion":"FC-native-saved-source-1-proposal","sourceTypeDigest":snapshot["sourceTypeDigest"],"sourceDTOSchemaDigest":schema_digest(),"sourceRuleSetDigest":source_rule,"capturedSourceLedgerDigest":ledger,"capturedSnapshot":snapshot,"objects":objects,"identitySlots":slot_rows(&own,&value)?,"referenceSlots":slot_rows(&refs,&value)?,"resources":bindings,"tree":tree});
    current_source7::envelope(&source_doc, "SavedSourceDocument")?;
    let bytes = ser(&source_doc)?;
    let source_hash = digest(&bytes);
    let source_path = format!("{root}/blobs/{source_hash}.source.json");
    let primary = projection(&source.content, &source_hash)?;
    let primary_hash = digest(&primary);
    deps.insert(source_path.clone(), bytes.clone());
    context_paths.insert(source_path.clone());
    let entries:BTreeMap<_,_>=deps.iter().map(|(p,b)|(p.clone(),json!({"relativePath":p,"sha256":digest(b),"byteLength":b.len().to_string(),"role":if context_paths.contains(p){"context-resource"}else{"source-resource"}}))).collect();
    let manifest = json!({"schemaVersion":"FC-native-source-manifest-2-proposal","primarySha256":primary_hash,"sourceDocumentRelativePath":source_path,"sourceDocumentSha256":source_hash,"sourceDocumentByteLength":bytes.len().to_string(),"sourceDTOSchemaDigest":schema_digest(),"sourceRuleSetDigest":source_rule,"dependencies":entries});
    current_source7::envelope(&manifest, "Manifest")?;
    deps.insert(
        format!("{root}/manifests/{primary_hash}.json"),
        ser(&manifest)?,
    );
    let total = deps
        .values()
        .try_fold(primary.len(), |n, b| n.checked_add(b.len()))
        .ok_or_else(|| error("bundle size overflow"))?;
    if total > MAX {
        return Err(Error::new("E_RESOURCE_LIMIT", "native output budget"));
    }
    Ok(SaveBundle {
        primary,
        dependencies: deps,
    })
}
fn restore_node(
    v: &Value,
    objects: &BTreeMap<String, (String, String)>,
    resources: &BTreeMap<String, (String, String, Vec<u8>)>,
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
            let (id, hash, bytes) = resources
                .get(k)
                .ok_or_else(|| error("missing typed bytes"))?;
            if v["sha256"] != *hash || v["byteLength"] != bytes.len().to_string() {
                return Err(error("typed BytesRef binding"));
            }
            json!(id)
        }
        _ => return Err(error("node")),
    })
}
pub fn decode(
    primary: &[u8],
    deps: &BTreeMap<String, Vec<u8>>,
    ids: Ids,
) -> Result<RestoredProject> {
    decode_checked(primary, deps, ids, None)
}

fn decode_checked(
    primary: &[u8],
    deps: &BTreeMap<String, Vec<u8>>,
    ids: Ids,
    registered_source: Option<&Value>,
) -> Result<RestoredProject> {
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
    current_source7::envelope(&manifest, "Manifest")?;
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
    current_source7::envelope(&saved, "SavedSourceDocument")?;
    if let Some(version) = native_core1_migration::qualified_version(&saved) {
        if registered_source.is_some() {
            return Err(error("candidate writer emitted retired Core1 rules"));
        }
        return native_core1_migration::migrate(primary, deps, source, ids, version);
    }
    let old_rule = rule_digest();
    let old_type = type_digest();
    let graph_rule = graph_provenance::rule_digest(&old_rule);
    let graph_type = graph_provenance::type_digest(&old_type);
    let (expanded_type, expanded_rule) = graph_provenance::expanded_digests(&old_type, &old_rule);
    let full_version = full_source7::Version::qualified(&saved);
    let full = full_version.is_some();
    if registered_source.is_some() && full_version == Some(full_source7::Version::LegacyDisplay1) {
        return Err(error("candidate writer emitted retired full source rules"));
    }
    if manifest["sourceRuleSetDigest"] != saved["sourceRuleSetDigest"]
        || manifest["sourceDTOSchemaDigest"] != schema_digest()
    {
        return Err(error("manifest rules not qualified"));
    }
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
        || saved["sourceDTOSchemaDigest"] != schema_digest()
        || !(full || (saved["sourceTypeDigest"] == old_type && saved["sourceRuleSetDigest"] == old_rule)
            || (saved["sourceTypeDigest"] == graph_type && saved["sourceRuleSetDigest"] == graph_rule)
            || (saved["sourceTypeDigest"] == expanded_type && saved["sourceRuleSetDigest"] == expanded_rule))
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
                .insert(
                    key.into(),
                    (
                        x["capturedResourceId"]
                            .as_str()
                            .ok_or_else(|| error("resource identity"))?
                            .to_owned(),
                        hash.into(),
                        b.clone(),
                    ),
                )
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
    if registered_source.is_some_and(|expected| *expected != restored) {
        return Err(error("candidate differs from registered source"));
    }
    let (content, source) = if full {
        let source = vesta_model::source7::SourceDocument::decode(restored.clone())?;
        (full_source7::restore_core(&restored)?, Some(source))
    } else {
        (current_source7::restore(&restored, &old_doc, &allbytes)?, None)
    };
    let capture = Capture {
        document: old_doc,
        content,
        state_digest: String::new(),
        resources: allbytes,
        source,
    };
    let content = &capture.content;
    if !full && graph_provenance::has_graph(&capture)
        != (saved["sourceTypeDigest"] == graph_type || saved["sourceTypeDigest"] == expanded_type)
    {
        return Err(error("source graph type does not match capture"));
    }
    let mut contexts = BTreeSet::from([source_path.to_owned()]);
    contexts.extend(content.resources.iter().filter(|r| r.media_type == CAPTURE_MEDIA)
        .map(|r| format!("{root}/blobs/{}.raw.bin", r.sha256)));
    for (key, row) in entries {
        let role = if contexts.contains(key) { "context-resource" } else { "source-resource" };
        if row["role"] != role {
            return Err(error("artifact dependency role differs"));
        }
    }
    let (actual, node) = if let Some(version) = full_version {
        full_source7::capture_version(&capture, version)?
    } else {
        current_source7::capture(&capture)?
    };
    if actual != restored
        || saved["capturedSourceLedgerDigest"] != digest(&vesta_model::source_audit::canonical(&node.ledger(captured)))
        || saved["capturedSnapshot"] != actual["identity"]
    {
        return Err(error("complete source content differs from capture"));
    }
    let (own, refs, expected_objects, rs) = current_source7::roles(&node)?;
    if saved["objects"] != json!(expected_objects)
        || saved["identitySlots"] != json!(slot_rows(&own, &restored)?)
        || saved["referenceSlots"] != json!(slot_rows(&refs, &restored)?)
    {
        return Err(error("typed identity visitor coverage mismatch"));
    }
    let mut combined = own;
    combined.extend(refs);

    for (i, r) in content.resources.iter().enumerate() {
        let descriptor = resource_descriptor(&actual, r)?;
        let expected_binding = json!({"key":format!("resource/{}",r.id),"capturedResourceId":r.id,"sha256":r.sha256,"byteLength":r.byte_length,"relativeBlobPath":format!("{root}/blobs/{}.raw.bin",r.sha256),"storageFormat":"raw.bin","sourceDescriptorDigest":descriptor});
        if saved["resources"].get(i) != Some(&expected_binding) {
            return Err(error("full resource source descriptor differs"));
        }
        let actual = resources
            .get(&format!("resource/{}", r.id))
            .ok_or_else(|| error("source resource population"))?;
        if actual.0 != r.id.to_string()
            || actual.1 != r.sha256
            || actual.2.len().to_string() != r.byte_length
        {
            return Err(error("resource semantic binding"));
        }
    }
    if stored_typed(&node, "", &combined, &rs)? != saved["tree"]
        || projection(content, stamp)? != primary
    {
        return Err(error("native/source full content comparison differs"));
    }
    if content.resources.len() != resources.len() {
        return Err(error("unexpected source resource"));
    }
    let capture = if full_version == Some(full_source7::Version::LegacyDisplay1) {
        full_source7::migrate_verified_legacy_display(capture, restored)?
    } else {
        capture
    };
    RestoredProject::new(capture, ids)
}

/// Diagnostic evidence from the same generated traversal consumed by encode/decode.
pub fn audit_evidence(source: &Capture) -> Result<Value> {
    use vesta_model::source_audit::{AuditSource, canonical};
    let (value, node) = capture_for_save(source)?;
    let (own, refs, objects, resources) = current_source7::roles(&node)?;
    let snapshot = &value["identity"];
    Ok(
        json!({"typeDigest":snapshot["sourceTypeDigest"],"ruleDigest":snapshot["policyDigest"],"coreDescriptor":ProjectContent::descriptor(),"source7":value,"canonicalCore":current_source7::core(source)?.wire(),"ledger":node.ledger(snapshot),"ledgerDigest":digest(&canonical(&node.ledger(snapshot))),"identitySlots":slot_rows(&own,&value)?,"referenceSlots":slot_rows(&refs,&value)?,"objects":objects,"resourceUses":resources,"canonicalCoreSHA":digest(&canonical(&current_source7::core(source)?.wire()))}),
    )
}

#[cfg(test)]
mod envelope_tests {
    use super::*;
    #[test]
    fn envelope_machine_schema_checks_pointer_integer_oneof_and_unknown_fields() {
        let slot = json!({"typedSourcePointer":"/a~1b/~0","storedNodePointer":"","objectKind":"Site","objectKey":"s"});
        current_source7::envelope(&slot, "IdentitySlot").unwrap();
        for bad in ["a", "/~2", "/~"] {
            let mut v = slot.clone();
            v["typedSourcePointer"] = json!(bad);
            assert!(
                current_source7::envelope(&v, "IdentitySlot").is_err(),
                "{bad}"
            );
        }
        for n in ["0", "-1", "9223372036854775807"] {
            current_source7::envelope(&json!({"kind":"I64","value":n}), "StoredNode").unwrap();
        }
        for n in ["-0", "01", "+1", "1.0"] {
            assert!(
                current_source7::envelope(&json!({"kind":"I64","value":n}), "StoredNode").is_err()
            );
        }
        assert!(
            current_source7::envelope(&json!({"kind":"Null","value":"unregistered"}), "StoredNode")
                .is_err()
        );
        assert!(
            current_source7::envelope(
                &json!({"kind":"F32Bits","ieeeHexBE":"80000000"}),
                "StoredNode"
            )
            .is_ok()
        );
        assert!(
            current_source7::envelope(
                &json!({"kind":"F32Bits","ieeeHexBE":"800000000"}),
                "StoredNode"
            )
            .is_err()
        );
    }
}
