//! Fixed-column observations from an original-style GSAS experiment.
use super::{full_source7, native_vesta_v2::source_only_capture, retained_source as retained};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use vesta_model::source_audit::{AuditSource, Node};
use vesta_model::source7::SourceDocument;
use vesta_model::{
    Capture, Cell3, Error, Ids, PersistentResource, RestoredProject, Result, digest,
};

mod chart;
mod magnetic;
#[cfg(test)]
mod tests;

const KEY: &str = "gsas-exp/primary";
const DIALECT: &str = "gsas-exp-observations-1";
const PRODUCER: &str = "draft.gsas-exp-reader";
const MAX: usize = 512 * 1024;

pub(super) fn rule_digest() -> String {
    digest(include_bytes!("../../contracts/gsas-format.contract.json"))
}

pub(super) fn magnetic_rule_digest() -> String {
    // This stable identifier denotes one installed active phase. The EXP may
    // declare other independent archived phases; its chart still binds exactly
    // one source phase number. No Source7 roles or archived rules changed.
    digest(&vesta_model::source_audit::canonical(&json!({
        "chartSchema":digest(chart::CHART.as_bytes()),
        "frameSchema":digest(chart::FRAME.as_bytes()),
        "spinRule":chart::SPIN_RULE,"momentRule":chart::MOMENT_RULE,
        "installation":"gsas-active-magnetic-1/independently-qualified-single-phase-exp"
    })))
}

fn invalid(message: &str) -> Error {
    Error::new("E_REOPEN_COVERAGE", message)
}

fn tokens<'a>(values: impl IntoIterator<Item = &'a str>) -> Node {
    Node::Array(values.into_iter().map(|s| Node::Text(s.into())).collect())
}

fn text(value: &str) -> Node {
    Node::Text(value.into())
}

fn words(payload: &str, width: usize, count: usize) -> Result<Vec<&str>> {
    let fixed = payload
        .get(..width * count)
        .ok_or_else(|| invalid("GSAS fixed-column record is truncated"))?;
    Ok((0..count)
        .map(|i| &fixed[i * width..(i + 1) * width])
        .collect())
}

fn decimal(token: &str, precision: usize) -> Result<f64> {
    let value = token.trim_start_matches(' ');
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value);
    let (integer, fraction) = digits
        .split_once('.')
        .ok_or_else(|| invalid("GSAS fixed decimal point"))?;
    if (integer.is_empty() && fraction.is_empty())
        || fraction.len() != precision
        || !integer
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err(invalid("GSAS fixed decimal field"));
    }
    value
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .ok_or_else(|| invalid("GSAS finite field"))
}

fn numbers(payload: &str, width: usize, count: usize, precision: usize) -> Result<Vec<f64>> {
    words(payload, width, count)?
        .into_iter()
        .map(|token| decimal(token, precision))
        .collect()
}

type Record<'a> = &'a retained::PhysicalLine<'a>;
type Records<'a> = BTreeMap<&'a str, Record<'a>>;

fn record<'a>(records: &Records<'a>, key: &str) -> Result<Record<'a>> {
    records
        .get(key)
        .copied()
        .ok_or_else(|| invalid("GSAS phase is missing a required record"))
}

fn atom_key(key: &str) -> bool {
    &key[4..8] == "  AT" && matches!(key.as_bytes()[11], b'A' | b'B' | b'M')
}

fn phase_records<'a>(records: &Records<'a>, phase: usize, kind: u8) -> Result<Node> {
    if kind == 4 {
        return Ok(Node::Map(BTreeMap::from([
            ("phaseNumber".into(), Node::U64(phase as u64)),
            ("phaseKind".into(), Node::U64(u64::from(kind))),
            ("resolution".into(), text("opaque-macromolecular-records")),
        ])));
    }
    let abc = &record(records, "    ABC ")?.text[12..];
    let angles = &record(records, "  ANGLES")?.text[12..];
    let group = &record(records, "  SG SYM")?.text[12..];
    if group.trim_matches(' ').is_empty() {
        return Err(invalid("GSAS phase setting is empty"));
    }
    let cell = Cell3 {
        length_a: numbers(abc, 10, 3, 6)?
            .try_into()
            .expect("three cell lengths"),
        angles_deg: numbers(angles, 10, 3, 4)?
            .try_into()
            .expect("three cell angles"),
    };
    cell.validate()?;
    let magnetic = matches!(kind, 2 | 3);
    let spin = match (magnetic, records.get("  SPNFLP")) {
        (true, Some(row)) => {
            let payload = &row.text[12..];
            let slots = words(payload, 4, 12)?;
            let populated = slots.iter().position(|word| *word == "    ").unwrap_or(slots.len());
            if populated == 0
                || !payload[48..].trim_matches(' ').is_empty()
                || slots[populated..].iter().any(|word| *word != "    ")
                || slots[..populated].iter().any(|word| {
                    let field = word.trim_start_matches(' ');
                    let unsigned = field.strip_prefix(['-', '+']).unwrap_or(field);
                    !matches!(unsigned, "0" | "0." | "0.0" | "1" | "1." | "1.0")
                })
            {
                return Err(invalid(
                    "GSAS SPNFLP requires fixed sign words followed by optional empty slots",
                ));
            }
            tokens(slots)
        }
        (false, None) => Node::Null,
        _ => {
            return Err(invalid(
                "GSAS SPNFLP is missing or belongs to a nonmagnetic phase",
            ));
        }
    };

    let mut atoms = BTreeMap::<u16, [Option<Record<'a>>; 3]>::new();
    for (&key, &row) in records {
        if !key.starts_with("  AT") {
            continue;
        }
        let serial = key[4..7]
            .trim_start_matches(' ')
            .parse::<u16>()
            .ok()
            .filter(|n| (1..=999).contains(n) && format!("{n:>3}").as_str() == &key[4..7])
            .ok_or_else(|| invalid("GSAS atom index is not a fixed I3 field"))?;
        let family = match key.as_bytes()[7] {
            b'A' => 0,
            b'B' => 1,
            b'M' => 2,
            _ => unreachable!("selected atom keys"),
        };
        atoms.entry(serial).or_default()[family] = Some(row);
    }
    if atoms.is_empty() {
        return Err(invalid("GSAS phase has no A/B atomic records"));
    }
    if let Some(row) = records.get("   NATOM") {
        let payload = &row.text[12..];
        let count = payload
            .get(..5)
            .and_then(|n| n.trim_start_matches(' ').parse::<usize>().ok());
        if count != Some(atoms.len()) || atoms.keys().copied().ne(1..=atoms.len() as u16) {
            return Err(invalid("GSAS NATOM disagrees with the atom population"));
        }
    }

    let atoms = atoms
        .into_iter()
        .map(|(serial, parts)| {
            let [Some(a), Some(b), moment] = parts else {
                return Err(invalid("GSAS atom lacks its keyed A/B pair"));
            };
            if magnetic != moment.is_some() {
                return Err(invalid(
                    "GSAS A/B/M records must join the same phase and atom",
                ));
            }
            let a = &a.text[12..];
            let b = &b.text[12..];
            let atom_type = a
                .get(2..10)
                .filter(|s| !s.trim().is_empty())
                .ok_or_else(|| invalid("GSAS atom type field"))?;
            let coordinates = a
                .get(10..50)
                .ok_or_else(|| invalid("GSAS atomic coordinates are truncated"))?;
            numbers(coordinates, 10, 4, 6)?;
            let label = a
                .get(50..58)
                .ok_or_else(|| invalid("GSAS atom label field"))?;
            let multiplicity = a
                .get(58..62)
                .filter(|s| s.trim().parse::<u32>().is_ok_and(|n| n > 0))
                .ok_or_else(|| invalid("GSAS atom multiplicity field"))?;
            let code = b
                .get(62..63)
                .filter(|s| matches!(*s, "I" | "A"))
                .ok_or_else(|| invalid("GSAS B record thermal code"))?;
            let thermal_words = words(b, 10, 6)?;
            // EXPEDT writes F10.6,50X for isotropic sites. Preserve the
            // inactive spaces and continue accepting the older numeric form.
            let used_thermal_words = if code == "I"
                && thermal_words[1..].iter().all(|word| word.trim_matches(' ').is_empty())
            {
                &thermal_words[..1]
            } else {
                thermal_words.as_slice()
            };
            for token in used_thermal_words {
                decimal(token, 6)?;
            }
            let moment_words = if let Some(row) = moment {
                let m = &row.text[12..];
                numbers(m, 10, 3, 5)?;
                let tail = m
                    .get(36..)
                    .ok_or_else(|| invalid("GSAS magnetic flags are truncated"))?;
                numbers(tail, 10, 3, 3)?;
                if &m[30..31] != " "
                    || &m[33..34] != " "
                    || !tail[30..].trim_matches(' ').is_empty()
                {
                    return Err(invalid("GSAS magnetic fixed-column framing"));
                }
                Node::Array(
                    words(m, 10, 3)?
                        .into_iter()
                        .chain(words(tail, 10, 3)?)
                        .map(text)
                        .collect(),
                )
            } else {
                Node::Null
            };
            Ok(Node::Map(BTreeMap::from([
                ("sourceAtomNumber".into(), Node::U64(u64::from(serial))),
                ("atomTypeWord".into(), text(atom_type)),
                ("labelWord".into(), text(label)),
                (
                    "fractionalAndOccupancyWords".into(),
                    tokens(words(coordinates, 10, 4)?),
                ),
                ("multiplicityWord".into(), text(multiplicity)),
                ("thermalCode".into(), text(code)),
                ("thermalWords".into(), tokens(thermal_words)),
                ("atomARawPayload".into(), text(a)),
                ("atomBRawPayload".into(), text(b)),
                ("momentTokens".into(), moment_words),
                (
                    "momentRawPayload".into(),
                    moment.map_or(Node::Null, |row| text(&row.text[12..])),
                ),
            ])))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Node::Map(BTreeMap::from([
        ("phaseNumber".into(), Node::U64(phase as u64)),
        ("phaseKind".into(), Node::U64(u64::from(kind))),
        ("groupWord".into(), text(group)),
        ("cellLengthWords".into(), tokens(words(abc, 10, 3)?)),
        ("cellAngleWords".into(), tokens(words(angles, 10, 3)?)),
        ("spinFlipWords".into(), spin),
        ("atoms".into(), Node::Array(atoms)),
        ("resolution".into(), text("source-only")),
    ])))
}

fn metadata(document: &Value, res: &PersistentResource, raw: &[u8]) -> Result<Value> {
    let lines = retained::lines(raw, MAX)?;
    let mut nphas = None;
    let mut phases: [Records<'_>; 9] = Default::default();
    let mut spans = Vec::new();
    for row in &lines {
        if row.text.len() > 80 || row.text.contains('\t') {
            return Err(invalid(
                "GSAS physical record exceeds the fixed ASCII columns",
            ));
        }
        if row.text.trim_matches(' ').is_empty() {
            continue;
        }
        let key = row
            .text
            .get(..12)
            .ok_or_else(|| invalid("GSAS record lacks a twelve-column key"))?;
        if key == " EXPR NPHAS " {
            if nphas.replace(row).is_some() {
                return Err(invalid("duplicate GSAS phase table"));
            }
        } else if key.starts_with("CRS") {
            // The Fedora EXPEDT writer places ABC two columns earlier in
            // the key. Both forms denote the same cell record for a phase.
            let field = match &key[4..] {
                "  ABC   " => "    ABC ",
                field => field,
            };
            if !matches!(field, "    ABC " | "  ANGLES" | "  SG SYM" | "  SPNFLP" | "   NATOM")
                && !atom_key(key)
            {
                continue;
            }
            let phase = key.as_bytes()[3];
            if !matches!(phase, b'1'..=b'9')
                || phases[usize::from(phase - b'1')]
                    .insert(field, row)
                    .is_some()
            {
                return Err(invalid("invalid or duplicate GSAS phase record key"));
            }
        } else {
            continue;
        }
        spans.push(retained::span(res, raw, row.start, row.end));
    }
    let nphas = nphas.ok_or_else(|| invalid("missing GSAS phase table"))?;
    let raw_kinds = words(&nphas.text[12..], 5, 9)?;
    if !nphas.text[57..].trim_matches(' ').is_empty() {
        return Err(invalid("GSAS phase table has extra data"));
    }
    let mut observed = Vec::new();
    for (i, (word, records)) in raw_kinds.iter().zip(phases.iter()).enumerate() {
        let kind = word
            .trim_start_matches(' ')
            .parse::<u8>()
            .ok()
            .filter(|n| *n <= 4 && (*n != 4 || i == 0))
            .ok_or_else(|| invalid("GSAS phase type is outside the original table"))?;
        if kind != 0 {
            observed.push(phase_records(records, i + 1, kind)?);
        } else if !records.is_empty() {
            return Err(invalid("GSAS selected records address an absent phase"));
        }
    }
    if observed.is_empty() {
        return Err(invalid("GSAS experiment has no phase"));
    }
    let primary = Node::from_wire(&json!({"kind":"BytesRef","resourceId":res.id,
        "sha256":res.sha256,"byteLength":res.byte_length}))?;
    let tree = Node::Map(BTreeMap::from([
        ("rawPrimary".into(), primary),
        ("phaseWords".into(), tokens(raw_kinds)),
        ("phases".into(), Node::Array(observed)),
    ]));
    Ok(
        json!({"owner":{"kind":"document","id":document},"producerId":PRODUCER,
        "producerVersion":"1","formatRef":"VM18-I14","dialect":DIALECT,"tree":tree.wire(),"spans":spans}),
    )
}

pub fn open(raw: &[u8], mut ids: Ids) -> Result<RestoredProject> {
    if raw.len() > MAX {
        return Err(invalid("GSAS EXP byte bound"));
    }
    let mut capture = source_only_capture(&mut ids);
    let res = retained::install(&mut capture, &mut ids, KEY, raw.to_vec())?;
    let mut source = full_source7::source_only_base(&capture);
    let history = metadata(&source["identity"]["documentId"], &res, raw)?;
    source["metadata"] = json!([history]);
    if let Some(approval) = magnetic::qualified(&res.sha256) {
        magnetic::Prepared::new(approval, raw, &history)?.install(&mut capture, &mut source, &mut ids)?;
    }
    capture.source = Some(SourceDocument::decode(source)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

/// Inspect the retained fixed-column phase and atom observations, with their original byte spans.
pub fn inspect_observations(capture: &Capture) -> Result<Option<Value>> {
    let Some(primary) = capture.content.resources.iter().find(|row| row.logical_key == KEY) else {
        return Ok(None);
    };
    let full = capture.source.as_ref().ok_or_else(|| invalid("GSAS source is missing"))?;
    let source = full.to_value()?;
    verify(capture, &source, projection_count(&full.audit()?))?;
    let row = observed_metadata(&source)
        .next()
        .ok_or_else(|| invalid("GSAS source observations are missing"))?;
    let data = Node::from_wire(&row["tree"])?.data()?;
    Ok(Some(json!({
        "primary": {
            "resourceId": primary.id,
            "sha256": primary.sha256,
            "byteLength": primary.byte_length,
        },
        "phaseWords": data["phaseWords"],
        "phases": data["phases"],
        "sourceSpans": row["spans"],
    })))
}

/// Current magnetic state for engineering inspection of an installed EXP.
pub fn inspect_magnetic(capture: &Capture) -> Result<Option<Value>> {
    if !capture.content.resources.iter().any(|row| row.logical_key == magnetic::CHART_KEY) {
        return Ok(None);
    }
    let full = capture.source.as_ref().ok_or_else(|| invalid("GSAS magnetic source is missing"))?;
    let source = full.to_value()?;
    verify(capture, &source, projection_count(&full.audit()?))?;
    Ok(Some(json!({"phaseGraph":source["phaseGraph"],"vectors":source["vectors"]})))
}

pub(super) fn projection_count(source: &Node) -> usize {
    match source {
        Node::Owned { node, .. } => projection_count(node),
        Node::TaggedVariant { tag, payload } => {
            usize::from(tag == magnetic::TAG) + projection_count(payload)
        }
        Node::Array(items) => items.iter().map(projection_count).sum(),
        // AuditTree preserves the registered wire shape as a generated Map.
        // Its kind and tag are plain Text fields, while a raw TaggedVariant
        // still takes the branch above. Count both forms across nested trees.
        Node::Map(members) => {
            let tagged = matches!(members.get("kind"), Some(Node::Text(kind)) if kind == "TaggedVariant")
                && matches!(members.get("tag"), Some(Node::Text(tag)) if tag == magnetic::TAG);
            usize::from(tagged) + members.values().map(projection_count).sum::<usize>()
        }
        _ => 0,
    }
}

fn observed_metadata(source: &Value) -> impl Iterator<Item = &Value> {
    source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| row["dialect"] == DIALECT || row["producerId"] == PRODUCER)
}

pub(super) fn verify(
    capture: &Capture, source: &Value, projections: usize,
) -> Result<std::collections::BTreeSet<String>> {
    let mut observed = observed_metadata(source);
    let resource = capture
        .content
        .resources
        .iter()
        .find(|row| row.logical_key == KEY);
    let approved = resource.and_then(|row| magnetic::qualified(&row.sha256));
    if (approved.is_none() && projections != 0) || projections > 1 {
        return Err(invalid("GSAS current projection requires a qualified original chart provider"));
    }
    if capture.content.resources.iter().any(|row| {
        row.logical_key.starts_with("gsas-exp/active/")
            && (approved.is_none() || row.logical_key != magnetic::CHART_KEY)
    }) {
        return Err(invalid("GSAS active chart does not have a registered logical resource use"));
    }
    match (observed.next(), resource) {
        (None, None) => Ok(Default::default()),
        (Some(row), Some(res)) if observed.next().is_none() => {
            let raw = retained::raw(capture, res)?;
            if *row != metadata(&source["identity"]["documentId"], res, raw)? {
                return Err(invalid(
                    "restored GSAS source observations or exact spans differ",
                ));
            }
            if let Some(approval) = approved {
                if projections != 1 {
                    return Err(invalid("GSAS qualified phase is missing the active projection"));
                }
                return magnetic::Prepared::new(approval, raw, row)?.verify(capture, source);
            }
            // EXP observations qualify only the archive. A current group or
            // magnetic moment needs the same producer evidence as a projection tag.
            let magnetic_group = source["phaseGraph"]["phases"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|phase| {
                    let group = &phase["structure"]["group"];
                    group["kind"] == "magnetic"
                        || group["operators"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .any(|operation| operation["timeSign"].as_i64() == Some(-1))
                });
            let magnetic_vector = source["vectors"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|row| row["value"]["unit"] == "muB");
            if magnetic_group || magnetic_vector {
                return Err(invalid(
                    "GSAS current magnetic state requires a qualified original chart provider",
                ));
            }
            Ok(Default::default())
        }
        _ => Err(invalid(
            "GSAS source observations require their one owned primary",
        )),
    }
}
