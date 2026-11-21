use serde_json::{Value, json};
use vesta_codecs::formats::live_state::{
    NativeStateInput, inspect_native_state, open_native_state,
};
use vesta_codecs::{decode, encode_verified};
use vesta_commands::{EngineV2, Limits};
use vesta_model::{Cell3, Error, Ids};

const GRID_NAN: u64 = 0x7ff8_0000_0000_0042;
const MATRIX: [&str; 16] = [
    "3ff0000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "3ff0000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "3ff0000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "0000000000000000",
    "3ff0000000000000",
];
fn input() -> Value {
    let z = "0000000000000000";
    let one = "3ff0000000000000";
    let anchor = [z, z, z];
    let phase = json!({"kind":"phase","index":0});
    let grid = |index| json!({"kind":"source-entity","index":index});
    let internal = || json!({"kind":"internal-cartesian","capturedSelector":-1});
    let layer = |owner, position, orientation| {
        json!({
            "owner":owner,"position":{"selector":position,"currentAnchor":anchor,"referenceAnchor":anchor},
            "orientation":{"selector":orientation,"storedFlagBits":"80000005","currentFirst":[one,z,z],
                "referenceFirst":[one,z,z],"currentSecond":[z,z,one],"referenceSecond":[z,z,one]},
            "effectiveMatrixBits":MATRIX,"effectiveTranslationBits":["40e00000","c0000000","40400000"]
        })
    };
    let view = json!({"affine4":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],
        "levels":[{"storedValueF32Bits":"3f800000","signMode":"both"}]});
    json!({
        "schemaVersion":"VestaDraft-native-current-state-1",
        "baseInput":null,
        "grids":[
            {"label":"masked f64","shape":[2,2,2],"encoding":"f64","buffer":"data64.bin","mask":"mask.bin",
             "coordinates":{"kind":"step","originBits":["8000000000000000",z,z],
                 "stepColumnsBits":[[one,z,z],[one,one,z],[z,z,one]]},"views":[view]},
            {"label":"fractional f32","shape":[2,2,2],"encoding":"f32","buffer":"data32.bin","mask":null,
             "coordinates":{"kind":"fractional","divisions":[1,1,1],
                 "rowsBits":[[one,z,z,z],[z,one,z,z],[z,z,one,z]]},"views":[view]},
        ],
        "layers":[layer(grid(0),json!({"kind":"entry","capturedSelector":0,"target":phase}),
            json!({"kind":"entry","capturedSelector":2,"target":grid(1)})),
            layer(phase,internal(),internal()),layer(grid(1),internal(),internal())]
    })
}

fn base() -> vesta_model::Capture {
    EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "Live phase",
        Ids::new([154; 16]),
        Limits::default(),
    )
    .unwrap()
    .capture()
    .unwrap()
}

fn read(path: &str) -> vesta_model::Result<Vec<u8>> {
    match path {
        "data64.bin" => Ok([1f64.to_bits() + 1, 0, 1 << 63, 2, 3, 4, 5, GRID_NAN]
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect()),
        "data32.bin" => Ok((0..8).flat_map(|n| (n as f32).to_le_bytes()).collect()),
        "mask.bin" => Ok(vec![1, 1, 1, 1, 1, 1, 1, 0]),
        _ => Err(Error::new("E_IO", "unknown live test buffer")),
    }
}

#[test]
fn native_state_preserves_masked_f64_and_two_typed_layer_targets_after_write_and_reopen() {
    let state = NativeStateInput::parse(&serde_json::to_vec(&input()).unwrap()).unwrap();
    let installed = open_native_state(state, Some(base()), read, Ids::new([155; 16])).unwrap();
    let mut engine = EngineV2::loaded(installed, Ids::new([156; 16]), Limits::default()).unwrap();
    let before = inspect_native_state(&engine.capture().unwrap())
        .unwrap()
        .unwrap();
    assert_eq!(before["grids"][0]["samples"][7], "7ff8000000000042");
    assert_eq!(before["grids"][0]["samples"][0], "3ff0000000000001");
    assert_eq!(before["grids"][0]["mask"], json!([1, 1, 1, 1, 1, 1, 1, 0]));
    assert_eq!(
        before["layers"][0]["position"]["selector"]["reference"]["target"]["objectKind"],
        "phase"
    );
    assert_eq!(
        before["layers"][0]["orientation"]["selector"]["reference"]["target"]["objectKind"],
        "source-entity"
    );
    engine
        .execute(
            &engine.document(),
            &json!({"command":"view.setCamera","params":{
                "rotation":[[1,0,0],[0,1,0],[0,0,1]],"projection":"parallel","zoom":2.25
            }}),
        )
        .unwrap();
    let bundle = encode_verified(
        &engine.capture().unwrap(),
        "live.vesta",
        Ids::new([157; 16]),
    )
    .unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([158; 16])).unwrap();
    let after = inspect_native_state(&reopened.capture).unwrap().unwrap();
    assert_eq!(reopened.content.camera.zoom, 2.25);
    assert_ne!(before["grids"][0]["ownerId"], after["grids"][0]["ownerId"]);
    assert_eq!(before["grids"][0]["samples"], after["grids"][0]["samples"]);
    assert_eq!(before["grids"][0]["mask"], after["grids"][0]["mask"]);
    assert_eq!(
        before["layers"][0]["effectiveNativeTranslation"],
        after["layers"][0]["effectiveNativeTranslation"]
    );
}

#[test]
fn native_state_rejects_unbound_owner_invalid_mask_and_incomplete_layer_population() {
    let mut stale = input();
    stale["layers"][0]["orientation"]["selector"]["target"]["index"] = json!(4);
    let state = NativeStateInput::parse(&serde_json::to_vec(&stale).unwrap()).unwrap();
    assert!(open_native_state(state, Some(base()), read, Ids::new([159; 16])).is_err());

    let mut incomplete = input();
    incomplete["layers"].as_array_mut().unwrap().pop();
    let state = NativeStateInput::parse(&serde_json::to_vec(&incomplete).unwrap()).unwrap();
    assert!(open_native_state(state, Some(base()), read, Ids::new([160; 16])).is_err());

    let state = NativeStateInput::parse(&serde_json::to_vec(&input()).unwrap()).unwrap();
    let wrong_mask = |path: &str| {
        if path == "mask.bin" {
            Ok(vec![2; 8])
        } else {
            read(path)
        }
    };
    assert!(open_native_state(state, Some(base()), wrong_mask, Ids::new([161; 16])).is_err());
}

#[test]
fn native_current_grid_refuses_same_bytes_in_the_wrong_descriptor_slot() {
    let state = NativeStateInput::parse(&serde_json::to_vec(&input()).unwrap()).unwrap();
    let mut candidate = open_native_state(state, Some(base()), read, Ids::new([162; 16]))
        .unwrap()
        .capture;
    let pointer = vesta_model::source7::native_grid_resource_use(0, "maskResource");
    let prefix = format!(
        "descriptor-data/{}",
        vesta_model::digest(pointer.as_bytes())
    );
    let scope = format!("{prefix}/scope");
    let evidence = format!(
        "{prefix}/target/{}",
        vesta_model::digest(b"/fieldBindings/0/evidence/0/evidenceResource",)
    );
    let rows = &mut candidate.content.resources;
    let a = rows.iter().position(|r| r.logical_key == scope).unwrap();
    let b = rows.iter().position(|r| r.logical_key == evidence).unwrap();
    assert_eq!(rows[a].sha256, rows[b].sha256);
    let id = rows[a].id.clone();
    rows[a].id = rows[b].id.clone();
    rows[b].id = id;
    candidate.state_digest =
        vesta_model::Capture::digest_state(&candidate.content, candidate.source.as_ref()).unwrap();
    let err = encode_verified(&candidate, "invalid.vesta", Ids::new([163; 16])).unwrap_err();
    assert_eq!(err.code, "E_REOPEN_COVERAGE");
}
