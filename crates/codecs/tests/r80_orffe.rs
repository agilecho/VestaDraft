use std::collections::BTreeMap;
use vesta_codecs::formats::resource_graph::{
    CaptureLimits, GraphArchive, discover_graph, import_r80_phase, materialize_r80_orffe,
};
use vesta_model::source_audit::{self, AuditSource, Remap};
use vesta_model::source7::resource_graph::orffe::R80GeometryMeasurement;
use vesta_model::{Ids, PersistentResource, digest};

const PRIMARY: &[u8] = include_bytes!("fixtures/r80/orffe-reopen.vesta");
const TABLE: &[u8] = include_bytes!("fixtures/r80/controlled-identity0.ffe");

fn captured(table: &[u8]) -> GraphArchive {
    let files = BTreeMap::from([
        ("source/orffe-reopen.vesta", PRIMARY),
        ("source/controlled-identity0.ffe", table),
    ]);
    discover_graph(
        "source/orffe-reopen.vesta",
        |key, _| Ok(files.get(key).map(|raw| raw.to_vec())),
        CaptureLimits::default(),
    )
    .unwrap()
}

fn resource(archive: &GraphArchive, ids: &mut Ids) -> PersistentResource {
    let file_index = archive
        .graph
        .files
        .iter()
        .position(|file| file.key == "source/controlled-identity0.ffe")
        .unwrap();
    let context = source_audit::canonical(&serde_json::to_value(&archive.graph).unwrap());
    PersistentResource {
        id: ids.next_id().into(),
        logical_key: format!("source-graph/{}/file/{file_index}", digest(&context)),
        sha256: digest(TABLE),
        byte_length: TABLE.len().to_string(),
        media_type: "application/octet-stream".into(),
    }
}

#[test]
fn exact_r80_phase_and_source_table_yield_ordered_typed_current_images() {
    let archive = captured(TABLE);
    let mut ids = Ids::new([22; 16]);
    let imported = import_r80_phase(&archive, &mut ids).unwrap();
    let phase = imported.phase();
    let resource = resource(&archive, &mut ids);
    let table = materialize_r80_orffe(&archive, &imported, phase, &resource).unwrap();
    assert_eq!(phase.cell.length_a, [10.; 3]);
    assert_eq!(phase.sites.len(), 7);
    assert_eq!(table.owner, phase.id);
    assert_eq!(table.resource, resource.id);
    let [
        R80GeometryMeasurement::Distance {
            images: distance,
            serial: 1,
            result: d,
        },
        R80GeometryMeasurement::Angle {
            images: angle,
            serial: 2,
            result: a,
        },
    ] = table.rows.as_slice()
    else {
        panic!("expected one ordered distance and angle");
    };
    assert_eq!(distance[0].site, phase.sites[0].id);
    assert_eq!(distance[1].site, phase.sites[1].id);
    assert_eq!(angle[0].site, phase.sites[1].id);
    assert_eq!(angle[1].site, phase.sites[0].id);
    assert_eq!(angle[2].site, phase.sites[3].id);
    for image in distance.iter().chain(angle.iter()) {
        assert_eq!(image.phase, phase.id);
        assert_eq!(image.op, phase.symmetry_op_id);
        assert_eq!(image.shift, [0; 3]);
    }
    assert_eq!(d.value_lexeme, "9.8765");
    assert_eq!(d.su_lexeme, "0.0123");
    assert_eq!(d.native_value.to_bits(), 9.8765f32.to_bits());
    assert_eq!(a.value_lexeme, "123.456");
    assert_eq!(a.su_lexeme, "0.0045");
    assert_eq!(a.native_su.to_bits(), 0.0045f32.to_bits());

    let mut fresh = Ids::new([23; 16]);
    let mut mappings: Remap = phase.audit().unwrap().fresh_map(&mut fresh).unwrap();
    mappings.insert(
        ("Resource".into(), resource.id.to_string()),
        fresh.next_id(),
    );
    let restored_table = {
        let mut data = table.clone();
        data.rebind(&mappings).unwrap();
        data
    };
    let R80GeometryMeasurement::Angle { images, .. } = &restored_table.rows[1] else {
        panic!("angle identity");
    };
    assert_eq!(
        images[1].site.as_str(),
        mappings[&("Site".into(), phase.sites[0].id.to_string())]
    );
    assert_ne!(restored_table.owner, table.owner);
    assert_ne!(restored_table.resource, table.resource);
}

#[test]
fn native_open_installs_current_orffe_table_and_reopens_it_from_the_artifact() {
    use serde_json::json;
    use vesta_codecs::{decode, encode_verified, open_selected_orffe};
    use vesta_commands::{EngineV2, Limits};

    let input = captured(TABLE);
    let restored = open_selected_orffe(&input, Ids::new([30; 16])).unwrap();
    let mut engine = EngineV2::loaded(restored, Ids::new([31; 16]), Limits::default()).unwrap();
    let first = engine.active_orffe().unwrap();
    let phase = engine.content().phases[0].clone();
    assert_eq!(first[0].owner, phase.id);
    let at = engine.document();
    let wrong_basis = engine.execute(
        &at,
        &json!({"command":"cell.set","params":{
            "phaseId":phase.id,"cell":{"lengthA":[11.,10.,10.],"anglesDeg":[90.,90.,90.]}
        }}),
    );
    assert_eq!(wrong_basis.unwrap_err().code, "E_REOPEN_OWNER");
    assert_eq!(engine.document(), at);
    assert_eq!(engine.active_orffe().unwrap(), first);
    engine
        .execute(
            &at,
            &json!({"command":"view.setCamera","params":{
                "rotation":[[0,-1,0],[1,0,0],[0,0,1]], "projection":"perspective", "zoom":1.5
            }}),
        )
        .unwrap();
    let artifact = encode_verified(
        &engine.capture().unwrap(),
        "orffe-native.vesta",
        Ids::new([32; 16]),
    )
    .unwrap();
    let restored = decode(
        &artifact.primary,
        &artifact.dependencies,
        Ids::new([33; 16]),
    )
    .unwrap();
    let reopened = EngineV2::loaded(restored, Ids::new([34; 16]), Limits::default()).unwrap();
    let later = reopened.active_orffe().unwrap();
    assert_eq!(later[0].owner, reopened.content().phases[0].id);
    assert_ne!(later[0].owner, first[0].owner);
    assert_ne!(later[0].resource, first[0].resource);
    assert_eq!(later[0].table_sha256, first[0].table_sha256);
    assert_eq!(reopened.content().camera.zoom, 1.5);
    assert!(
        artifact
            .dependencies
            .values()
            .any(|raw| raw.as_slice() == TABLE)
    );
}

#[test]
fn coincident_foreign_owners_changed_basis_and_same_blob_other_use_are_rejected() {
    let archive = captured(TABLE);
    let mut ids = Ids::new([24; 16]);
    let imported = import_r80_phase(&archive, &mut ids).unwrap();
    let foreign = import_r80_phase(&archive, &mut ids).unwrap();
    let mut resource = resource(&archive, &mut ids);
    assert_eq!(
        materialize_r80_orffe(&archive, &imported, foreign.phase(), &resource)
            .unwrap_err()
            .code,
        "E_REOPEN_OWNER"
    );

    let mut altered = imported.phase().clone();
    altered.cell.length_a[0] = 11.;
    assert_eq!(
        materialize_r80_orffe(&archive, &imported, &altered, &resource)
            .unwrap_err()
            .code,
        "E_REOPEN_OWNER"
    );
    altered = imported.phase().clone();
    altered.sites.swap(0, 1);
    assert_eq!(
        materialize_r80_orffe(&archive, &imported, &altered, &resource)
            .unwrap_err()
            .code,
        "E_REOPEN_OWNER"
    );

    resource.logical_key = "source/controlled-identity0.ffe".into();
    assert_eq!(
        materialize_r80_orffe(&archive, &imported, imported.phase(), &resource)
            .unwrap_err()
            .code,
        "E_REOPEN_OWNER"
    );
}

#[test]
fn another_structurally_valid_source_table_does_not_inherit_r80_install_authority() {
    let raw = std::str::from_utf8(TABLE)
        .unwrap()
        .replace("9.8765", "8.8765");
    let archive = captured(raw.as_bytes());
    assert_eq!(
        import_r80_phase(&archive, &mut Ids::new([25; 16]))
            .unwrap_err()
            .code,
        "E_REFERENCE_RULE"
    );
}
