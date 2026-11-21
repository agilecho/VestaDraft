use super::{entity, invalid, payload, tagged};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use vesta_model::{Result, digest, source_audit::canonical};

pub(super) const DIALECT: &str = "draft-nvis-nonperiodic-1";
const KEYWORDS: [&str; 5] = ["LBLAT", "LBLSP", "DLATM", "DLBND", "DLPLY"];
const COUNT_BOUND: usize = 4096;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Input {
    molecule_index: usize,
    labeled_atoms: Vec<[usize; 2]>,
    disabled_atoms: Vec<[usize; 2]>,
    disabled_bonds: Vec<[usize; 2]>,
}

fn rows(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

fn id(value: &Value) -> Result<&str> {
    value["id"]
        .as_str()
        .ok_or_else(|| invalid("NVIS actual nonperiodic definition has no identity"))
}

#[derive(Clone, Copy)]
enum Objects {
    Atoms,
    Bonds,
}

struct Population {
    atoms: Vec<([usize; 2], Value)>,
    bonds: Vec<([usize; 2], Value)>,
    digest: String,
}

impl Population {
    fn read(molecule: &Value) -> Result<Self> {
        if molecule["periodic"] != false {
            return Err(invalid("nonperiodic NVIS actual molecule type"));
        }
        let owner = id(molecule)?;
        let mut atoms = Vec::new();
        let mut bonds = Vec::new();
        let mut definition = Vec::new();
        for (frame_index, frame) in rows(&molecule["frames"]).enumerate() {
            let frame_id = id(frame)?;
            let frame_atoms = frame["atoms"]
                .as_array()
                .ok_or_else(|| invalid("NVIS frame atoms"))?;
            let frame_bonds = frame["bonds"]
                .as_array()
                .ok_or_else(|| invalid("NVIS frame bonds"))?;
            if atoms.len() + frame_atoms.len() > COUNT_BOUND
                || bonds.len() + frame_bonds.len() > COUNT_BOUND
            {
                return Err(invalid("NVIS nonperiodic population exceeds bound"));
            }
            for (i, atom) in frame_atoms.iter().enumerate() {
                atoms.push((
                    [frame_index, i],
                    json!({"kind":"molecule-atom","moleculeId":entity("molecule",owner),
                    "frameId":entity("frame",frame_id),"atomId":entity("atom",id(atom)?)}),
                ));
            }
            let mut topology = Vec::new();
            for (i, bond) in frame_bonds.iter().enumerate() {
                let endpoints = bond["atomIds"]
                    .as_array()
                    .ok_or_else(|| invalid("NVIS actual bond endpoints"))?;
                let local = endpoints
                    .iter()
                    .map(|id| {
                        frame_atoms
                            .iter()
                            .position(|atom| atom["id"] == *id)
                            .ok_or_else(|| invalid("NVIS bond endpoint belongs to another frame"))
                    })
                    .collect::<Result<Vec<_>>>()?;
                topology.push(json!({"order":bond["order"],"endpoints":local}));
                bonds.push((
                    [frame_index, i],
                    json!({"kind":"molecule-bond","moleculeId":entity("molecule",owner),
                    "frameId":entity("frame",frame_id),"bondId":entity("bond",id(bond)?)}),
                ));
            }
            definition.push(json!({"frameCode":frame["sourceModelCode"],"atoms":frame_atoms.iter().map(|a|
                json!({"label":a["label"],"cartesianA":a["cartesianA"],"species":a["species"]})).collect::<Vec<_>>(),
                "bonds":topology}));
        }
        if definition.is_empty() {
            return Err(invalid("NVIS actual molecule frame population missing"));
        }
        let digest = digest(&canonical(&json!({"frames":definition})));
        Ok(Self {
            atoms,
            bonds,
            digest,
        })
    }

    fn items(&self, which: Objects) -> &[([usize; 2], Value)] {
        match which {
            Objects::Atoms => &self.atoms,
            Objects::Bonds => &self.bonds,
        }
    }

    fn select(&self, which: Objects, picks: &[[usize; 2]]) -> Result<Value> {
        let mut seen = BTreeSet::new();
        let selections = picks
            .iter()
            .map(|p| {
                if !seen.insert(p) {
                    return Err(invalid("duplicate nonperiodic NVIS selection"));
                }
                let (_, target) = self
                    .items(which)
                    .iter()
                    .find(|(at, _)| at == p)
                    .ok_or_else(|| {
                        invalid("nonperiodic NVIS selection outside actual frame population")
                    })?;
                Ok(json!({"target":target,"applicability":"active"}))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(json!({"kind":"qualified","selections":selections}))
    }
}

fn record(
    molecule: &Value,
    pop: &Population,
    labels: &[[usize; 2]],
    hidden: &[[usize; 2]],
    bonds: &[[usize; 2]],
) -> Result<Value> {
    let absent = |why: &str| json!({"kind":"source-only","nativeIndices":[],"reason":why});
    let sets = [
        pop.select(Objects::Atoms, labels)?,
        absent("molecule-has-no-qualified-morphology-population"),
        pop.select(Objects::Atoms, hidden)?,
        pop.select(Objects::Bonds, bonds)?,
        absent("molecule-has-no-qualified-polyhedron-population"),
    ];
    let owner = id(molecule)?;
    let body = json!({"schemaVersion":"NativeFlagState-proposal-2",
        "owner":{"kind":"molecule","id":entity("molecule",owner)},"observations":[],
        "capturedEnumeration":{"sourceGraphDigest":pop.digest,"populationDigest":pop.digest,
            "recipeDigest":super::native_flags::rule_digest(),"algorithmId":DIALECT,"algorithmVersion":"1"},
        "currentFlagSets":KEYWORDS.into_iter().zip(sets).map(|(key,resolution)|
            json!({"keyword":key,"resolution":resolution})).collect::<Vec<_>>()});
    Ok(
        json!({"owner":{"kind":"molecule","id":owner},"producerId":"draft.native-object-flags","producerVersion":"1",
        "formatRef":"VM18-I01","dialect":DIALECT,"tree":tagged("NVIS2",&body)?,"spans":[]}),
    )
}

pub(super) fn install(inputs: Vec<Input>, doc: &mut Value) -> Result<()> {
    let mut new = Vec::new();
    for input in inputs {
        let mol = &doc["molecules"][input.molecule_index];
        let pop = Population::read(mol)?;
        new.push(record(
            mol,
            &pop,
            &input.labeled_atoms,
            &input.disabled_atoms,
            &input.disabled_bonds,
        )?);
    }
    doc["metadata"]
        .as_array_mut()
        .ok_or_else(|| invalid("native molecule flags metadata"))?
        .extend(new);
    Ok(())
}

pub(super) fn inspect(doc: &Value) -> Result<Vec<Value>> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for row in rows(&doc["metadata"]).filter(|r| r["dialect"] == DIALECT) {
        let owner = row["owner"]["id"]
            .as_str()
            .ok_or_else(|| invalid("NVIS nonperiodic owner id"))?;
        if !seen.insert(owner) {
            return Err(invalid("duplicate nonperiodic NVIS owner"));
        }
        let molecule = rows(&doc["molecules"])
            .find(|m| m["id"] == owner)
            .ok_or_else(|| invalid("NVIS owner no longer resolves to an actual molecule"))?;
        let pop = Population::read(molecule)?;
        let data = payload(&row["tree"])?;
        let recover = |set: usize, which: Objects| -> Result<Vec<[usize; 2]>> {
            let choices = data["currentFlagSets"][set]["resolution"]["selections"]
                .as_array()
                .ok_or_else(|| invalid("NVIS nonperiodic current selection array"))?;
            choices
                .iter()
                .map(|choice| {
                    pop.items(which)
                        .iter()
                        .find(|(_, target)| choice["target"] == *target)
                        .map(|(i, _)| *i)
                        .ok_or_else(|| {
                            invalid("NVIS nonperiodic target kind, frame or population changed")
                        })
                })
                .collect()
        };
        if *row
            != record(
                molecule,
                &pop,
                &recover(0, Objects::Atoms)?,
                &recover(2, Objects::Atoms)?,
                &recover(3, Objects::Bonds)?,
            )?
        {
            return Err(invalid(
                "NVIS complete nonperiodic layer or role mapping differs",
            ));
        }
        out.push(data);
    }
    Ok(out)
}
