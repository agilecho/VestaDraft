//! Generated, closed Source7 adapter for the implemented P1 population.
use super::{graph_provenance, native_vesta_v2::{Slot, error, pointer}};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use vesta_model::{Capture, DocumentAt, Error, PersistentResource, ProjectContent, Result, digest};
use vesta_model::source_audit::{self as sa, AuditSource, Node, Role};
use vesta_model::source7::schema_validation;
#[path = "source7_types.rs"]
mod types;
pub(super) const SOURCE_SCHEMA: &str =
    include_str!("../../contracts/codec-source-document.schema.json");
const AUDIT_SCHEMA: &str = include_str!("../../contracts/complete-source-audit.schema.json");
pub(super) const ENVELOPE_SCHEMA: &str =
    include_str!("../../contracts/native-source-envelope.schema.json");
// Exact external bindings used by the reachable Source7 schema, as pinned by
// FC-codec-source-document-bridge/4. The NS2 catalog is a separate registry.
const SOURCE7_EXTERNAL: &[(&str, &str, &str)] = &[
    ("SG1-r2", "structure-geometry.contract.json", include_str!("../../contracts/structure-geometry.contract.json")),
    ("FS1-r1", "field-science.contract.json", include_str!("../../../../contracts/source/adf539aac53ddd41-field-science.contract.json")),
    ("AUDIT1", "complete-source-audit.schema.json", AUDIT_SCHEMA),
    ("NATIVESECTION1", "native-section-source.schema.json", include_str!("../../contracts/native-section-source.schema.json")),
    ("NATIVESECTION2", "native-legacy-section-source.schema.json", include_str!("../../contracts/native-legacy-section-source.schema.json")),
    ("NBOND1", "native-bond-source.schema.json", include_str!("../../contracts/native-bond-source.schema.json")),
];
const IDENTITY: [[f64; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
pub(super) fn type_digest() -> String {
    digest(&sa::canonical(
        &json!({"version":"I33-CurrentSource7/generated-core2","core":ProjectContent::descriptor(),"source":types::Source7::descriptor(),"schema":digest(SOURCE_SCHEMA.as_bytes())}),
    ))
}
pub(super) fn rule_digest() -> String {
    digest(
        concat!(
            "I33-generated-native-core2/logical-resources/canonical-descriptor/one-rebind\n",
            include_str!("../../contracts/typed-source-restore.contract.json"),
            include_str!("../../contracts/native-source-envelope.contract.json"),
            include_str!("../../contracts/full-source-audit.contract.json")
        )
        .as_bytes(),
    )
}
pub(super) fn descriptor_digest(r: &PersistentResource) -> String {
    digest(&sa::canonical(
        &json!({"logicalKey":r.logical_key,"sha256":r.sha256,"byteLength":r.byte_length,"mediaType":r.media_type}),
    ))
}
pub(super) fn core(source: &Capture) -> Result<Node> {
    let mut node = source.content.audit()?;
    node.definitions()?;
    let resources = source
        .content
        .resources
        .iter()
        .map(|r| {
            Ok((
                r.id.to_string(),
                (
                    r.sha256.clone(),
                    source
                        .resources
                        .get(&r.sha256)
                        .ok_or_else(|| error("source bytes absent"))?
                        .clone(),
                ),
            ))
        })
        .collect::<Result<sa::ResourceBytes>>()?;
    node.bind_resources(&resources)?;
    Ok(node)
}
pub(super) fn snapshot(source: &Capture) -> Result<Value> {
    let n = core(source)?;
    let (source_type, policy) = graph_provenance::snapshot_digests(source, &type_digest(), &rule_digest())?;
    let mut resources = json!(source.content.resources);
    if graph_provenance::has_graph(source) {
        resources = json!({
            "nativeGraphBindings": source.content.resources.iter()
                .filter(|r| r.media_type == vesta_model::source7::resource_graph::CAPTURE_MEDIA)
                .map(|r| &r.sha256).collect::<Vec<_>>(),
            "logicalResources": resources
        });
    }
    Ok(
        json!({"documentId":source.document.document_id,"epoch":source.document.epoch,"revision":source.document.revision,"sourceTypeDigest":source_type,"sourceRootDigest":digest(&sa::canonical(&n.wire())),"resourceGraphDigest":digest(&sa::canonical(&resources)),"policyDigest":policy}),
    )
}
pub(super) fn ledger_digest(source: &Capture) -> Result<String> {
    Ok(digest(&sa::canonical(
        &capture(source)?.1.ledger(&snapshot(source)?),
    )))
}
pub(super) fn capture(source: &Capture) -> Result<(Value, Node)> {
    let c = &source.content;
    c.validate()?;
    if c.phases.len() != 1 {
        return Err(error("bounded native source requires one phase"));
    }
    let n = core(source)?;
    let phases:Vec<_>=c.phases.iter().map(|p|{
 let sites:Vec<_>=p.sites.iter().map(|s|json!({"id":s.id,"label":s.label,"species":{"element":s.element,"charge":null,"massNumber":null,"rawToken":s.element},"fractional":s.frac,"occupancy":s.occupancy,"adp":{"kind":"iso-U","uA2":s.u_iso_a2},"sourceSpans":[]})).collect();
 let rational=|n|json!({"numerator":n,"denominator":1});let matrix:Vec<Vec<Value>>=(0..3).map(|i|(0..3).map(|j|rational(if i==j{1}else{0})).collect()).collect();
 json!({"id":p.id,"label":p.label,"parent":null,"rotation":IDENTITY,"translationA":[0.,0.,0.],"fieldIds":[],"shapeId":null,"structure":{"cell":p.cell,"group":{"kind":"ordinary","symbol":"P 1","hallSymbol":"P 1","bnsNumber":null,"ogNumber":null,"settingName":"P1","originChoice":"1","changeOfBasis":IDENTITY,"originFractional":[0.,0.,0.],"table":null,"operators":[{"id":p.symmetry_op_id,"fractionalMatrix":matrix,"fractionalTranslation":[rational(0),rational(0),rational(0)],"timeSign":1}],"sourceSpans":[]},"sites":sites,"covariance":null}})
 }).collect();
    let resources:Vec<_>=c.resources.iter().map(|r|json!({"id":r.id,"sha256":r.sha256,"byteLength":r.byte_length,"storageKind":"owned","sourceDescriptorDigest":descriptor_digest(r)})).collect();
    let camera = Node::Map(
        [
            ("camera".into(), c.camera.audit()?),
            ("nativeCamera".into(), c.native_camera.audit()?),
        ]
        .into_iter()
        .collect(),
    );
    let mut v = json!({"schemaVersion":"CodecSourceDocument-proposal-7","identity":snapshot(source)?,"phaseGraph":{"root":c.phases[0].id,"phases":phases},"molecules":[],"fields":[],"scenes":[{"id":c.scene_id,"roots":[],"nodes":[],"render":{"kind":"Null"},"metadata":[],"cameraId":null,"renderContext":null,"cameraRaw":camera.wire()}],"dependencies":[],"opaqueBlocks":[],"resources":resources,"metadata":[{"owner":{"kind":"scene","id":c.scene_id},"producerId":"VestaDraft","producerVersion":"P2-03/core2","formatRef":"VM18-I01","dialect":"native-core-editor2","tree":n.wire(),"spans":[]}],"shapes":[],"vectors":[],"materials":[],"displays":[],"cameras":[],"meshes":[],"sourceEntities":[],"sourceEntryOrder":c.phases.iter().map(|p|json!({"kind":"phase","id":p.id})).collect::<Vec<_>>(),"sections":[],"bondRules":[],"boundaries":[],"planes":[]});
    let typed: types::Source7 = serde_json::from_value(v.clone())
        .map_err(|e| Error::new("E_SCHEMA", format!("generated Source7: {e}")))?;
    let mut node = typed.audit()?;
    let graphs = graph_provenance::verify(source)?;
    if !graphs.is_empty() {
        let Node::Map(roots) = &mut node else {
            return Err(error("generated Source7 root"));
        };
        let document: sa::Id<sa::Document> = source.document.document_id.clone().into();
        let Some(Node::Map(identity)) = roots.get_mut("identity") else {
            return Err(error("generated Source7 identity"));
        };
        identity.insert("documentId".into(), sa::identity::<sa::Document, _>(&document, Role::Definition)?);
        let Some(Node::Array(metadata)) = roots.get_mut("metadata") else {
            return Err(error("generated Source7 metadata"));
        };
        for graph in graphs {
            let record = types::GraphMetadata {
                owner: types::DocumentOwner { kind: "document".into(), id: document.clone() },
                producer_id: "VestaDraft".into(),
                producer_version: "P2-03/native-graph2".into(),
                format_ref: graph.format_ref().into(),
                dialect: graph.dialect().into(),
                tree: graph.audit_node(),
                spans: types::Empty,
            };
            metadata.push(record.audit()?);
            v["metadata"].as_array_mut().ok_or_else(|| error("Source7 metadata array"))?
                .push(serde_json::to_value(&record).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?);
        }
    }
    validate(&v)?;
    node.definitions()?;
    let resources = c
        .resources
        .iter()
        .map(|r| {
            Ok((
                r.id.to_string(),
                (
                    r.sha256.clone(),
                    source
                        .resources
                        .get(&r.sha256)
                        .ok_or_else(|| error("bytes absent"))?
                        .clone(),
                ),
            ))
        })
        .collect::<Result<sa::ResourceBytes>>()?;
    node.bind_resources(&resources)?;
    if node.data()? != v {
        return Err(error("generated Source7 member coverage"));
    }
    Ok((v, node))
}
pub(super) fn restore(
    v: &Value,
    doc: &DocumentAt,
    resources: &BTreeMap<String, Vec<u8>>,
) -> Result<ProjectContent> {
    validate(v)?;
    let raw = Node::from_wire(&v["metadata"][0]["tree"])?.data()?;
    let content: ProjectContent =
        serde_json::from_value(raw).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    content.validate()?;
    let expected = capture(&Capture {
        document: doc.clone(),
        state_digest: content.digest(),
        content: content.clone(),
        resources: resources.clone(),
        source: None,
    })?
    .0;
    if *v != expected {
        return Err(error(
            "Source7 complete generated source and active projection differ",
        ));
    }
    Ok(content)
}
pub(super) fn roles(
    n: &Node,
) -> Result<(
    BTreeMap<String, Slot>,
    BTreeMap<String, Slot>,
    Vec<Value>,
    BTreeMap<String, Value>,
)> {
    let defs = n.definitions()?;
    let mut keys = BTreeMap::new();
    let mut objects = vec![];
    for (i, ((kind, id), _)) in defs
        .iter()
        .filter(|((k, _), _)| k != "Resource")
        .enumerate()
    {
        let key = format!("object/{i}");
        keys.insert((kind.clone(), id.clone()), key.clone());
        objects.push(json!({"key":key,"objectKind":kind,"captureObjectId":id}));
    }
    let resource_keys: BTreeMap<_, _> = defs
        .keys()
        .filter(|(k, _)| k == "Resource")
        .map(|(_, id)| (id.clone(), format!("resource/{id}")))
        .collect();
    let mut own = BTreeMap::new();
    let mut refs = BTreeMap::new();
    let mut bytes = BTreeMap::new();
    fn resources(n: &Node, out: &mut BTreeMap<String, (String, String)>) {
        match n {
            Node::Resource { id, sha, len, .. } => {
                out.insert(id.clone(), (sha.clone(), len.clone()));
            }
            Node::Owned { node, .. } => resources(node, out),
            Node::Map(m) => {
                for n in m.values() {
                    resources(n, out)
                }
            }
            Node::Array(a) => {
                for n in a {
                    resources(n, out)
                }
            }
            _ => {}
        }
    }
    let mut rs = BTreeMap::new();
    resources(n, &mut rs);
    for (path, kind, id, role) in n.roles() {
        if kind == "Resource" {
            let (sha, len) = rs
                .get(&id)
                .ok_or_else(|| error("generated resource binding"))?;
            bytes.insert(path,json!({"kind":"BytesRef","resourceKey":resource_keys[&id],"sha256":sha,"byteLength":len}));
        } else {
            let slot = Slot {
                kind: kind.clone(),
                key: keys[&(kind, id)].clone(),
            };
            if role == Role::Definition {
                own.insert(path, slot);
            } else {
                refs.insert(path, slot);
            }
        }
    }
    Ok((own, refs, objects, bytes))
}
fn normalize(v: &mut Value, aliases: &BTreeMap<String, String>) {
    match v {
        Value::Object(o) => {
            if let Some(Value::String(r)) = o.get_mut("$ref") {
                let (file, fragment) = r.split_once('#').unwrap_or((r.as_str(), ""));
                if let Some(name) = aliases.get(file) {
                    *r = format!("{name}#{fragment}");
                }
            }
            for x in o.values_mut() {
                normalize(x, aliases)
            }
        }
        Value::Array(a) => {
            for x in a {
                normalize(x, aliases)
            }
        }
        _ => {}
    }
}
fn catalog() -> &'static Value {
    use std::sync::OnceLock;
    static C: OnceLock<Value> = OnceLock::new();
    C.get_or_init(|| {
        let mut files = BTreeMap::new();
        let mut aliases = BTreeMap::new();
        for &(namespace, name, source) in SOURCE7_EXTERNAL {
            aliases.insert(format!("urn:draft:input:{namespace}"), name.to_owned());
            files.insert(format!("/schemas/{name}"), serde_json::from_str(source).expect("frozen Source7 external binding"));
        }
        files.insert(
            "/schemas/codec-source-document.schema.json".into(),
            serde_json::from_str(SOURCE_SCHEMA).expect("frozen source7 schema"),
        );
        files.insert(
            "/schemas/native-source-envelope.schema.json".into(),
            serde_json::from_str(ENVELOPE_SCHEMA).expect("frozen envelope"),
        );
        for s in files.values_mut() {
            normalize(s, &aliases)
        }
        serde_json::to_value(files).expect("schema map")
    })
}
pub(super) fn validate(v: &Value) -> Result<()> {
    let c = catalog();
    let file = "/schemas/codec-source-document.schema.json";
    schema_validation::validate(v, &c[file], file, c, 0)
        .map_err(|e| Error::new("E_SCHEMA", format!("Source7: {e}")))
}
pub(super) fn envelope(v: &Value, definition: &str) -> Result<()> {
    let c = catalog();
    let file = "/schemas/native-source-envelope.schema.json";
    schema_validation::validate(v, &c[file]["$defs"][definition], file, c, 0)
        .map_err(|e| Error::new("E_SCHEMA", format!("envelope {definition}: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn source7_frozen_reference_closure() {
        fn visit<'a>(
            schema: &'a Value,
            file: &'a str,
            catalog: &'a Value,
            seen: &mut BTreeSet<(&'a str, &'a str)>,
        ) {
            if let Some(link) = schema.get("$ref").and_then(Value::as_str) {
                if seen.insert((file, link)) {
                    let (target, target_file) =
                        schema_validation::ref_target(link, file, catalog).unwrap();
                    visit(target, target_file, catalog, seen);
                }
            }
            for name in ["properties", "$defs"] {
                if let Some(members) = schema.get(name).and_then(Value::as_object) {
                    for child in members.values() {
                        visit(child, file, catalog, seen);
                    }
                }
            }
            for name in ["oneOf", "allOf", "anyOf", "prefixItems"] {
                if let Some(children) = schema.get(name).and_then(Value::as_array) {
                    for child in children {
                        visit(child, file, catalog, seen);
                    }
                }
            }
            for name in ["items", "additionalProperties"] {
                if let Some(child) = schema.get(name) {
                    visit(child, file, catalog, seen);
                }
            }
        }
        let expected = [
            ("SG1-r2", "51e05f021bad4c0a6497f9e32ab70e98bb76f00e3e0b90a57124b5db38d6fad0"),
            ("FS1-r1", "adf539aac53ddd41b46651886b00e9d5394a96c36b83f74aeb6e35984898000a"),
            ("AUDIT1", "f8894b66b54a40495296b5b58353a02bc29d358d231da5683dcac22b0fceab57"),
            ("NATIVESECTION1", "177c9c9295937e0881d36d2b5b2c2d6306ceb25a75d6e0714b4951aaa39fd7af"),
            ("NATIVESECTION2", "c465436e6f077077bda4a75b1bb2ce1b84fd009ca976e6ed6349181c6871db97"),
            ("NBOND1", "3abcd56d6dd846cc7b31ba58e7d18baf754cc7845d26c6a8df183ddefdd95dda"),
        ];
        assert_eq!(SOURCE7_EXTERNAL.len(), expected.len());
        for ((namespace, _, source), (expected_namespace, expected_digest)) in
            SOURCE7_EXTERNAL.iter().zip(expected)
        {
            assert_eq!(*namespace, expected_namespace);
            assert_eq!(digest(source.as_bytes()), expected_digest, "{namespace}");
        }
        let file = "/schemas/codec-source-document.schema.json";
        let schema = catalog();
        let mut seen = BTreeSet::new();
        visit(&schema[file], file, schema, &mut seen);
        assert!(seen.len() >= 86, "the frozen schema closure must be traversed");
        assert!(seen.contains(&(file, "field-science.contract.json#/$defs/Field")));
        assert!(seen.contains(&(file, "native-legacy-section-source.schema.json#/$defs/SectionRecord")));
        assert!(seen.contains(&(file, "native-bond-source.schema.json#/$defs/NativeBondRecord")));
    }
}
