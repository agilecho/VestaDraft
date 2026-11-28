use super::*;
use crate::{decode, encode_verified};
use vesta_commands::{EngineV2, Limits};

#[test]
fn gsas_original_chart_keeps_center_first_order_and_magnetic_character() {
    let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/gsas/spin-chart");
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
fn gsas_multiphase_exp_selects_qualified_phase_and_preserves_all_original_phases() {
    let raw = magnetic::multiphase_control_exp();
    let project = open(raw, Ids::new([80; 16])).unwrap();
    let before = project.capture.source.as_ref().unwrap().to_value().unwrap();
    let phase = &before["phaseGraph"]["phases"][0];
    assert_eq!(phase["label"], "GSAS phase 3");
    assert_eq!(before["phaseGraph"]["phases"].as_array().unwrap().len(), 1);
    assert_eq!(before["vectors"].as_array().unwrap().len(), 2);
    let phases = |wire: &Value| wire["metadata"].as_array().unwrap().iter()
        .find(|row| row["dialect"] == DIALECT).unwrap()["tree"]["members"]["phases"].clone();
    let old_phases = phases(&before);
    assert_eq!(old_phases["items"][0]["members"]["phaseNumber"]["value"], "1");
    assert_eq!(old_phases["items"][1]["members"]["phaseNumber"]["value"], "3");
    let retained_atom = &old_phases["items"][1]["members"]["atoms"]["items"][0]["members"];
    let thermal = &retained_atom["thermalWords"]["items"];
    assert_eq!(thermal[0]["value"], "  0.010000");
    assert_eq!(thermal[1]["value"], "          ");
    assert_eq!(phase["structure"]["sites"][0]["adp"]["uA2"], 0.01);
    let mut engine = EngineV2::loaded(project, Ids::new([81; 16]), Limits::default()).unwrap();
    engine.execute(&engine.document(), &json!({"command":"view.setCamera","params":{
        "rotation":[[1,0,0],[0,1,0],[0,0,1]],"projection":"parallel","zoom":2.75
    }})).unwrap();
    let saved = encode_verified(&engine.capture().unwrap(), "multiphase.vesta", Ids::new([82; 16])).unwrap();
    let reopened = decode(&saved.primary, &saved.dependencies, Ids::new([83; 16])).unwrap();
    let wire = reopened.capture.source.as_ref().unwrap().to_value().unwrap();
    assert_eq!(reopened.content.camera.zoom, 2.75);
    assert_eq!(phases(&wire), old_phases);
    assert_ne!(wire["phaseGraph"]["root"], before["phaseGraph"]["root"]);
    assert_eq!(wire["phaseGraph"]["phases"][0]["label"], "GSAS phase 3");
    assert_eq!(wire["vectors"][0]["value"]["components"], json!([-2., 1., 3.]));
    assert_eq!(reopened.resources[&digest(raw)].as_slice(), raw);
    assert!(inspect_magnetic(&reopened.capture).unwrap().is_some());

    // A byte-distinct unregistered original remains archive-only.
    let mut archived = raw.to_vec();
    archived.extend_from_slice(b"\r\n");
    let source_only = open(&archived, Ids::new([84; 16])).unwrap();
    assert!(inspect_magnetic(&source_only.capture).unwrap().is_none());
    assert!(source_only.capture.source.unwrap().to_value().unwrap()["phaseGraph"].is_null());

    let start = raw
        .windows(12)
        .position(|key| key == b"CRS3  AT  1B")
        .unwrap();
    let mut missing_anisotropic = raw.to_vec();
    missing_anisotropic[start + 74] = b'A';
    assert_eq!(
        open(&missing_anisotropic, Ids::new([85; 16])).unwrap_err().code,
        "E_REOPEN_COVERAGE"
    );
    let mut bad_isotropic = raw.to_vec();
    bad_isotropic[start + 12..start + 22].fill(b' ');
    assert_eq!(
        open(&bad_isotropic, Ids::new([85; 16])).unwrap_err().code,
        "E_REOPEN_COVERAGE"
    );
}

#[test]
fn gsas_archived_observations_are_inspectable_after_engineering_reopen() {
    let mut raw = magnetic::multiphase_control_exp().to_vec();
    raw.extend_from_slice(b"\r\n");
    let original = open(&raw, Ids::new([86; 16])).unwrap().capture;
    let before = inspect_observations(&original).unwrap().unwrap();
    assert_eq!(before["primary"]["sha256"], digest(&raw));
    assert_eq!(before["phases"][0]["phaseNumber"], 1);
    assert_eq!(before["phases"][0]["atoms"][1]["labelWord"], "O2      ");
    assert_eq!(before["phases"][1]["phaseNumber"], 3);
    assert_eq!(before["phases"][1]["spinFlipWords"][3], " -0.");
    assert!(inspect_magnetic(&original).unwrap().is_none());

    let bundle = encode_verified(&original, "observations.vesta", Ids::new([87; 16])).unwrap();
    let reopened = decode(&bundle.primary, &bundle.dependencies, Ids::new([88; 16])).unwrap();
    let after = inspect_observations(&reopened.capture).unwrap().unwrap();
    assert_eq!(before["phases"], after["phases"]);
    assert_eq!(before["phaseWords"], after["phaseWords"]);
    assert_ne!(before["primary"]["resourceId"], after["primary"]["resourceId"]);
    assert_eq!(before["primary"]["sha256"], after["primary"]["sha256"]);
    let mut saw_spin_flip = false;
    for span in after["sourceSpans"].as_array().unwrap() {
        let offset: usize = span["byteOffset"].as_str().unwrap().parse().unwrap();
        let length: usize = span["byteLength"].as_str().unwrap().parse().unwrap();
        let line = &raw[offset..offset + length];
        assert_eq!(span["sourceResourceId"], after["primary"]["resourceId"]);
        assert_eq!(span["sha256"], digest(line));
        saw_spin_flip |= line.starts_with(b"CRS3  SPNFLP") && line.ends_with(b"\r\n");
    }
    assert!(saw_spin_flip);
    assert!(inspect_magnetic(&reopened.capture).unwrap().is_none());
}

#[test]
fn gsas_expedt_spin_slots_keep_inactive_space_bytes() {
    let mut raw = magnetic::multiphase_control_exp().to_vec();
    let start = raw
        .windows(12)
        .position(|key| key == b"CRS3  SPNFLP")
        .unwrap()
        + 12;
    let spin = b"   1  -1  -1   1   1   1                        ";
    raw[start..start + 48].copy_from_slice(spin);
    let opened = open(&raw, Ids::new([94; 16])).unwrap();
    let observation = inspect_observations(&opened.capture).unwrap().unwrap();
    assert_eq!(
        observation["phases"][1]["spinFlipWords"],
        json!(["   1", "  -1", "  -1", "   1", "   1", "   1",
               "    ", "    ", "    ", "    ", "    ", "    "])
    );

    raw[start + 4..start + 8].fill(b' ');
    assert_eq!(
        open(&raw, Ids::new([94; 16])).unwrap_err().code,
        "E_REOPEN_COVERAGE"
    );
}

#[test]
fn gsas_fedora_abc_spelling_preserves_exact_bytes_and_rejects_duplicate_cells() {
    let raw = std::str::from_utf8(magnetic::multiphase_control_exp()).unwrap();
    let legacy = raw
        .lines()
        .find(|line| line.starts_with("CRS3    ABC "))
        .unwrap();
    let fedora = "CRS3  ABC     7.850000  7.814000 23.572001    N    0";
    let raw = raw.replacen(legacy, fedora, 1);
    let project = open(raw.as_bytes(), Ids::new([90; 16])).unwrap();
    let saved = encode_verified(&project.capture, "fedora-cell.vesta", Ids::new([91; 16])).unwrap();
    let reopened = decode(&saved.primary, &saved.dependencies, Ids::new([92; 16])).unwrap();
    let observed = inspect_observations(&reopened.capture).unwrap().unwrap();
    assert_eq!(
        observed["phases"][1]["cellLengthWords"],
        json!(["  7.850000", "  7.814000", " 23.572001"])
    );
    let start = raw.find(fedora).unwrap();
    let record = &raw.as_bytes()[start..start + fedora.len() + 2];
    assert!(observed["sourceSpans"].as_array().unwrap().iter().any(|span| {
        span["byteOffset"] == start.to_string() && span["sha256"] == digest(record)
    }));
    assert_eq!(
        reopened.resources[&digest(raw.as_bytes())].as_slice(),
        raw.as_bytes()
    );
    let duplicate = format!("{raw}{legacy}\r\n");
    assert_eq!(
        open(duplicate.as_bytes(), Ids::new([93; 16])).unwrap_err().message,
        "invalid or duplicate GSAS phase record key"
    );
}

#[test]
fn gsas_qualified_phase_selection_rejects_missing_and_repeated_phase_numbers() {
    let raw = magnetic::multiphase_control_exp();
    let mut archived = raw.to_vec();
    archived.extend_from_slice(b"\r\n");
    let source = open(&archived, Ids::new([85; 16])).unwrap().capture.source.unwrap().to_value().unwrap();
    let history = source["metadata"][0].clone();
    let approval = magnetic::qualified(&digest(raw)).unwrap();
    let phase3 = history["tree"]["members"]["phases"]["items"][1].clone();
    for phases in [json!([history["tree"]["members"]["phases"]["items"][0]]), json!([phase3, phase3])] {
        let mut wrong = history.clone();
        wrong["tree"]["members"]["phases"]["items"] = phases;
        let failure = magnetic::Prepared::new(approval, raw, &wrong).err().unwrap();
        assert_eq!(failure.code, "E_REOPEN_COVERAGE");
        assert_eq!(failure.message, "GSAS approved phase is missing or repeated");
    }
}
