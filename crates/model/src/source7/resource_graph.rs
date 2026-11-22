//! Closed wire records for native file captures under FC Graph2.
use serde::{Deserialize, Serialize};

pub mod orffe;

pub const CAPTURE_MEDIA: &str = "application/vnd.vestacraft.native-graph2+json";
pub const GRAPH_GROUP: &str = "source-graph";
pub const AVAILABLE_RULES: &str = "native-vesta-354-structure/1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapturedGraph {
    pub root_key: String,
    pub files: Vec<GraphFile>,
    pub captures: Vec<NodeCapture>,
    pub edges: Vec<GraphEdge>,
    pub optional_probes: Vec<OptionalProbe>,
    pub policy_digest: String,
    pub host_snapshot_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphFile {
    pub key: String,
    pub sha256: String,
    pub byte_length: String,
    pub original_name_hex: String,
    pub reference_rule_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeCapture {
    pub primary_key: String,
    pub captured: CapturedInput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapturedInput {
    pub primary_sha256: String,
    pub primary_name_hex: String,
    pub references: Vec<SourceReference>,
    pub files: Vec<FileBinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FileBinding {
    pub key: String,
    pub sha256: String,
    pub byte_length: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExplicitReference {
    pub reference_id: String,
    pub source_primary_sha256: String,
    pub path_span: Span,
    pub literal_path_hex: String,
    pub resource_key: String,
    pub resource_sha256: String,
    pub resource_byte_length: String,
    pub owner_locator: String,
    pub rule_id: String,
    pub rule_digest: String,
    pub edge_kind: Explicit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceReference {
    Explicit(ExplicitReference),
    Implicit(ImplicitReference),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Explicit {
    Explicit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImplicitReference {
    pub edge_kind: Implicit,
    pub reference_id: String,
    pub source_primary_sha256: String,
    pub seed_reference_id: String,
    pub derived_name_hex: String,
    pub resource_key: String,
    pub resource_sha256: String,
    pub resource_byte_length: String,
    pub owner_locator: String,
    pub rule_id: String,
    pub rule_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Implicit {
    Implicit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Span {
    pub byte_offset: String,
    pub byte_length: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphEdge {
    pub from_key: String,
    pub to_key: String,
    pub reference_id: String,
    pub kind: EdgeKind,
    pub recursive_decode: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EdgeKind {
    StructureImport,
    DensityImport,
    TextureImport,
    GeometryImport,
    ProducerPair,
    ProducerInclude,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalProbe {
    pub id: String,
    pub from_key: String,
    pub derived_name_hex: String,
    pub rule_id: String,
    pub rule_digest: String,
    pub result: ProbeResult,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum ProbeResult {
    Absent,
    Present {
        to_key: String,
        reference_id: String,
    },
}
