use super::*;
use crate::{decode, encode_verified};
use vesta_commands::{EngineV2, Limits};

#[test]
fn gsas_original_chart_keeps_center_first_order_and_magnetic_character() {
    let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../output/vesta-p2-03/20261122-c8-gsas-chart-static/fixtures");
    let raw = std::fs::read(data.join("authored.spin-chart.json")).unwrap();
    let exp = std::fs::read(data.join("authored.exp")).unwrap();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(data.join("authored.expected.json")).unwrap())
            .unwrap();
    let capture = open(&exp, Ids::new([66; 16])).unwrap().capture;
    let source = capture.source.unwrap().to_value().unwrap();
    let items = &source["metadata"][0]["tree"]["members"]["phases"]["items"];
    let words = items
        .as_array()
        .unwrap()
        .iter()
        .map(|phase| &phase["members"])
        .find(|phase| phase["phaseNumber"]["value"] == "3")
        .unwrap()["spinFlipWords"]["items"]
        .as_array()
        .unwrap();
    let signs: [i8; 12] = std::array::from_fn(|i| {
        let value = words[i]["value"]
            .as_str()
            .unwrap()
            .trim()
            .parse::<f32>()
            .unwrap();
        if value == -1. { -1 } else { 1 }
    });
    let rows = chart::Chart::parse(&raw).unwrap().expand(signs).unwrap();
    let expected_rows = expected["orderedOperations"].as_array().unwrap();
    assert_eq!(rows.len(), expected_rows.len());
    for ((op, sign), expected) in rows.iter().zip(expected_rows) {
        assert_eq!(json!(op.rotation), expected["rotation"]);
        assert_eq!(json!(op.translation), expected["translation"]);
        assert_eq!(json!(sign), expected["timeSign"]);
    }
    let mut wrong: Value = serde_json::from_slice(&raw).unwrap();
    wrong["baseOperators"][3]["generatorCodeMask"] = json!(0);
    assert!(
        chart::Chart::parse(&serde_json::to_vec(&wrong).unwrap())
            .unwrap()
            .expand(signs)
            .is_err()
    );
}

#[test]
fn gsas_qualified_control_installs_and_reopens_current_magnetic_state() {
    let raw = magnetic::control_exp();
    let opened = open(raw, Ids::new([71; 16])).unwrap();
    let mut engine = EngineV2::loaded(opened, Ids::new([72; 16]), Limits::default()).unwrap();
    let capture = engine.capture().unwrap();
    let before = capture.source.as_ref().unwrap().to_value().unwrap();
    let inspected = inspect_magnetic(&capture).unwrap().unwrap();
    assert_eq!(inspected["phaseGraph"], before["phaseGraph"]);
    assert_eq!(inspected["vectors"], before["vectors"]);
    let phase = &before["phaseGraph"]["phases"][0];
    let operators = phase["structure"]["group"]["operators"].as_array().unwrap();
    assert_eq!(phase["structure"]["group"]["kind"], "magnetic");
    assert_eq!(operators.len(), 2);
    assert_eq!(operators[0]["timeSign"], 1);
    assert_eq!(operators[1]["timeSign"], -1);
    assert_eq!(operators[1]["fractionalMatrix"][0][0]["numerator"], -1);
    let projection = before["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["tree"]["tag"] == magnetic::TAG)
        .unwrap();
    assert_eq!(
        projection["tree"]["payload"]["members"]["sourceFlagF32Bits"]["items"][3]["value"],
        "80000000"
    );
    let vectors = before["vectors"].as_array().unwrap();
    assert_eq!(vectors.len(), 2);
    for (index, (parts, shift)) in [([-2., 1., 3.], [0, 0, 0]), ([-3., -2., 1.], [1, -1, 0])]
        .into_iter()
        .enumerate()
    {
        let vector = &vectors[index];
        assert_eq!(vector["owner"]["id"], phase["id"]);
        assert_eq!(vector["value"]["anchor"]["operation"], operators[0]["id"]);
        assert_eq!(
            vector["value"]["anchor"]["site"],
            phase["structure"]["sites"][index]["id"]
        );
        assert_eq!(vector["value"]["anchor"]["shift"], json!(shift));
        assert_eq!(vector["value"]["components"], json!(parts));
        assert_eq!(vector["value"]["unit"], "muB");
    }
    assert!(engine.content().phases.is_empty());
    let mut edited = before.clone();
    edited["vectors"][0]["value"]["components"] = json!([1.25, 2., 3.]);
    let capture_with = |value| {
        let mut actor = capture.clone();
        actor.source = Some(SourceDocument::decode(value).unwrap());
        actor.state_digest = Capture::digest_state(&actor.content, actor.source.as_ref()).unwrap();
        actor
    };
    let changed = capture_with(edited);
    let bundle = encode_verified(&changed, "magnetic.vesta", Ids::new([73; 16])).unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([74; 16])).unwrap();
    let after = reopened
        .capture
        .source
        .as_ref()
        .unwrap()
        .to_value()
        .unwrap();
    let new = &after["phaseGraph"]["phases"][0];
    assert_ne!(new["id"], phase["id"]);
    assert_ne!(
        new["structure"]["group"]["operators"][0]["id"],
        operators[0]["id"]
    );
    assert_ne!(
        new["structure"]["sites"][0]["id"],
        phase["structure"]["sites"][0]["id"]
    );
    assert_ne!(after["vectors"][0]["id"], vectors[0]["id"]);
    assert_eq!(
        after["vectors"][0]["value"]["components"],
        json!([1.25, 2., 3.])
    );
    assert_eq!(after["vectors"][0]["value"]["anchor"]["phase"], new["id"]);
    assert!(capture.content.resources.iter().all(|old| {
        reopened.content.resources.iter().any(|new| {
            new.id != old.id
                && new.logical_key == old.logical_key
                && new.sha256 == old.sha256
                && new.byte_length == old.byte_length
        })
    }));
    assert_eq!(reopened.resources[&digest(raw)].as_slice(), raw);
    for (pointer, value) in [
        (
            "/phaseGraph/phases/0/structure/group/operators/1/timeSign",
            json!(1),
        ),
        (
            "/vectors/0/value/anchor/site",
            phase["structure"]["sites"][1]["id"].clone(),
        ),
        (
            "/vectors/0/metadata/0/tree/payload/members/scopeInput/members/resourceId/objectId",
            capture
                .content
                .resources
                .iter()
                .find(|r| r.logical_key == KEY)
                .map(|r| json!(r.id))
                .unwrap(),
        ),
    ] {
        let mut wrong = before.clone();
        *wrong.pointer_mut(pointer).unwrap() = value;
        assert!(encode_verified(&capture_with(wrong), "forged.vesta", Ids::new([75; 16])).is_err());
    }
    let mut dropped = before.clone();
    dropped["vectors"].as_array_mut().unwrap().pop();
    assert!(
        encode_verified(
            &capture_with(dropped),
            "missing-vector.vesta",
            Ids::new([76; 16])
        )
        .is_err()
    );

    engine = EngineV2::loaded(reopened, Ids::new([77; 16]), Limits::default()).unwrap();
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
        "continued-magnetic.vesta",
        Ids::new([78; 16]),
    )
    .unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([79; 16])).unwrap();
    assert_eq!(reopened.content.camera.zoom, 2.25);
    let again = reopened.capture.source.unwrap().to_value().unwrap();
    assert_eq!(
        again["vectors"][0]["value"]["components"],
        json!([1.25, 2., 3.])
    );
}

#[test]
fn gsas_unregistered_exp_cannot_select_the_active_route() {
    let mut raw = magnetic::control_exp().to_vec();
    raw.extend_from_slice(b"\r\n");
    let source = open(&raw, Ids::new([70; 16])).unwrap();
    assert!(inspect_magnetic(&source.capture).unwrap().is_none());
    let wire = source.capture.source.unwrap().to_value().unwrap();
    assert!(wire["phaseGraph"].is_null());
    assert_eq!(wire["vectors"], json!([]));
}

#[test]
fn gsas_multiphase_exp_is_retained_but_refuses_single_phase_installation() {
    let raw = magnetic::multiphase_control_exp();
    let error = open(raw, Ids::new([80; 16])).err().unwrap();
    assert_eq!(error.code, "E_REOPEN_COVERAGE");
    assert_eq!(
        error.message,
        "GSAS active installation requires exactly one declared phase"
    );

    // A byte-distinct, unregistered copy follows the ordinary archive path.
    let mut archived = raw.to_vec();
    archived.extend_from_slice(b"\r\n");
    let project = open(&archived, Ids::new([81; 16])).unwrap();
    let saved = encode_verified(
        &project.capture,
        "multiphase-archive.vesta",
        Ids::new([82; 16]),
    )
    .unwrap();
    let capture = decode(&saved.primary, &saved.dependencies, Ids::new([83; 16]))
        .unwrap()
        .capture;
    assert!(inspect_magnetic(&capture).unwrap().is_none());
    let wire = capture.source.as_ref().unwrap().to_value().unwrap();
    assert!(wire["phaseGraph"].is_null());
    let archive = wire["metadata"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["dialect"] == DIALECT)
        .unwrap();
    let phases = archive["tree"]["members"]["phases"]["items"]
        .as_array()
        .unwrap();
    let phase_numbers = phases
        .iter()
        .map(|phase| phase["members"]["phaseNumber"]["value"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(phase_numbers, ["1", "3"]);
    let primary = capture
        .content
        .resources
        .iter()
        .find(|row| row.logical_key == KEY)
        .unwrap();
    assert_eq!(
        retained::raw(&capture, primary).unwrap(),
        archived.as_slice()
    );
}
