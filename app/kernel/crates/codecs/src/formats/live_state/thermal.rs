use super::{entity, invalid, payload, tagged, words};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use vesta_model::Result;

const TAG: &str = "NativeThermalState1";
const DIALECT: &str = "native-current-thermal/1";
const RULE: &str = "draft.native-current-thermal-slots";

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum Owner {
    Phase { index: usize },
    Frame { molecule: usize, index: usize },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Slot {
    isotropic_bits: String,
    anisotropic_bits: [String; 6],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ThermalInput {
    owner: Owner,
    native_iso_flag_bits: String,
    native_aniso_selector_bits: String,
    native_aniso_table_state_bits: String,
    ordered_slots: Vec<Slot>,
}

fn population<'a>(source: &'a Value, kind: &str, id: &str) -> Result<(&'static str, &'a [Value])> {
    let (target, objects) = match kind {
        "phase" => {
            let owner = source["phaseGraph"]["phases"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|row| row["id"] == id);
            (
                "site",
                owner.and_then(|row| row["structure"]["sites"].as_array()),
            )
        }
        "frame" => {
            let owner = source["molecules"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|molecule| molecule["frames"].as_array().into_iter().flatten())
                .find(|row| row["id"] == id);
            ("atom", owner.and_then(|row| row["atoms"].as_array()))
        }
        _ => return Err(invalid("native thermal owner namespace")),
    };
    Ok((
        target,
        objects.ok_or_else(|| invalid("native thermal owner has no current population"))?,
    ))
}

pub(super) fn install(input: ThermalInput, source: &mut Value) -> Result<()> {
    let (kind, owner) = match input.owner {
        Owner::Phase { index } => ("phase", &source["phaseGraph"]["phases"][index]),
        Owner::Frame { molecule, index } => {
            ("frame", &source["molecules"][molecule]["frames"][index])
        }
    };
    let id = owner["id"]
        .as_str()
        .ok_or_else(|| invalid("native thermal owner missing"))?;
    let (target, objects) = population(source, kind, id)?;
    if input.ordered_slots.len() != objects.len() {
        return Err(invalid(
            "native thermal slots do not cover the installed population",
        ));
    }
    let slots = input
        .ordered_slots
        .into_iter()
        .zip(objects)
        .map(|(slot, row)| {
            let id = row["id"]
                .as_str()
                .ok_or_else(|| invalid("native thermal slot target"))?;
            let [iso] = words([slot.isotropic_bits], "F64Bits");
            Ok(json!({"target":entity(target,id),"isotropicSlot":iso,
            "anisotropicSlots":words(slot.anisotropic_bits,"F64Bits")}))
        })
        .collect::<Result<Vec<_>>>()?;
    let body = json!({
        "schemaVersion":"FC-native-thermal-state-1","captureRole":"actual-current-native-compatibility-state",
        "owner":entity(kind,id),"ruleId":RULE,"ruleVersion":"1","orderedSlots":slots,
        "nativeIsoFlagBits":input.native_iso_flag_bits,"nativeAnisoSelectorBits":input.native_aniso_selector_bits,
        "nativeAnisoTableStateBits":input.native_aniso_table_state_bits
    });
    let row = json!({"owner":{"kind":kind,"id":id},"producerId":RULE,"producerVersion":"1",
        "formatRef":"VM18-I01","dialect":DIALECT,"tree":tagged(TAG,&body)?,"spans":[]});
    source["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("native thermal metadata collection"))?
        .push(row);
    Ok(())
}

pub(super) fn inspect(source: &Value) -> Result<Vec<Value>> {
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    for row in source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| row["tree"]["tag"] == TAG || row["dialect"] == DIALECT)
    {
        let kind = row["owner"]["kind"]
            .as_str()
            .ok_or_else(|| invalid("native thermal owner kind"))?;
        let id = row["owner"]["id"]
            .as_str()
            .ok_or_else(|| invalid("native thermal owner id"))?;
        let body = payload(&row["tree"])?;
        if row["tree"]["tag"] != TAG
            || row["dialect"] != DIALECT
            || row["producerId"] != RULE
            || row["producerVersion"] != "1"
            || row["formatRef"] != "VM18-I01"
            || row["spans"] != json!([])
            || body["owner"] != entity(kind, id)
            || !seen.insert((kind, id))
        {
            return Err(invalid(
                "native thermal owner and current record differ or repeat",
            ));
        }
        let (target, objects) = population(source, kind, id)?;
        let slots = body["orderedSlots"]
            .as_array()
            .ok_or_else(|| invalid("native thermal slot collection"))?;
        if slots.len() != objects.len()
            || slots.iter().zip(objects).any(|(slot, row)| {
                row["id"]
                    .as_str()
                    .is_none_or(|id| slot["target"] != entity(target, id))
            })
        {
            return Err(invalid(
                "native thermal slots lost their actual ordered targets",
            ));
        }
        result.push(body);
    }
    Ok(result)
}
