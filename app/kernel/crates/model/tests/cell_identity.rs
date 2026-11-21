use vesta_model::{Cell3, ResourceStore};
#[test]
fn volume_and_invalid_geometry() {
    let c = Cell3 {
        length_a: [4.; 3],
        angles_deg: [90.; 3],
    };
    assert_eq!(c.volume().unwrap(), 64.);
    let bad = Cell3 {
        length_a: [-1., 4., 4.],
        angles_deg: [90.; 3],
    };
    assert!(bad.validate().is_err());
    let flat = Cell3 {
        length_a: [4.; 3],
        angles_deg: [10., 10., 170.],
    };
    assert!(flat.volume().is_err());
}
#[test]
fn resource_bytes_require_actual_owner() {
    let mut r = ResourceStore::default();
    let id = r
        .put(b"immutable source".to_vec(), "session-a", 1024)
        .unwrap();
    assert_eq!(r.read(&id, "session-a").unwrap(), b"immutable source");
    assert!(r.read(&id, "session-b").is_err());
}
