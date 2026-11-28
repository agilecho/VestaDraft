//! Selected GSAS magnetic state. Qualification belongs to the installed
//! provider, and is resolved again from the captured EXP on artifact reopen.
use super::{decimal, invalid, retained};
use crate::formats::{current_source7, live_state::tagged};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use vesta_model::source_audit::canonical;
use vesta_model::source7::{GSAS_MOMENT_QUANTITY_DIALECT, gsas_moment_use};
use vesta_model::{Capture, Cell3, Ids, PersistentResource, Result, digest};

use super::chart::{
    Chart, Frame, IDENTITY, MOMENT_RULE, NumericalPolicy, Operation, SPIN_RULE, dot,
};

pub(super) const TAG: &str = "GSASSpinProjection1";
const DIALECT: &str = "gsas-active-magnetic-1";
pub(super) const CHART_KEY: &str = "gsas-exp/active/chart";
const FRAME_PATH: &str = "/fieldBindings/0/frameResource";
const SOURCE_PATH: &str = "/fieldBindings/0/evidence/0/evidenceResource";
const PRODUCER_PATH: &str = "/evidence/0/evidenceResource";
const MOMENT_CONTEXT: &str = "gsas-moment-descriptor-1";
const DESCRIPTOR_SCHEMA: &[u8] =
    include_bytes!("../../../../model/src/source7/frozen/source-descriptor.schema.json");
const DESCRIPTOR_RULE: &[u8] =
    include_bytes!("../../../../model/src/source7/frozen/bound-descriptor-context.contract.json");

struct SiteRule {
    serial: u16,
    type_token: &'static str,
    element: &'static str,
    operation: usize,
    shift: [i32; 3],
    image_millionths: [i64; 3],
    display_scale: f64,
}

pub(super) struct Qualification {
    exp_sha256: &'static str,
    phase: u8,
    setting: &'static str,
    origin: &'static str,
    hall_symbol: &'static str,
    chart: &'static [u8],
    frame: &'static [u8],
    producer_evidence: &'static [u8],
    producer_version: &'static str,
    numerics: NumericalPolicy,
    sites: &'static [SiteRule],
}

// Only independently qualified, source-bound producer observations belong in
// this table. A portable artifact or client cannot add a qualification.
const QUALIFIED: &[Qualification] = &[Qualification {
    exp_sha256: "a16f9edf3aec9f3eff4623cbd58afd9bff767d4bb856c69f56ed1f7ba3a933c3",
    phase: 2,
    setting: "Fmmm in the stored EXP axes",
    origin: "stored EXP origin",
    hall_symbol: "-F 2 2",
    chart: include_bytes!("../../../contracts/gsas-qualifications/aps-ybafeo-chart.json"),
    frame: include_bytes!("../../../contracts/gsas-qualifications/aps-ybafeo-frame.json"),
    producer_evidence: include_bytes!("../../../contracts/gsas-qualifications/aps-ybafeo-evidence.json"),
    producer_version: "GSAS-r1253",
    numerics: NumericalPolicy {
        max_condition: 6.,
        length_a: 5.1e-7,
        angle_deg: 5.1e-5,
        orthonormal: 1e-12,
    },
    sites: &[
        SiteRule {
            serial: 1,
            type_token: "FE+3",
            element: "Fe",
            operation: 0,
            shift: [0, 0, 0],
            image_millionths: [0, 0, 0],
            display_scale: 1.,
        },
        SiteRule {
            serial: 2,
            type_token: "FE+3",
            element: "Fe",
            operation: 0,
            shift: [0, 0, 0],
            image_millionths: [0, 0, 500_000],
            display_scale: 1.,
        },
        SiteRule {
            serial: 3,
            type_token: "FE+3",
            element: "Fe",
            operation: 0,
            shift: [0, 0, 0],
            image_millionths: [0, 0, 170_000],
            display_scale: 1.,
        },
        SiteRule {
            serial: 4,
            type_token: "FE+3",
            element: "Fe",
            operation: 0,
            shift: [0, 0, 0],
            image_millionths: [0, 0, 330_000],
            display_scale: 1.,
        },
    ],
}];

pub(super) fn qualified(sha: &str) -> Option<&'static Qualification> {
    #[cfg(test)]
    {
        if sha == AUTHORED.exp_sha256 {
            return Some(&AUTHORED);
        }
        if sha == MULTIPHASE_CONTROL.exp_sha256 {
            return Some(&MULTIPHASE_CONTROL);
        }
    }
    QUALIFIED.iter().find(|q| q.exp_sha256 == sha)
}

fn text<'a>(node: &'a Value, field: &str) -> Result<&'a str> {
    node[field]["value"]
        .as_str()
        .ok_or_else(|| invalid("GSAS source word is missing"))
}

fn words<const N: usize>(node: &Value, field: &str, precision: usize) -> Result<[f64; N]> {
    let list = node[field]["items"]
        .as_array()
        .filter(|list| list.len() == N)
        .ok_or_else(|| invalid("GSAS source word population differs"))?;
    let mut output = [0.; N];
    for (out, word) in output.iter_mut().zip(list) {
        *out = decimal(
            word["value"]
                .as_str()
                .ok_or_else(|| invalid("GSAS decimal word is missing"))?,
            precision,
        )?;
    }
    Ok(output)
}

fn reference(kind: &str, id: &str) -> Value {
    json!({"kind":"EntityRef","objectKind":kind,"objectId":id})
}

fn bytes_ref(row: &PersistentResource) -> Value {
    json!({"kind":"BytesRef","resourceId":row.id,"sha256":row.sha256,"byteLength":row.byte_length})
}

fn metadata(kind: &str, owner: &str, dialect: &str, producer: &str, tree: Value) -> Value {
    json!({"owner":{"kind":kind,"id":owner},"producerId":producer,"producerVersion":"1",
        "formatRef":"VM18-I14","dialect":dialect,"tree":tree,"spans":[]})
}

struct Moment<'a> {
    rule: &'a SiteRule,
    site: Value,
    source: [f64; 3],
    cartesian: [f64; 3],
    span: Range<usize>,
}

pub(super) struct Prepared<'a> {
    approval: &'a Qualification,
    raw: &'a [u8],
    chart: Chart,
    cell: Cell3,
    operations: Vec<(Operation, i8)>,
    flags: [f32; 12],
    moments: Vec<Moment<'a>>,
}

impl<'a> Prepared<'a> {
    pub(super) fn new(approval: &'a Qualification, raw: &'a [u8], history: &Value) -> Result<Self> {
        // The qualification binds an original phase number, not its position
        // among declared phases. Other original phases remain archival facts.
        let mut selected = history["tree"]["members"]["phases"]["items"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|phase| {
                text(&phase["members"], "phaseNumber")
                    .ok()
                    .and_then(|word| word.parse::<u8>().ok()) == Some(approval.phase)
            });
        let phase = selected.next().ok_or_else(|| {
            invalid("GSAS approved phase is missing or repeated")
        })?;
        if selected.next().is_some() {
            return Err(invalid("GSAS approved phase is missing or repeated"));
        }
        let phase = &phase["members"];
        let chart = Chart::parse(approval.chart)?;
        let frame = Frame::parse(approval.frame)?;
        let source_digest = digest(raw);
        if source_digest != approval.exp_sha256
            || chart.source_input_sha256 != source_digest
            || frame.scope_input_digest != source_digest
            || chart.source_phase_number != approval.phase
            || text(phase, "phaseNumber")?.parse::<u8>() != Ok(approval.phase)
            || chart.source_group_symbol != text(phase, "groupWord")?.trim()
            || chart.setting_name != approval.setting
            || chart.origin_choice != approval.origin
            || !matches!(text(phase, "phaseKind")?, "2" | "3")
            || approval.producer_evidence.is_empty()
        {
            return Err(invalid(
                "GSAS chart and frame do not bind the approved original phase",
            ));
        }
        let cell = Cell3 {
            length_a: words(phase, "cellLengthWords", 6)?,
            angles_deg: words(phase, "cellAngleWords", 4)?,
        };
        cell.validate()?;
        let mut flags = [0f32; 12];
        let slots = phase["spinFlipWords"]["items"]
            .as_array()
            .filter(|rows| rows.len() == flags.len())
            .ok_or_else(|| invalid("GSAS spin slots"))?;
        for (index, (out, word)) in flags.iter_mut().zip(slots).enumerate() {
            if word["value"].as_str() == Some("    ") && index >= chart.used_spin_slots() {
                // The original reads blank fields as zero. Only the unused
                // suffix may omit values when activating an approved chart.
                continue;
            }
            *out = word["value"]
                .as_str()
                .and_then(|s| s.trim_start().parse::<f32>().ok())
                .filter(|n| *n == -1. || *n == 0. || *n == 1.)
                .ok_or_else(|| invalid("GSAS spin slot domain"))?;
        }
        let operations = chart.expand(flags.map(|word| if word == -1. { -1 } else { 1 }))?;
        let axes = frame.axes(&cell, approval.numerics, &operations)?;
        let atoms = phase["atoms"]["items"]
            .as_array()
            .filter(|rows| rows.len() == approval.sites.len())
            .ok_or_else(|| invalid("GSAS qualified site population differs"))?;
        let records = retained::lines(raw, super::MAX)?;
        let mut moments = Vec::with_capacity(atoms.len());
        for (row, rule) in atoms.iter().zip(approval.sites) {
            let atom = &row["members"];
            let token = text(atom, "atomTypeWord")?.trim();
            let label = text(atom, "labelWord")?.trim();
            if text(atom, "sourceAtomNumber")?.parse::<u16>() != Ok(rule.serial)
                || token != rule.type_token
                || !vesta_model::element(rule.element)
                || label.is_empty()
                || text(atom, "thermalCode")? != "I"
                || !rule.display_scale.is_finite()
                || rule.display_scale <= 0.
            {
                return Err(invalid(
                    "GSAS qualified site, species or isotropic displacement differs",
                ));
            }
            let [x, y, z, occupancy] = words(atom, "fractionalAndOccupancyWords", 6)?;
            let u = decimal(
                atom["thermalWords"]["items"][0]["value"]
                    .as_str()
                    .ok_or_else(|| invalid("GSAS isotropic displacement word is missing"))?,
                6,
            )?;
            // ATMON4–6 carry different lexical precision and remain archival.
            let rows = atom["momentTokens"]["items"]
                .as_array()
                .filter(|rows| rows.len() == 6)
                .ok_or_else(|| invalid("GSAS moment word population"))?;
            let component = |i: usize| {
                decimal(
                    rows[i]["value"]
                        .as_str()
                        .ok_or_else(|| invalid("GSAS moment component"))?,
                    5,
                )
            };
            let source = [component(0)?, component(1)?, component(2)?];
            let fractional = [x, y, z];
            let (operation, sign) = operations
                .get(rule.operation)
                .ok_or_else(|| invalid("GSAS anchor operation missing"))?;
            let image = std::array::from_fn(|i| {
                (fractional[i] * 1_000_000.).round() as i64 + i64::from(rule.shift[i]) * 1_000_000
            });
            if !operation.is_identity()
                || *sign != 1
                || image != rule.image_millionths
                || !(0. ..=1.).contains(&occupancy)
                || u < 0.
            {
                return Err(invalid(
                    "GSAS independent identity-image or site values differ",
                ));
            }
            let key = format!("CRS{}  AT{:>3}M", approval.phase, rule.serial);
            let row = records
                .iter()
                .find(|row| row.text.starts_with(&key))
                .ok_or_else(|| invalid("GSAS exact M evidence record missing"))?;
            let cartesian = std::array::from_fn(|i| dot(axes.map(|axis| axis[i]), source));
            moments.push(Moment { rule, site: json!({
                "label":label,"species":{"element":rule.element,"charge":null,"massNumber":null,"rawToken":token},
                "fractional":fractional,"occupancy":occupancy,"adp":{"kind":"iso-U","uA2":u},"sourceSpans":[]
            }), source, cartesian, span:row.start..row.end });
        }
        Ok(Self {
            approval,
            raw,
            chart,
            cell,
            operations,
            flags,
            moments,
        })
    }

    fn definition(&self, moment: &Moment<'_>) -> Vec<u8> {
        let a = self.approval;
        let reference = |alias: &str, b: &[u8]| json!({"resourceId":alias,"sha256":digest(b),"byteLength":b.len().to_string()});
        let evidence = |alias: &str, b: &[u8], span: Range<usize>| {
            json!({
                "evidenceResource":reference(alias,b),"byteOffset":span.start.to_string(),"byteLength":span.len().to_string(),
                "sourceSpanDigest":digest(&b[span]),"sourceVersion":a.producer_version
            })
        };
        canonical(&json!({
            "schemaVersion":"FC-source-descriptor-2","producerId":MOMENT_RULE,
            "producerVersion":"1","dialect":DIALECT,"scopeInputDigest":a.exp_sha256,"unknownSourceFields":[],
            "fieldBindings":[{"sourcePointer":format!("/phases/{}/atoms/{}/ATMON1-3",a.phase,moment.rule.serial),
                "physicalKind":"magnetic-moment","unit":"muB","basis":"producer-explicit",
                "frameResource":reference("frame",a.frame),"conversionRuleId":MOMENT_RULE,"conversionRuleVersion":"1",
                "evidence":[evidence("exp-m",self.raw,moment.span.clone())]}],
            "evidence":[evidence("producer-evidence",a.producer_evidence,0..a.producer_evidence.len())]
        }))
    }

    fn resources(
        &self,
        mut resolve: impl FnMut(&str, &[u8]) -> Result<PersistentResource>,
    ) -> Result<BTreeMap<String, PersistentResource>> {
        let mut rows = BTreeMap::new();
        let mut add = |key: String, raw: &[u8]| {
            let res = resolve(&key, raw)?;
            rows.insert(key, res);
            Ok::<_, vesta_model::Error>(())
        };
        add(CHART_KEY.into(), self.approval.chart)?;
        for (index, moment) in self.moments.iter().enumerate() {
            let prefix = descriptor_prefix(index);
            let definition = self.definition(moment);
            for (slot, data) in [
                ("definition".into(), definition.as_slice()),
                ("scope".into(), self.raw),
                (target(FRAME_PATH), self.approval.frame),
                (target(SOURCE_PATH), self.raw),
                (target(PRODUCER_PATH), self.approval.producer_evidence),
            ] {
                add(format!("{prefix}/{slot}"), data)?;
            }
        }
        Ok(rows)
    }

    fn render(
        &self,
        identities: &Identities,
        resources: &BTreeMap<String, PersistentResource>,
    ) -> Result<(Value, Value, Value)> {
        let id = identities.phase.as_str();
        let owner = json!({"kind":"phase","id":id});
        let operators: Vec<_> = self
            .operations
            .iter()
            .zip(&identities.ops)
            .map(|((op, sign), id)| op.wire(id, *sign))
            .collect();
        let mut sites = Vec::new();
        let mut vectors = Vec::new();
        for (i, moment) in self.moments.iter().enumerate() {
            let mut site = moment.site.clone();
            site["id"] = json!(identities.sites[i]);
            sites.push(site);
            let vector_id = &identities.vectors[i];
            let prefix = descriptor_prefix(i);
            let local = |slot: &str| &resources[&format!("{prefix}/{slot}")];
            let reference_row = |row: &PersistentResource| {
                json!({
                    "resourceId":reference("resource",row.id.as_str()),"sha256":row.sha256,"byteLength":row.byte_length,
                })
            };
            let binding = |pointer: &str, alias: &str| {
                json!({
                    "jsonPointer":pointer,"capturedAlias":alias,"target":reference_row(local(&target(pointer))),
                })
            };
            let def = local("definition");
            let context = tagged(
                "BoundDescriptorContext1",
                &json!({
                    "schemaVersion":"BoundDescriptorContext-proposal-1",
                    "owner":{"kind":"vector","id":reference("vector",vector_id)},
                    "sourceTypedPointer":gsas_moment_use(i),
                    "definition":reference_row(def),"scopeInput":reference_row(local("scope")),
                    "descriptorSchemaDigest":digest(DESCRIPTOR_SCHEMA),"referenceRuleDigest":digest(DESCRIPTOR_RULE),
                    "bindings":[binding(PRODUCER_PATH,"producer-evidence"),binding(SOURCE_PATH,"exp-m"),binding(FRAME_PATH,"frame")]
                }),
            )?;
            let word =
                |v: f64| json!({"kind":"F64Bits","ieeeHexBE":format!("{:016x}",v.to_bits())});
            let quantity = json!({"kind":"Map","members":{
                "sourceDescriptorDigest":{"kind":"Utf8","value":def.sha256},
                "value":{"kind":"Array","items":moment.source.map(word)},
                "unit":{"kind":"Utf8","value":"muB"},
                "frame":{"kind":"Utf8","value":"CMS2015-GSAS-system4"}
            }});
            vectors.push(json!({"id":vector_id,"owner":owner,"value":{
                "id":vector_id,"anchor":{"phase":id,"site":identities.sites[i],
                    "operation":identities.ops[moment.rule.operation],"shift":moment.rule.shift},
                "kind":"axial","basis":"cartesian","unit":"muB","components":moment.cartesian,
                "displayScale":moment.rule.display_scale,
            },"metadata":[metadata("vector",vector_id,MOMENT_CONTEXT,MOMENT_RULE,context),
                          metadata("vector",vector_id,GSAS_MOMENT_QUANTITY_DIALECT,MOMENT_RULE,quantity)]
            }));
        }
        let phase = json!({"id":id,"label":format!("GSAS phase {}",self.approval.phase),"parent":null,
            "rotation":IDENTITY,"translationA":[0,0,0],"fieldIds":[],"shapeId":null,
            "structure":{"cell":self.cell,"sites":sites,"covariance":null,
                "group":{"kind":"magnetic","symbol":self.chart.source_group_symbol,
                    "hallSymbol":self.approval.hall_symbol,"bnsNumber":null,"ogNumber":null,
                    "settingName":self.chart.setting_name,"originChoice":self.chart.origin_choice,
                    "changeOfBasis":IDENTITY,"originFractional":[0,0,0],"table":null,
                    "operators":operators,"sourceSpans":[]}}});
        let mut projection = tagged(
            TAG,
            &json!({
                "schemaVersion":"FC-GSAS-spin-projection-1","captureRole":"actual-current-qualified-spin-projection",
                "owner":reference("phase", id),"chartResource":bytes_ref(&resources[CHART_KEY]),
                "sourceFlagF32Bits":self.flags.map(|f| format!("{:08x}",f.to_bits())),
                "operationAssignments":self.operations.iter().zip(&identities.ops).map(|((_, sign), id)| json!({
                    "operation":reference("symmetry-operation",id),"timeSign":sign
                })).collect::<Vec<_>>(),"ruleId":SPIN_RULE,"ruleVersion":"1"
            }),
        )?;
        for (row, (_, sign)) in projection["payload"]["members"]["operationAssignments"]["items"]
            .as_array_mut()
            .expect("rendered operation list")
            .iter_mut()
            .zip(&self.operations)
        {
            row["members"]["timeSign"] = json!({"kind":"I64","value":sign.to_string()});
        }
        Ok((
            phase,
            Value::Array(vectors),
            metadata("phase", id, DIALECT, SPIN_RULE, projection),
        ))
    }

    pub(super) fn install(
        &self,
        capture: &mut Capture,
        source: &mut Value,
        ids: &mut Ids,
    ) -> Result<()> {
        if capture.content.resources.len() + 1 + 5 * self.moments.len() > 1024 {
            return Err(invalid(
                "GSAS active resource population exceeds the native bound",
            ));
        }
        let bindings =
            self.resources(|key, raw| retained::install(capture, ids, key, raw.to_vec()))?;
        let ids = Identities {
            phase: ids.next_id(),
            ops: (0..self.operations.len()).map(|_| ids.next_id()).collect(),
            sites: (0..self.moments.len()).map(|_| ids.next_id()).collect(),
            vectors: (0..self.moments.len()).map(|_| ids.next_id()).collect(),
        };
        let (phase, vectors, projection) = self.render(&ids, &bindings)?;
        source["phaseGraph"] = json!({"root":ids.phase,"phases":[phase]});
        source["sourceEntryOrder"] = json!([{"kind":"phase","id":ids.phase}]);
        source["vectors"] = vectors;
        source["metadata"]
            .as_array_mut()
            .expect("source-only metadata")
            .push(projection);
        source["resources"]
            .as_array_mut()
            .expect("source-only resources")
            .extend(bindings.values().map(|r| {
                json!({"id":r.id,"sha256":r.sha256,"byteLength":r.byte_length,"storageKind":"owned",
                "sourceDescriptorDigest":current_source7::descriptor_digest(r)})
            }));
        Ok(())
    }

    pub(super) fn verify(&self, capture: &Capture, source: &Value) -> Result<BTreeSet<String>> {
        let fail = || {
            invalid(
                "GSAS active phase, moment or logical resource differs from its qualified binding",
            )
        };
        let bindings = self.resources(|key, expected| {
            let resource = capture
                .content
                .resources
                .iter()
                .find(|row| row.logical_key == key)
                .ok_or_else(fail)?;
            if retained::raw(capture, resource)? != expected {
                return Err(fail());
            }
            Ok(resource.clone())
        })?;
        let phases = source["phaseGraph"]["phases"].as_array().ok_or_else(fail)?;
        let [phase] = phases.as_slice() else {
            return Err(fail());
        };
        let identity = |row: &Value| row["id"].as_str().map(str::to_owned).ok_or_else(fail);
        let collect = |node: &Value, count: usize| {
            node.as_array()
                .filter(|a| a.len() == count)
                .ok_or_else(fail)?
                .iter()
                .map(identity)
                .collect::<Result<Vec<_>>>()
        };
        let ids = Identities {
            phase: identity(phase)?,
            ops: collect(
                &phase["structure"]["group"]["operators"],
                self.operations.len(),
            )?,
            sites: collect(&phase["structure"]["sites"], self.moments.len())?,
            vectors: collect(&source["vectors"], self.moments.len())?,
        };
        let (expected_phase, mut expected_vectors, projection) = self.render(&ids, &bindings)?;
        for (expected, current) in expected_vectors
            .as_array_mut()
            .expect("rendered vectors")
            .iter_mut()
            .zip(source["vectors"].as_array().ok_or_else(fail)?)
        {
            // Source observations certify the initial conversion. The current value
            // and display scale are persisted independently of the original M rows.
            expected["value"]["components"] = current["value"]["components"].clone();
            expected["value"]["displayScale"] = current["value"]["displayScale"].clone();
        }
        let rows: Vec<_> = source["metadata"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|row| row["tree"]["tag"] == TAG || row["dialect"] == DIALECT)
            .collect();
        if rows != [&projection]
            || *phase != expected_phase
            || source["phaseGraph"]["root"] != ids.phase
            || source["vectors"] != expected_vectors
        {
            return Err(fail());
        }
        Ok(bindings.into_keys().collect())
    }
}

struct Identities {
    phase: String,
    sites: Vec<String>,
    ops: Vec<String>,
    vectors: Vec<String>,
}

fn descriptor_prefix(index: usize) -> String {
    format!(
        "descriptor-data/{}",
        digest(gsas_moment_use(index).as_bytes())
    )
}

fn target(pointer: &str) -> String {
    format!("target/{}", digest(pointer.as_bytes()))
}

#[cfg(test)]
const AUTHORED: Qualification = Qualification {
    exp_sha256: "1747820c4887c0a18ce77b11f39af950425a2a93ecc949e799ef7594516b01f9",
    phase: 3,
    setting: "authored-triclinic-control",
    origin: "authored-zero",
    hall_symbol: "-P 1",
    chart: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/single-phase/authored-single.chart.json"
    )),
    frame: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/single-phase/authored-single.frame.json"
    )),
    producer_evidence: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/single-phase/authored-single.expected.json"
    )),
    producer_version: "authored-test-control",
    numerics: NumericalPolicy {
        max_condition: 6.,
        length_a: 5.1e-7,
        angle_deg: 5.1e-5,
        orthonormal: 1e-12,
    },
    sites: &[
        SiteRule {
            serial: 1,
            type_token: "C",
            element: "C",
            operation: 0,
            shift: [0, 0, 0],
            image_millionths: [125_000, 200_000, 300_000],
            display_scale: 1.,
        },
        SiteRule {
            serial: 2,
            type_token: "C",
            element: "C",
            operation: 0,
            shift: [1, -1, 0],
            image_millionths: [1_375_000, -600_000, 750_000],
            display_scale: 1.,
        },
    ],
};

#[cfg(test)]
pub(super) fn control_exp() -> &'static [u8] {
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/single-phase/authored-single.exp"
    ))
}

#[cfg(test)]
const MULTIPHASE_CONTROL: Qualification = Qualification {
    exp_sha256: "8336542af0c66f83018661970a51e511245c98341e297472c383fb72ec7c3901",
    chart: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/multiphase/authored.chart.json"
    )),
    frame: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/multiphase/authored.frame.json"
    )),
    producer_evidence: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/multiphase/authored.expected.json"
    )),
    ..AUTHORED
};

#[cfg(test)]
pub(super) fn multiphase_control_exp() -> &'static [u8] {
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/gsas/multiphase/authored.exp"
    ))
}
