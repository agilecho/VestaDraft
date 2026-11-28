//! Registered AuditNode payloads. Runtime references are explicit typed nodes.
use super::{
    generated::{FieldBinding, Owner},
    resource_graph::orffe::{ACTIVE_TABLE_TAG, R80GeometryTable},
    schema_validation, source_error,
};
use crate::{
    Result, digest,
    source_audit::{AuditSource, Node, Remap, ResourceBytes, Role},
};
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, sync::OnceLock};

const FIELD: &str = "FieldStorageUse1";
const GRAPH: &str = "NativeGraphCapture2";
const DESCRIPTOR: &str = "BoundDescriptorContext1";
const DESCRIPTOR_SCHEMA: &[u8] = include_bytes!("frozen/source-descriptor.schema.json");
const DESCRIPTOR_RULE: &[u8] = include_bytes!("frozen/bound-descriptor-context.contract.json");
const DESCRIPTOR_FILE: &str = "/source7-tags/source-descriptor.json";

const FROZEN: &[(&str, &str, &str)] = &[
    (
        "NVIS1",
        "/$defs/NativeObjectFlagSource",
        include_str!("frozen/native-object-flags-source.schema.json"),
    ),
    (
        "NVIS2",
        "/$defs/NativeFlagState",
        include_str!("frozen/native-flag-state.schema.json"),
    ),
    (
        "FCON1",
        "",
        include_str!("frozen/fdat-connection-observation.schema.json"),
    ),
    (
        "NativeThermalState1",
        "",
        include_str!("frozen/native-thermal-state.schema.json"),
    ),
    (
        "NativeLayerCurrent1",
        "",
        include_str!(
            "../../../../contracts/source/31e8aa75b924339c-native-layer-current.schema.json"
        ),
    ),
    (
        "NativeScalarGridSource1",
        "",
        include_str!("frozen/native-scalar-grid-source.schema.json"),
    ),
    (
        "NativeFractionalGridCurrent1",
        "",
        include_str!(
            "../../../../contracts/source/024353fc084874aa-native-fractional-grid-current.schema.json"
        ),
    ),
    (
        "NativeScalarGridCurrent1",
        "",
        include_str!(
            "../../../../contracts/source/aae3f7260198c218-native-scalar-grid-current.schema.json"
        ),
    ),
    (
        "NativeStoredScalarIsoview1",
        "",
        include_str!(
            "../../../../contracts/source/5cdc2d8ddd4c1f14-native-stored-isoview.schema.json"
        ),
    ),
    (
        "GSASSpinProjection1",
        "",
        include_str!("frozen/gsas-spin-projection.schema.json"),
    ),
    (
        "NativeLegacyDisplayCurrent1",
        "/$defs/NativeLegacyDisplayCurrent",
        include_str!("frozen/vics-legacy-display-current.schema.json"),
    ),
    (
        "NativeLegacyDisplayCurrent2",
        "",
        include_str!("frozen/vics-legacy-display-current-2.schema.json"),
    ),
    (
        "NativeLegacyViewCurrent1",
        "/$defs/NativeLegacyViewCurrent",
        include_str!("frozen/vics-legacy-display-current.schema.json"),
    ),
];

pub(super) fn descriptor() -> Value {
    json!({
        "graphCapture": GRAPH,
        "nativeR80GeometryTable":R80GeometryTable::descriptor(),
        "fieldStorage": FIELD,
        "boundDescriptorSchemaSha256":digest(include_bytes!("frozen/bound-descriptor-context.schema.json")),
        "sourceDescriptorSchemaSha256":digest(DESCRIPTOR_SCHEMA),
        "boundDescriptorRuleSha256":digest(DESCRIPTOR_RULE),
        "typedObservations": ["MXDReportBlockObservations1"],
        "frozen": FROZEN.iter().map(|(tag, def, raw)| json!([tag,def,digest(raw.as_bytes())])).collect::<Vec<_>>(),
    })
}

fn map(node: &Node) -> Result<&BTreeMap<String, Node>> {
    if let Node::Map(m) = node {
        Ok(m)
    } else {
        Err(source_error("registered AuditNode payload must be Map"))
    }
}

fn members<'a>(
    node: &'a Node,
    required: &[&str],
    optional: &[&str],
) -> Result<&'a BTreeMap<String, Node>> {
    let m = map(node)?;
    if required.iter().any(|key| !m.contains_key(*key))
        || m.keys()
            .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return Err(source_error(
            "registered AuditNode payload has missing or extra members",
        ));
    }
    Ok(m)
}

fn label(node: &Node) -> Result<&str> {
    if let Node::Text(text) = node {
        Ok(text)
    } else {
        Err(source_error("registered AuditNode requires literal Utf8"))
    }
}

fn number(node: &Node) -> Result<u64> {
    if let Node::U64(value) = node {
        Ok(*value)
    } else {
        Err(source_error("registered AuditNode requires U64"))
    }
}

fn sequence(node: &Node) -> Result<&[Node]> {
    if let Node::Array(items) = node {
        Ok(items)
    } else {
        Err(source_error("registered AuditNode requires Array"))
    }
}

fn hex_digest(node: &Node) -> Result<&str> {
    let text = label(node)?;
    if text.len() != 64
        || !text
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(source_error("registered AuditNode requires SHA-256"));
    }
    Ok(text)
}

fn reference<'a>(node: &'a Node, expected: &[&str]) -> Result<(&'a str, &'a str)> {
    if let Node::Identity {
        kind,
        id,
        role: Role::Reference,
    } = node
    {
        if let Some(canonical) = canonical_kind(kind) {
            if (expected.is_empty() || expected.contains(&canonical)) && crate::valid_id(id) {
                return Ok((kind, id));
            }
        }
    }
    Err(source_error(
        "registered AuditNode requires a known typed EntityRef",
    ))
}

fn bytes(node: &Node) -> Result<(&str, &str, u64)> {
    if let Node::Resource {
        id,
        sha,
        len,
        role: Role::Reference,
    } = node
    {
        if crate::valid_id(id)
            && sha.len() == 64
            && sha
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        {
            if let Some(len) = len
                .parse::<u64>()
                .ok()
                .filter(|value| value.to_string() == *len)
            {
                return Ok((id, sha, len));
            }
        }
    }
    Err(source_error("registered AuditNode requires typed BytesRef"))
}

pub(super) fn canonical_kind(kind: &str) -> Option<&'static str> {
    let kinds = [
        ("document", "Document"),
        ("epoch", "Epoch"),
        ("scene", "Scene"),
        ("phase", "Phase"),
        ("site", "Site"),
        ("symmetry-operation", "SymmetryOp"),
        ("field", "Field"),
        ("molecule", "Molecule"),
        ("frame", "Frame"),
        ("atom", "Atom"),
        ("bond", "Bond"),
        ("basis-center", "BasisCenter"),
        ("scene-node", "SceneNode"),
        ("source-entity", "SourceEntity"),
        ("section", "Section"),
        ("bond-rule", "BondRule"),
        ("boundary", "Boundary"),
        ("plane", "Plane"),
        ("shape", "Shape"),
        ("shape-halfspace", "ShapeHalfspace"),
        ("vector", "Vector"),
        ("material", "Material"),
        ("display", "Display"),
        ("camera", "Camera"),
        ("mesh", "Mesh"),
        ("dependency", "Dependency"),
        ("opaque-block", "OpaqueBlock"),
        ("measurement-point", "MeasurementPoint"),
        ("resource", "Resource"),
    ];
    kinds
        .into_iter()
        .find_map(|(wire, canonical)| (kind == wire || kind == canonical).then_some(canonical))
}

pub(super) fn normalize_reference_roles(node: &mut Node) -> Result<()> {
    visit_mut(node, &mut |node| {
        if let Node::Identity { kind, .. } = node {
            *kind = canonical_kind(kind)
                .ok_or_else(|| source_error("unknown typed AuditNode reference kind"))?
                .into();
        }
        Ok(())
    })
}

pub(super) fn rebind_wire_tree(node: &mut Node, remap: &Remap) -> Result<()> {
    visit_mut(node, &mut |node| {
        if let Node::Identity { kind, id, .. } = node {
            let kind = canonical_kind(kind)
                .ok_or_else(|| source_error("unknown typed AuditNode reference kind"))?;
            *id = remap
                .get(&(kind.into(), id.clone()))
                .ok_or_else(|| source_error("AuditNode typed reference not in fresh mapping"))?
                .clone();
        }
        if let Node::Resource { .. } = node {
            node.rebind(remap)?;
        }
        Ok(())
    })
}

fn visit_mut(node: &mut Node, visitor: &mut impl FnMut(&mut Node) -> Result<()>) -> Result<()> {
    visitor(node)?;
    match node {
        Node::TaggedVariant { payload, .. } => visit_mut(payload, visitor)?,
        Node::Map(children) => {
            for node in children.values_mut() {
                visit_mut(node, visitor)?;
            }
        }
        Node::Array(children) => {
            for node in children {
                visit_mut(node, visitor)?;
            }
        }
        Node::Owned { .. } => {
            return Err(source_error("unexpected generated AuditNode owner variant"));
        }
        _ => {}
    }
    Ok(())
}

fn wire_kind(kind: &str) -> Result<String> {
    let canonical =
        canonical_kind(kind).ok_or_else(|| source_error("unknown typed EntityRef kind"))?;
    if canonical == "SymmetryOp" {
        return Ok("symmetry-operation".into());
    }
    let mut wire = String::new();
    for (index, c) in canonical.chars().enumerate() {
        if c.is_ascii_uppercase() && index != 0 {
            wire.push('-');
        }
        wire.push(c.to_ascii_lowercase());
    }
    Ok(wire)
}

fn pointer(value: &str) -> bool {
    value.starts_with('/') && {
        let mut chars = value.chars();
        while let Some(c) = chars.next() {
            if c == '~' && !matches!(chars.next(), Some('0' | '1')) {
                return false;
            }
        }
        true
    }
}

pub(super) fn verify(tree: &Node) -> Result<()> {
    match tree {
        Node::TaggedVariant { tag, payload } => {
            match tag.as_str() {
                GRAPH => {
                    let m = members(payload, &["graph", "files"], &[])?;
                    bytes(&m["graph"])?;
                    for (name, record) in map(&m["files"])? {
                        if name.contains(['\\', ':'])
                            || name.chars().any(char::is_control)
                            || name
                                .split('/')
                                .any(|component| matches!(component, "" | "." | ".."))
                        {
                            return Err(source_error("unsafe Graph2 captured file key"));
                        }
                        bytes(record)?;
                    }
                }
                FIELD => {
                    let m = members(
                        payload,
                        &[
                            "field",
                            "owner",
                            "usePointer",
                            "bytes",
                            "encoding",
                            "descriptorDigest",
                        ],
                        &[],
                    )?;
                    reference(&m["field"], &["Field"])?;
                    reference(&m["owner"], &["Phase", "Molecule", "Scene"])?;
                    bytes(&m["bytes"])?;
                    hex_digest(&m["descriptorDigest"])?;
                    if !pointer(label(&m["usePointer"])?)
                        || label(&m["encoding"])? != "ieee754-little-endian"
                    {
                        return Err(source_error("invalid Field storage use or encoding"));
                    }
                }
                DESCRIPTOR => bound_descriptor(payload)?,
                ACTIVE_TABLE_TAG => {
                    let table: R80GeometryTable = serde_json::from_value(payload.data()?)
                        .map_err(|e| source_error(e.to_string()))?;
                    if table.audit()?.wire() != payload.wire() {
                        return Err(source_error(
                            "ORFFE current table has untyped or changed roles",
                        ));
                    }
                }
                "MXDReportBlockObservations1" => mxd(payload)?,
                _ => verify_frozen(tag, payload)?,
            }
            verify(payload)?;
        }
        Node::Map(children) => {
            for child in children.values() {
                verify(child)?;
            }
        }
        Node::Array(children) => {
            for child in children {
                verify(child)?;
            }
        }
        Node::Identity { .. } => {
            reference(tree, &[])?;
        }
        Node::Resource { .. } => {
            bytes(tree)?;
        }
        Node::Owned { .. } => return Err(source_error("source AuditNode cannot define an object")),
        _ => {}
    }
    Ok(())
}

fn bound_descriptor(payload: &Node) -> Result<()> {
    let m = members(
        payload,
        &[
            "schemaVersion",
            "owner",
            "sourceTypedPointer",
            "scopeInput",
            "definition",
            "descriptorSchemaDigest",
            "referenceRuleDigest",
            "bindings",
        ],
        &[],
    )?;
    if label(&m["schemaVersion"])? != "BoundDescriptorContext-proposal-1"
        || !pointer(label(&m["sourceTypedPointer"])?)
    {
        return Err(source_error(
            "BoundDescriptorContext internal version or pointer",
        ));
    }
    let owner = members(&m["owner"], &["kind", "id"], &[])?;
    let allowed = canonical_kind(label(&owner["kind"])?)
        .ok_or_else(|| source_error("bound descriptor owner kind"))?;
    reference(&owner["id"], &[allowed])?;
    let resource_metadata = |node| -> Result<()> {
        let m = members(node, &["resourceId", "sha256", "byteLength"], &[])?;
        reference(&m["resourceId"], &["Resource"])?;
        hex_digest(&m["sha256"])?;
        let len = label(&m["byteLength"])?;
        if len
            .parse::<u64>()
            .ok()
            .is_none_or(|value| value.to_string() != len)
        {
            return Err(source_error("bound descriptor length"));
        }
        Ok(())
    };
    resource_metadata(&m["definition"])?;
    resource_metadata(&m["scopeInput"])?;
    hex_digest(&m["descriptorSchemaDigest"])?;
    hex_digest(&m["referenceRuleDigest"])?;
    let mut previous = None;
    for node in sequence(&m["bindings"])? {
        let binding = members(node, &["jsonPointer", "capturedAlias", "target"], &[])?;
        let at = label(&binding["jsonPointer"])?;
        if !pointer(at)
            || previous.is_some_and(|old| old >= at)
            || label(&binding["capturedAlias"])?.is_empty()
        {
            return Err(source_error(
                "bound descriptor binding pointer or captured alias",
            ));
        }
        previous = Some(at);
        resource_metadata(&binding["target"])?;
    }
    Ok(())
}

pub(super) fn verify_descriptor_bytes<'a>(
    tree: &'a Node,
    owner_kind: &str,
    owner_id: &str,
    resources: &ResourceBytes,
) -> Result<Option<(&'a str, &'a str)>> {
    let Node::TaggedVariant { tag, payload } = tree else {
        return Ok(None);
    };
    if tag != DESCRIPTOR {
        return Ok(None);
    }
    let ctx = map(payload)?;
    let owner = map(&ctx["owner"])?;
    let (kind, id) = reference(&owner["id"], &[])?;
    if canonical_kind(kind) != canonical_kind(owner_kind) || id != owner_id {
        return Err(source_error(
            "bound descriptor owner differs from its SourceMetadata",
        ));
    }
    if label(&ctx["descriptorSchemaDigest"])? != digest(DESCRIPTOR_SCHEMA)
        || label(&ctx["referenceRuleDigest"])? != digest(DESCRIPTOR_RULE)
    {
        return Err(source_error(
            "bound descriptor schema or role rule differs from frozen pin",
        ));
    }
    let (definition_sha, raw) = verified_resource(&ctx["definition"], resources)?;
    let (scope_sha, _) = verified_resource(&ctx["scopeInput"], resources)?;
    let definition = crate::json::parse(raw, 4 * 1024 * 1024)?;
    static CATALOG: OnceLock<Value> = OnceLock::new();
    let catalog = CATALOG.get_or_init(|| {
        json!({ (DESCRIPTOR_FILE): serde_json::from_slice::<Value>(DESCRIPTOR_SCHEMA)
            .expect("frozen SourceDescriptor2") })
    });
    schema_validation::validate(
        &definition,
        &catalog[DESCRIPTOR_FILE],
        DESCRIPTOR_FILE,
        catalog,
        0,
    )
    .map_err(|e| source_error(format!("frozen SourceDescriptor2: {e}")))?;
    if definition["scopeInputDigest"].as_str() != Some(scope_sha) {
        return Err(source_error("bound descriptor scope input digest"));
    }
    let mut occurrences = BTreeMap::new();
    let mut evidence = |value: &Value, pointer: String| {
        occurrences.insert(
            pointer,
            (value["evidenceResource"].clone(), Some(value.clone())),
        );
    };
    for (index, value) in definition["evidence"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        evidence(value, format!("/evidence/{index}/evidenceResource"));
    }
    for (field_index, field) in definition["fieldBindings"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        for (index, value) in field["evidence"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
        {
            evidence(
                value,
                format!("/fieldBindings/{field_index}/evidence/{index}/evidenceResource"),
            );
        }
    }
    for (index, field) in definition["fieldBindings"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        if !field["frameResource"].is_null() {
            occurrences.insert(
                format!("/fieldBindings/{index}/frameResource"),
                (field["frameResource"].clone(), None),
            );
        }
    }
    for binding in sequence(&ctx["bindings"])? {
        let binding = map(binding)?;
        let pointer = label(&binding["jsonPointer"])?;
        let (captured, span) = occurrences.remove(pointer).ok_or_else(|| {
            source_error("bound descriptor contains unknown or duplicate reference occurrence")
        })?;
        let (sha, raw) = verified_resource(&binding["target"], resources)?;
        if captured["resourceId"].as_str() != Some(label(&binding["capturedAlias"])?)
            || captured["sha256"].as_str() != Some(sha)
            || captured["byteLength"].as_str() != Some(raw.len().to_string().as_str())
        {
            return Err(source_error(
                "bound descriptor captured alias or exact target bytes differ",
            ));
        }
        if let Some(span) = span {
            let start = span["byteOffset"]
                .as_str()
                .and_then(|s| s.parse::<usize>().ok());
            let count = span["byteLength"]
                .as_str()
                .and_then(|s| s.parse::<usize>().ok());
            let bytes = start
                .zip(count)
                .filter(|(_, len)| *len != 0)
                .and_then(|(start, len)| start.checked_add(len).and_then(|end| raw.get(start..end)))
                .ok_or_else(|| {
                    source_error("bound descriptor evidence span exceeds original bytes")
                })?;
            if span["sourceSpanDigest"].as_str() != Some(digest(bytes).as_str()) {
                return Err(source_error("bound descriptor evidence span digest"));
            }
        }
    }
    if !occurrences.is_empty() {
        return Err(source_error(
            "bound descriptor logical binding omitted an actual reference",
        ));
    }
    Ok(Some((label(&ctx["sourceTypedPointer"])?, definition_sha)))
}

pub(super) fn verify_descriptor_installation(
    tree: &Node,
    typed_pointer: &str,
    installed: &mut BTreeMap<&str, &str>,
) -> Result<()> {
    let Node::TaggedVariant { payload, .. } = tree else {
        return Err(source_error("expected registered descriptor context"));
    };
    let context = map(payload)?;
    let prefix = format!("descriptor-data/{}", digest(typed_pointer.as_bytes()));
    let mut check = |slot: &str, record: &Node| -> Result<()> {
        let (_, id) = reference(&map(record)?["resourceId"], &["Resource"])?;
        let key = format!("{prefix}/{slot}");
        if installed.remove(key.as_str()) != Some(id) {
            return Err(source_error(format!("descriptor logical resource differs from installed {key}")));
        }
        Ok(())
    };
    check("definition", &context["definition"])?;
    check("scope", &context["scopeInput"])?;
    for binding in sequence(&context["bindings"])? {
        let binding = map(binding)?;
        let at = label(&binding["jsonPointer"])?;
        let target = format!("target/{}", digest(at.as_bytes()));
        check(&target, &binding["target"])?;
    }
    Ok(())
}

fn verified_resource<'a, 'b>(
    node: &'a Node,
    resources: &'b ResourceBytes,
) -> Result<(&'a str, &'b [u8])> {
    let record = map(node)?;
    let (_, id) = reference(&record["resourceId"], &["Resource"])?;
    let expected = label(&record["sha256"])?;
    let len = label(&record["byteLength"])?;
    let (sha, raw) = resources
        .get(id)
        .ok_or_else(|| source_error("bound descriptor resource not captured"))?;
    if sha != expected || digest(raw) != *sha || len != raw.len().to_string() {
        return Err(source_error(
            "bound descriptor logical resource content differs",
        ));
    }
    Ok((expected, raw))
}

fn mxd(payload: &Node) -> Result<()> {
    let m = members(
        payload,
        &[
            "reportBlockIndex",
            "unit",
            "quantityKind",
            "rawHeading",
            "producerContext",
            "sites",
        ],
        &[],
    )?;
    number(&m["reportBlockIndex"])?;
    label(&m["rawHeading"])?;
    map(&m["producerContext"])?;
    if label(&m["unit"])? != "1"
        || label(&m["quantityKind"])? != "source-cell-fractional-component-amplitude"
    {
        return Err(source_error("MXD observation unit or quantity"));
    }
    for item in sequence(&m["sites"])? {
        let site = members(
            item,
            &["sourceOrdinal", "sourceLabel4", "axes"],
            &["targetSite"],
        )?;
        number(&site["sourceOrdinal"])?;
        label(&site["sourceLabel4"])?;
        if let Some(target) = site.get("targetSite") {
            reference(target, &["Site"])?;
        }
        for item in sequence(&site["axes"])? {
            let axis = members(
                item,
                &["axis", "meanToken", "statisticToken", "referenceToken"],
                &[],
            )?;
            for item in axis.values() {
                label(item)?;
            }
        }
    }
    Ok(())
}

fn verify_frozen(tag: &str, payload: &Node) -> Result<()> {
    static CATALOG: OnceLock<Value> = OnceLock::new();
    let registry = CATALOG.get_or_init(|| {
        let files = FROZEN
            .iter()
            .map(|(tag, _, raw)| (format!("/source7-tags/{tag}.json"), *raw))
            .chain(std::iter::once(("/source7-tags/sg.json".into(), super::SG)));
        Value::Object(
            files
                .map(|(file, raw)| {
                    let resolved = raw.replace("urn:draft:input:SG1-r2#", "sg.json#");
                    (
                        file,
                        serde_json::from_str(&resolved).expect("frozen Source7 tag"),
                    )
                })
                .collect(),
        )
    });
    let (_, definition, _) = FROZEN
        .iter()
        .find(|(name, _, _)| *name == tag)
        .ok_or_else(|| source_error(format!("unregistered Source7 AuditNode tag {tag}")))?;
    let file = format!("/source7-tags/{tag}.json");
    let rule = registry[&file]
        .pointer(definition)
        .ok_or_else(|| source_error("registered schema pointer"))?;
    let projection = Projection {
        catalog: registry,
        tag,
    };
    projection
        .read(payload, rule, &file, definition)
        .map(|_| ())
        .map_err(|e| source_error(format!("registered Source7 tag {tag}: {e}")))
}

struct Projection<'a> {
    catalog: &'a Value,
    tag: &'a str,
}

impl Projection<'_> {
    fn read(&self, node: &Node, schema: &Value, file: &str, path: &str) -> Result<Value> {
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            if scalar_role(self.tag, file, path).is_none() {
                let (rule, target_file) =
                    schema_validation::ref_target(reference, file, self.catalog)
                        .map_err(source_error)?;
                return self.read(
                    node,
                    rule,
                    target_file,
                    reference.split_once('#').map_or("", |(_, p)| p),
                );
            }
        }
        if let Some(variants) = schema.get("oneOf").and_then(Value::as_array) {
            let mut selected = None;
            for (index, schema) in variants.iter().enumerate() {
                if let Ok(value) = self.read(node, schema, file, &format!("{path}/oneOf/{index}")) {
                    if selected.replace(value).is_some() {
                        return Err(source_error("ambiguous registered source variant"));
                    }
                }
            }
            return selected.ok_or_else(|| source_error("unqualified registered source variant"));
        }
        let kind = schema["properties"]["kind"]["const"].as_str();
        let value = if let Some(kind) = scalar_role(self.tag, file, path) {
            let (_, id) = reference(node, &[kind])?;
            json!(id)
        } else if self.tag == "FCON1" && path == "/properties/sourceEntryOrdinal" {
            let Node::U64(ordinal) = node else {
                return Err(source_error("FDAT source entry ordinal must be U64"));
            };
            json!(ordinal.to_string())
        } else if kind == Some("EntityRef") {
            let (kind, id) = reference(node, &[])?;
            json!({"kind":"EntityRef","objectKind":wire_kind(kind)?,"objectId":id})
        } else if kind == Some("BytesRef") {
            bytes(node)?;
            node.wire()
        } else if matches!((kind, node), (Some("F32Bits"), Node::F32(_)) | (Some("F64Bits"), Node::F64(_))) {
            node.wire()
        } else {
            match node {
                Node::Map(m) => Value::Object(
                    m.iter()
                        .map(|(key, node)| {
                            let member = schema
                                .get("properties")
                                .and_then(|v| v.get(key))
                                .ok_or_else(|| {
                                    source_error(format!(
                                        "unregistered typed payload member {key} at {path}"
                                    ))
                                })?;
                            Ok((
                                key.clone(),
                                self.read(node, member, file, &format!("{path}/properties/{key}"))?,
                            ))
                        })
                        .collect::<Result<Map<_, _>>>()?,
                ),
                Node::Array(items) => {
                    let element = schema
                        .get("items")
                        .ok_or_else(|| source_error("unregistered typed payload array"))?;
                    Value::Array(
                        items
                            .iter()
                            .map(|node| self.read(node, element, file, &format!("{path}/items")))
                            .collect::<Result<_>>()?,
                    )
                }
                Node::F32(value) => json!(format!("{value:08x}")),
                Node::F64(value) => json!(format!("{value:016x}")),
                Node::Identity { .. }
                | Node::Resource { .. }
                | Node::TaggedVariant { .. }
                | Node::Owned { .. } => {
                    return Err(source_error(
                        "unclassified typed payload reference or variant",
                    ));
                }
                _ => node.data()?,
            }
        };
        schema_validation::validate(&value, schema, file, self.catalog, 0).map_err(source_error)?;
        Ok(value)
    }
}

// NVIS and FCON predate explicit EntityRef objects in the ordinary payload
// schemas. These exact locations are specified as typed roles by their
// source-encoding contracts. The surrounding discriminators fix the kind.
fn scalar_role(tag: &str, file: &str, path: &str) -> Option<&'static str> {
    if file == "/source7-tags/sg.json" {
        return match path {
            "/$defs/Image/properties/phase" => Some("Phase"),
            "/$defs/Image/properties/site" => Some("Site"),
            "/$defs/Image/properties/operation" => Some("SymmetryOp"),
            _ => None,
        };
    }
    match (tag, path) {
        ("FCON1", "/properties/sourceResourceId") => Some("Resource"),
        ("NVIS1", "/$defs/NativeObjectFlagSource/properties/phaseId")
        | ("NVIS2", "/$defs/Owner/oneOf/0/properties/id") => Some("Phase"),
        ("NVIS2", "/$defs/Owner/oneOf/1/properties/id") => Some("Molecule"),
        ("NVIS2", "/$defs/SourceSpan/properties/resourceId") => Some("Resource"),
        ("NVIS1" | "NVIS2", "/$defs/Target/oneOf/1/properties/ruleId")
        | ("NVIS2", "/$defs/Target/oneOf/6/properties/ruleId") => Some("BondRule"),
        ("NVIS1" | "NVIS2", "/$defs/Target/oneOf/2/properties/boundaryRuleId") => Some("Boundary"),
        ("NVIS1" | "NVIS2", "/$defs/Target/oneOf/3/properties/shapeId") => Some("Shape"),
        ("NVIS1" | "NVIS2", "/$defs/Target/oneOf/3/properties/halfspaceId") => {
            Some("ShapeHalfspace")
        }
        ("NVIS1" | "NVIS2", "/$defs/Target/oneOf/3/properties/operationId") => Some("SymmetryOp"),
        (
            "NVIS2",
            "/$defs/Target/oneOf/4/properties/moleculeId"
            | "/$defs/Target/oneOf/5/properties/moleculeId"
            | "/$defs/Target/oneOf/6/properties/moleculeId",
        ) => Some("Molecule"),
        (
            "NVIS2",
            "/$defs/Target/oneOf/4/properties/frameId"
            | "/$defs/Target/oneOf/5/properties/frameId"
            | "/$defs/Target/oneOf/6/properties/frameId",
        ) => Some("Frame"),
        (
            "NVIS2",
            "/$defs/Target/oneOf/4/properties/atomId"
            | "/$defs/Target/oneOf/6/properties/centerAtomId"
            | "/$defs/Target/oneOf/6/properties/vertexAtomIds/items",
        ) => Some("Atom"),
        ("NVIS2", "/$defs/Target/oneOf/5/properties/bondId") => Some("Bond"),
        _ => None,
    }
}

pub(super) fn verify_field_use(field: &FieldBinding, index: usize) -> Result<()> {
    let (expected_kind, owner) = match &field.owner {
        Owner::Phase(owner) => ("Phase", owner.id.as_str()),
        Owner::Molecule(owner) => ("Molecule", owner.id.as_str()),
        Owner::Scene(owner) => ("Scene", owner.id.as_str()),
        _ => return Err(source_error("Field owner kind")),
    };
    let mut found = false;
    for item in &field.metadata {
        let Node::TaggedVariant { tag, payload } = &item.tree.0 else {
            continue;
        };
        if tag != FIELD {
            continue;
        }
        let m = map(payload)?;
        let (_, stored_field) = reference(&m["field"], &["Field"])?;
        let (_, stored_owner) = reference(&m["owner"], &[expected_kind])?;
        let (_, sha, len) = bytes(&m["bytes"])?;
        if found
            || !matches!(&item.owner, Owner::Field(owner) if owner.id == field.id)
            || stored_field != field.id.as_str()
            || stored_owner != owner
            || label(&m["usePointer"])? != format!("/fields/{index}/field/resource")
            || sha != field.field.resource.digest
            || Some(len) != field.field.resource.bytes.as_u64()
        {
            return Err(source_error(
                "Field storage use does not bind actual Field, owner and bytes",
            ));
        }
        found = true;
    }
    Ok(())
}
