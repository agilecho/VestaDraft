//! Requires the real linked binary. No mock workspace or fabricated receipt.
use std::{path::PathBuf, process::Command};

#[test]
fn verify_only_missing_receipt_never_creates_or_overwrites() {
    let plan = PathBuf::from(std::env::var_os("P2_PLAN").expect("frozen source-only plan"));
    let root = PathBuf::from("/Users/agilecho/code/VestaCraft");
    let state = plan.with_extension("state");
    assert!(!root.exists());
    assert!(!state.exists());
    let before = std::fs::read(&plan).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_p1-ca-bootstrap")).args(["--plan", plan.to_str().unwrap(), "--verify-only"]).output().unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!root.exists());
    assert!(!state.exists());
    assert_eq!(std::fs::read(plan).unwrap(), before);
}

#[test]
fn production_cli_rejects_test_root_override() {
    let plan = PathBuf::from(std::env::var_os("P2_PLAN").expect("frozen plan"));
    let base = PathBuf::from(std::env::var_os("P2_TEST_ROOT").expect("owned test base"));
    let dir = base.join(format!("production-cli-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&std::fs::read(plan).unwrap()).unwrap();
    value["workspaceRoot"] = serde_json::json!(dir.join("workspace"));
    let path = dir.join("plan.json");
    std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_p1-ca-bootstrap")).args(["--plan", path.to_str().unwrap()]).env("P2_TEST_ROOT", &base).output().unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(error["code"], "fixed-binding");
    assert!(!dir.join("workspace").exists());
    std::fs::remove_dir_all(dir).unwrap();
}
