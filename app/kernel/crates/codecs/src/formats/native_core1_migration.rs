//! Single qualified migration from the pinned Core1 engineering artifact.
use super::*;
#[path = "native_core1_types.rs"]
mod legacy_types;
#[path = "native_core1_reader.rs"]
mod reader;
pub(super) fn qualifies(saved: &Value) -> bool {
    saved["sourceTypeDigest"] == reader::type_digest()
        && saved["sourceRuleSetDigest"] == reader::rule_digest()
}
pub(super) fn migrate(
    primary: &[u8],
    deps: &BTreeMap<String, Vec<u8>>,
    source: &[u8],
    ids: Ids,
) -> Result<RestoredProject> {
    let old = reader::decode(primary, deps)?;
    let mut raw =
        serde_json::to_value(&old.content).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    let mut bytes = old.resources;
    let mut bindings = vec![];
    // These retired bridge IDs describe old entries, never a current lifetime. The common
    // RestoredProject allocator replaces every bridge/object ID exactly once.
    for (i, r) in raw["resources"]
        .as_array_mut()
        .ok_or_else(|| error("legacy resource entries"))?
        .iter_mut()
        .enumerate()
    {
        let bridge =
            digest(format!("Core1/resource/{i}/{}:{}", r["logicalKey"], digest(source)).as_bytes())
                [..32]
                .to_owned();
        r.as_object_mut()
            .ok_or_else(|| error("legacy resource"))?
            .insert("id".into(), json!(bridge));
        bindings.push(json!({"originalPath":format!("resource/{i}"),"currentResource":bridge}));
    }
    let original = digest(source);
    let source_id = digest(format!("Core1/source/{original}").as_bytes())[..32].to_owned();
    bytes.insert(original.clone(), source.to_vec());
    let existing = raw["resources"]
        .as_array()
        .ok_or_else(|| error("legacy resources"))?;
    let mut key = "legacy/core1-source".to_owned();
    while existing.iter().any(|r| r["logicalKey"] == key) {
        key.push('_');
    }
    raw["resources"].as_array_mut().ok_or_else(||error("legacy resources"))?.push(json!({"id":source_id,"logicalKey":key,"sha256":original,"byteLength":source.len().to_string(),"mediaType":"application/octet-stream"}));
    raw.as_object_mut().ok_or_else(||error("legacy core"))?.insert("legacySource".into(),json!({"sourceTypeDigest":reader::type_digest(),"sourceRuleDigest":reader::rule_digest(),"originalSourceDocument":source_id,"originalResources":bindings}));
    let content: ProjectContent = serde_json::from_value(raw)
        .map_err(|e| Error::new("E_SCHEMA", format!("qualified Core1 migration: {e}")))?;
    content.validate()?;
    RestoredProject::new(
        Capture {
            document: old.document,
            state_digest: content.digest(),
            content,
            resources: bytes,
            source: None,
        },
        ids,
    )
}
