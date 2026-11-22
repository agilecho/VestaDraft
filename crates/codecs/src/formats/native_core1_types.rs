//! Read-only, version-pinned Core1 preimage types. Never used by the current writer.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub use vesta_model::DocumentAt;
pub use vesta_model::digest;
pub use vesta_model::{Cell3, Error, Result, element, safe_key, valid_id, validate_native_camera};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Site {
    pub id: String,
    pub label: String,
    pub element: String,
    pub frac: [f64; 3],
    pub occupancy: f64,
    pub u_iso_a2: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Phase {
    pub id: String,
    pub label: String,
    pub cell: Cell3,
    pub sites: Vec<Site>,
    pub symmetry_op_id: String,
    pub boundary_min: [f64; 3],
    pub boundary_max: [f64; 3],
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Camera {
    pub rotation: [[f64; 3]; 3],
    pub projection: String,
    pub zoom: f64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageKey {
    pub phase: String,
    pub site: String,
    pub op: String,
    pub shift: [i32; 3],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PersistentResource {
    pub logical_key: String,
    pub sha256: String,
    pub byte_length: String,
    pub media_type: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProjectContent {
    pub native_camera: Option<serde_json::Value>,
    pub phases: Vec<Phase>,
    pub scene_id: String,
    pub camera: Camera,
    pub hidden: Vec<ImageKey>,
    pub comment: String,
    pub resources: Vec<PersistentResource>,
}
impl ProjectContent {
    pub fn validate(&self) -> Result<()> {
        use std::collections::BTreeSet;
        use vesta_model::Error;
        if self.phases.len() != 1 {
            return Err(Error::new(
                "E_NOT_IMPLEMENTED",
                "P2-03 supports one P1 periodic phase",
            ));
        }
        let mut ids = BTreeSet::new();
        for id in std::iter::once(&self.scene_id).chain(self.phases.iter().flat_map(|p| {
            std::iter::once(&p.id)
                .chain(std::iter::once(&p.symmetry_op_id))
                .chain(p.sites.iter().map(|s| &s.id))
        })) {
            if !valid_id(id) || !ids.insert(id) {
                return Err(Error::new("E_SCHEMA", "duplicate/invalid object identity"));
            }
        }
        for p in &self.phases {
            p.cell.volume()?;
            if p.label.contains(['\n', '\r', '\0']) || p.sites.len() > 100000 {
                return Err(Error::new("E_RESOURCE_LIMIT", "phase label/population"));
            }
            if p.boundary_min
                .iter()
                .chain(p.boundary_max.iter())
                .any(|v| !v.is_finite())
                || (0..3).any(|i| p.boundary_min[i] >= p.boundary_max[i])
            {
                return Err(Error::new("E_DOMAIN", "invalid boundary"));
            }
            for s in &p.sites {
                if !element(&s.element)
                    || s.label.is_empty()
                    || s.label.contains(['\n', '\r', '\0'])
                    || s.frac.iter().any(|v| !v.is_finite())
                    || !s.occupancy.is_finite()
                    || !(0.0..=1.0).contains(&s.occupancy)
                    || !s.u_iso_a2.is_finite()
                    || s.u_iso_a2 < 0.
                {
                    return Err(Error::new("E_DOMAIN", "invalid atom site"));
                }
            }
        }
        validate_camera(&self.camera)?;
        if let Some(v) = &self.native_camera {
            validate_native_camera(v)?;
        }
        let mut hidden = BTreeSet::new();
        for k in &self.hidden {
            let p = self
                .phases
                .iter()
                .find(|p| p.id == k.phase)
                .ok_or_else(|| Error::new("E_OBJECT_NOT_FOUND", "hidden phase"))?;
            if p.symmetry_op_id != k.op
                || !p.sites.iter().any(|s| s.id == k.site)
                || !hidden.insert(
                    serde_json::to_string(k).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?,
                )
            {
                return Err(Error::new("E_OBJECT_NOT_FOUND", "hidden image identity"));
            }
        }
        if self.resources.len() > 1024 || self.comment.len() > 1048576 {
            return Err(Error::new("E_RESOURCE_LIMIT", "project resources/comment"));
        }
        let mut keys = BTreeSet::new();
        for v in &self.resources {
            if !safe_key(&v.logical_key)
                || !keys.insert(&v.logical_key)
                || v.sha256.len() != 64
                || !v
                    .sha256
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                || vesta_model::json::u64_str(&serde_json::Value::String(v.byte_length.clone()))
                    .is_err()
            {
                return Err(Error::new("E_SCHEMA", "persistent resource binding"));
            }
        }
        Ok(())
    }
    pub fn digest(&self) -> String {
        vesta_model::digest(&serde_json::to_vec(self).expect("validated closed model"))
    }
}

pub fn validate_camera(c: &Camera) -> Result<()> {
    use vesta_model::Error;
    if c.rotation.iter().flatten().any(|x| !x.is_finite())
        || !c.zoom.is_finite()
        || c.zoom <= 0.
        || !["parallel", "perspective"].contains(&c.projection.as_str())
    {
        return Err(Error::new("E_DOMAIN", "invalid camera"));
    }
    for i in 0..3 {
        for j in 0..3 {
            let d = (0..3)
                .map(|k| c.rotation[i][k] * c.rotation[j][k])
                .sum::<f64>();
            if (d - if i == j { 1. } else { 0. }).abs() > 1e-9 {
                return Err(Error::new("E_DOMAIN", "rotation must be orthonormal"));
            }
        }
    }
    let m = c.rotation;
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    if (det - 1.).abs() > 1e-9 {
        return Err(Error::new("E_DOMAIN", "proper rotation required"));
    }
    Ok(())
}
#[derive(Clone, Debug)]
pub struct Capture {
    pub document: DocumentAt,
    pub content: ProjectContent,
    pub resources: BTreeMap<String, Vec<u8>>,
    pub state_digest: String,
}
