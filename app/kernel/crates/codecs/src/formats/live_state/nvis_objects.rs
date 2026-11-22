//! Selected current R80 P1 render population. Historical legacy flag
//! ordinals are never used to generate these targets.
use super::{entity, invalid};
use serde::Deserialize;
use serde_json::{Value, json};
use vesta_model::{Ids, Phase, Result, digest, source_audit::canonical};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ObjectInput {
    morphology_box_a: [[f64; 3]; 2],
    pub(super) labeled_face_indices: Vec<usize>,
    pub(super) disabled_bond_indices: Vec<usize>,
    pub(super) disabled_polyhedron_indices: Vec<usize>,
}

pub(super) struct Populations {
    faces: Vec<Target>,
    bonds: Vec<Target>,
    polyhedra: Vec<Target>,
    boundary_id: String,
    geometry: Value,
}

pub(super) struct Target {
    pub(super) address: Value,
    pub(super) active: bool,
}

const AXES: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn rows(v: &Value) -> impl Iterator<Item = &Value> {
    v.as_array().into_iter().flatten()
}

fn checked_bounds(bounds: [[f64; 3]; 2]) -> Result<()> {
    if (0..3).any(|i| {
        !bounds[0][i].is_finite()
            || !bounds[1][i].is_finite()
            || bounds[0][i] >= bounds[1][i]
            || bounds[0][i].abs().max(bounds[1][i].abs()) > 1e6
    }) {
        return Err(invalid("NVIS positive finite Cartesian box"));
    }
    Ok(())
}

fn rule(owner: &str, id: &str) -> Value {
    json!({"id":id,"owner":{"kind":"phase","id":owner},"value":{"id":id,
        "fromSelector":{"kind":"element","element":"Si"},"toSelector":{"kind":"element","element":"O"},
        "minimumA":1.9,"maximumA":2.1,"thresholdMode":"legacy-f32-square","extraAtoms":"none",
        "maxDepth":0,"maxImages":7,"bondValenceTable":null},"metadata":[]})
}

fn boundary(phase: &Phase, id: &str, rule: &str) -> Value {
    json!({"id":id,"owner":{"kind":"phase","id":phase.id},"value":{"phaseId":phase.id,
        "minimumFractional":phase.boundary_min,"maximumFractional":phase.boundary_max,
        "endpoints":"closed","includeBondExtensions":false,"ruleIds":[rule]},"metadata":[]})
}

fn check_recipe(phase: &Phase, raw: &[u8]) -> Result<Vec<usize>> {
    let raw =
        std::str::from_utf8(raw).map_err(|_| invalid("NVIS validated primary is not text"))?;
    let tokens = raw
        .split_once("\nSBOND\n")
        .and_then(|(_, body)| body.lines().next())
        .ok_or_else(|| invalid("NVIS selected primary has no bond recipe"))?;
    if !tokens.split_ascii_whitespace().eq([
        "1", "Si", "O", "1.90000", "2.10000", "0", "1", "1", "0", "1", "0.250", "2.000", "127",
        "127", "127",
    ]) || phase.cell.length_a != [10.; 3]
        || phase.cell.angles_deg != [90.; 3]
        || phase.boundary_min.iter().any(|&v| v < 0.)
        || phase.boundary_max.iter().any(|&v| v > 1.)
        || phase.sites.len() != 7
        || phase.sites[0].element != "Si"
        || phase.sites[0].frac != [0.5; 3]
    {
        return Err(invalid("NVIS selected P1 bond and boundary recipe"));
    }
    // All eligible images in the supported boundary remain origin instances.
    // The full octahedral addresses survive even when the current boundary
    // clips members. Neither ORFFE values nor archived ordinals define a bond.
    let mut signs = Vec::new();
    for site in &phase.sites[1..] {
        let delta = site.frac.map(|value| (value - 0.5) * 10.0);
        let sign = std::array::from_fn::<_, 3, _>(|i| {
            if delta[i] > 0.0 {
                1
            } else if delta[i] < 0.0 {
                -1
            } else {
                0
            }
        });
        let squared = delta.iter().map(|d| d * d).sum::<f64>();
        if site.element != "O"
            || !AXES.contains(&sign)
            || signs.contains(&sign)
            || !(1.9f64.powi(2)..=2.1f64.powi(2)).contains(&squared)
        {
            return Err(invalid(
                "NVIS current octahedral neighbor population differs",
            ));
        }
        signs.push(sign);
    }
    Ok((1..phase.sites.len()).collect())
}

pub(super) fn install(
    input: &ObjectInput,
    phase: &Phase,
    raw: &[u8],
    source: &mut Value,
    ids: &mut Ids,
) -> Result<()> {
    checked_bounds(input.morphology_box_a)?;
    check_recipe(phase, raw)?;
    let owner = phase.id.as_str();
    let node = source["phaseGraph"]["phases"]
        .as_array_mut()
        .and_then(|rows| rows.iter_mut().find(|item| item["id"] == owner))
        .ok_or_else(|| invalid("NVIS actual phase node"))?;
    if !node["shapeId"].is_null() {
        return Err(invalid(
            "NVIS box constructor requires no existing phase shape",
        ));
    }
    let shape = ids.next_id();
    node["shapeId"] = json!(shape);
    let [lo, hi] = input.morphology_box_a;
    let halfspaces = AXES
        .iter()
        .enumerate()
        .map(|(n, normal)| {
            let offset = if n % 2 == 0 { hi[n / 2] } else { -lo[n / 2] };
            json!({"id":ids.next_id(),"normal":normal,"offsetA":offset})
        })
        .collect::<Vec<_>>();
    let mut push = |field: &str, row: Value| -> Result<()> {
        source[field]
            .as_array_mut()
            .ok_or_else(|| invalid(format!("NVIS {field} population")))?
            .push(row);
        Ok(())
    };
    push(
        "shapes",
        json!({"id":shape,"owner":{"kind":"phase","id":owner},
        "value":{"id":shape,"phaseId":owner,"halfspaces":halfspaces},"metadata":[]}),
    )?;
    let bond = ids.next_id();
    push("bondRules", rule(owner, &bond))?;
    push("boundaries", boundary(phase, &ids.next_id(), &bond))?;
    Ok(())
}

pub(super) fn capture(phase: &Phase, raw: &[u8], source: &Value) -> Result<Option<Populations>> {
    let owner = phase.id.as_str();
    let phase_node = rows(&source["phaseGraph"]["phases"])
        .find(|n| n["id"] == owner)
        .ok_or_else(|| invalid("NVIS actual phase missing"))?;
    let Some(shape_id) = phase_node["shapeId"].as_str() else {
        return Ok(None);
    };
    let vertices = check_recipe(phase, raw)?;
    let ownership = json!({"kind":"phase","id":owner});
    let shape = rows(&source["shapes"])
        .find(|row| row["id"] == shape_id && row["owner"] == ownership)
        .ok_or_else(|| invalid("NVIS current shape ownership"))?;
    let flags = shape["value"]["halfspaces"]
        .as_array()
        .filter(|r| r.len() == 6)
        .ok_or_else(|| invalid("NVIS actual box halfspaces"))?;
    let mut bounds = [[0.; 3]; 2];
    let mut faces = Vec::new();
    for (i, (halfspace, normal)) in flags.iter().zip(AXES).enumerate() {
        if halfspace["normal"] != json!(normal) {
            return Err(invalid("NVIS oriented face order differs"));
        }
        let offset = halfspace["offsetA"]
            .as_f64()
            .ok_or_else(|| invalid("NVIS face offset"))?;
        let id = halfspace["id"]
            .as_str()
            .ok_or_else(|| invalid("NVIS actual halfspace id"))?;
        bounds[1 - i % 2][i / 2] = if i % 2 == 0 { offset } else { -offset };
        let signature = digest(&canonical(
            &json!({"normal":normal,"offsetA":offset,"operation":"identity-P1"}),
        ));
        faces.push(Target { address: json!({"kind":"morphology-face","shapeId":entity("shape",shape_id),
            "halfspaceId":entity("shape-halfspace",id),"operationId":entity("symmetry-operation",phase.symmetry_op_id.as_str()),
            "orientedFaceSignature":signature}), active: true });
    }
    checked_bounds(bounds)?;
    let rules = rows(&source["bondRules"])
        .filter(|r| r["owner"] == ownership)
        .collect::<Vec<_>>();
    let [rule_row] = rules.as_slice() else {
        return Err(invalid("NVIS requires the complete selected bond rule set"));
    };
    let rule_id = rule_row["id"]
        .as_str()
        .ok_or_else(|| invalid("NVIS bond rule id"))?;
    if **rule_row != rule(owner, rule_id) {
        return Err(invalid("NVIS actual bond recipe differs"));
    }
    let extents = rows(&source["boundaries"])
        .filter(|r| r["owner"] == ownership)
        .collect::<Vec<_>>();
    let [extent] = extents.as_slice() else {
        return Err(invalid("NVIS selected polyhedron boundary"));
    };
    let boundary_id = extent["id"]
        .as_str()
        .ok_or_else(|| invalid("NVIS boundary id"))?;
    let mut expected = boundary(phase, boundary_id, rule_id);
    // The previous typed boundary can precede an actual Engine boundary edit.
    // Its identity and recipe must still agree with the installed definitions.
    for key in ["minimumFractional", "maximumFractional"] {
        expected["value"][key] = extent["value"][key].clone();
    }
    if **extent != expected {
        return Err(invalid("NVIS actual boundary recipe differs"));
    }
    let origin = |i: usize| super::native_flags::image_ref(phase, i);
    let from = origin(0)?;
    let corner = vertices
        .iter()
        .map(|&i| origin(i))
        .collect::<Result<Vec<_>>>()?;
    let visible = |index| super::native_flags::active(phase, index);
    let bonds = corner
        .iter()
        .zip(&vertices)
        .map(|(to, &i)| Target {
            address: json!({"kind":"bond","ruleId":entity("bond-rule",rule_id),"images":[from,to]}),
            active: visible(0) && visible(i),
        })
        .collect();
    let polyhedra = vec![
        Target {
            address: json!({"kind":"polyhedron","boundaryRuleId":entity("boundary",boundary_id),
                "center":from,"vertices":corner}),
            active: visible(0) && vertices.iter().copied().all(visible),
        },
    ];
    Ok(Some(Populations {
        faces,
        bonds,
        polyhedra,
        boundary_id: boundary_id.into(),
        geometry: json!({"box":bounds,"bondEndpoints":vertices,
        "center":0,"shapeOperations":"identity-P1"}),
    }))
}

impl Populations {
    pub(super) fn sync_boundary(&self, phase: &Phase, source: &mut Value) -> Result<()> {
        let row = source["boundaries"].as_array_mut()
            .and_then(|rows| rows.iter_mut().find(|row| row["id"] == self.boundary_id))
            .ok_or_else(|| invalid("NVIS installed boundary lost its identity"))?;
        row["value"]["minimumFractional"] = json!(phase.boundary_min);
        row["value"]["maximumFractional"] = json!(phase.boundary_max);
        Ok(())
    }

    pub(super) fn geometry(&self) -> &Value {
        &self.geometry
    }

    pub(super) fn for_key(&self, key: &str) -> &[Target] {
        match key {
            "LBLSP" => &self.faces,
            "DLBND" => &self.bonds,
            "DLPLY" => &self.polyhedra,
            _ => &[],
        }
    }
}
