//! Source-bound VICS phase reconstruction. Historical observations stay
//! separate from the editable editor and from unobserved legacy UI defaults.
use super::super::{
    full_source7, native_vesta_v2::source_only_capture, retained_source as retained,
};
use super::{
    invalid,
    legacy_display::{self, Section},
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::{
    Capture, Cell3, Ids, PersistentResource, RestoredProject, Result, digest, source_audit::Node,
    source7::SourceDocument,
};

const KEY: &str = "vics/historical-source";
const DIALECT: &str = "vics-historical-phase-1";
const YBA: &str = "674ea1de5a9404baeeb96028047646abe401afee09390499ab0f16c654d90d9f";
const CATI: &str = "135f3d2a41f7df73fbc4b5aaf4fbafb1354918c94350483cb746962cf861cba2";
const IDENTITY: [[i32; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

pub(super) fn rule_digest() -> String {
    digest(&vesta_model::source_audit::canonical(&json!([
        include_str!("../../../contracts/vics-format.contract.json"),
        "source-thermal-serial-label-1"
    ])))
}

pub fn open(raw: &[u8], mut ids: Ids) -> Result<RestoredProject> {
    let mut capture = source_only_capture(&mut ids);
    let res = retained::install(&mut capture, &mut ids, KEY, raw.to_vec())?;
    let mut doc = full_source7::source_only_base(&capture);
    construct(&mut doc, &res, raw, &mut || Ok(ids.next_id()))?;
    capture.source = Some(SourceDocument::decode(doc)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

fn word(v: &str) -> Result<f64> {
    v.parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .ok_or_else(|| invalid("VICS finite source number"))
}

fn values<const N: usize>(fields: &[&str]) -> Result<[f64; N]> {
    let numbers = fields.iter().map(|w| word(w)).collect::<Result<Vec<_>>>()?;
    numbers
        .try_into()
        .map_err(|_| invalid("VICS source record width"))
}

fn payload<'a>(s: &Section<'a>) -> Result<Vec<Vec<&'a str>>> {
    Ok(retained::lines(s.bytes, 512 * 1024)?
        .into_iter()
        .skip(1)
        .map(|r| r.text.split_ascii_whitespace().collect())
        .filter(|row: &Vec<_>| !row.is_empty())
        .collect())
}

struct Data<'a> {
    title: &'a str,
    group: (&'a str, &'a str, &'a str),
    cell: Cell3,
    operators: Vec<([i32; 3], [i32; 3])>,
    sites: Vec<(&'a str, &'a str, [f64; 3], f64)>,
    bounds: [[f64; 3]; 2],
    sections: Vec<Section<'a>>,
}

fn fixed_group(orth_c: bool) -> BTreeSet<([i32; 3], [i32; 3])> {
    if orth_c {
        return [0, 1]
            .into_iter()
            .flat_map(|centre| {
                (0..8).map(move |bits| {
                    let signs = std::array::from_fn(|i| if bits & (1 << i) == 0 { 1 } else { -1 });
                    (signs, [centre, centre, 0])
                })
            })
            .collect();
    }
    // Conventional Pnma: two twofold generators and the origin inversion.
    (0..8)
        .map(|bits| {
            let a = bits & 1 != 0;
            let b = bits & 2 != 0;
            let invert = bits & 4 != 0;
            let x = if a ^ b ^ invert { -1 } else { 1 };
            let y = if a ^ invert { -1 } else { 1 };
            let z = if b ^ invert { -1 } else { 1 };
            ([x, y, z], [a as i32, b as i32, a as i32])
        })
        .collect()
}

impl<'a> Data<'a> {
    fn read(raw: &'a [u8]) -> Result<Self> {
        let orth_c = match digest(raw).as_str() {
            YBA => true,
            CATI => false,
            _ => {
                return Err(invalid(
                    "legacy VICS history needs a qualified historical source profile",
                ));
            }
        };
        let sections = legacy_display::sections(raw)?;
        let block = |key: &str| -> Result<&Section<'a>> {
            let mut found = sections.iter().filter(|s| s.keyword == key);
            let first = found
                .next()
                .ok_or_else(|| invalid(format!("VICS historical {key} absent")))?;
            if found.next().is_some() {
                return Err(invalid(format!("VICS historical {key} repeated")));
            }
            Ok(first)
        };
        let titles = retained::lines(block("TITLE")?.bytes, 512 * 1024)?;
        let [_, title] = titles.as_slice() else {
            return Err(invalid("legacy VICS TITLE extent"));
        };
        let title = title.text;
        let group = if orth_c {
            ("C m m m", "-C 2 2", "65:legacy-1")
        } else {
            ("P n m a", "-P 2ac 2n", "62:legacy-3")
        };
        let requested = payload(block("GROUP")?)?;
        let want = if orth_c {
            ["65", "1", "C", "m", "m", "m"]
        } else {
            ["62", "3", "P", "n", "m", "a"]
        };
        if requested.len() != 1 || requested[0] != want {
            return Err(invalid("VICS historical GROUP signature"));
        }
        let param = payload(block("CELLP")?)?;
        if param.len() != 2 {
            return Err(invalid("VICS historical cell extent"));
        }
        let [a, b, c, alpha, beta, gamma] = values(&param[0])?;
        let _: [f64; 6] = values(&param[1])?;
        let cell = Cell3 {
            length_a: [a, b, c],
            angles_deg: [alpha, beta, gamma],
        };
        cell.validate()?;
        if cell.angles_deg != [90.; 3] {
            return Err(invalid("VICS orthogonal historical metric"));
        }

        let ops = payload(block("SYMOP")?)?;
        let (sentinel, rows) = ops
            .split_last()
            .ok_or_else(|| invalid("VICS historical operation extent"))?;
        if sentinel.as_slice()
            != [
                "-1.0", "-1.0", "-1.0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
            ]
        {
            return Err(invalid("VICS operation sentinel"));
        }
        let mut operators = Vec::new();
        for row in rows {
            if row.len() != 12 {
                return Err(invalid("VICS operation width"));
            }
            let [x, y, z]: [f64; 3] = values(&row[..3])?;
            let base = [x, y, z];
            if base.iter().any(|v| *v != 0.0 && *v != 0.5)
                || [4, 5, 6, 8, 9, 10].into_iter().any(|i| row[i] != "0")
            {
                return Err(invalid("VICS selected exact affine operation domain"));
            }
            let signs = [row[3], row[7], row[11]].map(|w| match w {
                "1" => 1,
                "-1" => -1,
                _ => 0,
            });
            operators.push((signs, base.map(|v| (v * 2.) as i32)));
        }
        let got = operators.iter().copied().collect::<BTreeSet<_>>();
        if got.len() != operators.len() || got != fixed_group(orth_c) {
            return Err(invalid(
                "VICS actual ordered operators differ from the selected group",
            ));
        }
        let site_rows = payload(block("STRUC")?)?;
        let (sentinel, definitions) = site_rows
            .split_last()
            .ok_or_else(|| invalid("VICS historical sites"))?;
        if sentinel.as_slice() != ["0"; 7] || definitions.len() % 2 != 0 {
            return Err(invalid("VICS STRUC extent"));
        }
        let mut labels = BTreeSet::new();
        let mut sites = Vec::new();
        for (i, pair) in definitions.chunks_exact(2).enumerate() {
            let [serial, element, label, occupancy, x, y, z] = pair[0].as_slice() else {
                return Err(invalid("VICS site row width"));
            };
            let _: [f64; 3] = values(&pair[1])?;
            let occupancy = word(occupancy)?;
            if serial.parse::<usize>().ok() != Some(i + 1)
                || !vesta_model::element(element)
                || !labels.insert(*label)
                || !(0.0..=1.0).contains(&occupancy)
            {
                return Err(invalid("VICS source site identity or occupancy"));
            }
            sites.push((*element, *label, [word(x)?, word(y)?, word(z)?], occupancy));
        }
        let rows = payload(block("BOUND")?)?;
        let [limits] = rows.as_slice() else {
            return Err(invalid("VICS historical boundary extent"));
        };
        let [x0, x1, y0, y1, z0, z1] = values(limits)?;
        let bounds = [[x0, y0, z0], [x1, y1, z1]];
        if (0..3).any(|i| bounds[0][i] >= bounds[1][i]) {
            return Err(invalid("VICS historical boundary ordering"));
        }
        Ok(Self {
            title,
            group,
            cell,
            operators,
            sites,
            bounds,
            sections,
        })
    }

    fn phase(&self, next: &mut impl FnMut() -> Result<String>) -> Result<Value> {
        let id = next()?;
        let r = |n| json!({"numerator":n,"denominator":1});
        let mut sites = Vec::new();
        for (element, label, frac, occ) in &self.sites {
            sites.push(json!({"id":next()?,"label":label,
                "species":{"element":element,"charge":null,"massNumber":null,"rawToken":element},
                "fractional":frac,"occupancy":occ,"adp":null,"sourceSpans":[]}));
        }
        let operators=self.operators.iter().map(|(signs,shift)| {
            let rows=std::array::from_fn::<_,3,_>(|i|std::array::from_fn::<_,3,_>(|j|
                r(if i==j {signs[i]} else {0})));
            Ok(json!({"id":next()?,"fractionalMatrix":rows,"timeSign":1,
                "fractionalTranslation":shift.map(|c|if c==0 {r(0)} else {json!({"numerator":1,"denominator":2})})}))
        }).collect::<Result<Vec<_>>>()?;
        let (symbol, hall, setting) = self.group;
        Ok(
            json!({"id":id,"label":self.title,"parent":null,"rotation":IDENTITY,"translationA":[0,0,0],
            "structure":{"cell":self.cell,"sites":sites,"covariance":null,"group":{
                "kind":"ordinary","symbol":symbol,"hallSymbol":hall,"settingName":setting,"originChoice":"1",
                "bnsNumber":null,"ogNumber":null,"changeOfBasis":IDENTITY,"originFractional":[0,0,0],
                "table":null,"operators":operators,"sourceSpans":[]}},"fieldIds":[],"shapeId":null}),
        )
    }
}

fn text(v: impl Into<String>) -> Node {
    Node::Text(v.into())
}
fn tokens<'a>(words: impl IntoIterator<Item = &'a str>) -> Node {
    Node::Array(words.into_iter().map(text).collect())
}

fn thermal_sources(data: &Data<'_>) -> Result<(Node, Node)> {
    let mut sections = data
        .sections
        .iter()
        .filter(|s| matches!(s.keyword, "THERI" | "THERT" | "THERM"));
    let (Some(iso), Some(selector), Some(tensor)) =
        (sections.next(), sections.next(), sections.next())
    else {
        if data
            .sections
            .iter()
            .any(|s| matches!(s.keyword, "THERI" | "THERT" | "THERM"))
        {
            return Err(invalid("legacy VICS thermal source tables are incomplete"));
        }
        return Ok((Node::Null, Node::Array(Vec::new())));
    };
    if (iso.keyword, selector.keyword, tensor.keyword) != ("THERI", "THERT", "THERM")
        || sections.next().is_some()
    {
        return Err(invalid("legacy VICS thermal source table order"));
    }
    let selector_tokens = retained::lines(selector.bytes, 512 * 1024)?;
    if selector_tokens.len() != 1
        || selector_tokens[0]
            .text
            .split_ascii_whitespace()
            .ne(["THERT", "0"])
    {
        return Err(invalid("legacy VICS historical THERT selector"));
    }
    let iso_rows = payload(iso)?;
    let tensor_rows = payload(tensor)?;
    let (Some((iso_end, iso_rows)), Some((tensor_end, tensor_rows))) =
        (iso_rows.split_last(), tensor_rows.split_last())
    else {
        return Err(invalid("legacy VICS thermal source extent"));
    };
    if iso_end.as_slice() != ["0"; 3]
        || tensor_end.as_slice() != ["0"; 8]
        || iso_rows.len() != data.sites.len()
        || tensor_rows.len() != data.sites.len()
    {
        return Err(invalid("legacy VICS thermal source extent"));
    }
    let records = data
        .sites
        .iter()
        .zip(iso_rows.iter().zip(tensor_rows))
        .enumerate()
        .map(|(i, ((_, label, _, _), (iso, tensor)))| {
            let serial = (i + 1).to_string();
            let [iso_id, iso_label, scalar] = iso.as_slice() else {
                return Err(invalid("legacy VICS thermal scalar source width"));
            };
            if tensor.len() != 8
                || *iso_id != serial
                || tensor[0] != serial
                || iso_label != label
                || tensor[1] != *label
            {
                return Err(invalid(
                    "legacy VICS thermal record must match STRUC serial and label",
                ));
            }
            word(scalar)?;
            let _: [f64; 6] = values(&tensor[2..])?;
            Ok(Node::Map(BTreeMap::from([
                ("sourceSerial".into(), Node::U64((i + 1) as u64)),
                ("sourceLabel".into(), text(*label)),
                ("theriToken".into(), text(*scalar)),
                ("thermTokens".into(), tokens(tensor[2..].iter().copied())),
            ])))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok((text("0"), Node::Array(records)))
}

fn observations(
    data: &Data<'_>,
    res: &PersistentResource,
    raw: &[u8],
    phase: &Value,
) -> Result<Value> {
    let mut original = Vec::new();
    let mut bond_recipes = Vec::new();
    for section in &data.sections {
        let physical = retained::lines(section.bytes, 512 * 1024)?;
        original.push(Node::Map(BTreeMap::from([
            ("keyword".into(), text(section.keyword)),
            (
                "records".into(),
                tokens(physical.iter().map(|row| row.text)),
            ),
        ])));
        if section.keyword == "SBOND" {
            let rows = payload(section)?;
            let (sentinel, recipes) = rows
                .split_last()
                .ok_or_else(|| invalid("legacy VICS SBOND extent"))?;
            if sentinel.as_slice() != ["0"; 4] {
                return Err(invalid("legacy VICS SBOND sentinel"));
            }
            for (i, row) in recipes.iter().enumerate() {
                if !matches!(row.len(), 4 | 7)
                    || row[0].parse::<usize>().ok() != Some(i + 1)
                    || !row[1..3]
                        .iter()
                        .all(|element| vesta_model::element(element))
                    || word(row[3])? < 0.
                {
                    return Err(invalid("legacy VICS historical bond spec"));
                }
                bond_recipes.push(Node::Map(BTreeMap::from([
                    ("fieldCount".into(), Node::U64(row.len() as u64)),
                    ("ordinal".into(), Node::U64((i + 1) as u64)),
                    ("sourceTokens".into(), tokens(row.iter().copied())),
                ])));
            }
        }
    }
    let (selector, thermal) = thermal_sources(data)?;
    let tree = Node::Map(BTreeMap::from([
        ("originalRecords".into(), Node::Array(original)),
        ("thermalSelectorWord".into(), selector),
        ("thermalSourceRows".into(), thermal),
        ("originalBondRecipes".into(), Node::Array(bond_recipes)),
        (
            "rawPrimary".into(),
            Node::from_wire(&json!({"kind":"BytesRef","resourceId":res.id,
            "sha256":res.sha256,"byteLength":res.byte_length}))?,
        ),
    ]))
    .wire();
    Ok(
        json!({"owner":{"kind":"phase","id":phase},"producerId":"draft.vics-historical-reader","producerVersion":"1",
        "formatRef":"VM18-I02","dialect":DIALECT,"tree":tree,"spans":data.sections.iter()
            .map(|s|retained::span(res,raw,s.start,s.end)).collect::<Vec<_>>() }),
    )
}

fn construct(
    doc: &mut Value,
    res: &PersistentResource,
    raw: &[u8],
    next: &mut impl FnMut() -> Result<String>,
) -> Result<()> {
    let data = Data::read(raw)?;
    let phase = data.phase(next)?;
    let phase_id = &phase["id"];
    let b = next()?;
    let bound = json!({"id":b,"owner":{"kind":"phase","id":phase_id},"value":{"phaseId":phase_id,
        "minimumFractional":data.bounds[0],"maximumFractional":data.bounds[1],"endpoints":"closed",
        "includeBondExtensions":false,"ruleIds":[]},"metadata":[]});
    doc["sourceEntryOrder"] = json!([{"kind":"phase","id":phase_id}]);
    doc["boundaries"] = json!([bound]);
    doc["metadata"] = json!([observations(&data, res, raw, phase_id)?]);
    doc["phaseGraph"] = json!({"root":phase_id,"phases":[phase]});
    Ok(())
}

pub(super) fn verify(capture: &Capture, source: &Value) -> Result<()> {
    let mut meta = source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|r| r["dialect"] == DIALECT);
    let resource = capture
        .content
        .resources
        .iter()
        .find(|r| r.logical_key == KEY);
    match (meta.next(), resource) {
        (None, None) => Ok(()),
        (Some(row), Some(resource)) if meta.next().is_none() => {
            let raw = retained::raw(capture, resource)?;
            let phase = &source["phaseGraph"]["phases"][0];
            let mut ids = std::iter::once(&phase["id"])
                .chain(
                    phase["structure"]["sites"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|r| &r["id"]),
                )
                .chain(
                    phase["structure"]["group"]["operators"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|r| &r["id"]),
                )
                .chain(std::iter::once(&source["boundaries"][0]["id"]));
            let mut expected = json!({});
            construct(&mut expected, resource, raw, &mut || {
                ids.next()
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .ok_or_else(|| invalid("VICS restored historical identity lacks a definition"))
            })?;
            if ids.next().is_some()
                || ["phaseGraph", "boundaries"]
                    .into_iter()
                    .any(|k| source[k] != expected[k])
                || !source["sourceEntryOrder"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|entry| entry["kind"] == "phase")
                    .eq(expected["sourceEntryOrder"].as_array().unwrap().iter())
                || *row != expected["metadata"][0]
            {
                return Err(invalid(
                    "restored VICS historical phase, owners or source bytes differ",
                ));
            }
            Ok(())
        }
        _ => Err(invalid(
            "VICS historical phase lacks its one actual retained resource",
        )),
    }
}
