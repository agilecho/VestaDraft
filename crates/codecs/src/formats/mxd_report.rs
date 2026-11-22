//! FILE06 COORDN source observations. Absence of a qualified cell/control
//! association leaves every numeric token in its original source role.
use super::{
    full_source7, live_state::tagged, native_vesta_v2::source_only_capture,
    retained_source as retained,
};
use serde_json::{Value, json};
use vesta_model::{source7::SourceDocument, source_audit::Node};
use vesta_model::{Capture, Error, Ids, PersistentResource, RestoredProject, Result, digest};

const ORTHO: &str = "AVERAGE COORDINATES, (STANDARD DEVIATIONS, A^2) AND EXPERIMENTAL ONES";
const TRICL: &str = " MD-derived average atomic coordinates in unit cell(s)";
const TRICL_BOX: &str = " (standard deviations, A^2), and      ";
const TRICL_SYMMETRY: &str = " and experimentally determined ones  (number of symmetry operations=";
const ORTHO_SYMMETRY: &str = "  (NO.SYMM.=";
const ORTHO_VERSION: &str = "2025-11-20-00";
const TRICL_VERSION: &str = "2025-11-19-00";
const TAG: &str = "MXDReportBlockObservations1";
const KEY: &str = "mxd-file06/primary";
const DIALECT: &str = "mxd-file06-coordn-observations-1";

pub(super) fn rule_digest() -> String {
    digest(&vesta_model::source_audit::canonical(&json!([
        include_str!("../../contracts/mxd-report-format.contract.json"),
        "file06-selected-fixed-headings-declared-version-and-axis-spans-2"
    ])))
}

fn invalid(detail: &str) -> Error {
    Error::new("E_REOPEN_COVERAGE", detail)
}

fn fixed_number(raw: &str, places: usize, nonnegative: bool) -> Result<()> {
    let field = raw.trim_start_matches(' ');
    let unsigned = field.strip_prefix(['-', '+']).unwrap_or(field);
    let (whole, frac) = unsigned
        .split_once('.')
        .ok_or_else(|| invalid("MXD F-field missing decimal point"))?;
    if frac.len() != places
        || !whole
            .bytes()
            .chain(frac.bytes())
            .all(|b| b.is_ascii_digit())
        || nonnegative
            && field.starts_with('-')
            && !unsigned.bytes().all(|b| matches!(b, b'0' | b'.'))
    {
        return Err(invalid("malformed MXD fixed decimal field"));
    }
    Ok(())
}

fn read_group(rows: [&str; 3], start: usize) -> Result<Vec<Value>> {
    let n = rows[0].len();
    if n < 32 || n > 128 || n % 32 != 0 || rows.iter().any(|row| row.len() != n) {
        return Err(invalid(
            "MXD COORDN group requires three equal fixed-width axes",
        ));
    }
    (0..n/32).map(|i| {
        let at=32*i;
        let label=&rows[0][at+4..at+8];
        if label.trim().is_empty() || label.contains('\t') {
            return Err(invalid("missing MXD source label"));
        }
        let mut axes=Vec::with_capacity(3);
        for (j,row) in rows.iter().enumerate() {
            let cell=&row[at..at+32];
            if &cell[..4]!="    " || j>0 && &cell[4..8]!="    " || &cell[15..17]!=" (" || &cell[23..25]!=") " {
                return Err(invalid("MXD COORDN group delimiter or column differs"));
            }
            let (mean,stat,reference)=(&cell[8..15],&cell[17..23],&cell[25..32]);
            fixed_number(mean, 4, false)?;
            fixed_number(stat, 4, true)?;
            fixed_number(reference, 4, false)?;
            let axis = ["x", "y", "z"][j];
            axes.push(json!({"axis":axis,"meanToken":mean,"statisticToken":stat,"referenceToken":reference}));
        }
        Ok(json!({"sourceOrdinal":start+i,"sourceLabel4":label,"axes":axes}))
    }).collect()
}

fn printed_integer(word: &str) -> bool {
    let word = word.trim_start_matches(' ');
    !word.is_empty() && word.bytes().all(|b| b.is_ascii_digit())
}

fn symmetry_tail(header: &str) -> Result<&str> {
    if header.len() < 10
        || !printed_integer(&header[..3])
        || &header[3..4] != " "
        || &header[8..10] != ") "
        || header[..10].contains('\t')
    {
        return Err(invalid("malformed MXD COORDN symmetry header"));
    }
    Ok(&header[10..])
}

fn box_lengths(box_words: &str) -> Result<()> {
    if box_words.len() != 39 {
        return Err(invalid("MXD COORDN box header width"));
    }
    for at in [0, 13, 26] {
        let word = &box_words[at..at + 13];
        if &word[8..10] != "(X" || !printed_integer(&word[10..12]) || &word[12..] != ")" {
            return Err(invalid("malformed MXD COORDN box replication field"));
        }
        fixed_number(&word[..8], 4, true)?;
    }
    Ok(())
}

fn tricl_angles(angles: &str) -> Result<()> {
    if !(44..=48).contains(&angles.len()) || &angles[..9] != "         " {
        return Err(invalid("MXD COORDN triclinic angle header width"));
    }
    for at in [9, 22, 35] {
        fixed_number(&angles[at..at + 9], 5, true)?;
    }
    // The final 4X can be omitted at the end of a formatted sequential record.
    if &angles[18..22] != "    "
        || &angles[31..35] != "    "
        || !angles[44..].bytes().all(|b| b == b' ')
    {
        return Err(invalid("MXD COORDN triclinic angle delimiter"));
    }
    Ok(())
}

fn declared_release<'a>(lines: &[retained::PhysicalLine<'a>]) -> Result<Option<&'a str>> {
    let mut found = None;
    for line in lines {
        let Some((_, tail)) = line.text.split_once("F90 for all platform") else {
            continue;
        };
        let Some(version) = tail
            .trim_start_matches(' ')
            .strip_prefix("Version ")
            .and_then(|tail| tail.split_ascii_whitespace().next())
        else {
            return Err(invalid("MXD report version field"));
        };
        if !matches!(version, ORTHO_VERSION | TRICL_VERSION)
            || found.is_some_and(|old| old != version)
        {
            return Err(invalid(
                "MXD report has conflicting or unsupported releases",
            ));
        }
        found = Some(version);
    }
    Ok(found)
}

fn metadata(document: &Value, res: &PersistentResource, raw: &[u8]) -> Result<Vec<Value>> {
    let lines = retained::lines(raw, 512 * 1024)?;
    let release = declared_release(&lines)?;
    let mut records = Vec::new();
    let mut i = 0;
    let mut selected_wire = None;
    while i < lines.len() {
        let heading = lines[i].text;
        let (wire, version, headers) = if let Some(tail) = heading.strip_prefix(ORTHO) {
            let tail = tail
                .strip_prefix(ORTHO_SYMMETRY)
                .ok_or_else(|| invalid("MXD orthogonal COORDN heading context"))?;
            box_lengths(symmetry_tail(tail)?)?;
            ("ortho-20251120", ORTHO_VERSION, 1)
        } else if let Some(tail) = heading.strip_prefix(TRICL) {
            box_lengths(
                tail.strip_prefix(TRICL_BOX)
                    .ok_or_else(|| invalid("MXD triclinic COORDN box heading context"))?,
            )?;
            let tail = lines
                .get(i + 1)
                .and_then(|row| row.text.strip_prefix(TRICL_SYMMETRY))
                .ok_or_else(|| invalid("MXD triclinic COORDN symmetry heading context"))?;
            tricl_angles(symmetry_tail(tail)?)?;
            ("tricl-20251119", TRICL_VERSION, 2)
        } else if heading.trim_start().starts_with(ORTHO)
            || heading.trim_start().starts_with(TRICL.trim_start())
        {
            return Err(invalid("MXD COORDN heading indentation"));
        } else {
            i += 1;
            continue;
        };
        if release.is_some_and(|reported| reported != version) {
            return Err(invalid(
                "MXD COORDN wire differs from the declared report release",
            ));
        }
        if selected_wire.is_some_and(|old| old != wire) {
            return Err(invalid("mixed MXD report wire profiles in one source"));
        }
        selected_wire = Some(wire);
        let start = i;
        i += headers;
        if lines.get(i).is_none_or(|row| !row.text.trim().is_empty()) {
            return Err(invalid("MXD COORDN heading delimiter"));
        }
        let literal_header = lines[start..i]
            .iter()
            .map(|row| row.text)
            .collect::<Vec<_>>();
        i += 1;
        let mut sites = Vec::new();
        let mut axis_spans = Vec::new();
        let mut last_width = 128;
        while let Some(first) = lines.get(i).filter(|row| !row.text.trim().is_empty()) {
            if last_width != 128 {
                return Err(invalid("non-final short MXD COORDN group"));
            }
            let rows = lines
                .get(i..i + 3)
                .ok_or_else(|| invalid("truncated MXD COORDN axes"))?;
            sites.extend(read_group(
                [first.text, rows[1].text, rows[2].text],
                sites.len(),
            )?);
            axis_spans.extend(rows.iter().map(|row| retained::span(res, raw, row.start, row.end)));
            last_width = first.text.len();
            i += 3;
            if sites.len() > 1024 {
                return Err(invalid("MXD site count exceeds read bounds"));
            }
        }
        if sites.is_empty() || records.len() >= 32 {
            return Err(invalid("MXD block population"));
        }
        let body = json!({"reportBlockIndex":records.len(),"unit":"1","quantityKind":"source-cell-fractional-component-amplitude",
            "rawHeading":literal_header.join("\n"),"producerContext":{
                "wireDialect":wire,"producerIdentity":"unverified","reportedRelease":release,
                "originalReport":{"kind":"BytesRef","resourceId":res.id,"sha256":res.sha256,"byteLength":res.byte_length},
                "frameAndControls":"unavailable-for-active-geometry"},"sites":sites});
        let extent = retained::span(res, raw, lines[start].start, lines[i - 1].end);
        axis_spans.insert(0, extent);
        records.push(json!({"owner":{"kind":"document","id":document},"producerId":"draft.mxd-file06-reader",
            "producerVersion":"1","formatRef":"VM18-I29","dialect":DIALECT,"tree":tagged(TAG,&body)?,
            "spans":axis_spans}));
    }
    if records.is_empty() {
        return Err(invalid("no selected MXD COORDN block"));
    }
    Ok(records)
}

pub fn open(raw: &[u8], mut ids: Ids) -> Result<RestoredProject> {
    if raw.len() > 512 * 1024 {
        return Err(invalid("MXD report byte bound"));
    }
    let mut capture = source_only_capture(&mut ids);
    let res = retained::install(&mut capture, &mut ids, KEY, raw.to_vec())?;
    let mut value = full_source7::source_only_base(&capture);
    value["metadata"] = json!(metadata(&value["identity"]["documentId"], &res, raw)?);
    capture.source = Some(SourceDocument::decode(value)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

pub(super) fn verify(capture: &Capture, source: &Value) -> Result<()> {
    verified_reports(capture, source).map(|_| ())
}

fn verified_reports<'a>(capture: &Capture, source: &'a Value) -> Result<Vec<&'a Value>> {
    fn report_tags(node: &Value) -> usize {
        match node {
            Value::Object(fields) => {
                usize::from(node["kind"] == "TaggedVariant" && node["tag"] == TAG)
                    + fields.values().map(report_tags).sum::<usize>()
            }
            Value::Array(rows) => rows.iter().map(report_tags).sum(),
            _ => 0,
        }
    }
    let actual = source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| row["tree"]["tag"] == TAG || row["dialect"] == DIALECT)
        .collect::<Vec<_>>();
    if report_tags(source) != actual.len() {
        return Err(invalid("MXD COORDN observations must be top-level owned report blocks"));
    }
    let slot = capture
        .content
        .resources
        .iter()
        .find(|r| r.logical_key == KEY);
    let Some(row) = slot else {
        return if actual.is_empty() {
            Ok(actual)
        } else {
            Err(invalid("MXD retained report has no independent source use"))
        };
    };
    let expected = metadata(
        &source["identity"]["documentId"],
        row,
        retained::raw(capture, row)?,
    )?;
    if !actual.iter().copied().eq(expected.iter()) {
        return Err(invalid("MXD COORDN source blocks or spans differ"));
    }
    Ok(actual)
}

/// Inspect the retained COORDN blocks and exact physical input rows.
pub fn inspect(capture: &Capture) -> Result<Option<Value>> {
    let Some(source) = &capture.source else { return Ok(None); };
    let doc = source.to_value()?;
    let reports = verified_reports(capture, &doc)?;
    if reports.is_empty() {
        return Ok(None);
    }
    let reports = reports.into_iter().map(|report| {
        let observation = Node::from_wire(&report["tree"]["payload"])?.data()?;
        let sites = observation["sites"].as_array().ok_or_else(|| invalid("MXD report sites"))?;
        let spans = report["spans"].as_array().ok_or_else(|| invalid("MXD report spans"))?;
        let groups = spans[1..].chunks_exact(3).zip(sites.chunks(4)).map(|(rows, sites)| {
            let axes = ["x", "y", "z"].into_iter().zip(rows).map(|(axis, span)| {
                json!({"axis":axis,"span":span})
            }).collect::<Vec<_>>();
            json!({"firstSiteOrdinal":sites[0]["sourceOrdinal"],"siteCount":sites.len(),"axes":axes})
        }).collect::<Vec<_>>();
        Ok(json!({"owner":report["owner"],"blockSpan":spans[0],"observation":observation,"groups":groups}))
    }).collect::<Result<Vec<_>>>()?;
    Ok(Some(json!({"reports":reports})))
}
