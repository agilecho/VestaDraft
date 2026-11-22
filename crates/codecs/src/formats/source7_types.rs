//! The current implemented Source7 population. All persistent members and roles are
//! generated from these declarations. Unsupported populations have a closed empty type.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use vesta_model::{
    source_audit::{self as sa, AuditSource, Node, Remap, ValueSource},
    *,
};
use vesta_source_derive::source_model;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Vec<bool>", into = "Vec<bool>")]
pub struct Empty;
impl TryFrom<Vec<bool>> for Empty {
    type Error = String;
    fn try_from(v: Vec<bool>) -> std::result::Result<Self, String> {
        if v.is_empty() {
            Ok(Self)
        } else {
            Err("unimplemented Source7 population".into())
        }
    }
}
impl From<Empty> for Vec<bool> {
    fn from(_: Empty) -> Self {
        vec![]
    }
}
impl AuditSource for Empty {
    fn audit(&self) -> Result<Node> {
        Ok(Node::Array(vec![]))
    }
    fn rebind(&mut self, _: &Remap) -> Result<()> {
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"closed-empty-population"})
    }
}
impl ValueSource for Empty {}
source_model! {pub struct Identity {pub document_id:String,pub epoch:String,pub revision:String,pub source_type_digest:String,pub source_root_digest:String,pub resource_graph_digest:String,pub policy_digest:String}}
source_model! {pub struct Rational {pub numerator:i32,pub denominator:u32}}
source_model! {pub struct Operator {#[source(definition(SymmetryOp))] pub id:sa::Id<sa::SymmetryOp>,pub fractional_matrix:[[Rational;3];3],pub fractional_translation:[Rational;3],pub time_sign:i32}}
source_model! {pub struct Group {pub kind:String,pub symbol:String,pub hall_symbol:String,pub bns_number:Option<String>,pub og_number:Option<String>,pub setting_name:String,pub origin_choice:String,pub change_of_basis:[[f64;3];3],pub origin_fractional:[f64;3],pub table:Option<Empty>,pub operators:Vec<Operator>,pub source_spans:Empty}}
source_model! {pub struct Species {pub element:Option<String>,pub charge:Option<i32>,pub mass_number:Option<u32>,pub raw_token:String}}
source_model! {pub struct Adp {pub kind:String,pub u_a2:f64}}
source_model! {#[source(owner(Site,id))] pub struct SourceSite {#[source(definition(Site))] pub id:sa::Id<sa::Site>,pub label:String,pub species:Species,pub fractional:[f64;3],pub occupancy:f64,pub adp:Adp,pub source_spans:Empty}}
source_model! {pub struct Structure {pub cell:Cell3,pub group:Group,pub sites:Vec<SourceSite>,pub covariance:Option<Empty>}}
source_model! {#[source(owner(Phase,id))] pub struct SourcePhase {#[source(definition(Phase))] pub id:sa::Id<sa::Phase>,pub label:String,#[source(reference(Phase))] pub parent:Option<sa::Id<sa::Phase>>,pub rotation:[[f64;3];3],pub translation_a:[f64;3],pub structure:Structure,pub field_ids:Empty,pub shape_id:Option<Empty>}}
source_model! {pub struct PhaseGraph {#[source(reference(Phase))] pub root:sa::Id<sa::Phase>,pub phases:Vec<SourcePhase>}}
source_model! {#[source(owner(Scene,id))] pub struct SourceScene {#[source(definition(Scene))] pub id:sa::Id<sa::Scene>,pub roots:Empty,pub nodes:Empty,pub render:Node,pub metadata:Empty,pub camera_id:Option<Empty>,pub render_context:Option<Empty>,pub camera_raw:Node}}
source_model! {#[source(owner(Resource,id))] pub struct SourceResource {#[source(definition(Resource))] pub id:sa::Id<sa::Resource>,pub sha256:String,pub byte_length:String,pub storage_kind:String,pub source_descriptor_digest:String}}
source_model! {pub struct Owner {pub kind:String,#[source(reference(Scene))] pub id:sa::Id<sa::Scene>}}
source_model! {pub struct SourceMetadata {pub owner:Owner,pub producer_id:String,pub producer_version:String,pub format_ref:String,pub dialect:String,pub tree:Node,pub spans:Empty}}
source_model! {pub struct DocumentOwner {pub kind:String,#[source(reference(Document))] pub id:sa::Id<sa::Document>}}
source_model! {pub struct GraphMetadata {pub owner:DocumentOwner,pub producer_id:String,pub producer_version:String,pub format_ref:String,pub dialect:String,pub tree:Node,pub spans:Empty}}
source_model! {pub struct Entry {pub kind:String,#[source(reference(Phase))] pub id:sa::Id<sa::Phase>}}
source_model! {pub struct Source7 {pub schema_version:String,pub identity:Identity,pub phase_graph:PhaseGraph,pub molecules:Empty,pub fields:Empty,pub scenes:Vec<SourceScene>,pub dependencies:Empty,pub opaque_blocks:Empty,pub resources:Vec<SourceResource>,pub metadata:Vec<SourceMetadata>,pub shapes:Empty,pub vectors:Empty,pub materials:Empty,pub displays:Empty,pub cameras:Empty,pub meshes:Empty,pub source_entities:Empty,pub source_entry_order:Vec<Entry>,pub sections:Empty,pub bond_rules:Empty,pub boundaries:Empty,pub planes:Empty}}
