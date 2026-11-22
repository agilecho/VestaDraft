use crate::{
    copy::{CopyReceipt, manifest},
    fs_guard as fs,
    plan::{VerifiedInputs, read_plan},
    *,
};
use craft_anything_engine::workspace::Workspace;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedReceipt {
    schema: &'static str,
    workspace_root: PathBuf,
    app_root: PathBuf,
    manifest_sha256: String,
    plan_sha256: String,
    method_commit: &'static str,
    kind: &'static str,
    files: Vec<FileEntry>,
    copy_receipt_sha256: String,
    app_git: &'static str,
    worker_started: bool,
    model_calls: u64,
    capture_scope: &'static str,
    p201_installed: bool,
    u14_complete: bool,
}

pub(crate) fn verify_content(ws: &Workspace, r: &CopyReceipt, inputs: &VerifiedInputs) -> Result<()> {
    require(manifest(ws, &inputs.plan)? == r.manifest_sha256, "verify", "manifest-drift")?;
    require(json_bytes(r)? == json_bytes(&copy::make_receipt(inputs, r.manifest_sha256.clone()))?, "verify", "receipt-not-current-input")?;
    let mut expected = inputs.files.clone();
    expected.push(("craft.json".into(), io(fs::read_regular(&ws.root.join("craft.json"), 1024 * 1024), "verify")?));
    io(fs::compare_tree(&ws.root, &expected), "verify")?;
    let mut actual = Vec::new();
    for e in &inputs.tree.files {
        let b = io(fs::read_regular(&ws.root.join("investigation.cany").join(&e.path), e.bytes), "verify-study")?;
        actual.push(FileEntry { bytes: b.len() as u64, path: e.path.clone(), sha256: sha(&b) });
    }
    require(actual == inputs.tree.files, "verify", "study-content")?;
    Ok(())
}

/// Only reads: no create, copy, state writes, or native worker transitions.
pub fn verify_prepared(ws: &Workspace, r: &CopyReceipt) -> Result<PreparedReceipt> {
    let p = read_plan(&r.plan_path)?;
    let current = plan::inspect(&p)?;
    let recorded = copy::load_receipt(&p, true)?;
    require(json_bytes(r)? == json_bytes(&recorded)?, "verify", "journal-receipt")?;
    verify_content(ws, r, &current)?;
    Ok(PreparedReceipt {
        schema: "vesta-prepared-receipt/1",
        workspace_root: ws.root.clone(),
        app_root: ws.root.join("app"),
        manifest_sha256: r.manifest_sha256.clone(),
        plan_sha256: r.plan_sha256.clone(),
        method_commit: plan::METHOD,
        kind: "app",
        files: r.files.clone(),
        copy_receipt_sha256: sha(&json_bytes(r)?),
        app_git: "absent: locked new_project does not initialize Git; P2-01 installation has not run",
        worker_started: false,
        model_calls: 0,
        capture_scope: "startup only",
        p201_installed: false,
        u14_complete: false,
    })
}
