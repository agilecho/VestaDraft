use vesta_commands::digest;
use vesta_commands::field::{
    AxisOrder, FieldBuffer, FieldDescriptor, FieldKind, FieldResource, FieldUnit, SampleType,
};

fn descriptor(bytes: &[u8]) -> FieldDescriptor {
    FieldDescriptor {
        resource: FieldResource {
            digest: digest(bytes),
            bytes: bytes.len() as u64,
        },
        kind: FieldKind::ElectronDensity,
        unit: FieldUnit::EPerA3,
        declared_unit: None,
        core_shape: [2, 2, 2],
        closure: [true, false, false],
        periodic: [true, false, false],
        axis_order: AxisOrder::XFastest,
        origin_a: [0.25, 0., 1.5],
        dtype: SampleType::F32,
        step_basis_a: [[1., 0., 0.], [0.5, 2., 0.], [0., 0., 3.]],
    }
}

fn closed_samples() -> Vec<u8> {
    [-0.0f32, 1., -0.0, 2., 3., 2., 4., 5., 4., 6., 7., 6.]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect()
}

#[test]
fn x_fastest_closure_compares_every_sample_and_keeps_negative_zero() {
    let raw = closed_samples();
    let desc = descriptor(&raw);
    let stored = FieldBuffer::open(&desc, &raw).unwrap();
    assert_eq!(stored.stored_shape(), [3, 2, 2]);
    assert_eq!(stored.sample_count(), 12);
    assert_eq!(
        stored.sample_bits([2, 0, 0]),
        Some(u64::from(0x8000_0000u32))
    );
    assert_eq!(
        stored.sample_bits([1, 1, 1]),
        Some(u64::from(7f32.to_bits()))
    );
    assert_eq!(stored.sample_bits([3, 0, 0]), None);
}

#[test]
fn a_self_consistently_rehashed_bad_closure_is_still_invalid() {
    let mut raw = closed_samples();
    raw[8..12].copy_from_slice(&0f32.to_le_bytes());
    let err = FieldBuffer::open(&descriptor(&raw), &raw).unwrap_err();
    assert_eq!(err.code, "E_FIELD_CLOSURE");
}

#[test]
fn z_fastest_f64_checks_face_edge_and_corner_in_storage_order() {
    let raw: Vec<_> = (0..3u32)
        .flat_map(|x| {
            (0..3u32).flat_map(move |y| {
                (0..4u32).map(move |z| ((x % 2) * 100 + (y % 2) * 10 + z % 3) as f64)
            })
        })
        .flat_map(f64::to_le_bytes)
        .collect();
    let mut desc = descriptor(&raw);
    desc.core_shape = [2, 2, 3];
    desc.axis_order = AxisOrder::ZFastest;
    desc.dtype = SampleType::F64;
    desc.closure = [true, true, true];
    desc.periodic = [true, true, true];
    let stored = FieldBuffer::open(&desc, &raw).unwrap();
    assert_eq!(stored.stored_shape(), [3, 3, 4]);
    assert_eq!(stored.sample_bits([2, 2, 3]), Some(0f64.to_bits()));
    assert_eq!(stored.sample_bits([1, 2, 2]), Some(102f64.to_bits()));
}

#[test]
fn invalid_field_geometry_count_and_quantity_fail_before_install() {
    let raw = closed_samples();
    let mut desc = descriptor(&raw);
    desc.periodic[0] = false;
    assert_eq!(
        FieldBuffer::open(&desc, &raw).unwrap_err().code,
        "E_FIELD_CLOSURE"
    );
    desc.periodic[0] = true;
    desc.core_shape[1] = u64::MAX;
    assert_eq!(
        FieldBuffer::open(&desc, &raw).unwrap_err().code,
        "E_FIELD_STORAGE"
    );
    desc.core_shape[1] = 2;
    desc.unit = FieldUnit::FmPerA3;
    assert_eq!(
        FieldBuffer::open(&desc, &raw).unwrap_err().code,
        "E_FIELD_DESCRIPTOR"
    );
    desc.unit = FieldUnit::EPerA3;
    desc.step_basis_a[2] = [0., 0., 0.];
    assert_eq!(
        FieldBuffer::open(&desc, &raw).unwrap_err().code,
        "E_FIELD_DESCRIPTOR"
    );
    let mut bad = raw.clone();
    bad[4..8].copy_from_slice(&f32::INFINITY.to_le_bytes());
    assert_eq!(
        FieldBuffer::open(&descriptor(&bad), &bad).unwrap_err().code,
        "E_FIELD_VALUE"
    );
}
