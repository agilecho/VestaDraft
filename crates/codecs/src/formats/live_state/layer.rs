use super::{entity, invalid, payload, tagged, words};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use vesta_model::{Result, digest};

pub(super) const TAG: &str = "NativeLayerCurrent1";
const DIALECT: &str = "native-current-layer/1";
const PRODUCER: &str = "draft.native-current-layer";

pub(super) fn rule_digest() -> String {
    digest(include_bytes!(
        "../../../../../contracts/source/31e8aa75b924339c-native-layer-current.schema.json"
    ))
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum Locator {
    Phase { index: usize },
    Frame { molecule: usize, index: usize },
    SourceEntity { index: usize },
}

impl Locator {
    fn resolve(&self, source: &Value) -> Result<Value> {
        let (kind, row) = match *self {
            Self::Phase { index } => ("phase", &source["phaseGraph"]["phases"][index]),
            Self::Frame { molecule, index } => {
                ("frame", &source["molecules"][molecule]["frames"][index])
            }
            Self::SourceEntity { index } => ("source-entity", &source["sourceEntities"][index]),
        };
        let id = row["id"]
            .as_str()
            .ok_or_else(|| invalid("layer locator has no installed object"))?;
        Ok(entity(kind, id))
    }
}

#[derive(Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum Selector {
    InternalCartesian {
        captured_selector: i32,
    },
    Entry {
        captured_selector: i32,
        target: Locator,
    },
    Unresolved {
        captured_selector: i32,
        reason: UnresolvedReason,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
enum UnresolvedReason {
    OutOfPopulation,
    AmbiguousNativeInstance,
    UnqualifiedSourceIndex,
}

impl Selector {
    fn bind(self, source: &Value) -> Result<Value> {
        let (selector, reference) = match self {
            Self::InternalCartesian { captured_selector } => {
                (captured_selector, json!({"kind":"internal-cartesian"}))
            }
            Self::Entry {
                captured_selector,
                target,
            } => (
                captured_selector,
                json!({"kind":"entry","target":target.resolve(source)?}),
            ),
            Self::Unresolved {
                captured_selector,
                reason,
            } => (
                captured_selector,
                json!({"kind":"unresolved","reason":reason}),
            ),
        };
        Ok(json!({"capturedSelector":selector,"reference":reference}))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Position {
    selector: Selector,
    current_anchor: [String; 3],
    reference_anchor: [String; 3],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Orientation {
    selector: Selector,
    stored_flag_bits: String,
    current_first: [String; 3],
    reference_first: [String; 3],
    current_second: [String; 3],
    reference_second: [String; 3],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct LayerInput {
    owner: Locator,
    position: Position,
    orientation: Orientation,
    effective_matrix_bits: [String; 16],
    effective_translation_bits: [String; 3],
}

impl LayerInput {
    fn bind(self, source: &Value) -> Result<Value> {
        let p = self.position;
        let o = self.orientation;
        Ok(json!({
            "owner":self.owner.resolve(source)?,
            "position":{"selector":p.selector.bind(source)?,
                "currentAnchor":words(p.current_anchor,"F64Bits"),
                "referenceAnchor":words(p.reference_anchor,"F64Bits")},
            "orientation":{"selector":o.selector.bind(source)?,"storedFlagBits":o.stored_flag_bits,
                "currentFirst":words(o.current_first,"F64Bits"),
                "referenceFirst":words(o.reference_first,"F64Bits"),
                "currentSecond":words(o.current_second,"F64Bits"),
                "referenceSecond":words(o.reference_second,"F64Bits")},
            "effectiveNativeMatrix4":words(self.effective_matrix_bits,"F64Bits"),
            "effectiveNativeTranslation":words(self.effective_translation_bits,"F32Bits")
        }))
    }
}

pub(super) fn install(input: Vec<LayerInput>, source: &mut Value) -> Result<()> {
    let entries = input
        .into_iter()
        .map(|row| row.bind(source))
        .collect::<Result<Vec<_>>>()?;
    let document = source["identity"]["documentId"]
        .as_str()
        .ok_or_else(|| invalid("layer document absent"))?;
    let value = json!({
        "schemaVersion":"FC-native-layer-current-1","captureRole":"actual-current-native-3D-layer-population",
        "document":entity("document",document),"orderedEntries":entries
    });
    verify_population(source, &value)?;
    let record = json!({"owner":{"kind":"document","id":document},"producerId":PRODUCER,
        "producerVersion":"1","formatRef":"VM18-I01","dialect":DIALECT,"tree":tagged(TAG,&value)?,"spans":[]});
    source["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("document metadata collection"))?
        .push(record);
    Ok(())
}

fn rows(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

fn reference(value: &Value) -> Result<(&str, &str)> {
    let kind = value["objectKind"]
        .as_str()
        .ok_or_else(|| invalid("layer target kind"))?;
    let id = value["objectId"]
        .as_str()
        .ok_or_else(|| invalid("layer target id"))?;
    if value["kind"] != "EntityRef" {
        return Err(invalid("layer target typed reference"));
    }
    Ok((kind, id))
}

fn verify_population(source: &Value, body: &Value) -> Result<()> {
    let document = source["identity"]["documentId"]
        .as_str()
        .ok_or_else(|| invalid("layer document absent"))?;
    if body["document"] != entity("document", document) {
        return Err(invalid("layer population belongs to another document"));
    }
    let population = rows(&source["phaseGraph"]["phases"])
        .map(|row| ("phase", row))
        .chain(
            rows(&source["molecules"])
                .flat_map(|mol| rows(&mol["frames"]))
                .map(|row| ("frame", row)),
        )
        .chain(
            rows(&source["sourceEntities"])
                .filter(|row| row["nativeKind"] != "vend-native-stored-grid")
                .map(|row| ("source-entity", row)),
        )
        .map(|(kind, row)| {
            row["id"]
                .as_str()
                .map(|id| (kind, id))
                .ok_or_else(|| invalid("layer population object id"))
        })
        .collect::<Result<BTreeSet<_>>>()?;
    let mut observed = BTreeSet::new();
    for entry in rows(&body["orderedEntries"]) {
        if !observed.insert(reference(&entry["owner"])?) {
            return Err(invalid("duplicate layer owner"));
        }
        for role in ["position", "orientation"] {
            let binding = &entry[role]["selector"]["reference"];
            if binding["kind"] == "entry" && !population.contains(&reference(&binding["target"])?) {
                return Err(invalid("layer target outside installed current population"));
            }
        }
    }
    if !observed.iter().eq(population.iter()) {
        return Err(invalid(
            "layer population differs from installed current objects",
        ));
    }
    Ok(())
}

fn verified_payload(source: &Value) -> Result<Option<Value>> {
    let mut records = rows(&source["metadata"])
        .filter(|row| row["tree"]["tag"] == TAG || row["dialect"] == DIALECT);
    let Some(row) = records.next() else {
        return Ok(None);
    };
    if records.next().is_some()
        || row["tree"]["tag"] != TAG
        || row["dialect"] != DIALECT
        || row["producerId"] != PRODUCER
        || row["producerVersion"] != "1"
        || row["formatRef"] != "VM18-I01"
        || row["spans"] != json!([])
        || row["owner"] != json!({"kind":"document","id":source["identity"]["documentId"]})
    {
        return Err(invalid(
            "current layer must have one document-owned capture",
        ));
    }
    let body = payload(&row["tree"])?;
    verify_population(source, &body)?;
    Ok(Some(body))
}

pub(super) fn inspect(source: &Value) -> Result<Option<Value>> {
    Ok(verified_payload(source)?.map(|mut body| body["orderedEntries"].take()))
}

pub(super) fn verify(source: &Value) -> Result<()> {
    verified_payload(source).map(|_| ())
}
