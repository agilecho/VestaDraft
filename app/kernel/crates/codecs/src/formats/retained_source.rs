use serde_json::{Value, json};
use vesta_model::{Capture, Error, Ids, PersistentResource, Result, digest};

pub(super) fn install(
    capture: &mut Capture,
    ids: &mut Ids,
    key: &str,
    raw: Vec<u8>,
) -> Result<PersistentResource> {
    if capture
        .content
        .resources
        .iter()
        .any(|r| r.logical_key == key)
    {
        return Err(Error::new(
            "E_REOPEN_COVERAGE",
            "duplicate retained source use",
        ));
    }
    let sha = digest(&raw);
    let row = PersistentResource {
        id: ids.next_id().into(),
        logical_key: key.into(),
        sha256: sha.clone(),
        byte_length: raw.len().to_string(),
        media_type: "application/octet-stream".into(),
    };
    capture.resources.entry(sha).or_insert(raw);
    capture.content.resources.push(row.clone());
    Ok(row)
}

pub(super) fn raw<'a>(capture: &'a Capture, row: &PersistentResource) -> Result<&'a [u8]> {
    capture
        .resources
        .get(&row.sha256)
        .filter(|raw| raw.len().to_string() == row.byte_length && digest(raw) == row.sha256)
        .map(Vec::as_slice)
        .ok_or_else(|| {
            Error::new(
                "E_REOPEN_COVERAGE",
                "retained resource differs from owned bytes",
            )
        })
}

pub(super) fn span(row: &PersistentResource, raw: &[u8], start: usize, end: usize) -> Value {
    json!({"sourceResourceId":row.id,"byteOffset":start.to_string(),
        "byteLength":(end-start).to_string(),"sha256":digest(&raw[start..end])})
}

pub(super) struct PhysicalLine<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

/// LF and CRLF are kept in the extents. Selected text grammars consume ASCII
/// bytes only and do not normalize literal fields before preserving them.
pub(super) fn lines(raw: &[u8], max: usize) -> Result<Vec<PhysicalLine<'_>>> {
    if raw.is_empty()
        || raw.len() > max
        || !raw
            .iter()
            .all(|&b| matches!(b, b'\r' | b'\n' | b'\t' | b' '..=b'~'))
    {
        return Err(Error::new(
            "E_REOPEN_COVERAGE",
            "unqualified retained text source",
        ));
    }
    let text = std::str::from_utf8(raw).expect("checked ASCII");
    let mut offset = 0;
    text.split_inclusive('\n')
        .map(|line| {
            let body = line
                .strip_suffix('\n')
                .map_or(line, |body| body.strip_suffix('\r').unwrap_or(body));
            if body.contains('\r') {
                return Err(Error::new(
                    "E_REOPEN_COVERAGE",
                    "embedded CR in retained source line",
                ));
            }
            let start = offset;
            offset += line.len();
            Ok(PhysicalLine {
                text: body,
                start,
                end: offset,
            })
        })
        .collect()
}
