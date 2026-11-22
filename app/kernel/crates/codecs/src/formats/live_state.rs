//! Native working-state construction from explicit local values. This adapter
//! installs stored numerical values and evaluated placement without legacy
//! VESTA geometry inference or any physical unit authorization.
use super::{current_source7, full_source7, native_vesta_v2::source_only_capture};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use vesta_model::source_audit::Node;
use vesta_model::source7::SourceDocument;
use vesta_model::{Capture, Error, Ids, RestoredProject, Result};

mod grid;
mod layer;
mod legacy_display;
pub(super) mod native_flags;
mod nvis_molecule;
mod nvis_objects;
mod thermal;
mod vics_history;
mod view;
pub use native_flags::{is_historical_anatase, open_historical_anatase};
pub use vics_history::open as open_legacy_vics;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeStateInput {
    schema_version: String,
    base_input: Option<String>,
    grids: Vec<grid::GridInput>,
    layers: Option<Vec<layer::LayerInput>>,
    legacy_view: Option<view::ViewInput>,
    legacy_display: Option<legacy_display::DisplayInput>,
    native_flags: Option<native_flags::FlagsInput>,
    #[serde(default)]
    molecule_flags: Vec<nvis_molecule::Input>,
    #[serde(default)]
    thermal: Vec<thermal::ThermalInput>,
}

impl NativeStateInput {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let input: Self = serde_json::from_value(vesta_model::json::parse(bytes, 1024 * 1024)?)
            .map_err(|e| invalid(e.to_string()))?;
        if input.schema_version != "VestaDraft-native-current-state-1"
            || input.grids.len() > 32
            || input.thermal.len() > 128
            || input.molecule_flags.len() > 128
            || input.layers.as_ref().is_some_and(|rows| rows.len() > 1024)
            || (input.grids.is_empty()
                && input.layers.is_none()
                && input.legacy_view.is_none()
                && input.legacy_display.is_none()
                && input.native_flags.is_none()
                && input.molecule_flags.is_empty()
                && input.thermal.is_empty())
        {
            return Err(invalid(
                "unsupported native current-state manifest or population",
            ));
        }
        Ok(input)
    }

    pub fn base_input(&self) -> Option<&str> {
        self.base_input.as_deref()
    }
}

pub(super) fn rule_digest() -> String {
    vesta_model::digest(&vesta_model::source_audit::canonical(&json!({
        "construction":"VestaDraft-native-current-state-1",
        "wire":include_str!("../../contracts/native-current-state-input.contract.json"),
        "grid":grid::rule_digest(),"layer":layer::rule_digest(),"legacyDisplay":legacy_display::rule_digest(),"nativeFlags":native_flags::rule_digest(),"vicsHistoricalSource":vics_history::rule_digest(),"anataseHistoricalSource":native_flags::historical_rule_digest()
    })))
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new("E_REOPEN_COVERAGE", message)
}

fn entity(kind: &str, id: &str) -> Value {
    json!({"kind":"EntityRef","objectKind":kind,"objectId":id})
}

fn words<const N: usize>(values: [String; N], kind: &str) -> [Value; N] {
    values.map(|value| json!({"kind":kind,"ieeeHexBE":value}))
}

pub(super) fn tagged(tag: &str, payload: &Value) -> Result<Value> {
    fn lower(value: &Value) -> Result<Node> {
        Ok(match value {
            Value::Null => Node::Null,
            Value::Bool(value) => Node::Bool(*value),
            Value::String(text) => Node::Text(text.clone()),
            Value::Number(number) if number.is_u64() => Node::U64(number.as_u64().unwrap()),
            Value::Number(number) if number.is_i64() => Node::I64(number.as_i64().unwrap()),
            Value::Array(items) => Node::Array(items.iter().map(lower).collect::<Result<_>>()?),
            Value::Object(fields) => match fields.get("kind").and_then(Value::as_str) {
                Some("EntityRef" | "BytesRef" | "F32Bits" | "F64Bits") => {
                    let node = Node::from_wire(value)?;
                    if node.wire() != *value {
                        return Err(invalid(
                            "native current state contains a noncanonical exact word or reference",
                        ));
                    }
                    node
                }
                _ => Node::Map(
                    fields
                        .iter()
                        .map(|(key, value)| Ok((key.clone(), lower(value)?)))
                        .collect::<Result<_>>()?,
                ),
            },
            _ => {
                return Err(invalid(
                    "native payload requires explicit numerical word or integer",
                ));
            }
        })
    }
    Ok(Node::TaggedVariant {
        tag: tag.into(),
        payload: Box::new(lower(payload)?),
    }
    .wire())
}

fn payload(wire: &Value) -> Result<Value> {
    fn restore(node: &Node) -> Result<Value> {
        Ok(match node {
            Node::Array(items) => Value::Array(items.iter().map(restore).collect::<Result<_>>()?),
            Node::Map(fields) => Value::Object(
                fields
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), restore(v)?)))
                    .collect::<Result<_>>()?,
            ),
            Node::F32(_) | Node::F64(_) | Node::Identity { .. } | Node::Resource { .. } => {
                node.wire()
            }
            _ => node.data()?,
        })
    }
    if let Node::TaggedVariant { payload, .. } = Node::from_wire(wire)? {
        restore(&payload)
    } else {
        Err(invalid("missing registered current-state payload"))
    }
}

/// Install all resolved state in a private candidate before changing the engine.
pub fn open_native_state(
    mut input: NativeStateInput,
    base: Option<Capture>,
    mut read: impl FnMut(&str) -> Result<Vec<u8>>,
    mut ids: Ids,
) -> Result<RestoredProject> {
    let mut capture = base.unwrap_or_else(|| source_only_capture(&mut ids));
    let mut value = if let Some(source) = &capture.source {
        source.to_value()?
    } else if capture.content.phases.is_empty() {
        full_source7::source_only_base(&capture)
    } else {
        current_source7::capture(&capture)?.0
    };
    if value["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|row| row["tree"]["tag"] == layer::TAG)
    {
        return Err(invalid(
            "native layer setup requires a project without a prior layer population",
        ));
    }
    for grid in input.grids {
        grid::install(grid, &mut capture, &mut value, &mut read, &mut ids)?;
    }
    if let Some(layers) = input.layers {
        layer::install(layers, &mut value)?;
    }
    nvis_molecule::install(input.molecule_flags, &mut value)?;
    if let Some(flags) = input.native_flags {
        native_flags::install(flags, &mut capture, &mut value, &mut ids)?;
    }
    if let Some(display) = input.legacy_display {
        legacy_display::install(
            display, input.legacy_view.take(), &mut value, &mut capture, &mut read, &mut ids,
        )?;
    }
    if let Some(view) = input.legacy_view {
        view::install(view, capture.content.scene_id.as_str(), &mut value)?;
    }
    for slots in input.thermal {
        thermal::install(slots, &mut value)?;
    }
    // Authoritative document and scene data come from this same capture; no
    // second open of the base file is needed to acquire a target.
    capture.source = Some(SourceDocument::decode(value)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

pub(super) fn verify(capture: &Capture, value: &Value) -> Result<BTreeSet<String>> {
    layer::verify(value)?;
    view::inspect(value)?;
    legacy_display::verify(capture, value)?;
    vics_history::verify(capture, value)?;
    native_flags::verify_historical_anatase(capture, value)?;
    nvis_molecule::inspect(value)?;
    thermal::inspect(value)?;
    grid::verify(capture, value)
}

pub fn inspect_native_state(capture: &Capture) -> Result<Option<Value>> {
    let Some(source) = &capture.source else {
        return Ok(None);
    };
    let value = source.to_value()?;
    let grids = grid::inspect(capture, &value)?;
    let layers = layer::inspect(&value)?;
    let legacy_views = view::inspect(&value)?;
    let legacy_displays = legacy_display::inspect(&value)?;
    let legacy_source_views = legacy_display::inspect_source_views(capture)?;
    let thermal = thermal::inspect(&value)?;
    let mut flags = native_flags::inspect(capture, &value)?;
    flags.extend(nvis_molecule::inspect(&value)?);
    Ok((!grids.is_empty() || layers.is_some() || !legacy_views.is_empty() || !legacy_displays.is_empty()
        || !legacy_source_views.is_empty() || !thermal.is_empty() || !flags.is_empty()).then(|| json!({
        "document":capture.document,"grids":grids,"layers":layers,"legacyViews":legacy_views,
        "legacyDisplays":legacy_displays,"legacySourceViews":legacy_source_views,"thermal":thermal,"nativeFlags":flags
    })))
}
