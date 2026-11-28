use serde_json::json;
use vesta_commands::*;
pub fn engine() -> EngineV2 {
    engine_with_limits(Limits::default())
}

fn engine_with_limits(limits: Limits) -> EngineV2 {
    EngineV2::new(
        Cell3 {
            length_a: [4.; 3],
            angles_deg: [90.; 3],
        },
        "晶体",
        Ids::new([1; 16]),
        limits,
    )
    .unwrap()
}
#[test]
fn noop_does_not_reserve_a_second_history_entry_at_memory_limit() {
    let size = serde_json::to_vec(engine().content()).unwrap().len();
    let mut limits = Limits::default();
    limits.resident_bytes = 2 * size;
    let mut e = engine_with_limits(limits);
    assert_eq!(e.resident_bytes(), 2 * size);
    let before = e.document();
    let phase = e.content().phases[0].id.clone();
    e.execute(&before,&json!({"command":"cell.set","params":{"phaseId":phase,"cell":{"lengthA":[4,4,4],"anglesDeg":[90,90,90]}}})).unwrap();
    assert_eq!(e.document(), before);
}

#[test]
fn attachment_that_exceeds_combined_resident_budget_is_atomic() {
    let key = "source/observations.bin";
    let data = vec![0x5a; 4096];
    let mut reference = engine();
    reference.attach_resource(key, data.clone()).unwrap();
    let mut limited = engine_with_limits(Limits {
        resident_bytes: reference.resident_bytes() - 1,
        ..Limits::default()
    });
    let before = limited.capture().unwrap();
    let bytes = limited.resident_bytes();
    let history = limited.history_summary("unchanged");
    assert_eq!(
        limited.attach_resource(key, data).unwrap_err().code,
        "E_RESOURCE_LIMIT"
    );
    let after = limited.capture().unwrap();
    assert_eq!(before.document, after.document);
    assert_eq!(before.content, after.content);
    assert_eq!(before.resources, after.resources);
    assert_eq!(limited.history_summary("unchanged"), history);
    assert_eq!(limited.resident_bytes(), bytes);
    limited.attach_resource(key, vec![0x5a]).unwrap();
    let recovered = limited.capture().unwrap().resources;
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[&digest(&[0x5a])].as_slice(), &[0x5a]);
}
#[test]
fn real_edit_history_and_noop() {
    let mut e = engine();
    let p = e.content().phases[0].id.clone();
    let at = e.document();
    e.execute(&at,&json!({"command":"site.add","params":{"phaseId":p,"label":"C1","species":{"element":"C"},"frac":[0,0,0],"occupancy":1,"uIsoA2":0}})).unwrap();
    assert_eq!(e.content().phases[0].sites.len(), 1);
    let site = e.content().phases[0].sites[0].clone();
    for a in [5., 6.] {
        let at = e.document();
        e.execute(&at,&json!({"command":"cell.set","params":{"phaseId":p,"cell":{"lengthA":[a,4,4],"anglesDeg":[90,90,90]}}})).unwrap();
        assert_eq!(e.content().phases[0].cell.volume().unwrap(), a * 16.);
    }
    let r = e.document().revision;
    let at = e.document();
    e.execute(&at,&json!({"command":"cell.set","params":{"phaseId":p,"cell":{"lengthA":[6,4,4],"anglesDeg":[90,90,90]}}})).unwrap();
    assert_eq!(r, e.document().revision);
    let at = e.document();
    e.execute(&at, &json!({"command":"history.undo","params":{}}))
        .unwrap();
    assert_eq!(e.content().phases[0].cell.volume().unwrap(), 80.);
    let at = e.document();
    e.execute(&at, &json!({"command":"history.redo","params":{}}))
        .unwrap();
    assert_eq!(e.content().phases[0].cell.volume().unwrap(), 96.);
    assert_eq!(e.content().phases[0].sites[0], site);
    assert!(e.document().revision.parse::<u64>().unwrap() > r.parse::<u64>().unwrap());
}
#[test]
fn batch_failure_is_zero_net_change_and_stale_refuses() {
    let mut e = engine();
    let original = e.capture().unwrap();
    let p = e.content().phases[0].id.clone();
    let at = e.document();
    let ops = vec![
        json!({"command":"cell.set","params":{"phaseId":p,"cell":{"lengthA":[5,4,4],"anglesDeg":[90,90,90]}}}),
        json!({"command":"site.add","params":{"phaseId":p,"label":"bad","species":{"element":"C"},"frac":[0,0,0],"occupancy":2,"uIsoA2":0}}),
    ];
    assert!(e.batch(&at, &ops).is_err());
    assert_eq!(e.capture().unwrap().content, original.content);
    assert_eq!(e.document(), at);
    let mut bad = at.clone();
    bad.epoch = "0".repeat(32);
    assert!(
        e.execute(&bad, &json!({"command":"history.undo","params":{}}))
            .is_err()
    );
}

fn camera_edit(e: &EngineV2) -> serde_json::Value {
    let scene = e.content().scene_id.clone();
    let camera = json!({"sceneMatrixLogicalRows":[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]],"shiftViewX":0,"shiftViewY":0,"thirdSceneStoredF32":0,"zoomF32":2,"projectionSelectorBit21":0,"projectionZScaleF32":1,"localTranslationA":[0,0,0],"normalizationF64":1,"viewportLogicalWidth":100,"viewportLogicalHeight":100,"deviceScaleF64":1,"aspectF32":1,"aspectScaleF32":1,"projectionContext":"explicit-canonical-camera","nearView":1,"farView":10});
    json!({"component":"native-camera","targetSceneId":scene,"state":camera})
}

#[test]
fn typed_candidate_is_pure_owned_and_expires() {
    let mut e = engine();
    let at = e.document();
    let before = e.content().clone();
    let edit = camera_edit(&e);
    let c = e.prepare_state(&at, &edit, 0).unwrap();
    let other = e.prepare_state(&at, &edit, 0).unwrap();
    assert_eq!(e.content(), &before);
    assert_eq!(e.document(), at);
    assert!(
        e.apply_domain(&at, &c.candidate_id, &"0".repeat(64), 1)
            .is_err()
    );
    e.apply_domain(&at, &c.candidate_id, &c.candidate_digest, 2)
        .unwrap();
    assert_eq!(e.content().camera.zoom, 2.);
    assert_eq!(
        e.candidate_bytes(&other.candidate_id).unwrap_err().code,
        "E_OBJECT_NOT_FOUND"
    );
    assert!(
        e.apply_domain(&at, &c.candidate_id, &c.candidate_digest, 3)
            .is_err()
    );
    let at = e.document();
    let c = e.prepare_state(&at, &edit, 10).unwrap();
    assert!(
        e.apply_domain(&at, &c.candidate_id, &c.candidate_digest, 70000)
            .is_err()
    );
}

#[test]
fn attachment_releases_stale_candidate_bytes_and_invalid_deadline_keeps_valid_lease() {
    let mut e = engine();
    let mut reference = e.clone();
    let at = e.document();
    let edit = camera_edit(&e);
    let pending = e.prepare_state(&at, &edit, 0).unwrap();
    let resident = e.resident_bytes();
    assert_eq!(
        e.prepare_state(&at, &edit, u64::MAX).unwrap_err().code,
        "E_RESOURCE_LIMIT"
    );
    assert!(e.candidate_bytes(&pending.candidate_id).is_ok());
    assert_eq!(e.resident_bytes(), resident);
    let key = "source/observation.bin";
    e.attach_resource(key, vec![0x5a]).unwrap();
    reference.attach_resource(key, vec![0x5a]).unwrap();
    assert_eq!(
        e.candidate_bytes(&pending.candidate_id).unwrap_err().code,
        "E_OBJECT_NOT_FOUND"
    );
    assert_eq!(e.resident_bytes(), reference.resident_bytes());
}
fn assert_snapshot_confirmation_refused(owner: &str, token: &str, now: u64, code: &str, message: &str) {
    let mut e = engine();
    e.loaded_work_ref("existing.vesta", &"a".repeat(64));
    let phase = e.content().phases[0].id.clone();
    e.execute(&e.document(), &json!({"command":"cell.set","params":{
        "phaseId":phase,"cell":{"lengthA":[5,4,4],"anglesDeg":[90,90,90]}
    }})).unwrap();
    let context = e.context();
    assert_eq!(context["dirty"], true);
    assert_eq!(context["workRef"]["path"], "existing.vesta");
    let capture = e.capture().unwrap();
    let mut t = vesta_commands::save_tracker::SaveTracker::default();
    t.record("snapshot", capture.clone(), "owner", "token", 100)
        .unwrap();
    let error = t.confirm_untrusted("snapshot", owner, token, now).unwrap_err();
    assert_eq!(error, vesta_model::Error::new(code, message));
    assert_eq!(e.context(), context);
    assert_eq!(t.count(), 1);
    let retained = &t.pending["snapshot"];
    assert_eq!(retained.document, capture.document);
    assert_eq!(retained.state_digest, capture.state_digest);
    assert_eq!(retained.content, capture.content);
    assert_eq!(retained.resources, capture.resources);
}

#[test]
fn save_tracker_refuses_wrong_owner() {
    assert_snapshot_confirmation_refused(
        "forged-owner", "token", 99, "E_TARGET_TOKEN_INVALID", "snapshot authority/owner mismatch",
    );
}

#[test]
fn save_tracker_refuses_wrong_authority_token() {
    assert_snapshot_confirmation_refused(
        "owner", "forged-token", 99, "E_TARGET_TOKEN_INVALID", "snapshot authority/owner mismatch",
    );
}

#[test]
fn save_tracker_refuses_lease_at_expiration() {
    assert_snapshot_confirmation_refused(
        "owner", "token", 100, "E_TARGET_TOKEN_EXPIRED", "snapshot lease expired",
    );
}

#[test]
fn save_tracker_matching_public_claim_never_grants_saved() {
    assert_snapshot_confirmation_refused(
        "owner", "token", 99, "E_TARGET_TOKEN_INVALID",
        "typed bytes are not private host/HS4 publication and semantic proof",
    );
}
