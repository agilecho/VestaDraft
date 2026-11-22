use crate::source7::SourceDocument;
use crate::{Cell3, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DocumentAt {
    pub document_id: String,
    pub epoch: String,
    pub revision: String,
}
use crate::source_audit::{self as sa, AuditSource};
use vesta_source_derive::source_model;
source_model! {#[source(owner(Site,id))] pub struct Site {
 #[source(definition(Site))] pub id:sa::Id<sa::Site>,
 pub label:String,pub element:String,pub frac:[f64;3],pub occupancy:f64,pub u_iso_a2:f64,
}}
source_model! {#[source(owner(Phase,id))] pub struct Phase {
 #[source(definition(Phase))] pub id:sa::Id<sa::Phase>,
 pub label:String,pub cell:Cell3,pub sites:Vec<Site>,
 #[source(definition(SymmetryOp))] pub symmetry_op_id:sa::Id<sa::SymmetryOp>,
 pub boundary_min:[f64;3],pub boundary_max:[f64;3],
}}
source_model! {pub struct Camera {pub rotation:[[f64;3];3],pub projection:String,pub zoom:f64}}
source_model! {pub struct ImageKey {
 #[source(reference(Phase))] pub phase:sa::Id<sa::Phase>,
 #[source(reference(Site))] pub site:sa::Id<sa::Site>,
 #[source(reference(SymmetryOp))] pub op:sa::Id<sa::SymmetryOp>,pub shift:[i32;3],
}}
source_model! {#[source(owner(Resource,id))] pub struct PersistentResource {
 #[source(definition(Resource))] pub id:sa::Id<sa::Resource>,
 pub logical_key:String,pub sha256:String,pub byte_length:String,pub media_type:String,
}}
source_model! {pub struct NativeCameraState {
 pub scene_matrix_logical_rows:[[f64;4];4],pub shift_view_x:f64,pub shift_view_y:f64,
 pub third_scene_stored_f32:f64,pub zoom_f32:f64,pub projection_selector_bit21:u32,
 pub projection_z_scale_f32:f64,pub local_translation_a:[f64;3],pub normalization_f64:f64,
 pub viewport_logical_width:u32,pub viewport_logical_height:u32,pub device_scale_f64:f64,
 pub aspect_f32:f64,pub aspect_scale_f32:f64,pub projection_context:String,pub near_view:f64,pub far_view:f64,
}}
source_model! {pub struct LegacyBinding {
 pub original_path:String,
 #[source(reference(Resource))] pub current_resource:sa::Id<sa::Resource>,
}}
source_model! {pub struct LegacyProvenance {
 pub source_type_digest:String,pub source_rule_digest:String,
 #[source(reference(Resource))] pub original_source_document:sa::Id<sa::Resource>,
 pub original_resources:Vec<LegacyBinding>,
}}
source_model! {#[source(owner(Scene,scene_id))] pub struct ProjectContent {
 pub native_camera:Option<NativeCameraState>,pub phases:Vec<Phase>,
 #[source(definition(Scene))] pub scene_id:sa::Id<sa::Scene>,
 pub camera:Camera,pub hidden:Vec<ImageKey>,pub comment:String,pub resources:Vec<PersistentResource>,
 pub legacy_source:Option<LegacyProvenance>,
}}
impl ProjectContent {
    pub fn validate(&self) -> Result<()> {
        use crate::Error;
        use std::collections::BTreeSet;
        if self.phases.len() > 1024 {
            return Err(Error::new("E_RESOURCE_LIMIT", "editable phase count"));
        }
        self.audit()?.definitions()?;
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
            validate_native_camera(
                &serde_json::to_value(v).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?,
            )?;
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
                || crate::json::u64_str(&serde_json::Value::String(v.byte_length.clone())).is_err()
            {
                return Err(Error::new("E_SCHEMA", "persistent resource binding"));
            }
        }
        Ok(())
    }
    pub fn digest(&self) -> String {
        crate::digest(&serde_json::to_vec(self).expect("validated closed model"))
    }
    pub fn fresh_ids(&mut self, ids: &mut crate::Ids) -> Result<BTreeMap<String, String>> {
        let map = self.audit()?.fresh_map(ids)?;
        self.rebind(&map)?;
        self.validate()?;
        Ok(map.into_iter().map(|((_, old), new)| (old, new)).collect())
    }
}
pub fn valid_id(id: &str) -> bool {
    id.len() == 32
        && id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn safe_key(s: &str) -> bool {
    !s.is_empty()
        && s.len() < 512
        && s.is_ascii()
        && s.split('/').all(|c| {
            !c.is_empty()
                && c != "."
                && c != ".."
                && c.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"_- .".contains(&b))
                && !c.contains(' ')
        })
}
pub fn element(s: &str) -> bool {
    "H He Li Be B C N O F Ne Na Mg Al Si P S Cl Ar K Ca Sc Ti V Cr Mn Fe Co Ni Cu Zn Ga Ge As Se Br Kr Rb Sr Y Zr Nb Mo Tc Ru Rh Pd Ag Cd In Sn Sb Te I Xe Cs Ba La Ce Pr Nd Pm Sm Eu Gd Tb Dy Ho Er Tm Yb Lu Hf Ta W Re Os Ir Pt Au Hg Tl Pb Bi Po At Rn Fr Ra Ac Th Pa U Np Pu Am Cm Bk Cf Es Fm Md No Lr Rf Db Sg Bh Hs Mt Ds Rg Cn Nh Fl Mc Lv Ts Og".split_whitespace().any(|e|e==s)
}
pub fn validate_camera(c: &Camera) -> Result<()> {
    use crate::Error;
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
pub fn validate_native_camera(v: &serde_json::Value) -> Result<()> {
    use crate::Error;
    crate::json::schema(v, "sg2.schema.json", "NativeCamera")?;
    if v["projectionContext"] != "explicit-canonical-camera" {
        return Err(Error::new(
            "E_NOT_IMPLEMENTED",
            "camera source context not qualified",
        ));
    }
    for k in [
        "shiftViewX",
        "shiftViewY",
        "thirdSceneStoredF32",
        "zoomF32",
        "projectionZScaleF32",
        "aspectF32",
        "aspectScaleF32",
    ] {
        let x = v[k]
            .as_f64()
            .ok_or_else(|| Error::new("E_SCHEMA", "camera number"))?;
        if (x as f32) as f64 != x {
            return Err(Error::new("E_DOMAIN", "exact promoted f32 required"));
        }
    }
    let w = v["viewportLogicalWidth"]
        .as_u64()
        .ok_or_else(|| Error::new("E_DOMAIN", "viewport"))? as f64;
    let h = v["viewportLogicalHeight"]
        .as_u64()
        .ok_or_else(|| Error::new("E_DOMAIN", "viewport"))? as f64;
    if v["aspectF32"].as_f64() != Some((w / h) as f32 as f64)
        || v["aspectScaleF32"].as_f64() != Some((if w <= h { 1. } else { h / w }) as f32 as f64)
        || v["nearView"].as_f64() >= v["farView"].as_f64()
    {
        return Err(Error::new("E_DOMAIN", "camera derived context mismatch"));
    }
    Ok(())
}
#[derive(Clone, Debug)]
pub struct Capture {
    pub document: DocumentAt,
    pub content: ProjectContent,
    pub resources: BTreeMap<String, Vec<u8>>,
    pub state_digest: String,
    pub source: Option<SourceDocument>,
}

impl Capture {
    pub fn digest_state(
        content: &ProjectContent,
        source: Option<&SourceDocument>,
    ) -> Result<String> {
        let source_digest = source
            .map(|tree| -> Result<String> { Ok(crate::digest(&sa::canonical(&tree.to_value()?))) })
            .transpose()?;
        Ok(Self::digest_parts(content, source_digest.as_deref()))
    }

    pub fn digest_parts(content: &ProjectContent, source_digest: Option<&str>) -> String {
        let editor = content.digest();
        source_digest.map_or(editor.clone(), |source| {
            crate::digest(&sa::canonical(
                &serde_json::json!({"editor":editor,"typedSource":source}),
            ))
        })
    }
}

#[derive(Clone, Debug)]
pub struct RestoredProject {
    pub capture: Capture,
    pub next_ids: crate::Ids,
    pub identity_map: sa::Remap,
}
impl RestoredProject {
    pub fn new(mut capture: Capture, mut ids: crate::Ids) -> Result<Self> {
        capture.content.validate()?;
        let mut retired = BTreeSet::from([
            capture.document.document_id.clone(),
            capture.document.epoch.clone(),
        ]);
        let core = capture.content.audit()?.definitions()?;
        let source = capture
            .source
            .as_ref()
            .map(|source| source.audit()?.definitions())
            .transpose()?
            .unwrap_or_default();
        let mut kinds = BTreeMap::new();
        for (kind, id) in core.keys().chain(source.keys()) {
            if kinds
                .insert(id, kind)
                .is_some_and(|original| original != kind)
            {
                return Err(crate::Error::new(
                    "E_REFERENCE_FAILURE",
                    "identity kind differs in editor and source",
                ));
            }
            retired.insert(id.clone());
        }
        let mut allocate = || loop {
            let id = ids.next_id();
            if retired.insert(id.clone()) {
                break id;
            }
        };
        let keys: BTreeSet<_> = core.keys().chain(source.keys()).cloned().collect();
        let map: sa::Remap = keys.into_iter().map(|key| (key, allocate())).collect();
        let saved_document = ("Document".into(), capture.document.document_id.clone());
        let saved_epoch = ("Epoch".into(), capture.document.epoch.clone());
        let document_id = map
            .get(&saved_document)
            .cloned()
            .unwrap_or_else(&mut allocate);
        let epoch = map.get(&saved_epoch).cloned().unwrap_or_else(&mut allocate);
        if let Some(source) = &mut capture.source {
            source.rebind(&map)?;
        }
        capture.content.rebind(&map)?;
        capture.content.validate()?;
        capture.document = DocumentAt {
            document_id,
            epoch,
            revision: "0".into(),
        };
        capture.state_digest = Capture::digest_state(&capture.content, capture.source.as_ref())?;
        Ok(Self {
            capture,
            next_ids: ids,
            identity_map: map,
        })
    }
}

impl std::ops::Deref for RestoredProject {
    type Target = Capture;
    fn deref(&self) -> &Capture {
        &self.capture
    }
}
