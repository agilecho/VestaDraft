use crate::{fs_guard as fs, *};
use craft_anything_ai::{AiConfig, BackendKind, rebuild::kinds::KindRegistry};
use craft_anything_engine::{
    Settings,
    settings::{SETTINGS_VERSION, read_settings},
    workspace::TargetRef,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

pub const METHOD: &str = "1f8508a104134c436cbba5ea2f675619ef47702f";
pub const ROOT: &str = "/Users/agilecho/code/VestaCraft";
pub const TARGET: &str = "/Users/agilecho/Documents/VESTA/VESTA.app";
pub const TARGET_SHA: &str = "b4835068478ec1852f732c93ab25768f2b8af0166a92fc68dfd85704fe71c9e9";
pub const LIB_SHA: &str = "c2c677facf6c67783cc7bbe64e92b75beb1a567fc373eacabee58b11fb81a77f";
pub const STUDY_SHA: &str = "0b261acf9848b87db34e404ed7ebef48ff8b8857515987634e86fe0da48463ba";
pub const STACK: &str = "React + TypeScript (Vite)";
pub const REQUEST: &str = "Rebuild VESTA with all recorded capabilities; follow reference/requirements.md and reference/p1-design.md. Use the pinned architecture and preserve all recorded limits.\n";

fn root_binding(path: &Path) -> bool {
    if path == Path::new(ROOT) {
        return true;
    }
    // Only the library unit-test executable can exercise the real public API in an owned
    // temporary directory. The production library/CLI has no alternate-root switch or env path.
    #[cfg(test)]
    if let Some(base) = std::env::var_os("P2_TEST_ROOT") {
        return path != Path::new(&base) && path.starts_with(Path::new(&base));
    }
    false
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CopyEntry {
    pub source: PathBuf,
    pub relative_destination: String,
    pub sha256: String,
    pub bytes: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootstrapPlan {
    pub schema: String,
    pub workspace_root: PathBuf,
    pub config_file: PathBuf,
    pub target_path: PathBuf,
    pub target_sha256: String,
    pub target_executable_relative_path: String,
    pub method_commit: String,
    pub stack: String,
    pub builder: String,
    pub request_file: PathBuf,
    pub request_sha256: String,
    pub study_root: PathBuf,
    pub study_manifest_sha256: String,
    pub study_tree_manifest_path: PathBuf,
    pub expected_files: u64,
    pub expected_evidence: u64,
    pub expected_unknowns: u64,
    pub expected_kind: String,
    pub capture_entries: Vec<CopyEntry>,
    pub reference_entries: Vec<CopyEntry>,
    pub mechanics_entries: Vec<CopyEntry>,
    #[serde(skip)]
    pub(crate) path: PathBuf,
    #[serde(skip)]
    pub(crate) plan_sha: String,
}

/// Settings are deliberately never serialized or Debug-printed.
pub struct VerifiedInputs {
    pub(crate) plan: BootstrapPlan,
    pub(crate) settings: Settings,
    pub(crate) request: String,
    pub(crate) tree: StudyTree,
    pub(crate) headers: BTreeMap<String, serde_json::Value>,
    pub(crate) files: Vec<(String, Vec<u8>)>,
}

pub fn read_plan(path: &Path) -> Result<BootstrapPlan> {
    let bytes = io(fs::read_regular(path, 4 * 1024 * 1024), "plan")?;
    let mut p: BootstrapPlan = decode(&bytes)?;
    p.path = path.to_path_buf();
    p.plan_sha = sha(&bytes);
    validate_plan(&p)?;
    Ok(p)
}

fn hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(crate) fn validate_plan(p: &BootstrapPlan) -> Result<()> {
    require(
        p.schema == "vesta-bootstrap/1"
            && p.method_commit == METHOD
            && root_binding(&p.workspace_root)
            && p.target_path == Path::new(TARGET)
            && p.target_sha256 == TARGET_SHA
            && p.target_executable_relative_path == "Contents/MacOS/VESTA"
            && p.stack == STACK
            && p.builder == "Codex"
            && p.expected_kind == "app"
            && p.expected_files == 2068
            && p.expected_evidence == 2037
            && p.expected_unknowns == 24
            && p.study_manifest_sha256 == STUDY_SHA,
        "plan",
        "fixed-binding",
    )?;
    for path in [&p.config_file, &p.request_file, &p.study_root, &p.study_tree_manifest_path, &p.path] {
        io(fs::absolute(path, false), "plan-path")?;
        require(!path.starts_with(&p.workspace_root), "plan", "input-in-output")?;
    }
    require(hex(&p.request_sha256), "plan", "digest")?;
    let mut destinations = BTreeSet::new();
    for (entries, prefix) in [(&p.capture_entries, "capture/"), (&p.reference_entries, "reference/"), (&p.mechanics_entries, "app/")] {
        for e in entries {
            io(fs::relative(&e.relative_destination), "entry-path")?;
            io(fs::absolute(&e.source, false), "entry-source")?;
            require(
                e.relative_destination.starts_with(prefix)
                    && hex(&e.sha256)
                    && e.bytes <= 64 * 1024 * 1024
                    && !e.source.starts_with(&p.workspace_root)
                    && destinations.insert(e.relative_destination.clone()),
                "plan",
                "entry-binding-or-duplicate",
            )?;
        }
    }
    for set in [&p.capture_entries, &p.reference_entries, &p.mechanics_entries] {
        require(!set.is_empty(), "plan", "empty-entries")?;
    }
    let required = [
        "capture/window.png",
        "capture/ui-tree.json",
        "reference/requirements.md",
        "reference/p1-design.md",
        "reference/interfaces.md",
        "reference/ca-wiring.md",
        "reference/module-plan.md",
        "reference/study-report.md",
        "app/MECHANICS.md",
        "app/mechanics.json",
    ];
    require(required.iter().all(|s| destinations.contains(*s)), "plan", "missing-required-entry")?;
    require(
        p.mechanics_entries.len() == 2
            && p.capture_entries.len() == 2
            && !destinations.contains("reference/locator-map.json")
            && !destinations.contains("reference/bootstrap-sources.json"),
        "plan",
        "reserved-entry",
    )?;
    // No planned file can be a parent of another; case-fold collisions are unsafe on macOS.
    let paths: Vec<_> = destinations.iter().collect();
    let mut folded = BTreeSet::new();
    for a in &paths {
        require(folded.insert(a.to_lowercase()), "plan", "case-collision")?;
        for b in &paths {
            require(a == b || !b.starts_with(&format!("{a}/")), "plan", "prefix-collision")?;
        }
    }
    Ok(())
}

pub fn preflight(p: &BootstrapPlan) -> Result<VerifiedInputs> {
    io(fs::absolute(&p.workspace_root, true), "preflight-root")?;
    require(!io(p.workspace_root.try_exists(), "preflight-root")?, "preflight", "root-exists")?;
    inspect(p)
}

pub(crate) fn checked_entry(e: &CopyEntry) -> Result<Vec<u8>> {
    let b = io(fs::read_regular(&e.source, e.bytes), "input")?;
    require(b.len() as u64 == e.bytes && sha(&b) == e.sha256, "input", "content-mismatch")?;
    Ok(b)
}

fn validate_settings(config: &str) -> Result<Settings> {
    let settings = read_settings(config).map_err(|_| BootstrapError::new("config", "invalid"))?;
    require(settings.version == SETTINGS_VERSION && settings.deep_mode, "config", "version-or-deep")?;
    let ai = AiConfig::from_table(&settings.ai).map_err(|_| BootstrapError::new("config", "backend-table"))?;
    require(ai.backends.iter().filter(|b| b.config.name == "Codex").count() == 1, "config", "backend-missing-or-duplicate")?;
    let b = ai.get("Codex").ok_or_else(|| BootstrapError::new("config", "backend-missing"))?;
    require(b.config.enabled && b.config.kind == BackendKind::AgentCli, "config", "backend-disabled-or-type")?;
    Ok(settings)
}

pub(crate) fn inspect(p: &BootstrapPlan) -> Result<VerifiedInputs> {
    validate_plan(p)?;
    require(sha(&io(fs::read_regular(&p.path, 4 * 1024 * 1024), "plan")?) == p.plan_sha, "input", "plan-drift")?;
    let config = io(fs::read_regular(&p.config_file, 1024 * 1024), "config")?;
    let config = std::str::from_utf8(&config).map_err(|_| BootstrapError::new("config", "invalid"))?;
    let settings = validate_settings(config)?;
    // AgentCli authentication belongs to the CLI; do not read KeyStore or construct AiHub.
    require(
        sha(&io(fs::read_regular(&p.target_path.join(&p.target_executable_relative_path), 128 * 1024 * 1024), "target")?) == p.target_sha256,
        "target",
        "executable-sha",
    )?;
    require(
        sha(&io(fs::read_regular(&p.target_path.join("Contents/MacOS/libVESTA.dylib"), 256 * 1024 * 1024), "target")?) == LIB_SHA,
        "target",
        "companion-sha",
    )?;
    let request = io(fs::read_regular(&p.request_file, 16384), "request")?;
    require(sha(&request) == p.request_sha256 && request == REQUEST.as_bytes(), "request", "content-mismatch")?;
    let request = String::from_utf8(request).map_err(|_| BootstrapError::new("request", "utf8"))?;
    let target = TargetRef::Local { path: p.target_path.clone() };
    require(KindRegistry::default().detect(&request, &target, &p.stack).is_some_and(|k| k.id() == "app"), "kind", "not-app")?;
    let manifest = io(fs::read_regular(&p.study_tree_manifest_path, 8 * 1024 * 1024), "study")?;
    require(sha(&manifest) == p.study_manifest_sha256, "study", "manifest-sha")?;
    let tree: StudyTree = decode(&manifest)?;
    require(
        tree.schema == "vesta-study-tree/1" && json_bytes(&tree)? == manifest && tree.files.len() as u64 == p.expected_files,
        "study",
        "noncanonical-or-count",
    )?;
    let mut prev = None;
    for e in &tree.files {
        io(fs::relative(&e.path), "study-path")?;
        require(hex(&e.sha256) && prev.is_none_or(|s: &str| s < e.path.as_str()), "study", "order-or-digest")?;
        prev = Some(e.path.as_str());
    }
    let (present, _) = io(fs::tree(&p.study_root), "study")?;
    require(present == tree.files.iter().map(|e| e.path.clone()).collect::<Vec<_>>(), "study", "file-set")?;
    let mut files = Vec::new();
    let mut headers = BTreeMap::new();
    let (mut evidence, mut unknowns) = (0, 0);
    for e in &tree.files {
        let bytes = io(fs::read_regular(&p.study_root.join(&e.path), e.bytes), "study")?;
        require(bytes.len() as u64 == e.bytes && sha(&bytes) == e.sha256, "study", "content-drift")?;
        if e.path.starts_with("evidence/") || e.path.starts_with("unknowns/") {
            let value = strict_json(&bytes)?;
            let id = value.get("id").and_then(|v| v.as_str()).ok_or_else(|| BootstrapError::new("study", "missing-id"))?;
            require(Path::new(&e.path).file_stem().and_then(|s| s.to_str()) == Some(id) && !headers.contains_key(id), "study", "id-path-or-duplicate")?;
            if e.path.starts_with("unknowns/") {
                require(value.get("revision").and_then(|v| v.as_u64()).is_some_and(|v| v > 0), "study", "unknown-revision")?;
                unknowns += 1;
            } else {
                evidence += 1;
            }
            headers.insert(id.into(), serde_json::json!({"id":id,"revision":value.get("revision")}));
        }
        files.push((format!("investigation.cany/{}", e.path), bytes));
    }
    require(evidence == p.expected_evidence && unknowns == p.expected_unknowns, "study", "record-count")?;
    for e in p.capture_entries.iter().chain(&p.reference_entries).chain(&p.mechanics_entries) {
        files.push((e.relative_destination.clone(), checked_entry(e)?));
    }
    validate_capture_and_mechanics(&files)?;
    let locator = serde_json::json!({"schema":"vesta-locator-map/1","studyRoot":p.study_root,"studyManifestSha256":p.study_manifest_sha256,"files":tree.files.iter().map(|e|serde_json::json!({"sourceRelativePath":e.path,"currentSource":p.study_root.join(&e.path),"derivedPath":format!("investigation.cany/{}",e.path),"sha256":e.sha256})).collect::<Vec<_>>(),"externalHistoricalPaths":"unchanged; missing external artifacts remain missing, not resolved by guessing"});
    files.push(("reference/locator-map.json".into(), json_bytes(&locator)?));
    files.push(("reference/bootstrap-sources.json".into(),json_bytes(&serde_json::json!({"schema":"vesta-bootstrap-sources/1","planSha256":p.plan_sha,"captureScope":"startup only; no edited-state or runtime qualification","references":p.reference_entries,"caDesign":"C4 inherited; C5 conditional / C6 browser increment not consumed","mechanics":"S06 input, not MechanicsCheck or scientific validation"}))?));
    files.sort_by(|a, b| a.0.cmp(&b.0));
    // Re-enumerate after every source read. Content is re-read again before commit.
    require(io(fs::tree(&p.study_root), "study")?.0 == present, "study", "concurrent-write")?;
    Ok(VerifiedInputs { plan: p.clone(), settings, request, tree, headers, files })
}

fn validate_capture_and_mechanics(files: &[(String, Vec<u8>)]) -> Result<()> {
    let find = |name: &str| files.iter().find(|(p, _)| p == name).map(|(_, b)| b.as_slice()).ok_or_else(|| BootstrapError::new("assets", "missing"));
    let png = find("capture/window.png")?;
    require(
        png.len() > 24
            && &png[..8] == b"\x89PNG\r\n\x1a\n"
            && &png[12..16] == b"IHDR"
            && png[16..20] == 1720u32.to_be_bytes()
            && png[20..24] == 1456u32.to_be_bytes(),
        "capture",
        "startup-image",
    )?;
    let ax = strict_json(find("capture/ui-tree.json")?)?;
    let nodes = ax.get("nodes").and_then(|v| v.as_array()).ok_or_else(|| BootstrapError::new("capture", "nodes"))?;
    require(
        nodes.len() == 344
            && nodes.first().is_some_and(|n| n.get("title").and_then(|v| v.as_str()) == Some("VESTA"))
            && nodes.iter().all(|n| n.get("role").and_then(|v| v.as_str()).is_some() && n.get("title").is_none_or(|v| v.is_string())),
        "capture",
        "ax-shape",
    )?;
    let mechanics: craft_anything_engine::mechanics::Mechanics = decode(find("app/mechanics.json")?)?;
    let md = std::str::from_utf8(find("app/MECHANICS.md")?).map_err(|_| BootstrapError::new("mechanics", "markdown"))?;
    let mut names = BTreeSet::new();
    require(mechanics.version == 1 && !mechanics.values.is_empty() && mechanics.values.len() <= 2000, "mechanics", "version-or-values")?;
    for v in mechanics.values {
        require(
            !v.name.is_empty() && names.insert(v.name.clone()) && !v.source.trim().is_empty() && !v.estimated && md.contains(&v.name) && md.contains(&v.source),
            "mechanics",
            "source-binding",
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_json_rejects_duplicate_nested_keys_and_keeps_u64() {
        assert!(strict_json(br#"{"a":{"b":1,"b":2}}"#).is_err());
        assert_eq!(strict_json(br#"{"n":18446744073709551615}"#).unwrap()["n"].as_u64(), Some(u64::MAX));
        assert!(strict_json(b"{} {}").is_err());
        assert!(strict_json(b"{\"n\":NaN}").is_err());
    }
    #[test]
    fn public_registry_distinguishes_pinned_request_and_website() {
        let t = TargetRef::Local { path: PathBuf::from(TARGET) };
        let registry = KindRegistry::default();
        assert_eq!(registry.detect(REQUEST, &t, STACK).unwrap().id(), "app");
        assert_eq!(registry.detect(&format!("{REQUEST} website"), &t, STACK).unwrap().id(), "website");
    }
    #[test]
    fn closed_plan_rejects_wrong_sha_and_destinations() {
        // Uses the actual frozen plan; it is intentionally NOT a qualified execution input.
        let path = std::env::var_os("P2_PLAN").expect("explicit frozen plan");
        let p = read_plan(Path::new(&path)).unwrap();
        let mut bad = p.clone();
        bad.target_sha256 = "0".repeat(64);
        assert_eq!(validate_plan(&bad).unwrap_err().code, "fixed-binding");
        bad = p.clone();
        bad.capture_entries[0].relative_destination = "../escape".into();
        assert!(validate_plan(&bad).is_err());
        bad = p.clone();
        bad.reference_entries.push(bad.reference_entries[0].clone());
        assert_eq!(validate_plan(&bad).unwrap_err().code, "entry-binding-or-duplicate");
        assert!(!p.workspace_root.exists());
    }
    #[test]
    fn actual_config_validator_rejects_malformed_disabled_wrong_type_and_missing() {
        let valid =
            format!("version = {SETTINGS_VERSION}\ndeep_mode = true\n[[ai.backends]]\nname = 'Codex'\nkind = 'agent_cli'\nmodel = 'codex'\nenabled = true\n");
        assert!(validate_settings(&valid).is_ok());
        for text in [
            "version = 'SECRET_SENTINEL'".to_string(),
            "deep_mode = false".into(),
            format!("version = {SETTINGS_VERSION}\ndeep_mode = true"),
            valid.replace("enabled = true", "enabled = false"),
            valid.replace("agent_cli", "openai"),
        ] {
            let error = validate_settings(&text).err().expect("must refuse");
            assert_eq!(error.stage, "config");
            assert!(!error.to_string().contains("SECRET_SENTINEL"));
        }
    }
}
