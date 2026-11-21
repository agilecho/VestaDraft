//! Read FDAT connection observations in the held official directory profile.
use super::{full_source7, live_state::tagged, native_vesta_v2::source_only_capture};
use serde_json::{Value, json};
use vesta_model::source_audit::canonical;
use vesta_model::source7::SourceDocument;
use vesta_model::{Capture, Error, Ids, PersistentResource, RestoredProject, Result, digest};

const KEY: &str = "fdat/primary";
const C57: &str = "fdat-c57-source-1";
const C57_SHA: &str = "865511e972955252a226fb606824c5852aea4f08a23128648f5221996777456e";
const UNINTERPRETED: &str = "fdat-source-uninterpreted-1";
const PUBLISHED: &str = "fdat-source-published-count-1";
const TAG: &str = "FCON1";

#[derive(Clone, Copy)]
pub enum FdatLayout {
    Uninterpreted,
    Published,
}

impl FdatLayout {
    fn dialect(self) -> &'static str {
        match self {
            Self::Uninterpreted => UNINTERPRETED,
            Self::Published => PUBLISHED,
        }
    }
    fn parse_dialect(value: &Value) -> Result<Self> {
        match value.as_str() {
            Some(UNINTERPRETED | C57) => Ok(Self::Uninterpreted),
            Some(PUBLISHED) => Ok(Self::Published),
            _ => Err(invalid("FDAT observation dialect")),
        }
    }
}

pub(super) fn rule_digest() -> String {
    let contract = include_str!("../../contracts/fdat-connection.contract.json");
    let framing = include_str!("../../contracts/fdat-format.contract.json");
    digest(&canonical(&json!([
        KEY,
        UNINTERPRETED,
        PUBLISHED,
        contract,
        framing
    ])))
}

fn invalid(detail: &str) -> Error {
    Error::new("E_REOPEN_COVERAGE", detail)
}

pub fn is_selected_fdat(raw: &[u8]) -> bool {
    raw.starts_with(b"#")
        && raw.get(80..82) == Some(b"\r\n")
        && raw
            .get(9..17)
            .is_some_and(|x| x.iter().all(u8::is_ascii_digit))
}

fn counts(line: &[u8]) -> Result<[usize; 11]> {
    let mut fields = [0; 11];
    for (i, token) in line[23..56].chunks_exact(3).enumerate() {
        fields[i] = decimal(token)?;
    }
    Ok(fields)
}

fn decimal(token: &[u8]) -> Result<usize> {
    let token = std::str::from_utf8(token)
        .map_err(|_| invalid("FDAT ASCII field"))?
        .trim_start_matches(' ');
    let digits = token.strip_prefix('+').unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(invalid("FDAT unsigned decimal field"));
    }
    token.parse().map_err(|_| invalid("FDAT decimal extent"))
}

struct Source<'a> {
    raw: &'a [u8],
    nat: usize,
    nsat: usize,
    ncon: usize,
    table_offset: usize,
    table_end: usize,
    table: Vec<&'a str>,
}

impl<'a> Source<'a> {
    fn read(raw: &'a [u8]) -> Result<Self> {
        if !is_selected_fdat(raw)
            || raw.len() > 256 * 1024
            || !raw
                .iter()
                .all(|&b| matches!(b, b'\r' | b'\n' | b' '..=b'~'))
        {
            return Err(invalid("unqualified FDAT fixed-record source"));
        }
        let lines = raw.split_inclusive(|&b| b == b'\n').collect::<Vec<_>>();
        if lines
            .iter()
            .any(|r| !r.ends_with(b"\r\n") || r.len() > 82 || r[..r.len() - 2].contains(&b'\r'))
        {
            return Err(invalid("FDAT record exceeds its physical extent"));
        }
        let [
            cards,
            nrfac,
            nrem,
            ndis,
            nerr,
            nopr,
            nrad,
            nat,
            nsat,
            nbnd,
            ncon,
        ] = counts(&lines[0][..80])?;
        let n = nat + nsat;
        // The selected directory has one result, one symmetry and one radius
        // record before the flag-2 coordinate rows. Keep the original rows.
        if n == 0
            || nrfac == 0
            || nrfac > 15
            || nrem != 0
            || ndis != 0
            || nerr != 0
            || nopr == 0
            || nopr > 5
            || nrad == 0
            || nrad > 16
            || nbnd != 0
            || raw[56] != b'1'
            || raw[57] != b'3' && raw[57] != b'0'
            || raw[58] != b'2'
        {
            return Err(invalid(
                "FDAT directory outside selected fixed-record profile",
            ));
        }
        if ncon != 0 && (ncon < n || (ncon - n) % 2 != 0) {
            return Err(invalid("FDAT connection count or pair parity"));
        }
        let start = 5 + n.div_ceil(3);
        let end = cards;
        if (ncon == 0 && end != start) || (ncon != 0 && end <= start) {
            return Err(invalid(
                "FDAT directory does not delimit the declared connection table",
            ));
        }
        let Some(table_rows) = lines.get(start..end) else {
            return Err(invalid("FDAT declared records truncated"));
        };
        if lines[end..].iter().any(|row| *row != b"\r\n") {
            return Err(invalid("bytes outside the single FDAT source entry"));
        }
        for (i, row) in lines[5..start].iter().enumerate() {
            let body = &row[..row.len() - 2];
            let used = (n - 3 * i).min(3);
            let active = 27 * used - 1;
            if body.len() < active || !body[active..].iter().all(|x| *x == b' ') {
                return Err(invalid("FDAT coordinate record width"));
            }
            for j in 0..used {
                let at = 27 * j;
                let slot = &body[at..at + 26];
                if slot[..5].iter().all(|x| *x == b' ')
                    || (j + 1 < used && body[at + 26] != b' ')
                    || slot[5..].chunks_exact(7).any(|w| {
                        std::str::from_utf8(w)
                            .ok()
                            .and_then(|s| s.trim_start_matches(' ').parse::<i32>().ok())
                            .is_none()
                    })
                {
                    return Err(invalid(
                        "FDAT coordinate fields or source atom correspondence",
                    ));
                }
            }
        }
        let table_offset = lines[..start].iter().map(|r| r.len()).sum::<usize>();
        let table_end = table_offset + table_rows.iter().map(|r| r.len()).sum::<usize>();
        let table = table_rows
            .iter()
            .map(|r| std::str::from_utf8(&r[..r.len() - 2]).expect("checked ASCII"))
            .collect();
        Ok(Self {
            raw,
            nat,
            nsat,
            ncon,
            table_offset,
            table_end,
            table,
        })
    }

    fn observation(&self, layout: FdatLayout) -> Result<Value> {
        let n = self.nat + self.nsat;
        if self.ncon == 0 {
            return Ok(json!({"kind":"source-table-absent","sourceIntegerCount":0,
                "declaredAtoms":n,"graphKnownEmpty":false}));
        }
        if matches!(layout, FdatLayout::Uninterpreted) {
            return Ok(json!({"kind":"source-uninterpreted","declaredAtoms":n,
                "sourceIntegerCount":self.ncon,"rawPhysicalRows":self.table,"activeGraphEstablished":false}));
        }
        if n == 100 {
            return Err(Error::new(
                "E_FDAT_SOURCE_LAYOUT_REQUIRED",
                "published FDAT rule leaves N=100 open",
            ));
        }
        let (width, per_row) = if n < 100 { (2, 40) } else { (3, 26) };
        if self.table.len() != self.ncon.div_ceil(per_row) {
            return Err(invalid("FDAT published layout physical row count"));
        }
        let mut values = Vec::with_capacity(self.ncon);
        let mut tokens = Vec::with_capacity(self.ncon);
        for row in &self.table {
            let active = width * (self.ncon - values.len()).min(per_row);
            if row.len() < active || !row.as_bytes()[active..].iter().all(|x| *x == b' ') {
                return Err(invalid("FDAT connection record width or padding"));
            }
            for token in row.as_bytes()[..active].chunks_exact(width) {
                let value = decimal(token)?;
                if value > n {
                    return Err(invalid(
                        "FDAT connection ordinal outside the source atom population",
                    ));
                }
                values.push(value);
                tokens.push(std::str::from_utf8(token).expect("checked ASCII"));
            }
        }
        if values.len() != self.ncon {
            return Err(invalid("FDAT connection count differs"));
        }
        let mut pairs = Vec::new();
        let mut dummy = Vec::new();
        for (i, &to) in values[..n].iter().enumerate() {
            if to == 0 {
                dummy.push(i + 1);
            } else {
                pairs.push([i + 1, to]);
            }
        }
        for pair in values[n..].chunks_exact(2) {
            if pair.contains(&0) {
                return Err(invalid("FDAT explicit pair has a dummy endpoint"));
            }
            pairs.push([pair[0], pair[1]]);
        }
        Ok(
            json!({"kind":"source-parsed","declaredAtoms":n,"sourceIntegerCount":self.ncon,
            "sourceWidth":width,"sourceIntegerTokens":tokens,"sourceIntegers":values,
            "directedSourcePairs":pairs,"sourceDummyOrdinals":dummy,"rawPhysicalRows":self.table,
            "activeGraphEstablished":false,"chemicalBondOrdersEstablished":false}),
        )
    }

    fn record(
        &self,
        document: &Value,
        resource: &PersistentResource,
        layout: FdatLayout,
    ) -> Result<Value> {
        let (mode, selected) = match layout {
            FdatLayout::Uninterpreted => ("legacy-compatibility", "native-preserve-uninterpreted"),
            FdatLayout::Published => ("corrected-explicit", "published-count-rule"),
        };
        let body = json!({
            "schemaVersion":"FdatConnectionObservation-proposal-1",
            "sourceResourceId":{"kind":"EntityRef","objectKind":"resource","objectId":resource.id},
            "sourceResourceSha256":resource.sha256,"sourceEntryOrdinal":"0",
            "NAT":self.nat,"NSAT":self.nsat,"NCON":self.ncon,"semanticsMode":mode,
            "connectionLayout":selected,"layoutEvidenceDigest":null,"observation":self.observation(layout)?
        });
        let mut tree = tagged(TAG, &body)?;
        tree["payload"]["members"]["sourceEntryOrdinal"] = json!({"kind":"U64","value":"0"});
        let span = |start: usize, end: usize| {
            json!({"sourceResourceId":resource.id,"byteOffset":start.to_string(),
            "byteLength":(end-start).to_string(),"sha256":digest(&self.raw[start..end])})
        };
        let mut spans = vec![span(0, 82)];
        if self.ncon != 0 {
            spans.push(span(self.table_offset, self.table_end));
        }
        let dialect = if matches!(layout, FdatLayout::Uninterpreted) && resource.sha256 == C57_SHA {
            C57
        } else {
            layout.dialect()
        };
        Ok(
            json!({"owner":{"kind":"document","id":document},"producerId":"draft.fdat-connections",
            "producerVersion":"1","formatRef":"VM18-I09","dialect":dialect,
            "tree":tree,"spans":spans}),
        )
    }
}

pub fn open_selected_fdat(raw: &[u8], layout: FdatLayout, mut ids: Ids) -> Result<RestoredProject> {
    let source = Source::read(raw)?;
    let mut capture = source_only_capture(&mut ids);
    let resource = PersistentResource {
        id: ids.next_id().into(),
        logical_key: KEY.into(),
        sha256: digest(raw),
        byte_length: raw.len().to_string(),
        media_type: "application/octet-stream".into(),
    };
    capture
        .resources
        .insert(resource.sha256.clone(), raw.to_vec());
    capture.content.resources.push(resource.clone());
    let mut value = full_source7::source_only_base(&capture);
    let row = source.record(&value["identity"]["documentId"], &resource, layout)?;
    value["metadata"] = json!([row]);
    capture.source = Some(SourceDocument::decode(value)?);
    full_source7::capture(&capture)?;
    RestoredProject::new(capture, ids)
}

pub(super) fn verify(capture: &Capture, source: &Value) -> Result<()> {
    let mut records = source["metadata"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| {
            row["tree"]["tag"] == TAG
                || matches!(
                    row["dialect"].as_str(),
                    Some(UNINTERPRETED | PUBLISHED | C57)
                )
        });
    let slot = capture
        .content
        .resources
        .iter()
        .find(|r| r.logical_key == KEY);
    match (records.next(), slot) {
        (None, None) => Ok(()),
        (Some(row), Some(resource)) if records.next().is_none() => {
            let raw = capture
                .resources
                .get(&resource.sha256)
                .filter(|b| {
                    b.len().to_string() == resource.byte_length && digest(b) == resource.sha256
                })
                .ok_or_else(|| invalid("FDAT owned source differs"))?;
            if *row
                != Source::read(raw)?.record(
                    &source["identity"]["documentId"],
                    resource,
                    FdatLayout::parse_dialect(&row["dialect"])?,
                )?
            {
                return Err(invalid(
                    "FDAT source observation differs from directory and exact record",
                ));
            }
            Ok(())
        }
        _ => Err(invalid(
            "FDAT observation and actual resource lack a unique binding",
        )),
    }
}
