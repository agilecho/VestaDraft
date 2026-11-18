use crate::{
    fs_guard as fs,
    plan::{BootstrapPlan, VerifiedInputs},
    *,
};
use craft_anything_ai::rebuild::project::{Place, new_project};
use craft_anything_engine::workspace::{Status, TargetRef, Workspace};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CopyReceipt {
    pub schema: String,
    pub plan_path: PathBuf,
    pub plan_sha256: String,
    pub workspace_root: PathBuf,
    pub manifest_sha256: String,
    pub files: Vec<FileEntry>,
    pub study_headers: std::collections::BTreeMap<String, serde_json::Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Event {
    index: u64,
    previous_sha256: String,
    stage: String,
    receipt: CopyReceipt,
}

pub fn state_dir(p: &BootstrapPlan) -> PathBuf {
    p.path.with_extension("state")
}
pub(crate) fn make_receipt(inputs: &VerifiedInputs, manifest_sha256: String) -> CopyReceipt {
    CopyReceipt {
        schema: "vesta-copy-receipt/1".into(),
        plan_path: inputs.plan.path.clone(),
        plan_sha256: inputs.plan.plan_sha.clone(),
        workspace_root: inputs.plan.workspace_root.clone(),
        manifest_sha256,
        files: inputs.files.iter().map(|(p, b)| FileEntry { bytes: b.len() as u64, path: p.clone(), sha256: sha(b) }).collect(),
        study_headers: inputs.headers.clone(),
    }
}

fn events(p: &BootstrapPlan) -> Result<Vec<(Event, String)>> {
    let state = state_dir(p);
    io(fs::absolute(&state, false), "journal")?;
    let dir = state.join("events");
    let (files, dirs) = io(fs::tree(&dir), "journal")?;
    require(dirs.is_empty(), "journal", "unknown-directory")?;
    let mut result = Vec::new();
    let mut previous = String::new();
    let mut previous_stage = String::new();
    let mut original_receipt: Option<CopyReceipt> = None;
    let mut original_manifest = String::new();
    for (index, name) in files.iter().enumerate() {
        require(name == &format!("{index:06}.json"), "journal", "sequence")?;
        let b = io(fs::read_regular(&dir.join(name), 4 * 1024 * 1024), "journal")?;
        let e: Event = decode(&b)?;
        require(
            e.index == index as u64
                && e.previous_sha256 == previous
                && e.receipt.plan_path == p.path
                && e.receipt.plan_sha256 == p.plan_sha
                && e.receipt.workspace_root == p.workspace_root
                && e.receipt.schema == "vesta-copy-receipt/1",
            "journal",
            "identity",
        )?;
        let transition = match e.stage.as_str() {
            "create-intent" => index == 0 && e.receipt.manifest_sha256.is_empty(),
            "workspace-created" => index == 1 && previous_stage == "create-intent" && e.receipt.manifest_sha256.len() == 64,
            "copy-started" => matches!(previous_stage.as_str(), "workspace-created" | "copy-started"),
            "copy-complete" => previous_stage == "copy-started",
            _ => false,
        };
        require(transition, "journal", "stage-transition")?;
        if index == 0 {
            original_receipt = Some(e.receipt.clone());
        }
        if index == 1 {
            original_manifest = e.receipt.manifest_sha256.clone();
        }
        let mut normalized = e.receipt.clone();
        normalized.manifest_sha256.clear();
        let original = original_receipt.as_ref().ok_or_else(|| BootstrapError::new("journal", "missing-intent"))?;
        require(json_bytes(original)? == json_bytes(&normalized)?, "journal", "receipt-changed")?;
        require(index == 0 || e.receipt.manifest_sha256 == original_manifest, "journal", "manifest-changed")?;
        previous_stage = e.stage.clone();
        previous = sha(&b);
        result.push((e, previous.clone()));
    }
    require(!result.is_empty(), "journal", "empty")?;
    // The external evidence directory is owned too: an unknown empty directory or staged file
    // invalidates recovery/verify just as an unknown workspace write does.
    let (all_files, all_dirs) = io(fs::tree(&state), "journal")?;
    let known = &result.last().ok_or_else(|| BootstrapError::new("journal", "empty"))?.0.receipt;
    let mut allowed_files: std::collections::BTreeMap<String, &FileEntry> = std::collections::BTreeMap::new();
    let mut allowed_dirs: BTreeSet<String> = ["events".into(), "staging".into()].into_iter().collect();
    for entry in &known.files {
        let name = format!("staging/{}", entry.path);
        io(fs::relative(&name), "journal")?;
        for parent in Path::new(&name).ancestors().skip(1).filter(|p| !p.as_os_str().is_empty()) {
            allowed_dirs.insert(parent.to_string_lossy().into_owned());
        }
        allowed_files.insert(name, entry);
    }
    require(all_dirs.iter().all(|p| allowed_dirs.contains(p)), "journal", "unknown-directory")?;
    for name in all_files {
        if name.starts_with("events/") {
            continue;
        }
        let entry = allowed_files.get(&name).ok_or_else(|| BootstrapError::new("journal", "unknown-file"))?;
        let bytes = io(fs::read_regular(&state.join(&name), entry.bytes), "journal")?;
        require(bytes.len() as u64 == entry.bytes && sha(&bytes) == entry.sha256, "journal", "staged-content-drift")?;
    }
    Ok(result)
}

fn append(p: &BootstrapPlan, stage: &str, receipt: &CopyReceipt) -> Result<()> {
    let all = events(p)?;
    let index = all.len() as u64;
    let previous_sha256 = all.last().map(|(_, s)| s.clone()).unwrap_or_default();
    let e = Event { index, previous_sha256, stage: stage.into(), receipt: receipt.clone() };
    io(fs::write_new(&state_dir(p).join("events").join(format!("{index:06}.json")), &json_bytes(&e)?), "journal")
}

pub fn load_receipt(p: &BootstrapPlan, complete: bool) -> Result<CopyReceipt> {
    let all = events(p)?;
    let (last, _) = all.last().ok_or_else(|| BootstrapError::new("journal", "empty"))?;
    require(
        if complete { last.stage == "copy-complete" } else { matches!(last.stage.as_str(), "workspace-created" | "copy-started" | "copy-complete") },
        "recovery",
        "incomplete-create-intent",
    )?;
    Ok(last.receipt.clone())
}

pub(crate) fn manifest(ws: &Workspace, p: &BootstrapPlan) -> Result<String> {
    require(ws.root == p.workspace_root, "manifest", "root")?;
    io(fs::absolute(&ws.root, false), "manifest")?;
    let bytes = io(fs::read_regular(&ws.root.join("craft.json"), 1024 * 1024), "manifest")?;
    let actual = craft_anything_engine::workspace::read_manifest(&bytes).map_err(|_| BootstrapError::new("manifest", "invalid"))?;
    require(
        actual == ws.manifest
            && actual.version == craft_anything_engine::workspace::MANIFEST_VERSION
            && actual.request == plan::REQUEST.trim()
            && actual.target == (TargetRef::Local { path: p.target_path.clone() })
            && actual.stack == p.stack
            && actual.builder.as_deref() == Some("Codex")
            && !actual.generate_assets
            && !actual.app_in_root
            && actual.status == Status::New
            && actual.checks.is_empty()
            && actual.agent_session.is_none()
            && actual.agent_session_by.is_none()
            && !actual.awaiting_check
            && actual.not_working_checks == 0
            && actual.last_reply.is_empty()
            && actual.start_files.is_empty(),
        "manifest",
        "not-pristine-app",
    )?;
    let kind = craft_anything_ai::rebuild::kinds::KindRegistry::default().detect(&actual.request, &actual.target, &actual.stack);
    require(kind.is_some_and(|k| k.id() == "app"), "manifest", "kind")?;
    Ok(sha(&bytes))
}

/// The only construction path: the locked public API. Existing roots never reach it.
pub fn create(p: &BootstrapPlan, inputs: &VerifiedInputs) -> Result<Workspace> {
    require(p.plan_sha == inputs.plan.plan_sha && json_bytes(p)? == json_bytes(&inputs.plan)?, "create", "input-plan")?;
    let current = plan::preflight(p)?;
    require(current.files == inputs.files && current.headers == inputs.headers, "create", "input-drift")?;
    let state = state_dir(p);
    io(fs::absolute(&state, true), "create-state")?;
    require(!io(state.try_exists(), "create-state")?, "create", "state-already-exists")?;
    io(std::fs::create_dir(&state), "create-state")?;
    io(std::fs::create_dir(state.join("events")), "create-state")?;
    let receipt = make_receipt(inputs, String::new());
    let e = Event { index: 0, previous_sha256: String::new(), stage: "create-intent".into(), receipt };
    io(fs::write_new(&state.join("events/000000.json"), &json_bytes(&e)?), "create-state")?;
    io(fs::absolute(&p.workspace_root, true), "create-root")?;
    require(!io(p.workspace_root.try_exists(), "create-root")?, "create", "root-exists")?;
    let target = p.target_path.to_str().ok_or_else(|| BootstrapError::new("create", "target-utf8"))?;
    let ws = new_project(&current.settings, target, &current.request, Place::At(&p.workspace_root, None), Some(&p.stack), false, Some("Codex"))
        .map_err(|_| BootstrapError::new("create", "public-api-failed-outcome-needs-inspection"))?;
    let receipt = make_receipt(inputs, manifest(&ws, p)?);
    require(io(fs::tree(&ws.root), "create")?.0 == vec!["craft.json".to_string()], "create", "unknown-write")?;
    append(p, "workspace-created", &receipt)?;
    Ok(ws)
}

fn partial_workspace(ws: &Workspace, inputs: &VerifiedInputs, receipt: &CopyReceipt) -> Result<()> {
    require(manifest(ws, &inputs.plan)? == receipt.manifest_sha256, "copy", "manifest-drift")?;
    let (files, dirs) = io(fs::tree(&ws.root), "copy")?;
    let expected: std::collections::BTreeMap<_, _> = inputs.files.iter().map(|(p, b)| (p.as_str(), b)).collect();
    let allowed_dirs: BTreeSet<_> = expected
        .keys()
        .flat_map(|s| Path::new(s).ancestors().skip(1).filter(|p| !p.as_os_str().is_empty()).map(|p| p.to_string_lossy().into_owned()))
        .collect();
    require(dirs.iter().all(|p| allowed_dirs.contains(p)), "copy", "unknown-directory")?;
    for path in files {
        if path == "craft.json" {
            continue;
        }
        let b = expected.get(path.as_str()).ok_or_else(|| BootstrapError::new("copy", "unknown-file"))?;
        require(io(fs::read_regular(&ws.root.join(path), b.len() as u64), "copy")? == **b, "copy", "changed-destination")?;
    }
    Ok(())
}

pub fn copy_study(ws: &Workspace, inputs: &VerifiedInputs) -> Result<CopyReceipt> {
    let p = &inputs.plan;
    let receipt = load_receipt(p, false)?;
    require(json_bytes(&receipt)? == json_bytes(&make_receipt(inputs, receipt.manifest_sha256.clone()))?, "copy", "receipt-drift")?;
    partial_workspace(ws, inputs, &receipt)?;
    let fresh = plan::inspect(p)?;
    require(fresh.files == inputs.files && fresh.headers == inputs.headers && json_bytes(&fresh.tree)? == json_bytes(&inputs.tree)?, "copy", "source-drift")?;
    if events(p)?.last().is_some_and(|(e, _)| e.stage == "copy-complete") {
        verify_prepared(ws, &receipt)?;
        return Ok(receipt);
    }
    append(p, "copy-started", &receipt)?;
    let stage = state_dir(p).join("staging");
    io(fs::absolute(&stage, true), "copy-stage")?;
    if !io(stage.try_exists(), "copy-stage")? {
        io(std::fs::create_dir(&stage), "copy-stage")?;
    }
    let (stage_files, stage_dirs) = io(fs::tree(&stage), "copy-stage")?;
    let tops = ["investigation.cany", "capture", "reference", "app"];
    require(stage_files.iter().chain(&stage_dirs).all(|s| tops.contains(&s.split('/').next().unwrap_or(""))), "copy-stage", "unknown-write")?;
    for top in tops {
        partial_workspace(ws, inputs, &receipt)?;
        let prefix = format!("{top}/");
        let expected: Vec<_> = inputs.files.iter().filter_map(|(path, b)| path.strip_prefix(&prefix).map(|s| (s.to_string(), b.clone()))).collect();
        require(!expected.is_empty(), "copy", "empty-tree")?;
        let target = ws.root.join(top);
        let staged = stage.join(top);
        io(fs::absolute(&target, true), "copy-target")?;
        if io(target.try_exists(), "copy-target")? {
            // Complete only missing files; changed bytes or unknown nodes stop before writing.
            io(fs::stage_tree(&staged, &expected), "recovery-stage")?;
            io(fs::stage_tree(&target, &expected), "recovery-target")?;
            continue;
        }
        io(fs::stage_tree(&staged, &expected), "copy-stage")?;
        partial_workspace(ws, inputs, &receipt)?;
        io(fs::commit_tree(&staged, &target, &expected), "copy-commit")?;
    }
    let after = plan::inspect(p)?;
    require(after.files == inputs.files && after.headers == inputs.headers, "copy", "source-drift-after-copy")?;
    verify::verify_content(ws, &receipt, &after)?;
    append(p, "copy-complete", &receipt)?;
    Ok(receipt)
}

/// Resume only this plan's journal. This function NEVER invokes new_project.
pub fn resume(p: &BootstrapPlan) -> Result<(Workspace, VerifiedInputs)> {
    let receipt = load_receipt(p, false)?;
    let inputs = plan::inspect(p)?;
    let ws = Workspace::open(&p.workspace_root).map_err(|_| BootstrapError::new("recovery", "workspace-unreadable"))?;
    require(manifest(&ws, p)? == receipt.manifest_sha256, "recovery", "manifest-drift")?;
    partial_workspace(&ws, &inputs, &receipt)?;
    Ok((ws, inputs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn fixture(label: &str) -> (PathBuf, BootstrapPlan) {
        let base = PathBuf::from(std::env::var_os("P2_TEST_ROOT").expect("explicit owned root"));
        let dir = base.join(format!("bootstrap-{label}-{}-{}", std::process::id(), SEQ.fetch_add(1, Ordering::SeqCst)));
        std::fs::create_dir(&dir).unwrap();
        let frozen = plan::read_plan(&PathBuf::from(std::env::var_os("P2_PLAN").expect("frozen C4 plan"))).unwrap();
        let mut value = serde_json::to_value(frozen).unwrap();
        value["workspaceRoot"] = serde_json::json!(dir.join("workspace"));
        value["configFile"] = serde_json::json!(dir.join("fixture-config.toml"));
        std::fs::write(
            dir.join("fixture-config.toml"),
            format!(
                "version = {}\ndeep_mode = true\n[[ai.backends]]\nname = 'Codex'\nkind = 'agent_cli'\nmodel = 'codex'\nenabled = true\n",
                craft_anything_engine::settings::SETTINGS_VERSION
            ),
        )
        .unwrap();
        let path = dir.join("plan.json");
        std::fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
        let p = plan::read_plan(&path).unwrap();
        (dir, p)
    }
    fn evidence(label: &str) -> PathBuf {
        let dir = PathBuf::from(std::env::var_os("P2_TEST_EVIDENCE").expect("explicit test evidence")).join(label);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
    fn snapshot(root: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
        fn walk(root: &Path, p: &Path, out: &mut std::collections::BTreeMap<String, Vec<u8>>) {
            for e in std::fs::read_dir(p).unwrap() {
                let e = e.unwrap();
                let path = e.path();
                let ty = e.file_type().unwrap();
                assert!(!ty.is_symlink());
                if ty.is_dir() {
                    walk(root, &path, out)
                } else {
                    assert!(ty.is_file());
                    out.insert(path.strip_prefix(root).unwrap().to_str().unwrap().to_string(), std::fs::read(path).unwrap());
                }
            }
        }
        let mut out = std::collections::BTreeMap::new();
        walk(root, root, &mut out);
        out
    }
    /// Independent reader uses std::fs and the external C4 expected manifest, never the product
    /// verifier/reader to establish copied bytes. Complete path set is also independently built.
    fn independent_content(p: &BootstrapPlan, ws: &Workspace, dir: &Path) {
        let actual = snapshot(&ws.root);
        let mut expected = std::collections::BTreeSet::new();
        let frozen: serde_json::Value = serde_json::from_slice(&std::fs::read(&p.study_tree_manifest_path).unwrap()).unwrap();
        for row in frozen["files"].as_array().unwrap() {
            let relative = row["path"].as_str().unwrap();
            let key = format!("investigation.cany/{relative}");
            let original = std::fs::read(p.study_root.join(relative)).unwrap();
            assert_eq!(original.len() as u64, row["bytes"].as_u64().unwrap());
            assert_eq!(sha(&original), row["sha256"].as_str().unwrap());
            assert!(actual.get(&key) == Some(&original), "{key}: byte mismatch");
            expected.insert(key);
        }
        for entry in p.capture_entries.iter().chain(&p.reference_entries).chain(&p.mechanics_entries) {
            assert!(actual.get(&entry.relative_destination) == Some(&std::fs::read(&entry.source).unwrap()), "{}: byte mismatch", entry.relative_destination);
            expected.insert(entry.relative_destination.clone());
        }
        for name in ["craft.json", "reference/locator-map.json", "reference/bootstrap-sources.json"] {
            expected.insert(name.into());
        }
        assert_eq!(actual.keys().cloned().collect::<std::collections::BTreeSet<_>>(), expected);
        let m: serde_json::Value = serde_json::from_slice(actual.get("craft.json").unwrap()).unwrap();
        assert_eq!(m["stack"], plan::STACK);
        assert_eq!(m["builder"], "Codex");
        assert_eq!(m["request"], plan::REQUEST.trim());
        assert_eq!(m["status"], "new");
        assert_eq!(m["generate_assets"], false);
        assert!(m.get("app_in_root").is_none_or(|v| v.as_bool() == Some(false)));
        assert_eq!(m["target"]["path"], plan::TARGET);
        assert!(!ws.root.join("app/.git").exists());
        let records: Vec<_> = actual.iter().map(|(path, b)| FileEntry { path: path.clone(), bytes: b.len() as u64, sha256: sha(b) }).collect();
        std::fs::write(dir.join("independent-file-content.json"), serde_json::to_vec_pretty(&records).unwrap()).unwrap();
        std::fs::write(dir.join("actual-craft.json"), actual.get("craft.json").unwrap()).unwrap();
    }

    #[test]
    fn temporary_api_copy_verify_and_rejections() {
        let (dir, p) = fixture("complete");
        let out = evidence("temporary-api");
        let inputs = plan::preflight(&p).unwrap();
        let ws = create(&p, &inputs).unwrap();
        let native_manifest = std::fs::read(ws.root.join("craft.json")).unwrap();
        let error = create(&p, &inputs).err().expect("second create must refuse");
        assert_eq!(error.code, "root-exists");
        assert_eq!(std::fs::read(ws.root.join("craft.json")).unwrap(), native_manifest);
        let receipt = copy_study(&ws, &inputs).unwrap();
        let reopened = Workspace::open(&ws.root).unwrap();
        independent_content(&p, &reopened, &out);
        let before = snapshot(&ws.root);
        let prepared = verify_prepared(&reopened, &receipt).unwrap();
        assert!(snapshot(&ws.root) == before, "verify must not write");
        std::fs::write(out.join("temporary-prepared-receipt.json"), serde_json::to_vec_pretty(&prepared).unwrap()).unwrap();
        std::fs::write(out.join("actual-copy-receipt.json"), serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
        // Unknown writes are genuine on-disk faults; the verifier must retain them and refuse.
        std::fs::write(ws.root.join("app/unknown.txt"), b"unexpected writer").unwrap();
        assert!(verify_prepared(&reopened, &receipt).is_err());
        assert_eq!(std::fs::read(ws.root.join("app/unknown.txt")).unwrap(), b"unexpected writer");
        std::fs::remove_file(ws.root.join("app/unknown.txt")).unwrap();
        let bad = ws.root.join("reference/requirements.md");
        let mut bytes = std::fs::read(&bad).unwrap();
        bytes[0] ^= 1;
        std::fs::write(&bad, &bytes).unwrap();
        assert!(verify_prepared(&reopened, &receipt).is_err());
        assert_eq!(std::fs::read(&bad).unwrap(), bytes);
        std::fs::write(out.join("scope.json"),b"{\"temporaryOnly\":true,\"formalRootCreated\":false,\"sourcePackageInstalled\":false,\"workerStarted\":false,\"repeatCreateRejected\":true,\"unknownWriteRejected\":true,\"changedBytesRejected\":true}").unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    #[ignore = "explicitly invoked child; expected abrupt exit75, not a passing test"]
    fn copy_crash_child() {
        let path = PathBuf::from(std::env::var_os("P2_CRASH_PLAN").expect("owned child plan"));
        let p = plan::read_plan(&path).unwrap();
        let inputs = plan::preflight(&p).unwrap();
        let ws = create(&p, &inputs).unwrap();
        fs::crash_after_writes(7);
        let _ = copy_study(&ws, &inputs);
        panic!("copy did not terminate at the armed durable write");
    }

    #[test]
    fn interrupted_process_resumes_actual_copy_without_second_create() {
        let (dir, p) = fixture("interrupt");
        let out = evidence("process-interruption");
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--ignored", "--exact", "copy::tests::copy_crash_child", "--nocapture"])
            .env("P2_CRASH_PLAN", &p.path)
            .output()
            .unwrap();
        std::fs::write(out.join("child.stdout"), &child.stdout).unwrap();
        std::fs::write(out.join("child.stderr"), &child.stderr).unwrap();
        assert_eq!(child.status.code(), Some(75));
        let native_manifest = std::fs::read(p.workspace_root.join("craft.json")).unwrap();
        let original_events = events(&p).unwrap();
        assert_eq!(original_events.len(), 3);
        assert_eq!(original_events[2].0.stage, "copy-started");
        assert!(load_receipt(&p, true).is_err());
        let (ws, inputs) = resume(&p).unwrap();
        let receipt = copy_study(&ws, &inputs).unwrap();
        let prepared = verify_prepared(&ws, &receipt).unwrap();
        assert_eq!(std::fs::read(ws.root.join("craft.json")).unwrap(), native_manifest);
        let all = events(&p).unwrap();
        assert_eq!(all.iter().filter(|(e, _)| e.stage == "create-intent").count(), 1);
        assert_eq!(all.iter().filter(|(e, _)| e.stage == "workspace-created").count(), 1);
        assert_eq!(all.last().unwrap().0.stage, "copy-complete");
        independent_content(&p, &Workspace::open(&ws.root).unwrap(), &out);
        std::fs::write(out.join("temporary-prepared-receipt.json"), serde_json::to_vec_pretty(&prepared).unwrap()).unwrap();
        std::fs::write(out.join("scope.json"),b"{\"temporaryOnly\":true,\"expectedChildExit\":75,\"actualChildExit\":75,\"createIntentCount\":1,\"workspaceCreatedCount\":1,\"manifestBytesUnchanged\":true,\"resumedCompleteContent\":true}").unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }
}
