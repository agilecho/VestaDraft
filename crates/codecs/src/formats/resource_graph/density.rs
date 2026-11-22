//! Qualification for the captured Model Electron Densities / VS-0005 pair.
use super::{
    CaptureLimits, GraphArchive, r80::import_selected_p1, selected, verify_artifact_graph,
};
use vesta_model::field::{
    AxisOrder, FieldDescriptor, FieldKind, FieldResource, FieldUnit, SampleType,
};
use vesta_model::source7::resource_graph::EdgeKind;
use vesta_model::{Error, Ids, Phase, Result, digest};

const C1_DENSITY_GRID: &str = "e59640b2ff92e330d123b74ce1001a221a128a46c0bc9ec1c949195f706b6eb2";

pub fn is_c1_density_source(raw: &[u8]) -> bool {
    digest(raw) == selected::C1_DENSITY_PRIMARY
}

pub(crate) fn read_c1_density<'a>(
    archive: &'a GraphArchive,
    ids: &mut Ids,
) -> Result<(Phase, FieldDescriptor, &'a [u8])> {
    let invalid = || Error::new("E_REFERENCE_RULE", "unqualified C1 source density pair");
    verify_artifact_graph(&archive.graph, &archive.files, CaptureLimits::default())?;
    let root = &archive.graph.root_key;
    let [edge] = archive.graph.edges.as_slice() else {
        return Err(invalid());
    };
    let primary = archive.files.get(root).ok_or_else(invalid)?;
    let grid = archive.files.get(&edge.to_key).ok_or_else(invalid)?;
    if !is_c1_density_source(primary)
        || digest(grid) != C1_DENSITY_GRID
        || edge.kind != EdgeKind::DensityImport
        || edge.from_key != *root
        || edge.recursive_decode
    {
        return Err(invalid());
    }
    let phase = import_selected_p1(selected::ascii(primary, root)?, 1, "THERI 1", ids)?;
    let header = |at| -> [u8; 4] {
        grid[at..at + 4]
            .try_into()
            .expect("verified Grid 3.0 header")
    };
    let shape = [112, 116, 120].map(|at| u32::from_le_bytes(header(at)) as u64);
    let length = [128, 132, 136].map(|at| f64::from(f32::from_le_bytes(header(at))));
    let angles = [140, 144, 148].map(|at| f64::from(f32::from_le_bytes(header(at))));
    if length != phase.cell.length_a || angles != phase.cell.angles_deg || angles != [90.; 3] {
        return Err(Error::new(
            "E_REOPEN_OWNER",
            "grid basis differs from imported C1 phase",
        ));
    }
    let mut steps = [[0.; 3]; 3];
    for axis in 0..3 {
        steps[axis][axis] = length[axis] / shape[axis] as f64;
    }
    let data = &grid[152..];
    let descriptor = FieldDescriptor {
        resource: FieldResource {
            digest: digest(data),
            bytes: data.len() as u64,
        },
        kind: FieldKind::ElectronDensity,
        unit: FieldUnit::EPerA3,
        declared_unit: None,
        core_shape: shape,
        closure: [false; 3],
        periodic: [true; 3],
        axis_order: AxisOrder::XFastest,
        origin_a: [0.; 3],
        dtype: SampleType::F32,
        step_basis_a: steps,
    };
    Ok((phase, descriptor, data))
}
