//! Historical Core1 finite adapter; exact preimage descriptor and semantics only.
//! Empty unsupported roots are explicit; a nonempty unsupported population is refused.
use super::*;
pub(super) const SOURCE_SCHEMA: &str =
    include_str!("../../contracts/codec-source-document.schema.json");
const AUDIT_SCHEMA: &str = include_str!("../../contracts/complete-source-audit.schema.json");
const IDENTITY: [[f64; 3]; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];

// Exhaustive Rust patterns fail compilation when a persistent typed member is added.
// Serde captures all members; this gate also forces a new identity/resource-role review.
fn closed_types(c: &ProjectContent) {
    let ProjectContent {
        native_camera: _,
        phases,
        scene_id: _,
        camera,
        hidden,
        comment: _,
        resources,
    } = c;
    let Camera {
        rotation: _,
        projection: _,
        zoom: _,
    } = camera;
    for p in phases {
        let Phase {
            id: _,
            label: _,
            cell,
            sites,
            symmetry_op_id: _,
            boundary_min: _,
            boundary_max: _,
        } = p;
        let Cell3 {
            length_a: _,
            angles_deg: _,
        } = cell;
        for s in sites {
            let Site {
                id: _,
                label: _,
                element: _,
                frac: _,
                occupancy: _,
                u_iso_a2: _,
            } = s;
        }
    }
    for k in hidden {
        let ImageKey {
            phase: _,
            site: _,
            op: _,
            shift: _,
        } = k;
    }
    for r in resources {
        let PersistentResource {
            logical_key: _,
            sha256: _,
            byte_length: _,
            media_type: _,
        } = r;
    }
}

pub(super) fn type_digest() -> String {
    digest(
        format!(
            "I33-CurrentSource7/native-core1\n{}\n{}",
            SOURCE_SCHEMA, CORE_SCHEMA
        )
        .as_bytes(),
    )
}
pub(super) fn rule_digest() -> String {
    digest(
        concat!(
            "I33-CurrentSource7/native-core-adapter1\n",
            include_str!("../../contracts/typed-source-restore.contract.json"),
            include_str!("../../contracts/native-source-envelope.contract.json"),
            include_str!("../../contracts/full-source-audit.contract.json")
        )
        .as_bytes(),
    )
}

fn audit(
    v: &Value,
    path: &str,
    ids: &BTreeMap<String, Slot>,
    bytes: &BTreeMap<String, Value>,
) -> Result<Value> {
    if let Some(s) = ids.get(path) {
        return Ok(json!({"kind":"EntityRef","objectKind":s.kind,"objectId":v}));
    }
    if let Some(r) = bytes.get(path) {
        return Ok(r.clone());
    }
    match v {
        Value::Array(a) => Ok(
            json!({"kind":"Array","items":a.iter().enumerate().map(|(i,v)|audit(v,&format!("{path}/{i}"),ids,bytes)).collect::<Result<Vec<_>>>()?}),
        ),
        Value::Object(o) => Ok(
            json!({"kind":"Map","members":o.iter().map(|(k,v)|Ok((k.clone(),audit(v,&pointer(path,k),ids,bytes)?))).collect::<Result<BTreeMap<_,_>>>()?}),
        ),
        _ => stored(v, "", &BTreeMap::new(), &BTreeMap::new()),
    }
}
fn audit_value(v: &Value) -> Result<Value> {
    Ok(match v["kind"].as_str() {
        Some("EntityRef") => v["objectId"].clone(),
        Some("BytesRef") => v["sha256"].clone(),
        Some("Array") => Value::Array(
            v["items"]
                .as_array()
                .ok_or_else(|| error("audit array"))?
                .iter()
                .map(audit_value)
                .collect::<Result<_>>()?,
        ),
        Some("Map") => Value::Object(
            v["members"]
                .as_object()
                .ok_or_else(|| error("audit map"))?
                .iter()
                .map(|(k, v)| Ok((k.clone(), audit_value(v)?)))
                .collect::<Result<_>>()?,
        ),
        _ => restore_node(v, &BTreeMap::new(), &BTreeMap::new(), 0)?,
    })
}

pub(super) fn capture(source: &Capture) -> Result<Value> {
    let c = &source.content;
    c.validate()?;
    closed_types(c);
    let value = serde_json::to_value(c).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    let (mut core_ids, refs, _) = core_slots(c)?;
    core_ids.extend(refs);
    let mut core_bytes = BTreeMap::new();
    for (i, r) in c.resources.iter().enumerate() {
        core_bytes.insert(format!("/resources/{i}/sha256"),json!({"kind":"BytesRef","resourceId":r.sha256,"sha256":r.sha256,"byteLength":r.byte_length}));
    }
    let phases:Vec<_>=c.phases.iter().map(|p| {
        let sites:Vec<_>=p.sites.iter().map(|s|json!({"id":s.id,"label":s.label,"species":{"element":s.element,"charge":null,"massNumber":null,"rawToken":s.element},"fractional":s.frac,"occupancy":s.occupancy,"adp":{"kind":"iso-U","uA2":s.u_iso_a2},"sourceSpans":[]})).collect();
        let rational=|n|json!({"numerator":n,"denominator":1});
        let matrix:Vec<Vec<Value>>=(0..3).map(|i|(0..3).map(|j|rational(if i==j{1}else{0})).collect()).collect();
        json!({"id":p.id,"label":p.label,"parent":null,"rotation":IDENTITY,"translationA":[0.,0.,0.],"fieldIds":[],"shapeId":null,"structure":{"cell":p.cell,"group":{"kind":"ordinary","symbol":"P 1","hallSymbol":"P 1","bnsNumber":null,"ogNumber":null,"settingName":"P1","originChoice":"1","changeOfBasis":IDENTITY,"originFractional":[0.,0.,0.],"table":null,"operators":[{"id":p.symmetry_op_id,"fractionalMatrix":matrix,"fractionalTranslation":[rational(0),rational(0),rational(0)],"timeSign":1}],"sourceSpans":[]},"sites":sites,"covariance":null}})
    }).collect();
    let resources:Vec<_>=c.resources.iter().map(|r|json!({"id":r.sha256,"sha256":r.sha256,"byteLength":r.byte_length,"storageKind":"owned","sourceDescriptorDigest":digest(&serde_json::to_vec(r).expect("closed resource"))})).collect();
    let v = json!({"schemaVersion":"CodecSourceDocument-proposal-7","identity":{"documentId":source.document.document_id,"epoch":source.document.epoch,"revision":source.document.revision,"sourceTypeDigest":type_digest(),"sourceRootDigest":source.state_digest,"resourceGraphDigest":digest(&ser(&json!(c.resources))?),"policyDigest":rule_digest()},
        "phaseGraph":{"root":c.phases[0].id,"phases":phases},"molecules":[],"fields":[],
        "scenes":[{"id":c.scene_id,"roots":[],"nodes":[],"render":{"kind":"Null"},"metadata":[],"cameraId":null,"renderContext":null,"cameraRaw":audit(&json!({"camera":c.camera,"nativeCamera":c.native_camera}),"",&BTreeMap::new(),&BTreeMap::new())?}],
        "dependencies":[],"opaqueBlocks":[],"resources":resources,
        "metadata":[{"owner":{"kind":"scene","id":c.scene_id},"producerId":"VestaDraft","producerVersion":"P2-03/core1","formatRef":"VM18-I01","dialect":"native-core-editor1","tree":audit(&value,"",&core_ids,&core_bytes)?,"spans":[]}],
        "shapes":[],"vectors":[],"materials":[],"displays":[],"cameras":[],"meshes":[],"sourceEntities":[],
        "sourceEntryOrder":c.phases.iter().map(|p|json!({"kind":"phase","id":p.id})).collect::<Vec<_>>(),
        "sections":[],"bondRules":[],"boundaries":[],"planes":[]});
    validate(&v)?;
    Ok(v)
}

pub(super) fn restore(v: &Value, doc: &DocumentAt) -> Result<ProjectContent> {
    validate(v)?;
    let raw = audit_value(&v["metadata"][0]["tree"])?;
    let content: ProjectContent =
        serde_json::from_value(raw).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    content.validate()?;
    let expected = capture(&Capture {
        document: doc.clone(),
        state_digest: content.digest(),
        content: content.clone(),
        resources: BTreeMap::new(),
    })?;
    // Covers every root, source metadata, active phase/site projection, aliases and resource descriptor.
    // Unsupported populations, unknown nodes and self-consistent but altered ledger/rules are refused.
    if *v != expected {
        return Err(error(
            "Source7 complete typed source and active projection differ",
        ));
    }
    Ok(content)
}

pub(super) fn roles(
    c: &ProjectContent,
) -> Result<(BTreeMap<String, Slot>, BTreeMap<String, Slot>, Vec<Value>)> {
    let (core_own, core_refs, objects) = core_slots(c)?;
    let mut own = BTreeMap::new();
    let mut refs = BTreeMap::new();
    own.insert("/scenes/0/id".into(), core_own["/sceneId"].clone());
    refs.insert("/metadata/0/owner/id".into(), core_own["/sceneId"].clone());
    refs.insert("/phaseGraph/root".into(), core_own["/phases/0/id"].clone());
    for (i, p) in c.phases.iter().enumerate() {
        let base = format!("/phaseGraph/phases/{i}");
        own.insert(
            format!("{base}/id"),
            core_own[&format!("/phases/{i}/id")].clone(),
        );
        own.insert(
            format!("{base}/structure/group/operators/0/id"),
            core_own[&format!("/phases/{i}/symmetryOpId")].clone(),
        );
        refs.insert(
            format!("/sourceEntryOrder/{i}/id"),
            core_own[&format!("/phases/{i}/id")].clone(),
        );
        for (j, _) in p.sites.iter().enumerate() {
            own.insert(
                format!("{base}/structure/sites/{j}/id"),
                core_own[&format!("/phases/{i}/sites/{j}/id")].clone(),
            );
        }
    }
    let raw = serde_json::to_value(c).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    for (path, s) in core_own.into_iter().chain(core_refs) {
        refs.insert(
            format!("/metadata/0/tree{}/objectId", node_pointer(&path, &raw)?),
            s,
        );
    }
    Ok((own, refs, objects))
}
pub(super) fn resource_slots(c: &ProjectContent) -> Result<BTreeMap<String, Value>> {
    let mut out = BTreeMap::new();
    let raw = serde_json::to_value(c).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    for (i, r) in c.resources.iter().enumerate() {
        let value = json!({"kind":"BytesRef","sha256":r.sha256,"byteLength":r.byte_length,"resourceKey":format!("resource/{i}")});
        out.insert(format!("/resources/{i}/sha256"), value.clone());
        let base = format!(
            "/metadata/0/tree{}",
            node_pointer(&format!("/resources/{i}/sha256"), &raw)?
        );
        out.insert(format!("{base}/resourceId"), value);
    }
    Ok(out)
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
            // Equivalent signed-integer grammar supported by the inherited validator.
            if o.get("pattern").and_then(Value::as_str) == Some("^(0|-[1-9][0-9]*|[1-9][0-9]*)$") {
                o.insert("pattern".into(), json!("^(0|-?[1-9][0-9]*)$"));
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
fn validate(v: &Value) -> Result<()> {
    use std::sync::OnceLock;
    static C: OnceLock<Value> = OnceLock::new();
    let c = C.get_or_init(|| {
        let old: Value =
            serde_json::from_str(vesta_bindings::resource_roles::CATALOG).expect("frozen catalog");
        let mut files = BTreeMap::new();
        let mut aliases = BTreeMap::new();
        for (k, v) in old.as_object().expect("catalog") {
            let name = k.rsplit('/').next().expect("filename");
            aliases.insert(k.clone(), name.to_owned());
            if v.get("$defs")
                .is_some_and(|d| d.get("SGPhaseGraph").is_some())
            {
                aliases.insert("urn:draft:input:SG1-r2".into(), name.to_owned());
            }
            files.insert(format!("/schemas/{name}"), v.clone());
        }
        aliases.insert(
            "urn:draft:input:AUDIT1".into(),
            "complete-source-audit.schema.json".into(),
        );
        files.insert(
            "/schemas/codec-source-document.schema.json".into(),
            serde_json::from_str(SOURCE_SCHEMA).expect("frozen source7 schema"),
        );
        files.insert(
            "/schemas/complete-source-audit.schema.json".into(),
            serde_json::from_str(AUDIT_SCHEMA).expect("frozen audit schema"),
        );
        for s in files.values_mut() {
            normalize(s, &aliases)
        }
        serde_json::to_value(files).expect("schema map")
    });
    let file = "/schemas/codec-source-document.schema.json";
    vesta_bindings::decoder::validate(v, &c[file], file, c, 0)
        .map_err(|e| Error::new("E_SCHEMA", format!("Source7: {e}")))
}
