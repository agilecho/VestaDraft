use super::*;
fn policy() -> Value {
    json!({"budget":{"maxInputBytes":"16777216","maxResidentBytes":"16777216","maxFieldValues":"100000","maxMeshVertices":"100000","maxOutputBytes":"16777216"},"maxResponseCacheBytes":"4194304","maxPendingUploads":4,"maxPendingSnapshots":4,"advanceWorkUnits":100,"cancelGraceMs":100,"hostCapabilitiesDigest":profiles()["hostCapabilitiesDigest"],"maxPendingDomainCandidates":4,"maxDomainResultBytes":"1048576","domainCandidateRetentionMs":"60000"})
}
struct Harness {
    s: Session,
    trace: Vec<Value>,
}
impl Harness {
    fn new() -> Self {
        let root = std::env::var("VESTA_LIFECYCLE_ROOT").unwrap_or_else(|_| {
            std::env::temp_dir()
                .join(format!("vesta-lifecycle-{}", std::process::id()))
                .to_string_lossy()
                .into_owned()
        });
        std::fs::create_dir_all(&root).unwrap();
        let mut h = Self {
            s: Session::new(Paths::new(&root).unwrap(), Ids::new([77; 16]), None, None).unwrap(),
            trace: vec![],
        };
        let r=h.raw(json!({"protocol":PROTOCOL,"op":"hello","policy":policy(),"expectedProfiles":profiles()}));
        assert_eq!(r[0]["kind"], "ok");
        let r = h.send(
            "new",
            json!({"cell":{"lengthA":[4,4,4],"anglesDeg":[90,90,90]},"label":"cancel proof"}),
        );
        assert_eq!(r[0]["kind"], "ok");
        h
    }
    fn raw(&mut self, v: Value) -> Vec<Value> {
        let r = self.s.handle(&serde_json::to_vec(&v).unwrap());
        self.trace.push(json!({"in":v,"out":r}));
        r
    }
    fn send(&mut self, op: &str, body: Value) -> Vec<Value> {
        self.raw(json!({"protocol":PROTOCOL,"sessionId":self.s.id,"contextVersion":self.s.cv.to_string(),"seq":(self.s.highest+1).to_string(),"op":op,"body":body}))
    }
    fn control(&mut self, op: &str, seq: u64) -> Vec<Value> {
        self.raw(
            json!({"protocol":PROTOCOL,"sessionId":self.s.id,"op":op,"targetSeq":seq.to_string()}),
        )
    }
    fn execute(&mut self) -> u64 {
        let at = self.s.engine().unwrap().document();
        let phase = self.s.engine().unwrap().content().phases[0].id.clone();
        let r=self.send("execute",json!({"document":at,"invocation":{"command":"cell.set","params":{"phaseId":phase,"cell":{"lengthA":[5,4,4],"anglesDeg":[90,90,90]}}}}));
        assert_eq!(r[0]["kind"], "accepted");
        self.s.highest
    }
}
#[test]
fn cancel_before_commit_and_too_late_are_deterministic() {
    let mut h = Harness::new();
    let before = h.s.engine().unwrap().context();
    let history = h.s.engine().unwrap().history_summary("unchanged");
    let seq = h.execute();
    assert!(h.s.has_pending());
    let status = h.control("status", seq);
    assert_eq!(status[0]["result"]["targetState"], "active");
    let cancelled = h.control("cancel", seq);
    assert_eq!(
        cancelled[0]["result"]["outcome"], "cancelled-before-commit",
        "{cancelled:?}"
    );
    assert_eq!(cancelled[1]["code"], "E_CANCELLED");
    assert!(!h.s.has_pending());
    assert!(h.s.advance().is_empty());
    assert_eq!(h.s.engine().unwrap().context(), before);
    assert_eq!(h.s.engine().unwrap().history_summary("unchanged"), history);
    let seq = h.execute();
    let terminal = h.s.advance();
    h.trace.push(json!({"advance":terminal}));
    assert_eq!(terminal[0]["kind"], "ok");
    let committed = h.s.engine().unwrap().context();
    assert_eq!(committed["revision"], "1");
    let r = h.control("cancel", seq);
    assert_eq!(r[0]["result"]["outcome"], "too-late");
    assert_eq!(r[0]["result"]["committedRevision"], "1");
    assert_eq!(h.s.engine().unwrap().context(), committed);
    let root = std::env::var("VESTA_LIFECYCLE_ROOT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join(format!("vesta-lifecycle-{}", std::process::id()))
            .to_string_lossy()
            .into_owned()
    });
    std::fs::write(
        std::path::Path::new(&root).join("deterministic-trace.json"),
        serde_json::to_vec_pretty(&h.trace).unwrap(),
    )
    .unwrap();
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Ok(root) = std::env::var("VESTA_LIFECYCLE_ROOT") {
            let _ = std::fs::write(
                std::path::Path::new(&root).join("deterministic-trace.json"),
                serde_json::to_vec_pretty(&self.trace).unwrap(),
            );
        }
    }
}
