use super::super::{
    full_source7, graph_provenance, native_vesta_v2::source_only_capture, resource_graph,
    retained_source as retained,
};
use super::{entity, invalid, nvis_objects, payload, tagged};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::source_audit::{Node, canonical};
use vesta_model::source7::SourceDocument;
use vesta_model::{
    Capture, Ids, ImageKey, PersistentResource, Phase, RestoredProject, Result, digest,
};

const KEYS: [&str; 5] = ["LBLAT", "LBLSP", "DLATM", "DLBND", "DLPLY"];
const OLD: &str = "nvis-vesta354-source-1";
const CURRENT: &str = "nvis-draft-origin-image-1";
const ANATASE_KEY: &str = "native-anatase/historical-primary";
const ANATASE_DIALECT: &str = "vesta-anatase-source-flags-1";
const ANATASE_SHA: &str = "0b504afd99de60ecc2cee12f916aab80d786507dfc1abc1469f667a617beee06";
const ANATASE_PRODUCER: &str = "draft.native-anatase-source-reader";

pub(super) fn rule_digest() -> String {
    digest(&canonical(&json!([
        include_str!("../../../contracts/native-flag-state.contract.json"),
        "draft.explicit-r80-p1-box-neighbor-population-2",
        super::nvis_molecule::DIALECT
    ])))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct FlagsInput {
    labeled_site_indices: Vec<usize>,
    disabled_site_indices: Vec<usize>,
    selected_objects: Option<nvis_objects::ObjectInput>,
}

struct Context<'a> {
    owner: &'a Phase,
    resource: &'a PersistentResource,
    raw: &'a [u8],
    graph_sha: String,
}

fn context<'a>(capture: &'a Capture, source: &Value) -> Result<Context<'a>> {
    let [owner] = capture.content.phases.as_slice() else {
        return Err(invalid(
            "selected NVIS mode needs one actual loaded P1 phase",
        ));
    };
    let table = source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|row| {
            row["dialect"] == "native-r80-orffe-1" && row["owner"]["id"] == owner.id.as_str()
        });
    if !table {
        return Err(invalid(
            "NVIS phase is not bound to the selected original engineering source",
        ));
    }
    let mut originals = Vec::new();
    for proof in graph_provenance::verify(capture)? {
        if let Some(row) = proof.files.get(proof.root_key()) {
            let raw = retained::raw(capture, row)?;
            if resource_graph::is_r80_source(raw) {
                originals.push((*row, raw, proof.context.sha256.clone()));
            }
        }
    }
    let [(resource, raw, sha)] = originals.as_slice() else {
        return Err(invalid(
            "NVIS requires one independent verified R80 primary resource",
        ));
    };
    Ok(Context {
        owner,
        resource,
        raw,
        graph_sha: sha.clone(),
    })
}

struct Observation<'a> {
    key: &'a str,
    tokens: Vec<&'a str>,
    indices: Vec<usize>,
    start: usize,
    end: usize,
}

fn observations(raw: &[u8]) -> Result<Vec<Observation<'_>>> {
    let lines = retained::lines(raw, 512 * 1024)?;
    let mut observations = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let key = line.text.trim();
        if !KEYS.contains(&key) {
            continue;
        }
        let row = lines
            .get(i + 1)
            .ok_or_else(|| invalid("missing native flag list"))?;
        let tokens = row.text.split_ascii_whitespace().collect::<Vec<_>>();
        let Some((&"-1", fields)) = tokens.split_last() else {
            return Err(invalid("native flag sentinel"));
        };
        let indices = fields
            .iter()
            .map(|word| {
                if word.len() > 10
                    || word.starts_with('0') && word.len() > 1
                    || !word.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(invalid("invalid native flag index lexeme"));
                }
                word.parse::<i32>()
                    .map(|v| v as usize)
                    .map_err(|_| invalid("invalid native flag index extent"))
            })
            .collect::<Result<Vec<_>>>()?;
        observations.push(Observation {
            key,
            tokens,
            indices,
            start: row.start,
            end: row.end,
        });
    }
    if observations.len() != KEYS.len() || !observations.iter().map(|row| row.key).eq(KEYS) {
        return Err(invalid(
            "selected native flag population has a missing or repeated source list",
        ));
    }
    Ok(observations)
}

pub fn is_historical_anatase(raw: &[u8]) -> bool {
    raw.len() <= 512 * 1024 && digest(raw) == ANATASE_SHA
}

pub(super) fn historical_rule_digest() -> String {
    digest(&canonical(&json!([
        ANATASE_DIALECT,
        ANATASE_SHA,
        rule_digest()
    ])))
}

fn source_morphology(resource: &PersistentResource, raw: &[u8]) -> Result<(Node, Vec<Value>)> {
    let lines = retained::lines(raw, 512 * 1024)?;
    let mut sections = Vec::new();
    let header = &lines[0];
    let mut spans = vec![retained::span(resource, raw, header.start, header.end)];
    for (keyword, next) in [("SHAPE", "BOUND"), ("SPLAN", "LBLAT")] {
        let mut matches = lines
            .iter()
            .enumerate()
            .filter(|(_, row)| row.text == keyword);
        let (start, _) = matches
            .next()
            .ok_or_else(|| invalid("Anatase morphology source section"))?;
        if matches.next().is_some() {
            return Err(invalid("Anatase morphology section repeated"));
        }
        let end = lines[start + 1..]
            .iter()
            .position(|row| row.text == next)
            .map(|n| start + 1 + n)
            .filter(|&n| n > start + 1)
            .ok_or_else(|| invalid("Anatase morphology source extent"))?;
        let rows = lines[start + 1..end]
            .iter()
            .map(|row| Node::Text(row.text.into()))
            .collect();
        sections.push(Node::Map(BTreeMap::from([
            ("keyword".into(), Node::Text(keyword.into())),
            ("sourceRecords".into(), Node::Array(rows)),
        ])));
        spans.push(retained::span(
            resource,
            raw,
            lines[start].start,
            lines[end - 1].end,
        ));
    }
    Ok((Node::Array(sections), spans))
}

fn historical_anatase(
    document: &Value,
    resource: &PersistentResource,
    raw: &[u8],
) -> Result<Value> {
    if !is_historical_anatase(raw) {
        return Err(invalid(
            "Anatase archival source is outside the qualified input profile",
        ));
    }
    let observations = observations(raw)?;
    let (morphology, mut spans) = source_morphology(resource, raw)?;
    let lists = observations
        .iter()
        .map(|row| {
            Node::Map(BTreeMap::from([
                ("keyword".into(), Node::Text(row.key.into())),
                (
                    "sourceTokens".into(),
                    Node::Array(
                        row.tokens
                            .iter()
                            .map(|word| Node::Text((*word).into()))
                            .collect(),
                    ),
                ),
                (
                    "sourceIndices".into(),
                    Node::Array(
                        row.indices
                            .iter()
                            .map(|&index| Node::U64(index as u64))
                            .collect(),
                    ),
                ),
                ("resolution".into(), Node::Text("source-only".into())),
            ]))
        })
        .collect::<Vec<_>>();
    let primary = Node::from_wire(&json!({"kind":"BytesRef","resourceId":resource.id,
        "sha256":resource.sha256,"byteLength":resource.byte_length}))?;
    let tree = Node::Map(BTreeMap::from([
        ("rawPrimary".into(), primary),
        (
            "sourceFormatHeader".into(),
            Node::Text("#VESTA_FORMAT_VERSION 2.90.1".into()),
        ),
        ("sourceLists".into(), Node::Array(lists)),
        ("morphologySourceRecords".into(), morphology),
    ]));
    spans.extend(
        observations
            .iter()
            .map(|row| retained::span(resource, raw, row.start, row.end)),
    );
    Ok(
        json!({"owner":{"kind":"document","id":document},"producerId":ANATASE_PRODUCER,
        "producerVersion":"1","formatRef":"VM18-I01","dialect":ANATASE_DIALECT,"tree":tree.wire(),"spans":spans}),
    )
}

pub fn open_historical_anatase(raw: &[u8], mut ids: Ids) -> Result<RestoredProject> {
    if !is_historical_anatase(raw) {
        return Err(invalid(
            "historical Anatase needs the exact archived primary",
        ));
    }
    let mut capture = source_only_capture(&mut ids);
    let res = retained::install(&mut capture, &mut ids, ANATASE_KEY, raw.to_vec())?;
    let mut doc = full_source7::source_only_base(&capture);
    doc["metadata"] = json!([historical_anatase(
        &doc["identity"]["documentId"],
        &res,
        raw
    )?]);
    capture.source = Some(SourceDocument::decode(doc)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

pub(super) fn verify_historical_anatase(capture: &Capture, source: &Value) -> Result<()> {
    let mut rows = source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| row["dialect"] == ANATASE_DIALECT || row["producerId"] == ANATASE_PRODUCER);
    let res = capture
        .content
        .resources
        .iter()
        .find(|row| row.logical_key == ANATASE_KEY);
    match (rows.next(), res) {
        (None, None) => Ok(()),
        (Some(row), Some(res)) if rows.next().is_none() => {
            if *row
                != historical_anatase(
                    &source["identity"]["documentId"],
                    res,
                    retained::raw(capture, res)?,
                )?
            {
                return Err(invalid(
                    "Anatase archived source list or exact span differs",
                ));
            }
            Ok(())
        }
        _ => Err(invalid(
            "Anatase source lists require their one archived primary",
        )),
    }
}

fn row(document_owner: &str, dialect: &str, tree: Value, spans: &[Value]) -> Value {
    json!({"owner":{"kind":"phase","id":document_owner},"producerId":"draft.native-object-flags",
        "producerVersion":"1","formatRef":"VM18-I01","dialect":dialect,"tree":tree,"spans":spans})
}

pub(super) fn image_ref(owner: &Phase, index: usize) -> Result<Value> {
    let site = owner
        .sites
        .get(index)
        .ok_or_else(|| invalid("current native atom image outside actual phase"))?;
    Ok(json!({
        "phase":entity("phase",owner.id.as_str()),"site":entity("site",site.id.as_str()),
        "operation":entity("symmetry-operation",owner.symmetry_op_id.as_str()),"shift":[0,0,0]}))
}

fn image(owner: &Phase, index: usize) -> Result<Value> {
    Ok(json!({"kind":"atom-image","image":image_ref(owner,index)?}))
}

pub(super) fn active(owner: &Phase, index: usize) -> bool {
    let frac = owner.sites[index].frac;
    (0..3).all(|a| owner.boundary_min[a] <= frac[a] && frac[a] <= owner.boundary_max[a])
}

fn selections(owner: &Phase, indices: &[usize]) -> Result<Value> {
    let mut seen = BTreeSet::new();
    let selected=indices.iter().map(|&index| {
        if !seen.insert(index) { return Err(invalid("duplicate current flag selection")); }
        Ok(json!({"target":image(owner,index)?,"applicability":if active(owner,index) {"active"} else {"latent"}}))
    }).collect::<Result<Vec<_>>>()?;
    Ok(json!({"kind":"qualified","selections":selected}))
}

fn enumeration(
    context: &Context<'_>,
    historical: bool,
    objects: Option<&nvis_objects::Populations>,
) -> Value {
    let (algorithm, population) = if historical {
        (
            "draft.vesta354-source-list-reader",
            json!({"orderedTokens":"source-only","primarySHA":context.resource.sha256}),
        )
    } else {
        (
            "draft.native-current-origin-image",
            json!({
                "phaseCell":context.owner.cell,"boundary":[context.owner.boundary_min,context.owner.boundary_max],
                "atomPopulation":context.owner.sites.iter().enumerate().map(|(i,s)|
                    json!({"index":i,"fractional":s.frac,"label":s.label,"active":active(context.owner,i)})
                ).collect::<Vec<_>>(),
                "otherNativePopulations":objects.map(nvis_objects::Populations::geometry)
            }),
        )
    };
    json!({"sourceGraphDigest":context.graph_sha,"recipeDigest":rule_digest(),
        "populationDigest":digest(&canonical(&population)),"algorithmId":algorithm,"algorithmVersion":"1"})
}

type SelectedObjects<'a> = [&'a [usize]; 3];

fn object_selection(pop: &[nvis_objects::Target], indices: &[usize]) -> Result<Value> {
    let mut seen = BTreeSet::new();
    let selected = indices
        .iter()
        .map(|&i| {
            if !seen.insert(i) {
                return Err(invalid("duplicate NVIS current object"));
            }
            let target = pop.get(i).ok_or_else(|| {
                invalid("NVIS current object index outside the actual population")
            })?;
            Ok(json!({"target":target.address,"applicability":if target.active {"active"} else {"latent"}}))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({"kind":"qualified","selections":selected}))
}

fn records(
    context: &Context<'_>,
    labels: &[usize],
    hidden: &[usize],
    objects: Option<&nvis_objects::Populations>,
    picks: SelectedObjects<'_>,
) -> Result<(Value, Value)> {
    let scope = observations(context.raw)?;
    let owner = context.owner.id.as_str();
    let spans = scope
        .iter()
        .map(|r| retained::span(context.resource, context.raw, r.start, r.end))
        .collect::<Vec<_>>();
    let old = scope
        .iter()
        .map(|r| {
            json!({"keyword":r.key,"sourceTokens":r.tokens,"sourceIndices":r.indices,
        "resolution":{"kind":"source-only","reason":"historical-native-population-unverified"}})
        })
        .collect::<Vec<_>>();
    let old_body = json!({"schemaVersion":"NativeObjectFlags-proposal-1","phaseId":entity("phase",owner),
        "enumeration":enumeration(context,true,None),"records":old});
    let archived=scope.iter().map(|r| json!({"kind":"parsed-source","keyword":r.key,"sourceTokens":r.tokens,
        "sourceIndices":r.indices,"sourceSpan":{"resourceId":entity("resource",context.resource.id.as_str()),
            "sha256":context.resource.sha256,"byteOffset":r.start.to_string(),"byteLength":(r.end-r.start).to_string()}}))
        .collect::<Vec<_>>();
    let mut current = Vec::new();
    for source in &scope {
        let resolution = match source.key {
            "LBLAT" => selections(context.owner, labels)?,
            "DLATM" => selections(context.owner, hidden)?,
            key if objects.is_some() => {
                let index = match key {
                    "LBLSP" => 0,
                    "DLBND" => 1,
                    _ => 2,
                };
                object_selection(objects.unwrap().for_key(key), picks[index])?
            }
            _ => json!({"kind":"source-only","nativeIndices":source.indices,
                "reason":"no-qualified-current-native-population"}),
        };
        current.push(json!({"keyword":source.key,"resolution":resolution}));
    }
    let current_body = json!({"schemaVersion":"NativeFlagState-proposal-2",
        "owner":{"kind":"phase","id":entity("phase",owner)},"capturedEnumeration":enumeration(context,false,objects),
        "observations":archived,"currentFlagSets":current});
    Ok((
        row(owner, OLD, tagged("NVIS1", &old_body)?, &spans),
        row(owner, CURRENT, tagged("NVIS2", &current_body)?, &spans),
    ))
}

fn hidden_index(owner: &Phase, image: &ImageKey) -> Result<usize> {
    if image.phase != owner.id || image.op != owner.symmetry_op_id || image.shift != [0; 3] {
        return Err(invalid(
            "current native flag adapter requires actual origin images",
        ));
    }
    owner
        .sites
        .iter()
        .position(|site| site.id == image.site)
        .ok_or_else(|| invalid("native hidden image owner"))
}

pub(super) fn install(
    input: FlagsInput,
    capture: &mut Capture,
    source: &mut Value,
    ids: &mut Ids,
) -> Result<()> {
    let metadata = source["metadata"]
        .as_array()
        .ok_or_else(|| invalid("NVIS source metadata"))?;
    if metadata.iter().any(|row| {
        row["tree"]["tag"] == "NVIS1"
            || row["tree"]["tag"] == "NVIS2" && row["dialect"] != super::nvis_molecule::DIALECT
    }) {
        return Err(invalid("native flags already installed"));
    }
    let c = context(capture, source)?;
    if let Some(objects) = &input.selected_objects {
        nvis_objects::install(objects, c.owner, c.raw, source, ids)?;
    }
    let population = nvis_objects::capture(c.owner, c.raw, source)?;
    let picks = input.selected_objects.as_ref().map_or([&[][..]; 3], |o| {
        [
            &o.labeled_face_indices[..],
            &o.disabled_bond_indices[..],
            &o.disabled_polyhedron_indices[..],
        ]
    });
    let (old, current) = records(
        &c,
        &input.labeled_site_indices,
        &input.disabled_site_indices,
        population.as_ref(),
        picks,
    )?;
    let hidden = input
        .disabled_site_indices
        .iter()
        .map(|&index| ImageKey {
            phase: c.owner.id.clone(),
            site: c.owner.sites[index].id.clone(),
            op: c.owner.symmetry_op_id.clone(),
            shift: [0; 3],
        })
        .collect();
    capture.content.hidden = hidden;
    source["metadata"]
        .as_array_mut()
        .unwrap()
        .extend([old, current]);
    Ok(())
}

pub(in crate::formats) fn refresh(capture: &Capture, source: &mut Value) -> Result<()> {
    let rows = source["metadata"]
        .as_array()
        .ok_or_else(|| invalid("NVIS source metadata"))?;
    let positions = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            if row["dialect"] == super::nvis_molecule::DIALECT {
                return false;
            }
            matches!(row["dialect"].as_str(), Some(OLD | CURRENT))
                || matches!(row["tree"]["tag"].as_str(), Some("NVIS1" | "NVIS2"))
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if positions.is_empty() {
        return Ok(());
    }
    let [old_pos, active_pos] = positions.as_slice() else {
        return Err(invalid("NVIS source and current records are a pair"));
    };
    let c = context(capture, source)?;
    let old = &rows[*old_pos];
    let actual = &rows[*active_pos];
    let body = payload(&actual["tree"])?;
    let list = body["currentFlagSets"][0]["resolution"]["selections"]
        .as_array()
        .ok_or_else(|| invalid("NVIS label selection table"))?;
    let labels = list
        .iter()
        .map(|entry| {
            let target = &entry["target"];
            let id = target["image"]["site"]["objectId"]
                .as_str()
                .ok_or_else(|| invalid("NVIS typed atom owner"))?;
            let index = c
                .owner
                .sites
                .iter()
                .position(|s| s.id.as_str() == id)
                .ok_or_else(|| {
                    invalid("NVIS label does not address the current site population")
                })?;
            if *target != image(c.owner, index)? {
                return Err(invalid("NVIS label does not own that origin image"));
            }
            Ok(index)
        })
        .collect::<Result<Vec<_>>>()?;
    let hidden = capture
        .content
        .hidden
        .iter()
        .map(|value| hidden_index(c.owner, value))
        .collect::<Result<Vec<_>>>()?;
    let objects = nvis_objects::capture(c.owner, c.raw, source)?;
    let mut picks: [Vec<usize>; 3] = Default::default();
    if let Some(pop) = &objects {
        for ((position, key), slots) in [(1, "LBLSP"), (3, "DLBND"), (4, "DLPLY")]
            .into_iter()
            .zip(&mut picks)
        {
            let actual = &body["currentFlagSets"][position];
            let existing = actual["resolution"]["selections"]
                .as_array()
                .filter(|_| actual["keyword"] == key)
                .ok_or_else(|| invalid("NVIS missing selected current object population"))?;
            for row in existing {
                let at = pop
                    .for_key(key)
                    .iter()
                    .position(|target| row["target"] == target.address)
                    .ok_or_else(|| {
                        invalid("NVIS selected face, bond or polyhedron has no current target")
                    })?;
                if !matches!(row["applicability"].as_str(), Some("active" | "latent")) {
                    return Err(invalid("NVIS selected object applicability"));
                }
                slots.push(at);
            }
        }
    }
    let (expected, updated) = records(
        &c,
        &labels,
        &hidden,
        objects.as_ref(),
        [&picks[0], &picks[1], &picks[2]],
    )?;
    if *old != expected
        || actual["tree"]["tag"] != "NVIS2"
        || actual["dialect"] != CURRENT
        || body["observations"] != payload(&updated["tree"])?["observations"]
    {
        return Err(invalid(
            "NVIS exact historic source list or owning span differs",
        ));
    }
    source["metadata"][*active_pos] = updated;
    if let Some(objects) = &objects {
        objects.sync_boundary(c.owner, source)?;
    }
    Ok(())
}

pub(super) fn inspect(capture: &Capture, source: &Value) -> Result<Vec<Value>> {
    if !source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|row| row["dialect"] == CURRENT)
    {
        return Ok(Vec::new());
    }
    let mut state = source.clone();
    refresh(capture, &mut state)?;
    state["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| row["dialect"] == CURRENT)
        .map(|row| payload(&row["tree"]))
        .collect()
}
