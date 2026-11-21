//! Frozen Source7 and its generated persistent types.
use crate::{
    Error, Result, digest,
    source_audit::{AuditSource, Node, Remap, ResourceBytes, Role, ValueSource, canonical},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

mod registry;
pub mod resource_graph;
pub mod schema_validation;
mod semantic;

const SOURCE: &str = include_str!("../../../codecs/contracts/codec-source-document.schema.json");
const AUDIT: &str = include_str!("../../../codecs/contracts/complete-source-audit.schema.json");
const SG: &str = include_str!("../../../codecs/contracts/structure-geometry.contract.json");
const FS: &str =
    include_str!("../../../../../contracts/source/adf539aac53ddd41-field-science.contract.json");
const MODERN_SECTION: &str =
    include_str!("../../../codecs/contracts/native-section-source.schema.json");
const LEGACY_SECTION: &str =
    include_str!("../../../codecs/contracts/native-legacy-section-source.schema.json");
const BOND: &str = include_str!("../../../codecs/contracts/native-bond-source.schema.json");
const ROLE_COVERAGE: &str = include_str!("coverage.json");
const CATALOG_ROOT: &str = "/source7/fc.json";

/// Registered NativeScalarGridSource1 data occurrence in the closed AuditNode.
pub fn native_grid_data_use(source_index: usize) -> String {
    native_grid_resource_use(source_index, "dataResource")
}

pub fn native_grid_resource_use(source_index: usize, role: &str) -> String {
    format!("/sourceEntities/{source_index}/sourceTree/payload/members/{role}")
}

fn source_error(detail: impl Into<String>) -> Error {
    Error::new("E_SCHEMA", detail)
}

// The generator selects exactly one branch by a frozen discriminator or a
// member unique to that closed variant. Runtime type trials are not used.
macro_rules! closed_union {
    ($type:ident { $($variant:ident($payload:ty) |$value:ident| $guard:expr,)* }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $type {
            $($variant($payload),)*
        }
        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                match self {
                    $(Self::$variant(value) => value.serialize(serializer),)*
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                let raw = Value::deserialize(deserializer)?;
                let matches = [$( (|$value: &Value| $guard)(&raw), )*];
                if matches.iter().filter(|&&yes| yes).count() != 1 {
                    return Err(serde::de::Error::custom(concat!("closed Source7 variant ", stringify!($type))));
                }
                $(
                    if (|$value: &Value| $guard)(&raw) {
                        return serde_json::from_value::<$payload>(raw)
                            .map(Self::$variant)
                            .map_err(serde::de::Error::custom);
                    }
                )*
                Err(serde::de::Error::custom("Source7 variant dispatch"))
            }
        }
        impl crate::source_audit::AuditSource for $type {
            fn audit(&self) -> crate::Result<crate::source_audit::Node> {
                match self {
                    $(Self::$variant(value) => value.audit(),)*
                }
            }
            fn rebind(&mut self, map: &crate::source_audit::Remap) -> crate::Result<()> {
                match self {
                    $(Self::$variant(value) => value.rebind(map),)*
                }
            }
            fn descriptor() -> Value {
                serde_json::json!({
                    "type": stringify!($type),
                    "dispatch": "generated-disjoint-closed",
                    "variants": [$( [serde_json::json!(stringify!($variant)), <$payload>::descriptor()], )*],
                })
            }
        }
        impl crate::source_audit::ValueSource for $type {}
    };
}
mod generated;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct NullValue(());
impl AuditSource for NullValue {
    fn audit(&self) -> Result<Node> {
        Ok(Node::Null)
    }
    fn rebind(&mut self, _: &Remap) -> Result<()> {
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"null-only"})
    }
}
impl ValueSource for NullValue {}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
struct AuditTree(Node);

impl<'de> Deserialize<'de> for AuditTree {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = Value::deserialize(d)?;
        let tree = Node::from_wire(&raw).map_err(serde::de::Error::custom)?;
        if tree.wire() != raw {
            return Err(serde::de::Error::custom("noncanonical Source7 audit value"));
        }
        registry::verify(&tree).map_err(serde::de::Error::custom)?;
        Ok(Self(tree))
    }
}
impl AuditSource for AuditTree {
    fn audit(&self) -> Result<Node> {
        let mut node = self.0.audit()?;
        // A registered type may use the case-sensitive wire spelling of its
        // namespace. The objectKind data is preserved while the identity role
        // itself addresses the one canonical restoration-map entry.
        registry::normalize_reference_roles(&mut node)?;
        Ok(node)
    }
    fn rebind(&mut self, map: &Remap) -> Result<()> {
        registry::rebind_wire_tree(&mut self.0, map)
    }
    fn descriptor() -> Value {
        json!({"type":"Source7-registered-AuditNode","algebra":Node::descriptor(),"registry":registry::descriptor()})
    }
}
impl ValueSource for AuditTree {}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SourceDocument(generated::SourceDocument);

impl SourceDocument {
    pub fn decode(input: Value) -> Result<Self> {
        let catalog = catalog();
        schema_validation::validate(&input, &catalog[CATALOG_ROOT], CATALOG_ROOT, catalog, 0)
            .map_err(|e| source_error(format!("frozen Source7 schema {e}")))?;
        let document: generated::SourceDocument = serde_json::from_value(input.clone())
            .map_err(|e| source_error(format!("generated Source7 {e}")))?;
        let candidate = Self(document);
        let audit = candidate.0.audit()?;
        semantic::validate(&candidate.0)?;
        audit.definitions()?;
        if candidate.to_value()? != input || audit.data()? != input {
            return Err(source_error("generated Source7 exact member coverage"));
        }
        Ok(candidate)
    }

    pub fn to_value(&self) -> Result<Value> {
        serde_json::to_value(&self.0).map_err(|e| source_error(e.to_string()))
    }

    /// Verifies all declared resources, typed byte uses and every source span.
    pub fn validate_resources(&self, resources: &ResourceBytes) -> Result<()> {
        self.check_resources(resources, None)
    }

    /// Validate the descriptor resources selected by the native installer.
    /// The source tree's self-declared SHA and EntityRefs do not select a slot.
    pub fn validate_installed_resources(
        &self,
        resources: &ResourceBytes,
        installed: &[crate::PersistentResource],
    ) -> Result<()> {
        self.check_resources(resources, Some(installed))
    }

    fn check_resources(
        &self,
        resources: &ResourceBytes,
        installed: Option<&[crate::PersistentResource]>,
    ) -> Result<()> {
        let mut node = self.0.audit()?;
        node.bind_resources(resources)?;
        for resource in &self.0.resources {
            let &(ref sha, ref bytes) = resources
                .get(resource.id.as_str())
                .ok_or_else(|| source_error("Source7 logical resource missing"))?;
            if sha != &resource.sha256 || bytes.len().to_string() != resource.byte_length {
                return Err(source_error(
                    "Source7 resource descriptor differs from bytes",
                ));
            }
        }
        let raw = self.to_value()?;
        semantic::verify_spans(&raw, &self.0, resources, installed)
    }
}
impl<'de> Deserialize<'de> for SourceDocument {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        Self::decode(Value::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
impl AuditSource for SourceDocument {
    fn audit(&self) -> Result<Node> {
        self.0.audit()
    }

    fn rebind(&mut self, map: &Remap) -> Result<()> {
        let defs = self.0.audit()?.definitions()?;
        let originals: BTreeSet<_> = defs.keys().map(|(_, id)| id).collect();
        let mut fresh = BTreeSet::new();
        for key in defs.keys() {
            let id = map
                .get(key)
                .ok_or_else(|| source_error("Source7 definition lacks fresh mapping"))?;
            if !crate::valid_id(id) || originals.contains(id) || !fresh.insert(id) {
                return Err(source_error("Source7 remap is not a fresh typed bijection"));
            }
        }
        let mut candidate = self.0.clone();
        candidate.rebind(map)?;
        let data = serde_json::to_value(candidate).map_err(|e| source_error(e.to_string()))?;
        *self = Self::decode(data)?;
        Ok(())
    }

    fn descriptor() -> Value {
        json!({
            "type":"Source7",
            "frozenRoleCoverageSha256":digest(ROLE_COVERAGE.as_bytes()),
            "generated":generated::SourceDocument::descriptor(),
        })
    }
}
impl ValueSource for SourceDocument {}

fn catalog() -> &'static Value {
    static CATALOG: OnceLock<Value> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let files = [
            ("fc.json", SOURCE, ""),
            ("audit.json", AUDIT, "urn:draft:input:AUDIT1"),
            ("sg.json", SG, "urn:draft:input:SG1-r2"),
            ("fs.json", FS, "urn:draft:input:FS1-r1"),
            (
                "modern.json",
                MODERN_SECTION,
                "urn:draft:input:NATIVESECTION1",
            ),
            (
                "legacy.json",
                LEGACY_SECTION,
                "urn:draft:input:NATIVESECTION2",
            ),
            ("bond.json", BOND, "urn:draft:input:NBOND1"),
        ];
        let aliases: BTreeMap<_, _> = files
            .iter()
            .map(|(name, _, alias)| (*alias, *name))
            .collect();
        fn normalize(v: &mut Value, aliases: &BTreeMap<&str, &str>) {
            match v {
                Value::Object(o) => {
                    if let Some(Value::String(r)) = o.get_mut("$ref") {
                        if let Some((file, fragment)) = r.split_once('#') {
                            if !file.is_empty() {
                                *r = format!("{}#{fragment}", aliases.get(file).unwrap_or(&file));
                            }
                        }
                    }
                    for v in o.values_mut() {
                        normalize(v, aliases);
                    }
                }
                Value::Array(a) => {
                    for v in a {
                        normalize(v, aliases);
                    }
                }
                _ => {}
            }
        }
        Value::Object(
            files
                .into_iter()
                .map(|(file, raw, _)| {
                    let mut schema: Value =
                        serde_json::from_str(raw).expect("frozen Source7 schema");
                    normalize(&mut schema, &aliases);
                    (format!("/source7/{file}"), schema)
                })
                .collect(),
        )
    })
}
