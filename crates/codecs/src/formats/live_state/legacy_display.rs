use super::super::{current_source7, retained_source as retained};
use super::{entity, invalid, payload, tagged, view, words};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::{Capture, Ids, PersistentResource, Result, digest};

const KEY: &str = "vics/legacy-primary";
const HISTORY: &str = "vics-legacy-archive-1";
const FIELD: &str = "nativeLegacyDisplayCurrent";
const TAG: &str = "NativeLegacyDisplayCurrent1";
const HEADERS: &[&str] = &[
    "TITLE", "GROUP", "SYMOP", "CELLP", "STRUC", "THERI", "THERT", "THERM", "ATOMT", "MODEL",
    "BOUND", "SBOND", "PLANE", "ATOMS", "BONDS", "POLYS", "ATOMP", "BONDP", "POLYP", "HKLPP",
    "UCOLP", "BKGRC", "PROJT", "COMPS", "SCENE", "LIGHT", "ATOMM", "BONDM", "POLYM", "HKLPM",
    "DELOB",
];

pub(super) fn rule_digest() -> String {
    digest(include_bytes!(
        "../../../contracts/vics-legacy-display.contract.json"
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct DisplayInput {
    legacy_source: String,
    #[serde(default)]
    current_values: bool,
    scene_slots: Map<String, Value>,
    lights: Vec<Value>,
    atom_types: Vec<Value>,
    phase_boundaries: Vec<Value>,
    bond_rules: Vec<Value>,
    shape_alpha: Vec<Value>,
    polyhedron_alpha: Vec<Value>,
}

pub(super) struct Section<'a> {
    pub(super) keyword: &'a str,
    pub(super) bytes: &'a [u8],
    pub(super) start: usize,
    pub(super) end: usize,
}

pub(super) fn sections(raw: &[u8]) -> Result<Vec<Section<'_>>> {
    let lines = retained::lines(raw, 512 * 1024)?;
    let begins = lines
        .iter()
        .filter_map(|line| {
            let keyword = line.text.split_ascii_whitespace().next()?;
            HEADERS.contains(&keyword).then_some((keyword, line.start))
        })
        .collect::<Vec<_>>();
    if begins.first() != Some(&("TITLE", 0))
        || begins.len() > 512
        || !["GROUP", "CELLP", "STRUC", "MODEL", "SCENE", "LIGHT"]
            .iter()
            .all(|k| begins.iter().any(|r| &r.0 == k))
    {
        return Err(invalid("legacy VICS source framing"));
    }
    Ok(begins
        .iter()
        .enumerate()
        .map(|(i, &(keyword, start))| {
            let end = begins.get(i + 1).map_or(raw.len(), |r| r.1);
            Section {
                keyword,
                bytes: &raw[start..end],
                start,
                end,
            }
        })
        .collect())
}

fn history(scene: &Value, resource: &PersistentResource, raw: &[u8]) -> Result<Value> {
    use vesta_model::source_audit::Node;
    let groups = sections(raw)?;
    let tree = Node::Map(BTreeMap::from([
        (
            "legacyResource".into(),
            Node::from_wire(&json!({"kind":"BytesRef","resourceId":resource.id,
            "sha256":resource.sha256,"byteLength":resource.byte_length}))?,
        ),
        (
            "orderedSections".into(),
            Node::Array(
                groups
                    .iter()
                    .map(|section| Node::Text(section.keyword.into()))
                    .collect(),
            ),
        ),
    ]))
    .wire();
    Ok(
        json!({"owner":{"kind":"scene","id":scene},"producerId":"draft.vics-legacy-reader",
        "producerVersion":"1","formatRef":"VM18-I02","dialect":HISTORY,"tree":tree,
        "spans":groups.iter().map(|s| retained::span(resource,raw,s.start,s.end)).collect::<Vec<_>>() }),
    )
}

fn integer(text: &str) -> Result<i32> {
    let (sign, digits) = if let Some(t) = text.strip_prefix('-') {
        (-1, t)
    } else {
        (1, text.strip_prefix('+').unwrap_or(text))
    };
    let (radix, digits) = if let Some(t) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16, t)
    } else if digits.len() > 1 && digits.starts_with('0') {
        (8, &digits[1..])
    } else {
        (10, digits)
    };
    if digits.is_empty() || !digits.chars().all(|digit| digit.is_digit(radix)) {
        return Err(invalid("legacy VICS integer digits"));
    }
    i64::from_str_radix(digits, radix)
        .ok()
        .and_then(|v| i32::try_from(sign * v).ok())
        .ok_or_else(|| invalid("legacy VICS native integer"))
}

fn float(token: &str) -> Result<f32> {
    token
        .parse::<f32>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| invalid("legacy VICS finite float"))
}

fn color(xs: &[f32]) -> Result<Vec<u8>> {
    xs.iter()
        .map(|&v| {
            if !(0.0..=1.0).contains(&v) {
                return Err(invalid("legacy VICS qualified normalized channel"));
            }
            Ok((f64::from(v * 255.) + 0.5).floor() as u8)
        })
        .collect()
}

fn f32_word(n: f32) -> Value {
    json!({"kind":"F32Bits","ieeeHexBE":format!("{:08x}",n.to_bits())})
}

fn apply_scene(
    raw: &[u8],
    slots: &mut Map<String, Value>,
    lights: &mut [Value],
    polyhedra: &mut [Value],
    shapes: &mut [Value],
) -> Result<()> {
    // The supplied snapshot provides slots that old records leave untouched.
    let mut flags = slots
        .get("0x5b0")
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| invalid("legacy VICS full prior display word"))?;
    for record in sections(raw)? {
        let text = std::str::from_utf8(record.bytes).expect("checked ASCII");
        let t: Vec<_> = text.split_ascii_whitespace().skip(1).collect();
        match (record.keyword, t.as_slice()) {
            ("MODEL", [selector] | [selector, _]) => {
                slots.insert(
                    "0x5b4".into(),
                    json!(
                        selector
                            .parse::<i32>()
                            .ok()
                            .and_then(|n| n.checked_sub(1))
                            .ok_or_else(|| invalid("legacy VICS MODEL selector"))?
                    ),
                );
                if t.len() == 2 {
                    flags = t[1]
                        .parse::<i32>()
                        .map_err(|_| invalid("VICS MODEL full flags"))?
                        as u32;
                }
            }
            ("BONDS" | "POLYS" | "COMPS", [v]) => {
                let key = match record.keyword {
                    "BONDS" => "0x28c",
                    "POLYS" => "0x2a4",
                    _ => "0x234",
                };
                slots.insert(key.into(), json!(integer(v)?));
            }
            ("PROJT", [v] | [v, _]) => {
                if integer(v)? == 0 {
                    flags |= 0x200000;
                }
                if t.len() == 2 {
                    slots.insert("0x40c".into(), f32_word(float(t[1])?));
                }
            }
            ("ATOMS", [v]) => {
                if integer(v)? != 0 {
                    flags |= 0x20000;
                }
            }
            ("ATOMP", [a, b, c, size] | [a, b, c, size, _] | [a, b, c, size, _, _]) => {
                for (key, token) in [("0x268", a), ("0x26c", b), ("0x274", c)] {
                    slots.insert(key.into(), json!(integer(token)?));
                }
                let q = f64::from(float(size)?);
                let n = if q > 1. { q } else { q * 100. };
                if n < i32::MIN as f64 || n > i32::MAX as f64 {
                    return Err(invalid("VICS ATOMP size overflow"));
                }
                slots.insert("0x270".into(), json!(n as i32));
                if t.len() > 4 {
                    slots.insert("0x278".into(), f32_word(float(t[4])?));
                }
                if t.len() == 6 && integer(t[5])? != 0 {
                    flags |= 0x800;
                }
            }
            ("BONDP", [a, b, radius, discarded, secondary, r, g, bcol]) => {
                let _ = float(discarded)?;
                slots.insert("0x284".into(), json!(integer(a)?));
                slots.insert("0x288".into(), json!(integer(b)?));
                slots.insert("0x29c".into(), f32_word(float(radius)? * 0.5));
                slots.insert("0x298".into(), f32_word(float(secondary)?));
                slots.insert(
                    "0x2a0:3".into(),
                    json!(color(&[float(r)?, float(g)?, float(bcol)?])?),
                );
            }
            ("POLYP", [alpha, mode, scale, r, g, b]) => {
                let rgba = color(&[float(r)?, float(g)?, float(b)?, float(alpha)?])?;
                for object in polyhedra.iter_mut() {
                    let value = object
                        .as_object_mut()
                        .ok_or_else(|| invalid("current VICS polyhedron row"))?;
                    value.insert("alpha".into(), json!(rgba[3]));
                }
                slots.insert("0x2b0:4".into(), json!(rgba));
                slots.insert("0x2a8".into(), json!(integer(mode)?));
                slots.insert("0x2ac".into(), f32_word(float(scale)?));
            }
            ("HKLPP", [alpha, mode, scale, r, g, b]) => {
                let _ = [float(r)?, float(g)?, float(b)?];
                let opacity = color(&[float(alpha)?])?[0];
                if let Some(first) = shapes.first_mut() {
                    let value = first
                        .as_object_mut()
                        .ok_or_else(|| invalid("current VICS shape row"))?;
                    value.insert("alpha".into(), json!(opacity));
                }
                slots.insert("0x2b8".into(), json!(integer(mode)?));
                slots.insert("0x2b4".into(), f32_word(float(scale)?));
            }
            ("UCOLP", [mode, width, r, g, b]) => {
                let mut rgb: [u8; 3] =
                    serde_json::from_value(slots.get("0x230:3").cloned().unwrap_or(Value::Null))
                        .map_err(|_| {
                            invalid("actual prior legacy UCOLP scratch bytes are required")
                        })?;
                for (i, token) in [r, g, b].into_iter().enumerate() {
                    let (prefix, stops) = token
                        .split_once('.')
                        .map_or((*token, false), |(n, _)| (n, true));
                    if prefix.is_empty() {
                        break;
                    }
                    rgb[i] = integer(prefix)? as u8;
                    if stops {
                        break;
                    } // '%i' leaves the dot ahead of the next conversion.
                }
                slots.insert("0x230:3".into(), json!(rgb));
                slots.insert(
                    "0x228".into(),
                    json!(
                        i16::try_from(integer(mode)?)
                            .map_err(|_| invalid("VICS signed16 UCOLP mode"))?
                    ),
                );
                slots.insert("0x22c".into(), f32_word(float(width)?));
            }
            ("BKGRC", [_, _, _]) => {
                let rgb = t.iter().map(|v| float(v)).collect::<Result<Vec<_>>>()?;
                slots.insert("0x3fc:3".into(), json!(color(&rgb)?));
            }
            ("LIGHT", values) if values.len() == 31 => {
                let v = values
                    .iter()
                    .map(|w| float(w))
                    .collect::<Result<Vec<_>>>()?;
                let matrix = std::array::from_fn::<_, 16, _>(|i| {
                    format!("{:08x}", v[(i % 4) * 4 + i / 4].to_bits())
                });
                lights[0] = json!({"index":0,"matrix16":words(matrix,"F32Bits"),
                    "position4":v[16..20].iter().copied().map(f32_word).collect::<Vec<_>>(),
                    "direction3":v[20..23].iter().copied().map(f32_word).collect::<Vec<_>>(),
                    "rgba":[color(&v[23..27])?,color(&v[27..31])?,vec![255;4]]});
            }
            ("ATOMM" | "BONDM" | "POLYM" | "HKLPM", values) if values.len() == 13 => {
                let (rgba, shininess) = match record.keyword {
                    "ATOMM" => ("0x3cc:4", "0x3d0"),
                    "BONDM" => ("0x3d4:4", "0x3d8"),
                    "POLYM" => ("0x3dc:4", "0x3e0"),
                    _ => ("0x3f4:4", "0x3f8"),
                };
                let rgb = values[8..12]
                    .iter()
                    .map(|w| float(w))
                    .collect::<Result<Vec<_>>>()?;
                slots.insert(rgba.into(), json!(color(&rgb)?));
                slots.insert(shininess.into(), f32_word(float(values[12])?));
            }
            (
                "MODEL" | "BONDS" | "POLYS" | "COMPS" | "PROJT" | "ATOMS" | "ATOMP" | "BKGRC"
                | "BONDP" | "POLYP" | "HKLPP" | "UCOLP" | "LIGHT" | "ATOMM" | "BONDM" | "POLYM"
                | "HKLPM",
                _,
            ) => return Err(invalid("unsupported selected VICS display record")),
            _ => {}
        }
    }
    slots.insert("0x5b0".into(), json!(flags));
    Ok(())
}

fn rows(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

fn closed_mesh(row: &Value) -> bool {
    let mesh = &row["value"]["geometry"];
    let mut edges = BTreeMap::<(u64, u64), (usize, i32)>::new();
    if rows(&mesh["verticesA"]).count() < 4 || rows(&mesh["triangles"]).count() < 4 {
        return false;
    }
    for face in rows(&mesh["triangles"]) {
        let [Some(a), Some(b), Some(c)] = [face[0].as_u64(), face[1].as_u64(), face[2].as_u64()]
        else {
            return false;
        };
        for (i, j) in [(a, b), (b, c), (c, a)] {
            if i == j {
                return false;
            }
            let uses = edges.entry((i.min(j), i.max(j))).or_default();
            uses.0 += 1;
            uses.1 += if i < j { 1 } else { -1 };
        }
    }
    edges
        .values()
        .all(|(count, orientation)| *count == 2 && *orientation == 0)
}

struct Population<'a> {
    field: &'static str,
    kind: &'static str,
    objects: Vec<&'a Value>,
}

fn populations<'a>(source: &'a Value, scene: &str) -> [Population<'a>; 5] {
    let owns_phase = |row: &&Value| row["owner"]["kind"] == "phase";
    [
        Population {
            field: "atomTypes",
            kind: "display",
            objects: rows(&source["displays"])
                .filter(|r| r["owner"] == json!({"kind":"scene","id":scene}))
                .collect(),
        },
        Population {
            field: "phaseBoundaries",
            kind: "boundary",
            objects: rows(&source["boundaries"]).filter(owns_phase).collect(),
        },
        Population {
            field: "bondRules",
            kind: "bond-rule",
            objects: rows(&source["bondRules"]).filter(owns_phase).collect(),
        },
        Population {
            field: "shapeAlpha",
            kind: "shape",
            objects: rows(&source["shapes"])
                .filter(|r| {
                    rows(&source["phaseGraph"]["phases"]).any(|p| {
                        p["shapeId"] == r["id"]
                            && r["owner"] == json!({"kind":"phase","id":p["id"]})
                    })
                })
                .collect(),
        },
        Population {
            field: "polyhedronAlpha",
            kind: "mesh",
            objects: rows(&source["meshes"])
                .filter(owns_phase)
                .filter(|r| closed_mesh(r))
                .collect(),
        },
    ]
}

fn role_ref(pop: &Population<'_>, item: &Value) -> Result<Value> {
    let id = item["id"]
        .as_str()
        .ok_or_else(|| invalid("legacy display target id"))?;
    Ok(entity(pop.kind, id))
}

fn owner_ref(item: &Value) -> Result<Value> {
    let id = item["owner"]["id"]
        .as_str()
        .ok_or_else(|| invalid("legacy display phase owner id"))?;
    Ok(entity("phase", id))
}

fn lights_check(lights: &[Value]) -> Result<()> {
    if !lights
        .iter()
        .map(|row| row["index"].as_u64())
        .eq((0..4).map(Some))
    {
        return Err(invalid(
            "VICS display requires four ordered, explicit light entries",
        ));
    }
    Ok(())
}

pub(super) fn install(
    mut input: DisplayInput,
    current_view: Option<view::ViewInput>,
    source: &mut Value,
    capture: &mut Capture,
    read: &mut impl FnMut(&str) -> Result<Vec<u8>>,
    ids: &mut Ids,
) -> Result<()> {
    let scene = capture.content.scene_id.as_str().to_owned();
    let raw = read(&input.legacy_source)?;
    let archived_view = source_view(&raw)?;
    lights_check(&input.lights)?;
    if input.current_values {
        if current_view.is_none() {
            return Err(invalid("VICS current display requires an explicit current view"));
        }
    } else {
        apply_scene(
            &raw,
            &mut input.scene_slots,
            &mut input.lights,
            &mut input.polyhedron_alpha,
            &mut input.shape_alpha,
        )?;
    }
    let mut body = json!({"schemaVersion":"FC-vics-legacy-display-current-1","captureRole":"actual-current-native-display",
        "scene":entity("scene",&scene),"sceneSlots":input.scene_slots,"lights":input.lights});
    for (pop, values) in populations(source, &scene).into_iter().zip([
        input.atom_types,
        input.phase_boundaries,
        input.bond_rules,
        input.shape_alpha,
        input.polyhedron_alpha,
    ]) {
        if values.len() != pop.objects.len() {
            return Err(invalid(format!(
                "legacy display {} population differs",
                pop.field
            )));
        }
        let items = values
            .into_iter()
            .zip(&pop.objects)
            .map(|(mut row, object)| {
                let m = row
                    .as_object_mut()
                    .ok_or_else(|| invalid("VICS display population row"))?;
                if m.contains_key("target") || m.contains_key("owner") {
                    return Err(invalid(
                        "VICS display cannot supply raw target or owner IDs",
                    ));
                }
                m.insert("target".into(), role_ref(&pop, object)?);
                if pop.kind != "display" {
                    m.insert("owner".into(), owner_ref(object)?);
                }
                Ok(row)
            })
            .collect::<Result<Vec<_>>>()?;
        body[pop.field] = json!(items);
    }
    let target = source["scenes"]
        .as_array_mut()
        .and_then(|r| r.iter_mut().find(|v| v["id"] == scene))
        .ok_or_else(|| invalid("VICS display actual scene missing"))?;
    if target["render"]["kind"] == "Null" {
        target["render"] = json!({"kind":"Map","members":{}});
    }
    let members = target["render"]["members"]
        .as_object_mut()
        .ok_or_else(|| invalid("VICS scene render map"))?;
    if members.contains_key(FIELD) {
        return Err(invalid("legacy display already installed"));
    }
    members.insert(FIELD.into(), tagged(TAG, &body)?);
    view::install(current_view.unwrap_or(archived_view), &scene, source)?;
    let resource = retained::install(capture, ids, KEY, raw)?;
    let row = history(&json!(scene), &resource, retained::raw(capture, &resource)?)?;
    source["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("VICS document metadata"))?
        .push(row);
    source["resources"].as_array_mut().ok_or_else(|| invalid("VICS resource inventory"))?
        .push(json!({"id":resource.id,"sha256":resource.sha256,"byteLength":resource.byte_length,"storageKind":"owned",
            "sourceDescriptorDigest":current_source7::descriptor_digest(&resource)}));
    Ok(())
}

fn source_view(raw: &[u8]) -> Result<view::ViewInput> {
    let groups = sections(raw)?;
    let mut scenes = groups.iter().filter(|s| s.keyword == "SCENE");
    let scene_section = scenes
        .next()
        .ok_or_else(|| invalid("legacy VICS SCENE source missing"))?;
    if scenes.next().is_some() {
        return Err(invalid("ambiguous legacy VICS SCENE source"));
    }
    let lines = retained::lines(scene_section.bytes, 4096)?;
    let rows = lines[1..]
        .iter()
        .filter(|r| !r.text.trim().is_empty())
        .map(|r| r.text.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if !rows.iter().map(Vec::len).eq([4, 4, 4, 4, 2, 1, 1]) {
        return Err(invalid("legacy VICS SCENE matrix and setting row extent"));
    }
    let mut matrix = [0f64; 16];
    for (r, row) in rows[..4].iter().enumerate() {
        for (c, token) in row.iter().enumerate() {
            matrix[4 * c + r] = token
                .parse::<f64>()
                .ok()
                .filter(|n| n.is_finite())
                .ok_or_else(|| invalid("legacy VICS SCENE finite f64 word"))?;
        }
    }
    let settings = rows[4..]
        .iter()
        .flatten()
        .map(|v| float(v).map(|n| format!("{:08x}", n.to_bits())))
        .collect::<Result<Vec<_>>>()?;
    Ok(view::ViewInput {
        matrix_bits: matrix.map(|n| format!("{:016x}", n.to_bits())),
        settings_bits: settings
            .try_into()
            .map_err(|_| invalid("legacy VICS SCENE four settings"))?,
    })
}

pub(super) fn inspect_source_views(capture: &Capture) -> Result<Vec<Value>> {
    capture.content.resources.iter()
        .filter(|row| matches!(row.logical_key.as_str(), KEY | super::vics_history::KEY))
        .map(|row| {
            let view = source_view(retained::raw(capture, row)?)?;
            Ok(json!({
                "role":"original-scene-record",
                "sourceResourceId":row.id,"sha256":row.sha256,"byteLength":row.byte_length,
                "matrix16":words(view.matrix_bits,"F64Bits"),"settings4":words(view.settings_bits,"F32Bits"),
            }))
        }).collect()
}

pub(super) fn inspect(source: &Value) -> Result<Vec<Value>> {
    let mut result = Vec::new();
    for scene in rows(&source["scenes"]) {
        let Some(node) = scene["render"]["members"].get(FIELD) else {
            continue;
        };
        let id = scene["id"]
            .as_str()
            .ok_or_else(|| invalid("VICS actual scene id"))?;
        let body = payload(node)?;
        if node["tag"] != TAG || body["scene"] != entity("scene", id) {
            return Err(invalid("VICS legacy display scene owner differs"));
        }
        lights_check(
            body["lights"]
                .as_array()
                .ok_or_else(|| invalid("VICS light collection"))?,
        )?;
        for pop in populations(source, id) {
            let entries = body[pop.field]
                .as_array()
                .ok_or_else(|| invalid("VICS display population"))?;
            let mut seen = BTreeSet::new();
            if entries.len() != pop.objects.len() {
                return Err(invalid("VICS display population coverage"));
            }
            for (entry, object) in entries.iter().zip(&pop.objects) {
                if entry["target"] != role_ref(&pop, object)?
                    || !seen.insert(object["id"].as_str())
                    || pop.kind != "display" && entry["owner"] != owner_ref(object)?
                {
                    return Err(invalid("VICS display target, order or owner differs"));
                }
            }
        }
        result.push(body);
    }
    Ok(result)
}

pub(super) fn verify(capture: &Capture, source: &Value) -> Result<()> {
    let views = inspect(source)?;
    let mut rows = rows(&source["metadata"]).filter(|r| r["dialect"] == HISTORY);
    let resource = capture
        .content
        .resources
        .iter()
        .find(|r| r.logical_key == KEY);
    match (rows.next(), resource, views.len()) {
        (None, None, 0) => Ok(()),
        (Some(row), Some(res), 1) if rows.next().is_none() => {
            if !view::inspect(source)?.iter().any(|camera| camera["scene"] == views[0]["scene"]) {
                return Err(invalid("VICS current display has no matching current scene view"));
            }
            if *row
                != history(
                    &views[0]["scene"]["objectId"],
                    res,
                    retained::raw(capture, res)?,
                )?
            {
                return Err(invalid(
                    "VICS original record spans differ from retained bytes",
                ));
            }
            Ok(())
        }
        _ => Err(invalid(
            "VICS legacy display and owned history missing or ambiguous",
        )),
    }
}
