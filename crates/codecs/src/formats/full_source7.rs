//! Complete Project Source7, kept alongside the native editor projection.
use super::current_source7;
use super::graph_provenance;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use vesta_model::field::{ActiveField, FieldDescriptor, FieldOwner, restore_fields};
use vesta_model::source_audit::{AuditSource, Node, canonical};
use vesta_model::source7::SourceDocument;
use vesta_model::source7::resource_graph::orffe::{
    ACTIVE_TABLE_TAG, R80_TABLE_DIALECT, R80GeometryTable, RIETAN_OWNER_DIALECT,
    RIETAN_TABLE_DIALECT, TABLE_PRODUCER_VERSION, active_tables,
};
use vesta_model::{Capture, Error, PersistentResource, ProjectContent, Result, digest};

const AUTHOR: &str = "VestaDraft";
const EDITOR: &str = "native-editor-projection-1";
const GRAPH_PREFIX: &str = "native-graph2-";
const C1_DENSITY: &str = "native-c1-density-grid3-le32-1";

fn verify_orffe(
    source: &Capture,
    value: &Value,
    graphs: &[graph_provenance::VerifiedGraph<'_>],
) -> Result<()> {
    let wrong_producer = || {
        Error::new("E_REOPEN_OWNER", "ORFFE table does not belong to its installed producer")
    };
    let mut tables = active_tables(value)?;
    let metadata = value["metadata"]
        .as_array()
        .ok_or_else(|| invalid("missing metadata"))?;
    let table_has_dialect = |table: &R80GeometryTable, dialect: &str| {
        metadata.iter().any(|row| {
            row["owner"]["id"] == table.owner.as_str()
                && row["tree"]["tag"] == ACTIVE_TABLE_TAG
                && row["dialect"] == dialect
        })
    };
    let mut seen = BTreeSet::new();
    for owner in metadata.iter()
        .filter(|row| row["dialect"] == RIETAN_OWNER_DIALECT)
    {
        let id = owner["owner"]["id"].as_str().ok_or_else(|| invalid("RIETAN owner identity"))?;
        if owner["producerId"] != AUTHOR
            || owner["owner"]["kind"] != "phase"
            || owner["producerVersion"] != TABLE_PRODUCER_VERSION
            || owner["formatRef"] != "RIETAN"
            || !owner["spans"].as_array().is_some_and(Vec::is_empty)
            || !seen.insert(id)
        {
            return Err(wrong_producer());
        }
        let phase = value["phaseGraph"]["phases"].as_array().into_iter().flatten()
            .find(|phase| phase["id"] == id)
            .ok_or_else(|| invalid("RIETAN source phase missing"))?;
        let mut matched = None;
        for graph in graphs {
            if let Some(primary) = graph.files.get(graph.root_key())
                && owner["tree"] == raw_ref(primary)
            {
                if matched.replace(graph).is_some() {
                    return Err(invalid("ambiguous RIETAN source root"));
                }
            }
        }
        let graph = matched.ok_or_else(|| invalid("RIETAN phase lacks its actual source root"))?;
        let archive = graph.archive(source)?;
        let expected = super::resource_graph::rietan_binding(&archive, phase, &source.content.resources)?;
        let observed = tables.iter().position(|table| table.owner.as_str() == id)
            .map(|index| tables.remove(index));
        if observed.as_ref().is_some_and(|table| !table_has_dialect(table, RIETAN_TABLE_DIALECT)) {
            return Err(wrong_producer());
        }
        if expected != observed {
            return Err(invalid("RIETAN table differs from its qualified source phase and bytes"));
        }
    }
    for table in tables {
        if !table_has_dialect(&table, R80_TABLE_DIALECT) {
            return Err(wrong_producer());
        }
        let phase = source
            .content
            .phases
            .iter()
            .find(|phase| phase.id == table.owner)
            .ok_or_else(|| invalid("active ORFFE phase missing from the native editor"))?;
        let graph = graphs
            .iter()
            .find(|graph| graph.files.values().any(|r| r.id == table.resource))
            .ok_or_else(|| invalid("active ORFFE table lacks the captured graph binding"))?;
        let resource = graph
            .files
            .values()
            .find(|r| r.id == table.resource)
            .ok_or_else(|| invalid("active ORFFE logical resource missing"))?;
        let archive = graph.archive(source)?;
        super::resource_graph::verify_r80_binding(&archive, phase, resource, &table)?;
    }
    Ok(())
}

fn raw_ref(resource: &PersistentResource) -> Value {
    json!({"kind":"BytesRef","resourceId":resource.id,
        "sha256":resource.sha256,"byteLength":resource.byte_length})
}

pub(super) fn with_rietan(
    source: &Capture,
    phase: Value,
    table: Option<&R80GeometryTable>,
    primary: &PersistentResource,
) -> Result<SourceDocument> {
    let phase_id = phase["id"].clone();
    let owner = json!({"kind":"phase","id":phase_id});
    let row = |dialect, tree| json!({"owner":owner,"producerId":AUTHOR,
        "producerVersion":TABLE_PRODUCER_VERSION,"formatRef":"RIETAN","dialect":dialect,
        "tree":tree,"spans":[]});
    let mut metadata = vec![row(RIETAN_OWNER_DIALECT, raw_ref(primary))];
    if let Some(table) = table {
        let tree = Node::TaggedVariant { tag: ACTIVE_TABLE_TAG.into(), payload: Box::new(table.audit()?) };
        metadata.push(row(RIETAN_TABLE_DIALECT, tree.wire()));
    }
    let mut value = source_only_base(source);
    value["phaseGraph"] = json!({"root":phase_id,"phases":[phase]});
    value["sourceEntryOrder"] = json!([{"kind":"phase","id":phase_id}]);
    value["metadata"] = json!(metadata);
    SourceDocument::decode(value)
}

pub(super) fn source_only_base(source: &Capture) -> Value {
    let resources: Vec<_> = source.content.resources.iter().map(|r| json!({
        "id":r.id,"sha256":r.sha256,"byteLength":r.byte_length,"storageKind":"owned",
        "sourceDescriptorDigest":current_source7::descriptor_digest(r)
    })).collect();
    let zero = "0".repeat(64);
    json!({
        "schemaVersion":"CodecSourceDocument-proposal-7",
        "identity":{"documentId":source.document.document_id,"epoch":source.document.epoch,
            "revision":source.document.revision,"sourceTypeDigest":zero,"sourceRootDigest":zero,
            "resourceGraphDigest":zero,"policyDigest":zero},
        "phaseGraph":null,
        "scenes":[{"id":source.content.scene_id,"roots":[],"nodes":[],
            "render":{"kind":"Null"},"metadata":[],"cameraId":null,"renderContext":null,
            "cameraRaw":{"kind":"Null"}}],
        "sourceEntryOrder":[],
        "metadata":[],"resources":resources,
        "molecules":[],"fields":[],"dependencies":[],"opaqueBlocks":[],"shapes":[],"vectors":[],
        "materials":[],"displays":[],"cameras":[],"meshes":[],"sourceEntities":[],
        "sections":[],"bondRules":[],"boundaries":[],"planes":[]
    })
}

pub(super) fn with_orffe(source: &Capture, table: &R80GeometryTable) -> Result<SourceDocument> {
    let (mut value, _) = capture(source)?;
    let tree = Node::TaggedVariant {
        tag: ACTIVE_TABLE_TAG.into(),
        payload: Box::new(table.audit()?),
    };
    let metadata = value["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("ORFFE metadata list"))?;
    let mut row = editor_metadata(source, "VM18-I01", R80_TABLE_DIALECT, tree);
    row["owner"] = json!({"kind":"phase","id":table.owner});
    metadata.push(row);
    SourceDocument::decode(value)
}

pub(super) fn with_c1_density(
    source: &Capture,
    field_id: &str,
    descriptor: &FieldDescriptor,
    resource: &PersistentResource,
) -> Result<SourceDocument> {
    use vesta_model::field::{FIELD_ENCODING, FIELD_STORAGE_USE};

    let (mut value, _) = current_source7::capture(source)?;
    let phase_id = &source.content.phases[0].id;
    let descriptor = serde_json::to_value(descriptor)
        .map_err(|error| Error::new("E_SCHEMA", error.to_string()))?;
    let text = |value: &str| json!({"kind":"Utf8","value":value});
    let owner = json!({"kind":"phase","id":phase_id});
    value["phaseGraph"]["phases"][0]["fieldIds"] = json!([field_id]);
    value["fields"] = json!([{
        "id":field_id, "owner":owner, "field":descriptor,
        "metadata":[{
            "owner":{"kind":"field","id":field_id},
            "producerId":AUTHOR,"producerVersion":"P2-03/full-source7",
            "formatRef":"FS1-r1","dialect":C1_DENSITY,"spans":[],
            "tree":{"kind":"TaggedVariant","tag":FIELD_STORAGE_USE,"payload":{"kind":"Map","members":{
                "field":{"kind":"EntityRef","objectKind":"Field","objectId":field_id},
                "owner":{"kind":"EntityRef","objectKind":"Phase","objectId":phase_id},
                "usePointer":text("/fields/0/field/resource"),
                "encoding":text(FIELD_ENCODING),
                "descriptorDigest":text(&digest(&canonical(&descriptor))),
                "bytes":{"kind":"BytesRef","resourceId":resource.id,
                    "sha256":resource.sha256,"byteLength":resource.byte_length}
            }}}
        }]
    }]);
    SourceDocument::decode(value)
}

fn verify_c1_density(
    source: &Capture,
    value: &Value,
    active: &[ActiveField],
    graphs: &[graph_provenance::VerifiedGraph<'_>],
) -> Result<()> {
    let selected: Vec<_> = value["fields"].as_array().into_iter().flatten()
        .zip(active)
        .filter(|(row, _)| row["metadata"].as_array().into_iter().flatten()
            .any(|m| m["producerId"] == AUTHOR && m["dialect"] == C1_DENSITY))
        .map(|(_, field)| field).collect();
    if selected.is_empty() { return Ok(()); }
    let [field] = selected.as_slice() else { return Err(invalid("ambiguous native C1 field use")) };
    let mut originals = graphs.iter().filter(|graph| {
        graph.files.get(graph.root_key()).and_then(|r| source.resources.get(&r.sha256))
            .is_some_and(|raw| super::resource_graph::is_c1_density_source(raw))
    });
    let graph = originals.next().ok_or_else(|| invalid("native C1 grid source missing"))?;
    if originals.next().is_some() { return Err(invalid("ambiguous native C1 grid source")); }
    let archive = graph.archive(source)?;
    let (original, descriptor, raw) = super::resource_graph::read_c1_density(
        &archive, &mut vesta_model::Ids::new([0; 16]),
    )?;
    let valid_phase = match &field.owner {
        FieldOwner::Phase(id) => source.content.phases.iter()
            .any(|phase| phase.id.as_str() == id.as_str() && phase.cell == original.cell),
        _ => false,
    };
    if !valid_phase || descriptor != field.descriptor
        || source.resources.get(&descriptor.resource.digest).map(Vec::as_slice) != Some(raw)
    {
        return Err(invalid("C1 decoder, captured basis and active Field storage differ"));
    }
    Ok(())
}

fn invalid(message: &str) -> Error {
    Error::new("E_REOPEN_COVERAGE", message)
}

// Exact tuple emitted by the retired full writer (including the C11 and
// C20 artifacts). Keep the old schema/rule pair together during validation.
const RETIRED_TYPE: &str = "f8aed2eca193acdc3c2b40abafb3b04b316f42509481e9005a5c74dc8828f996";
const RETIRED_RULE: &str = "0de0c6961d53d00843a92eb6cc3d2d172dc434c7f512501331db5846ac9c0150";
const RETIRED_SCHEMA: &str = "666ba0fa0135f6956bcbf2a4ca49650ce0a3de82c4d01cb1c07f86d7eb9f9bef";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Version {
    Current,
    LegacyDisplay1,
}

impl Version {
    pub(super) fn qualified(saved: &Value) -> Option<Self> {
        [Self::Current, Self::LegacyDisplay1].into_iter().find(|version| {
            let (types, rules) = version.digests();
            let schema = match version {
                Self::Current => digest(current_source7::SOURCE_SCHEMA.as_bytes()),
                Self::LegacyDisplay1 => RETIRED_SCHEMA.to_owned(),
            };
            saved["sourceTypeDigest"] == types
                && saved["sourceRuleSetDigest"] == rules
                && saved["sourceDTOSchemaDigest"] == schema
        })
    }

    fn digests(self) -> (String, String) {
        match self {
            Self::Current => (type_digest(), rule_digest()),
            Self::LegacyDisplay1 => (RETIRED_TYPE.into(), RETIRED_RULE.into()),
        }
    }
}

fn type_digest() -> String {
    digest(&canonical(&json!({
        "version":"VestaDraft/Source7/typed-project-2",
        "wire":digest(current_source7::SOURCE_SCHEMA.as_bytes()),
        "editor":ProjectContent::descriptor(),
        "types":SourceDocument::descriptor(),
    })))
}

fn rule_digest() -> String {
    digest(&canonical(&json!({
        "sourceRules":current_source7::rule_digest(),
        "fieldStorage":include_str!("../../contracts/field-storage-use-1.json"),
        "descriptorInstallation":"descriptor-data/<sha256(sourceTypedPointer)>/{definition,scope,target/<sha256(jsonPointer)>}; unique independently installed logical resources",
        "selectedGridDescriptor":super::scalar_grid_source::rule_digest(),
        "opaqueTail":super::opaque_tail::rule_digest(),
        "nativeCurrentState":super::live_state::rule_digest(),
        "fdatConnections":super::fdat_source::rule_digest(),
        "mxdFile06":super::mxd_report::rule_digest(),
        "gsasExpSourceObservations":super::gsas_exp::rule_digest(),
        "gsasMagneticState":super::gsas_exp::magnetic_rule_digest(),
        "graphRules":super::resource_graph::expanded_rule_digest(),
        "selectedNativeImports":[C1_DENSITY,RIETAN_OWNER_DIALECT,RIETAN_TABLE_DIALECT],
        "editor":EDITOR,
    })))
}

fn editor_metadata(source: &Capture, format_ref: &str, dialect: &str, tree: Node) -> Value {
    json!({
        "owner":{"kind":"document","id":source.document.document_id},
        "producerId":AUTHOR,
        "producerVersion":"P2-03/full-source7",
        "formatRef":format_ref,
        "dialect":dialect,
        "tree":tree.wire(),
        "spans":[]
    })
}

fn actual_resource_rows(value: &mut Value, capture: &Capture) -> Result<()> {
    let rows = value["resources"]
        .as_array_mut()
        .ok_or_else(|| invalid("typed resources must be an array"))?;
    let mut seen = BTreeSet::new();
    for raw in rows.iter() {
        let id = raw["id"]
            .as_str()
            .ok_or_else(|| invalid("source resource identity"))?;
        let current = capture
            .content
            .resources
            .iter()
            .find(|r| r.id.as_str() == id)
            .ok_or_else(|| invalid("source resource has no current owner"))?;
        if !seen.insert(id.to_owned())
            || raw["sha256"] != current.sha256
            || raw["byteLength"] != current.byte_length
        {
            return Err(invalid("source and actual resource differ"));
        }
    }
    for resource in &capture.content.resources {
        if !seen.contains(resource.id.as_str()) {
            rows.push(json!({"id":resource.id,"sha256":resource.sha256,"byteLength":resource.byte_length,
                "storageKind":"owned","sourceDescriptorDigest":current_source7::descriptor_digest(resource)}));
        }
    }
    Ok(())
}

fn update_editable_phases(source: &mut Value, content: &ProjectContent) -> Result<()> {
    if content.phases.is_empty() {
        return Ok(());
    }
    let phases = source["phaseGraph"]["phases"]
        .as_array_mut()
        .ok_or_else(|| invalid("editor phase has no typed structure"))?;
    for phase in &content.phases {
        let row = phases
            .iter_mut()
            .find(|p| p["id"] == phase.id.as_str())
            .ok_or_else(|| invalid("editor and typed phase identities disagree"))?;
        let structure = row["structure"]
            .as_object_mut()
            .ok_or_else(|| invalid("editor phase lacks a structure"))?;
        let group = &structure["group"];
        let operators = group["operators"]
            .as_array()
            .ok_or_else(|| invalid("editor phase symmetry operators"))?;
        if group["kind"] != "ordinary"
            || !matches!(group["symbol"].as_str(), Some("P 1" | "P1"))
            || operators.len() != 1
            || operators[0]["id"] != phase.symmetry_op_id.as_str()
        {
            return Err(invalid(
                "native P1 editor projection needs qualified symmetry",
            ));
        }
        structure.insert("cell".into(), json!(phase.cell));
        let sites = structure
            .get_mut("sites")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| invalid("editor phase site collection"))?;
        for row in sites.iter() {
            let id = row["id"]
                .as_str()
                .ok_or_else(|| invalid("typed site identity"))?;
            if !phase.sites.iter().any(|site| site.id.as_str() == id) {
                return Err(invalid("editor projection lost a typed site"));
            }
        }
        for site in &phase.sites {
            if let Some(row) = sites.iter_mut().find(|row| row["id"] == site.id.as_str()) {
                if row["species"]["element"] != site.element
                    || row["label"] != site.label
                    || row["occupancy"].as_f64() != Some(site.occupancy)
                    || !(row["adp"]["kind"] == "iso-U"
                        && row["adp"]["uA2"].as_f64() == Some(site.u_iso_a2)
                        || row["adp"].is_null() && site.u_iso_a2 == 0.0)
                {
                    return Err(invalid("native editor has unqualified site semantics"));
                }
                row["fractional"] = json!(site.frac);
            } else {
                sites.push(json!({"id":site.id,"label":site.label,
                    "species":{"element":site.element,"charge":null,"massNumber":null,"rawToken":site.element},
                    "fractional":site.frac,"occupancy":site.occupancy,
                    "adp":{"kind":"iso-U","uA2":site.u_iso_a2},"sourceSpans":[]}));
            }
        }
    }
    Ok(())
}

fn editor_projection(source: &Capture, typed: &Value) -> Result<Node> {
    let scenes = typed["scenes"]
        .as_array()
        .ok_or_else(|| invalid("typed scene collection"))?;
    let node = current_source7::core(source)?;
    if scenes
        .iter()
        .any(|scene| scene["id"] == source.content.scene_id.as_str())
    {
        return Ok(node);
    }
    if !scenes.is_empty() {
        return Err(invalid(
            "native editor projection has no matching current scene",
        ));
    }
    let Node::Owned { node, .. } = node else {
        return Err(invalid("native editor projection owner"));
    };
    let Node::Map(mut fields) = *node else {
        return Err(invalid("native editor projection members"));
    };
    // The private editor still needs an internal identity when Source7 has no scene.
    // Its implementation identity is deliberately absent from the persisted type graph.
    fields.insert("sceneId".into(), Node::Null);
    Ok(Node::Map(fields))
}

fn resource_uses(node: &Node, document: &str) -> Vec<Value> {
    fn collect(node: &Node, pointer: String, owner: (&str, &str), uses: &mut Vec<Value>) {
        match node {
            Node::Owned { kind, id, node } => collect(node, pointer, (kind, id), uses),
            Node::Map(fields) => {
                for (key, value) in fields {
                    collect(value, super::native_vesta_v2::pointer(&pointer, key), owner, uses);
                }
            }
            Node::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    collect(item, format!("{pointer}/{i}"), owner, uses);
                }
            }
            Node::TaggedVariant { payload, .. } => {
                collect(payload, format!("{pointer}/payload"), owner, uses);
            }
            Node::Resource { id, sha, len, role } => uses.push(json!({
                "ownerKind":owner.0,"ownerId":owner.1,"pointer":pointer,"resourceId":id,
                "sha256":sha,"byteLength":len,"definesResource":*role == vesta_model::source_audit::Role::Definition,
            })),
            _ => {}
        }
    }
    let mut rows = Vec::new();
    collect(node, String::new(), ("Document", document), &mut rows);
    rows
}

pub(super) fn capture(source: &Capture) -> Result<(Value, Node)> {
    capture_version(source, Version::Current)
}

pub(super) fn capture_version(source: &Capture, version: Version) -> Result<(Value, Node)> {
    let (mut value, projections) = if let Some(full) = &source.source {
        (full.to_value()?, super::gsas_exp::projection_count(&full.audit()?))
    } else {
        (current_source7::capture(source)?.0, 0)
    };
    update_editable_phases(&mut value, &source.content)?;
    actual_resource_rows(&mut value, source)?;
    super::live_state::native_flags::refresh(source,&mut value)?;
    let mut qualified = super::scalar_grid_source::verify(source, &value)?;
    qualified.extend(super::live_state::verify(
        source, &value, version == Version::LegacyDisplay1,
    )?);
    qualified.extend(super::gsas_exp::verify(source, &value, projections)?);
    if source.content.resources.iter().any(|row| {
        let key = row.logical_key.as_str();
        ["descriptor-data/", "native-current-grid/", "native-scalar-grid/"]
            .iter().any(|prefix| key.starts_with(*prefix)) && !qualified.contains(key)
    }) {
        return Err(invalid("native grid resource lacks a qualified use"));
    }
    super::fdat_source::verify(source, &value)?;
    super::mxd_report::verify(source, &value)?;
    let editor = editor_projection(source, &value)?;
    let graphs = graph_provenance::verify(source)?;
    verify_orffe(source, &value, &graphs)?;
    let metadata = value["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("required document metadata collection"))?;
    metadata.retain(|row| {
        !(row["producerId"] == AUTHOR
            && row["dialect"].as_str().is_some_and(|s| {
                s == EDITOR || s == "native-core-editor2" || s.starts_with(GRAPH_PREFIX)
            }))
    });
    metadata.insert(0, editor_metadata(source, "VM18-I01", EDITOR, editor));
    for graph in &graphs {
        metadata.push(editor_metadata(
            source,
            graph.format_ref(),
            graph.dialect(),
            graph.audit_node(),
        ));
    }
    // Identity digests are computed from the captured state before the envelope or projection is built.
    let blocks: Vec<_> = metadata
        .iter()
        .filter(|row| {
            row["dialect"]
                .as_str()
                .is_some_and(|s| s.starts_with(GRAPH_PREFIX))
        })
        .cloned()
        .collect();
    if let Some(scenes) = value["scenes"].as_array_mut() {
        for scene in scenes {
            if let Some(render) = scene["renderContext"].as_object_mut() {
                render.insert("documentId".into(), json!(source.document.document_id));
                render.insert("epoch".into(), json!(source.document.epoch));
                render.insert("revision".into(), json!(source.document.revision));
            }
        }
    }
    let (types, rules) = version.digests();
    value["identity"] = json!({"documentId":source.document.document_id,
        "epoch":source.document.epoch,"revision":source.document.revision,
        "sourceTypeDigest":types,"sourceRootDigest":"0".repeat(64),
        "resourceGraphDigest":"0".repeat(64),"policyDigest":rules});
    super::opaque_tail::bind(source, &mut value, &graphs)?;
    let identity = std::mem::replace(&mut value["identity"], Value::Null);
    let source_root = digest(&canonical(&value));
    value["identity"] = identity;
    value["identity"]["sourceRootDigest"] = json!(source_root);
    let doc = SourceDocument::decode(value.clone())?;
    if doc.to_value()? != value {
        return Err(invalid("typed Source7 changed captured values"));
    }
    let active_fields = restore_fields(
        &value,
        &source.content.resources,
        &source.resources,
        100_000,
    )?;
    verify_c1_density(source, &value, &active_fields, &graphs)?;
    let mut node = doc.audit()?;
    node.definitions()?;
    let mut resources = vesta_model::source_audit::ResourceBytes::new();
    for r in &source.content.resources {
        let bytes = source
            .resources
            .get(&r.sha256)
            .ok_or_else(|| invalid("typed resource bytes missing"))?;
        resources.insert(r.id.to_string(), (r.sha256.clone(), bytes.clone()));
    }
    doc.validate_installed_resources(&resources, &source.content.resources)?;
    node.bind_resources(&resources)?;
    let graph = json!({"domain":"FC-complete-resource-closure-1", "nativeGraphs":blocks,
        "typedResourceUses":resource_uses(&node, &source.document.document_id),
        "activeFieldUses":value["fields"], "sourceTypeDigest":value["identity"]["sourceTypeDigest"],
        "ruleSetDigest":value["identity"]["policyDigest"]});
    value["identity"]["resourceGraphDigest"] = json!(digest(&canonical(&graph)));
    let doc = SourceDocument::decode(value.clone())?;
    let mut node = doc.audit()?;
    node.bind_resources(&resources)?;
    Ok((value, node))
}

pub(super) fn migrate_verified_legacy_display(
    mut capture: Capture,
    mut restored: Value,
) -> Result<Capture> {
    super::live_state::migrate_verified_legacy_display(&mut restored)?;
    let (types, rules) = Version::Current.digests();
    // The old capture has already reproduced its complete opaque footprints.
    // Rebind their version pins before independently validating the new capture.
    let opaque = restored["opaqueBlocks"]
        .as_array_mut()
        .ok_or_else(|| invalid("legacy full opaque population"))?;
    for row in opaque {
        let baseline = &mut row["editDependencies"]["baseline"];
        baseline["sourceTypeDigest"] = json!(types);
        baseline["policyDigest"] = json!(rules);
    }
    capture.source = Some(SourceDocument::decode(restored)?);
    let (current, _) = self::capture(&capture)?;
    capture.source = Some(SourceDocument::decode(current)?);
    Ok(capture)
}

pub(super) fn restore_core(value: &Value) -> Result<ProjectContent> {
    let rows = value["metadata"]
        .as_array()
        .ok_or_else(|| invalid("typed project metadata required"))?;
    let mut rows = rows
        .iter()
        .filter(|row| row["producerId"] == AUTHOR && row["dialect"] == EDITOR);
    let row = rows
        .next()
        .ok_or_else(|| invalid("field project editor projection missing"))?;
    if rows.next().is_some()
        || row["owner"] != json!({"kind":"document","id":value["identity"]["documentId"]})
    {
        return Err(invalid("field project editor projection owner differs"));
    }
    let mut editor = Node::from_wire(&row["tree"])?.data()?;
    if editor.get("sceneId") == Some(&Value::Null) {
        if !value["scenes"].as_array().is_some_and(Vec::is_empty) {
            return Err(invalid("private editor scene alongside typed scenes"));
        }
        let document = value["identity"]["documentId"]
            .as_str()
            .ok_or_else(|| invalid("typed document identity"))?;
        let scope = digest(format!("VestaDraft/private-editor-scene/1\n{document}").as_bytes());
        editor["sceneId"] = json!(&scope[..32]);
    }
    let content: ProjectContent =
        serde_json::from_value(editor).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    content.validate()?;
    Ok(content)
}
