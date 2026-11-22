use super::{CATALOG_ROOT, catalog, generated::*, registry, schema_validation, source_error};
use crate::{
    Result, digest,
    source_audit::{AuditSource, Id, Kind, Node, ResourceBytes},
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate(source: &SourceDocument) -> Result<()> {
    let mut entries: BTreeSet<&str> = BTreeSet::new();
    if let Some(graph) = &source.phase_graph {
        phase_graph(graph)?;
        entries.extend(graph.phases.iter().map(|phase| phase.id.as_str()));
    }
    entries.extend(source.molecules.iter().map(|molecule| molecule.id.as_str()));
    entries.extend(
        source
            .source_entities
            .iter()
            .map(|entity| entity.id.as_str()),
    );
    for entry in &source.source_entry_order {
        let id = match entry {
            SourceEntryRef::Phase(entry) => entry.id.as_str(),
            SourceEntryRef::Molecule(entry) => entry.id.as_str(),
            SourceEntryRef::SourceEntity(entry) => entry.id.as_str(),
        };
        if !entries.remove(id) {
            return Err(source_error(
                "sourceEntryOrder repeats an entry or includes a non-model entity",
            ));
        }
    }
    if !entries.is_empty() {
        return Err(source_error("sourceEntryOrder omits a model entity"));
    }

    for molecule in &source.molecules {
        for frame in &molecule.frames {
            let atoms: BTreeSet<_> = frame.atoms.iter().map(|atom| &atom.id).collect();
            for bond in &frame.bonds {
                if bond.atom_ids[0] == bond.atom_ids[1]
                    || !bond.atom_ids.iter().all(|id| atoms.contains(id))
                {
                    return Err(source_error(
                        "bond endpoints must be distinct atoms of its own molecule frame",
                    ));
                }
            }
        }
    }
    let graph = source.phase_graph.as_ref();
    let phases: BTreeMap<_, _> = graph
        .iter()
        .flat_map(|graph| &graph.phases)
        .map(|phase| (phase.id.as_str(), phase))
        .collect();
    let fields: BTreeMap<_, _> = source
        .fields
        .iter()
        .map(|field| (field.id.as_str(), field))
        .collect();
    let shapes: BTreeMap<_, _> = source
        .shapes
        .iter()
        .map(|shape| (shape.id.as_str(), shape))
        .collect();
    for phase in phases.values() {
        for id in &phase.field_ids {
            if !fields
                .get(id.as_str())
                .is_some_and(|field| phase_owner(&field.owner, &phase.id))
            {
                return Err(source_error(
                    "phase Field use points outside the phase owner",
                ));
            }
        }
        if let Some(id) = &phase.shape_id {
            if !shapes
                .get(id.as_str())
                .is_some_and(|shape| phase_owner(&shape.owner, &phase.id))
            {
                return Err(source_error(
                    "phase shape use points outside the phase owner",
                ));
            }
        }
    }
    for (index, field) in source.fields.iter().enumerate() {
        if !matches!(
            field.owner,
            Owner::Phase(_) | Owner::Molecule(_) | Owner::Scene(_)
        ) {
            return Err(source_error("Field owner must be Phase, Molecule or Scene"));
        }
        registry::verify_field_use(field, index)?;
    }
    for scene in &source.scenes {
        let ids: BTreeMap<_, _> = scene.nodes.iter().map(|node| node_link(node)).collect();
        let roots: BTreeSet<_> = scene.roots.iter().map(Id::as_str).collect();
        if roots.len() != scene.roots.len() || roots.iter().any(|id| ids.get(id) != Some(&None)) {
            return Err(source_error(
                "scene roots must be unique parentless nodes of the same scene",
            ));
        }
        for (&id, &parent) in &ids {
            if parent.is_none() && !roots.contains(id) {
                return Err(source_error("scene roots omit a parentless node"));
            }
        }
        acyclic_local(&ids)?;
        if let Some(context) = &scene.render_context {
            if context.document_id != source.identity.document_id
                || context.epoch != source.identity.epoch
                || context.revision != source.identity.revision
            {
                return Err(source_error("render context belongs to another snapshot"));
            }
        }
    }

    for shape in &source.shapes {
        alias(&shape.id, &shape.value.id)?;
        if !phase_owner(&shape.owner, &shape.value.phase_id) {
            return Err(source_error("SGShape phase differs from owner"));
        }
    }
    for vector in &source.vectors {
        alias(&vector.id, &vector.value.id)?;
        image(&phases, &vector.value.anchor)?;
    }
    for display in &source.displays {
        for item in &display.value.hidden_images {
            image(&phases, item)?;
        }
    }
    for mesh in &source.meshes {
        alias(&mesh.id, &mesh.value.object_id)?;
        if matches!(mesh.owner, Owner::Phase(_)) && !phase_owner(&mesh.owner, &mesh.value.phase_id)
        {
            return Err(source_error("SGRenderMesh phase differs from phase owner"));
        }
    }

    let mut sg_rules = BTreeMap::new();
    for rule in &source.bond_rules {
        if let SourceDocumentBondRulesItem::Branch0(rule) = rule {
            alias(&rule.id, &rule.value.id)?;
            let Owner::Phase(owner) = &rule.owner else {
                return Err(source_error("SG bond rule requires a phase owner"));
            };
            let selectors = [&rule.value.from_selector, &rule.value.to_selector];
            for selector in selectors {
                if let SGSiteSelector::Branch0(selector) = selector {
                    if !phases
                        .get(owner.id.as_str())
                        .and_then(|phase| phase.structure.as_ref())
                        .is_some_and(|structure| {
                            structure
                                .sites
                                .iter()
                                .any(|site| site.id == selector.site_id)
                        })
                    {
                        return Err(source_error("SG bond rule site belongs to another phase"));
                    }
                }
            }
            sg_rules.insert(rule.id.as_str(), &rule.owner);
        }
    }
    for rule in &source.bond_rules {
        if let SourceDocumentBondRulesItem::Branch1(rule) = rule {
            if !matches!(
                rule.owner,
                Owner::Phase(_) | Owner::Molecule(_) | Owner::SourceEntity(_)
            ) || rule
                .sg_projection_rule_ids
                .iter()
                .any(|id| sg_rules.get(id.as_str()) != Some(&&rule.owner))
            {
                return Err(source_error(
                    "native bond SG projection must resolve to an SG rule of the same owner",
                ));
            }
        }
    }
    for boundary in &source.boundaries {
        if !phase_owner(&boundary.owner, &boundary.value.phase_id)
            || boundary
                .value
                .rule_ids
                .iter()
                .any(|id| sg_rules.get(id.as_str()) != Some(&&boundary.owner))
        {
            return Err(source_error(
                "SG boundary phase and rules must have the same owner",
            ));
        }
    }
    let mut selected_points = BTreeMap::new();
    for plane in &source.planes {
        alias(&plane.id, &plane.value.id)?;
        match &plane.value.source {
            SGPlaneSpec::Branch0(spec) if phase_owner(&plane.owner, &spec.phase_id) => {}
            SGPlaneSpec::Branch1(spec) => {
                let Owner::Phase(owner) = &plane.owner else {
                    return Err(source_error("selected-image plane has no phase owner"));
                };
                for point in &spec.points {
                    image(&phases, &point.key)?;
                    if selected_points
                        .insert(
                            point.point_id.as_str(),
                            (owner.id.as_str(), point.key.phase.as_str()),
                        )
                        .is_some()
                    {
                        return Err(source_error(
                            "selected measurement point has multiple definitions",
                        ));
                    }
                }
            }
            _ => {
                return Err(source_error(
                    "SG plane source requires its actual phase owner",
                ));
            }
        }
    }
    for phase in phases.values() {
        if let Some(covariance) = phase.structure.as_ref().and_then(|s| s.covariance.as_ref()) {
            validate_covariance(covariance, phase.id.as_str(), &phases, &selected_points)?;
        }
    }
    for section in &source.sections {
        if !matches!(section.owner, Owner::Phase(_) | Owner::SourceEntity(_)) {
            return Err(source_error(
                "section requires an actual Phase or SourceEntity owner",
            ));
        }
        if section.geometry_qualification == "qualified-selected"
            && (!matches!(section.owner, Owner::Phase(_))
                || section.projection_descriptor_digest.is_none()
                || section.field_ids.is_empty())
        {
            return Err(source_error(
                "selected section lacks qualified projection inputs",
            ));
        }
    }
    Ok(())
}

fn validate_covariance(
    covariance: &SGCovariance,
    owner: &str,
    phases: &BTreeMap<&str, &SGPhaseNode>,
    points: &BTreeMap<&str, (&str, &str)>,
) -> Result<()> {
    let count = covariance.parameters.len();
    if covariance.matrix.len() != count || covariance.matrix.iter().any(|row| row.len() != count) {
        return Err(source_error(
            "SG covariance parameter and matrix dimensions differ",
        ));
    }
    let mut seen = BTreeSet::new();
    for parameter in &covariance.parameters {
        let fingerprint = serde_json::to_vec(parameter).map_err(|e| source_error(e.to_string()))?;
        if !seen.insert(fingerprint) {
            return Err(source_error("SG covariance repeats a parameter index"));
        }
        let (phase, site) = match parameter {
            SGParameter::Branch0(p) => (&p.phase_id, Some(&p.site_id)),
            SGParameter::Branch1(p) => {
                if points.get(p.point_id.as_str()) != Some(&(owner, owner)) {
                    return Err(source_error(
                        "SG Cartesian covariance point needs an actual selected measurement point of its owning phase",
                    ));
                }
                continue;
            }
            SGParameter::Branch2(p) => (&p.phase_id, None),
            SGParameter::Branch3(p) => (&p.phase_id, None),
            SGParameter::Branch4(p) => (&p.phase_id, Some(&p.site_id)),
            SGParameter::Branch5(p) => (&p.phase_id, Some(&p.site_id)),
        };
        let resolved = phases
            .get(phase.as_str())
            .and_then(|phase| phase.structure.as_ref());
        if resolved.is_none_or(|structure| {
            site.is_some_and(|site| {
                !structure
                    .sites
                    .iter()
                    .any(|candidate| &candidate.id == site)
            })
        }) {
            return Err(source_error(
                "SG covariance parameter has stale phase or site scope",
            ));
        }
    }
    Ok(())
}

fn alias<K: Kind>(id: &Id<K>, inner: &Id<K>) -> Result<()> {
    if id != inner {
        return Err(source_error(format!(
            "{} wrapper and inner alias differ",
            K::NAME
        )));
    }
    Ok(())
}

fn phase_owner(owner: &Owner, id: &Id<crate::source_audit::Phase>) -> bool {
    matches!(owner, Owner::Phase(owner) if &owner.id == id)
}

fn phase_graph(graph: &SGPhaseGraph) -> Result<()> {
    let ids: BTreeMap<_, _> = graph
        .phases
        .iter()
        .map(|phase| (phase.id.as_str(), phase.parent.as_ref().map(Id::as_str)))
        .collect();
    if ids.get(graph.root.as_str()) != Some(&None)
        || ids.values().filter(|v| v.is_none()).count() != 1
    {
        return Err(source_error(
            "SG phase graph requires one actual parentless root",
        ));
    }
    acyclic_local(&ids)
}

fn acyclic_local(nodes: &BTreeMap<&str, Option<&str>>) -> Result<()> {
    let mut completed = BTreeSet::new();
    for &start in nodes.keys() {
        let mut path = BTreeSet::new();
        let mut current = Some(start);
        while let Some(id) = current {
            if completed.contains(id) {
                break;
            }
            if !path.insert(id) {
                return Err(source_error("source parent graph contains a cycle"));
            }
            current = *nodes
                .get(id)
                .ok_or_else(|| source_error("parent belongs to another owner"))?;
        }
        completed.extend(path);
    }
    Ok(())
}

fn node_link(node: &SceneNode) -> (&str, Option<&str>) {
    macro_rules! link {
        ($v:expr) => {
            ($v.id.as_str(), $v.parent.as_ref().map(Id::as_str))
        };
    }
    match node {
        SceneNode::Group(v) => link!(v),
        SceneNode::PhaseInstance(v) => link!(v),
        SceneNode::MoleculeInstance(v) => link!(v),
        SceneNode::FieldInstance(v) => link!(v),
        SceneNode::ExplicitMesh(v) => link!(v),
        SceneNode::NativeOpaque(v) => link!(v),
        SceneNode::SectionInstance(v) => link!(v),
        SceneNode::PlaneInstance(v) => link!(v),
    }
}

fn image(phases: &BTreeMap<&str, &SGPhaseNode>, image: &Image) -> Result<()> {
    let structure = phases
        .get(image.phase.as_str())
        .and_then(|phase| phase.structure.as_ref())
        .ok_or_else(|| source_error("SG image phase has no qualified periodic structure"))?;
    if !structure.sites.iter().any(|site| site.id == image.site)
        || !structure
            .group
            .operators
            .iter()
            .any(|operation| operation.id == image.operation)
    {
        return Err(source_error(
            "SG image site or operation belongs to another phase",
        ));
    }
    Ok(())
}

pub(super) fn verify_spans(
    source: &Value,
    typed: &SourceDocument,
    resources: &ResourceBytes,
    installed: Option<&[crate::PersistentResource]>,
) -> Result<()> {
    let schemas = catalog();
    let by_digest: BTreeMap<_, _> = resources
        .values()
        .map(|(sha, bytes)| (sha.as_str(), bytes.as_slice()))
        .collect();
    let mut expected_descriptors = BTreeMap::new();
    for (m, molecule) in typed.molecules.iter().enumerate() {
        for (f, frame) in molecule.frames.iter().enumerate() {
            for (a, atom) in frame.atoms.iter().enumerate() {
                let base = format!("/molecules/{m}/frames/{f}/atoms/{a}");
                let mut add = |path: String, quantity: &Quantity| {
                    let key = format!("{base}/{path}/sourceDescriptorDigest");
                    expected_descriptors.insert(
                        key,
                        ("atom", atom.id.as_str(), quantity.source_descriptor_digest.clone()),
                    );
                };
                if let Some(quantity) = &atom.velocity {
                    add("velocity".into(), quantity);
                }
                if let Some(quantity) = &atom.force {
                    add("force".into(), quantity);
                }
                for (index, quantity) in atom.partial_charges.iter().enumerate() {
                    add(format!("partialCharges/{index}"), quantity);
                }
            }
        }
    }
    for (index, entry) in typed.source_entities.iter().enumerate() {
        let wire = &source["sourceEntities"][index]["sourceTree"];
        let roles: &[&str] = match (entry.native_kind.as_str(), wire["tag"].as_str()) {
            ("vend-native-stored-grid", Some("NativeScalarGridSource1"))
            | ("native-fractional-current-grid", Some("NativeFractionalGridCurrent1")) =>
                &["dataResource"],
            ("native-current-numerical-grid", Some("NativeScalarGridCurrent1")) =>
                &["dataResource", "maskResource"],
            _ => continue,
        };
        for &role in roles {
            let reference = &wire["payload"]["members"][role];
            if role == "maskResource" && reference["kind"] == "Null" {
                continue;
            }
            let resource = typed.resources.iter()
                .find(|resource| Some(resource.id.as_str()) == reference["resourceId"].as_str())
                .ok_or_else(|| source_error("native grid descriptor use has no resource"))?;
            expected_descriptors.insert(super::native_grid_resource_use(index, role),
                ("source-entity", entry.id.as_str(), resource.source_descriptor_digest.clone()));
        }
    }
    for (index, vector) in typed.vectors.iter().enumerate() {
        for row in source["vectors"][index]["metadata"].as_array().into_iter().flatten()
            .filter(|row| row["dialect"] == super::GSAS_MOMENT_QUANTITY_DIALECT)
        {
            let expected_digest = row["tree"]["members"]["sourceDescriptorDigest"]["value"]
                .as_str().ok_or_else(|| source_error("GSAS source moment descriptor digest"))?;
            if expected_descriptors.insert(
                super::gsas_moment_use(index), ("vector", vector.id.as_str(), expected_digest.into()),
            ).is_some() {
                return Err(source_error("GSAS source moment repeats its descriptor use"));
            }
        }
    }
    let mut visitor = Spans {
        schemas,
        resources,
        by_digest,
        expected_descriptors,
        installed: installed.map(|rows| rows.iter()
            .filter(|row| row.logical_key.starts_with("descriptor-data/"))
            .map(|row| (row.logical_key.as_str(), row.id.as_str())).collect()),
    };
    visitor.visit(source, &schemas[CATALOG_ROOT], CATALOG_ROOT)?;
    if !visitor.expected_descriptors.is_empty() {
        return Err(source_error(
            "Source7 descriptor use lacks an independently addressed BoundDescriptorContext1",
        ));
    }
    if visitor.installed.is_some_and(|rows| !rows.is_empty()) {
        return Err(source_error("installed descriptor resource does not belong to an actual use"));
    }
    Ok(())
}

struct Spans<'a> {
    schemas: &'a Value,
    resources: &'a ResourceBytes,
    by_digest: BTreeMap<&'a str, &'a [u8]>,
    expected_descriptors: BTreeMap<String, (&'static str, &'a str, String)>,
    installed: Option<BTreeMap<&'a str, &'a str>>,
}

impl Spans<'_> {
    fn visit(&mut self, value: &Value, schema: &Value, file: &str) -> Result<()> {
        if let Some(link) = schema.get("$ref").and_then(Value::as_str) {
            let (target, target_file) =
                schema_validation::ref_target(link, file, self.schemas).map_err(source_error)?;
            let ptr = link.split_once('#').map_or("", |(_, ptr)| ptr);
            match (target_file, ptr) {
                ("/source7/fc.json", "/$defs/SourceMetadata")
                    if value["tree"]["tag"] == "BoundDescriptorContext1" =>
                {
                    let owner = value["owner"]["id"]
                        .as_str()
                        .ok_or_else(|| source_error("descriptor metadata owner id"))?;
                    let kind = value["owner"]["kind"]
                        .as_str()
                        .ok_or_else(|| source_error("descriptor metadata owner kind"))?;
                    let typed = Node::from_wire(&value["tree"])?;
                    let (pointer, digest) =
                        registry::verify_descriptor_bytes(&typed, kind, owner, self.resources)?
                            .ok_or_else(|| source_error("bound descriptor type mismatch"))?;
                    let (expected_kind, expected_owner, expected_digest) = self.expected_descriptors.remove(pointer)
                        .ok_or_else(|| source_error("bound descriptor points outside an actual typed use or repeats it"))?;
                    if owner != expected_owner || kind != expected_kind || digest != expected_digest {
                        return Err(source_error(
                            "bound descriptor does not match actual typed owner and definition digest",
                        ));
                    }
                    if let Some(installed) = &mut self.installed {
                        registry::verify_descriptor_installation(&typed, pointer, installed)?;
                    }
                }
                ("/source7/fc.json", "/$defs/RawSpan") => {
                    let id = value["sourceResourceId"]
                        .as_str()
                        .ok_or_else(|| source_error("raw span resource ID"))?;
                    let (_, bytes) = self
                        .resources
                        .get(id)
                        .ok_or_else(|| source_error("raw span logical resource missing"))?;
                    let start = decimal(&value["byteOffset"])?;
                    let len = decimal(&value["byteLength"])?;
                    let span = extent(bytes, start, len)?;
                    if value["sha256"].as_str() != Some(digest(span).as_str()) {
                        return Err(source_error(
                            "raw span SHA-256 differs from the captured bytes",
                        ));
                    }
                }
                ("/source7/sg.json", "/$defs/SGSourceSpan") => {
                    let sha = value["resource"]
                        .as_str()
                        .ok_or_else(|| source_error("SG span resource digest"))?;
                    let bytes = self
                        .by_digest
                        .get(sha)
                        .ok_or_else(|| source_error("SG source span bytes missing"))?;
                    let offset = value["byteOffset"]
                        .as_u64()
                        .ok_or_else(|| source_error("SG span offset"))?;
                    let len = value["byteLength"]
                        .as_u64()
                        .ok_or_else(|| source_error("SG span length"))?;
                    extent(bytes, offset, len)?;
                }
                ("/source7/sg.json", "/$defs/SGTableIdentity") => {
                    for key in ["resource", "licenseResource"] {
                        let sha = value[key]
                            .as_str()
                            .ok_or_else(|| source_error("SG table byte digest"))?;
                        if !self.by_digest.contains_key(sha) {
                            return Err(source_error("SG table data or license bytes missing"));
                        }
                    }
                }
                ("/source7/audit.json", "/$defs/AuditNode") => return Ok(()),
                _ => {}
            }
            self.visit(value, target, target_file)?;
        }
        if let Some(branches) = schema.get("oneOf").and_then(Value::as_array) {
            for branch in branches {
                if schema_validation::validate(value, branch, file, self.schemas, 0).is_ok() {
                    self.visit(value, branch, file)?;
                }
            }
        }
        if let Some(object) = value.as_object() {
            for (key, member) in object {
                if let Some(member_schema) = schema.get("properties").and_then(|v| v.get(key)) {
                    self.visit(member, member_schema, file)?;
                }
            }
        }
        if let Some(array) = value.as_array() {
            if let Some(item) = schema.get("items") {
                for value in array {
                    self.visit(value, item, file)?;
                }
            }
        }
        Ok(())
    }
}

fn decimal(value: &Value) -> Result<u64> {
    value
        .as_str()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| source_error("raw span integer range"))
}

fn extent(bytes: &[u8], start: u64, len: u64) -> Result<&[u8]> {
    let range = usize::try_from(start)
        .ok()
        .zip(usize::try_from(len).ok())
        .and_then(|(start, len)| start.checked_add(len).map(|end| start..end));
    range
        .and_then(|range| bytes.get(range))
        .ok_or_else(|| source_error("source span leaves captured byte extent"))
}
