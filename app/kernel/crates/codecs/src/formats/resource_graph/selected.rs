//! Selected format parsers used to prove reference closure over actual bytes.
use super::{CaptureLimits, Occurrence, consume, suffix};
use std::collections::BTreeSet;
use vesta_model::source7::resource_graph::EdgeKind;
use vesta_model::{Error, Result, digest};

pub(super) const VESTA_RULE: &str =
    include_str!("../../../contracts/native-vesta-format-contract.json");
pub(super) const DENSITY_RULE: &str =
    include_str!("../../../contracts/native-resource-syntax-contract.json");
pub(super) const GRID_RULE: &str =
    include_str!("../../../contracts/binary-grid-format-contract.json");
pub(super) const ORFFE_RULE: &str = include_str!("../../../contracts/orffe-resource.contract.json");
pub(super) const GAMESS_RULE: &str =
    include_str!("../../../contracts/gamess-mmp-format-contract.json");
pub(super) const RIETAN_RULE: &str = include_str!("../../../contracts/rietan-format-contract.json");
pub(super) const FEFF_RULE: &str = include_str!("../../../contracts/feff10-source.contract.json");
pub(super) const FEFF_LEXER: &str = include_str!("../../../contracts/feff10-lexer.contract.json");
pub(super) const FEFF_FIXTURE_PRIMARY: &str =
    "12263ea6231c0cf690c10a173215c0f27d94e3a8e2c468cebf80cde7b6d1e8b9";
pub(super) const FEFF_FIXTURE_CHILD: &str =
    "2ede634fa6a972344eec197aaeecd79497146d3110abcbc87039f1763d1bc09e";
pub(super) const FEFF_FIXTURE_SHARED: &str =
    "f69f683693ac39ba807b4de7d0988e60e8656f82d233cf18d11f589319b41f4a";
pub(super) const ORFFE_FIXTURE_PRIMARY: &str =
    "5d4af2606713cce90e58dd89ecd0ed5908163248d8978bbf9e37e851a82777ce";
pub(super) const C1_DENSITY_PRIMARY: &str =
    "4e19562240ec86ff2e4a62c2352c189c8e2fd023dfabd5798e832181133b1460";
pub(super) const FAPATITE_INS: &str =
    "83d7ba293c38c7ee20ea22205a31a39ede21d0acdc228e29ed19478f74deefd7";
pub(super) const AGI_LST: &str = "47896b24f93eb137645f96611e2fd509412e6886eb70cf48ccccdd3e262280f3";

pub(super) fn format_error(key: &str, reason: &str) -> Error {
    Error::new("E_REFERENCE_FORMAT", format!("{key}: {reason}"))
}

pub(super) fn ascii<'a>(bytes: &'a [u8], key: &str) -> Result<&'a str> {
    if !bytes
        .iter()
        .all(|&b| b.is_ascii_graphic() || matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
    {
        return Err(format_error(
            key,
            "unqualified text encoding or control byte",
        ));
    }
    std::str::from_utf8(bytes).map_err(|_| format_error(key, "source not UTF-8"))
}

fn finite(token: &str) -> Option<f64> {
    let number = if token.contains(['d', 'D']) {
        token.replace(['d', 'D'], "e").parse::<f64>().ok()?
    } else {
        token.parse::<f64>().ok()?
    };
    number.is_finite().then_some(number)
}

fn tuple<const N: usize>(tokens: impl Iterator<Item = Option<f64>>) -> Option<[f64; N]> {
    let mut tokens = tokens;
    let mut parsed = [0.; N];
    for value in &mut parsed {
        *value = tokens.next()??;
    }
    tokens.next().is_none().then_some(parsed)
}

pub(super) fn grid(bytes: &[u8], key: &str, budget: &mut CaptureLimits) -> Result<()> {
    let invalid = || {
        format_error(
            key,
            "unsupported or malformed flat Grid 3.0 scalar resource",
        )
    };
    let periodic = match suffix(key) {
        "pgrid" => 1,
        "ggrid" => 0,
        _ => return Err(invalid()),
    };
    if bytes.len() < 152 {
        return Err(invalid());
    }
    let integer = |i| i32::from_le_bytes(bytes[i..i + 4].try_into().expect("checked header"));
    if [integer(0), integer(4), integer(8), integer(12)] != [3, 0, 0, 0]
        || [integer(96), integer(100), integer(104), integer(108)] != [periodic, 0, 1, 3]
    {
        return Err(invalid());
    }
    let minimum = if periodic == 1 { 1 } else { 2 };
    let mut samples = 1usize;
    for offset in [112, 116, 120] {
        let count = integer(offset);
        if count < minimum {
            return Err(invalid());
        }
        samples = samples.checked_mul(count as usize).ok_or_else(invalid)?;
    }
    if usize::try_from(integer(124)).ok() != Some(samples)
        || samples
            .checked_mul(4)
            .and_then(|payload| payload.checked_add(152))
            != Some(bytes.len())
    {
        return Err(invalid());
    }
    consume(&mut budget.samples, samples, "scalar samples", key)?;
    let real = |i| f32::from_le_bytes(bytes[i..i + 4].try_into().expect("checked header"));
    if [real(128), real(132), real(136)]
        .iter()
        .any(|n| !n.is_finite() || *n <= 0.)
    {
        return Err(invalid());
    }
    let angles = [real(140), real(144), real(148)];
    if angles
        .iter()
        .any(|n| !n.is_finite() || *n <= 0. || *n >= 180.)
    {
        return Err(invalid());
    }
    let [a, b, c] = angles.map(|n| f64::from(n).to_radians().cos());
    if 1. + 2. * a * b * c - a * a - b * b - c * c <= 0.
        || bytes[152..].chunks_exact(4).any(|sample| {
            !f32::from_le_bytes(sample.try_into().expect("exact sample size")).is_finite()
        })
    {
        return Err(invalid());
    }
    Ok(())
}

pub(super) enum FeffLine {
    Record,
    Reference(Occurrence),
    End,
}

fn feff_words(line: &str) -> Vec<(usize, &str)> {
    // Frozen BWORDS first-two-word partition, including an empty comma field.
    let mut out = Vec::with_capacity(2);
    let mut begin = None;
    let mut after_comma = true;
    let line = line.trim_end_matches([' ', '\t']);
    for (i, byte) in line.bytes().enumerate() {
        match byte {
            b' ' | b'\t' => {
                if let Some(start) = begin.take() {
                    out.push((start, &line[start..i]));
                    after_comma = false;
                }
            }
            b',' => {
                if let Some(start) = begin.take() {
                    out.push((start, &line[start..i]));
                } else if after_comma {
                    out.push((i, ""));
                }
                after_comma = true;
            }
            _ if begin.is_none() => begin = Some(i),
            _ => {}
        }
        if out.len() == 2 {
            return out;
        }
    }
    if let Some(start) = begin {
        out.push((start, &line[start..]));
    }
    out
}

pub(super) fn feff_line(line: &str, offset: usize, key: &str) -> Result<FeffLine> {
    let invalid = |reason| format_error(key, reason);
    if line.len() > 150 {
        return Err(invalid("FEFF input line would be truncated"));
    }
    let normalized = line.trim_start_matches([' ', '\t']);
    if normalized.is_empty() || normalized.starts_with([';', '*', '%', '#']) {
        return Ok(FeffLine::Record);
    }
    let words = feff_words(line);
    let (_, command) = words
        .first()
        .copied()
        .ok_or_else(|| invalid("empty FEFF command"))?;
    let keyword = command.to_ascii_lowercase();
    let reference_rule = match keyword.as_str() {
        "include" => "feff10-include/1",
        "load" => "feff10-load/1",
        "end" => return Ok(FeffLine::End),
        "title" | "reciprocal" | "lattice" | "coordinates" | "target" | "atoms" | "potentials" => {
            return Ok(FeffLine::Record);
        }
        _ if finite(command).is_some() => return Ok(FeffLine::Record),
        _ => return Err(invalid("unqualified active FEFF card")),
    };
    let (start, arg) = words
        .get(1)
        .copied()
        .ok_or_else(|| invalid("missing FEFF include file name"))?;
    if arg.len() > 90 {
        return Err(invalid("FEFF filename word would be truncated"));
    }
    let closer = match arg.as_bytes().first() {
        Some(b'"') => Some('"'),
        Some(b'{') => Some('}'),
        Some(b'(') => Some(')'),
        Some(b'<') => Some('>'),
        Some(b'\'') => Some('\''),
        Some(b'[') => Some(']'),
        _ => None,
    };
    let (path_start, path) = if let Some(closer) = closer {
        let (path, _) = arg[1..]
            .split_once(closer)
            .ok_or_else(|| invalid("unclosed FEFF filename delimiter"))?;
        (start + 1, path)
    } else {
        (start, arg)
    };
    if path.is_empty() {
        return Err(invalid("empty FEFF include file name"));
    }
    Ok(FeffLine::Reference(Occurrence {
        offset: offset + path_start,
        literal: path.into(),
        kind: EdgeKind::ProducerInclude,
        rule_id: reference_rule,
    }))
}

pub(super) fn gamess(bytes: &[u8], key: &str, budget: &mut CaptureLimits) -> Result<()> {
    let text = ascii(bytes, key)?;
    let invalid = || format_error(key, "unqualified GAMESS Cartesian/C1 global-basis input");
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    consume(&mut budget.records, text.lines().count(), "records", key)?;
    let control = lines.next().ok_or_else(invalid)?.to_ascii_uppercase();
    let control = control
        .strip_prefix("$CONTRL ")
        .and_then(|value| value.strip_suffix(" $END"))
        .ok_or_else(invalid)?;
    let mut keywords = BTreeSet::new();
    for setting in control.split_ascii_whitespace() {
        let (name, value) = setting.split_once('=').ok_or_else(invalid)?;
        if !keywords.insert(name)
            || !match name {
                "COORD" => matches!(value, "UNIQUE" | "CART" | "PRINAXIS"),
                "UNITS" => matches!(value, "ANGS" | "BOHR"),
                "SCFTYP" => matches!(value, "RHF" | "UHF"),
                "RUNTYP" => value == "ENERGY",
                _ => false,
            }
        {
            return Err(invalid());
        }
    }
    if !keywords.contains("COORD")
        || !keywords.contains("UNITS")
        || !lines
            .next()
            .is_some_and(|line| line.eq_ignore_ascii_case("$BASIS GBASIS=STO NGAUSS=3 $END"))
        || !lines
            .next()
            .is_some_and(|line| line.eq_ignore_ascii_case("$DATA"))
        || lines.next().is_none_or(|title| title.starts_with('$'))
        || !lines
            .next()
            .is_some_and(|line| line.eq_ignore_ascii_case("C1"))
    {
        return Err(invalid());
    }
    let mut count = 0;
    while let Some(row) = lines.next() {
        if row.eq_ignore_ascii_case("$END") {
            return if count > 0 && lines.next().is_none() {
                Ok(())
            } else {
                Err(invalid())
            };
        }
        let mut tokens = row.split_ascii_whitespace();
        let label = tokens.next().ok_or_else(invalid)?;
        let charge = tokens.next().and_then(finite).ok_or_else(invalid)?;
        if !label.bytes().all(|c| c.is_ascii_alphanumeric())
            || charge.fract() != 0.
            || charge <= 0.
            || charge > 118.
            || tuple::<3>(tokens.map(finite)).is_none()
        {
            return Err(invalid());
        }
        count += 1;
    }
    Err(invalid())
}

pub(super) fn surface(bytes: &[u8], key: &str, budget: &mut CaptureLimits) -> Result<()> {
    let text = ascii(bytes, key)?;
    let invalid = || format_error(key, "malformed selected MacMolPlt flat surface");
    consume(&mut budget.records, text.lines().count(), "records", key)?;
    let mut lines = text.lines();
    if lines.next().is_none_or(|title| title.trim().is_empty()) {
        return Err(invalid());
    }
    let mut dims = lines.next().ok_or_else(invalid)?.split_ascii_whitespace();
    let mut shape = [0; 3];
    for axis in &mut shape {
        *axis = dims
            .next()
            .and_then(|token| token.parse::<usize>().ok())
            .filter(|count| *count >= 2)
            .ok_or_else(invalid)?;
    }
    if dims.next().is_some() {
        return Err(invalid());
    }
    let mut triple =
        || -> Option<[f64; 3]> { tuple(lines.next()?.split_ascii_whitespace().map(finite)) };
    let origin = triple().ok_or_else(invalid)?;
    let increment = triple().ok_or_else(invalid)?;
    if origin
        .into_iter()
        .zip(increment)
        .zip(shape)
        .any(|((o, d), n)| d == 0. || !(o + (n - 1) as f64 * d).is_finite())
    {
        return Err(invalid());
    }
    let size = shape
        .into_iter()
        .try_fold(1usize, usize::checked_mul)
        .ok_or_else(invalid)?;
    consume(&mut budget.samples, size, "scalar samples", key)?;
    let mut samples = lines.flat_map(str::split_ascii_whitespace);
    for _ in 0..size {
        if samples.next().and_then(finite).is_none() {
            return Err(invalid());
        }
    }
    if samples.next().is_some() {
        return Err(invalid());
    }
    Ok(())
}

pub(super) fn rietan(bytes: &[u8], key: &str, budget: &mut CaptureLimits) -> Result<()> {
    let text = ascii(bytes, key)?;
    consume(&mut budget.records, text.lines().count(), "records", key)?;
    if !matches!(
        (suffix(key), digest(bytes).as_str()),
        ("ins", FAPATITE_INS) | ("lst", AGI_LST)
    ) {
        return Err(format_error(
            key,
            "RIETAN input/report has no selected source identity",
        ));
    }
    Ok(())
}

pub(super) fn native_orffe_path_offset(bytes: &[u8], expected: &str, key: &str) -> Result<usize> {
    let text = ascii(bytes, key)?;
    let marker = format!("\nIMPORT_ORFFE\n{expected}\n\n");
    text.find(&marker)
        .map(|offset| offset + "\nIMPORT_ORFFE\n".len())
        .ok_or_else(|| format_error(key, "pinned VESTA ORFFE path not found"))
}

#[derive(PartialEq)]
struct Site<'a> {
    label: &'a str,
    fractional: [f64; 3],
}

fn owner_sites<'a>(bytes: &'a [u8], key: &str) -> Result<Vec<Site<'a>>> {
    let text = ascii(bytes, key)?;
    let wrong = || {
        Error::new(
            "E_REOPEN_OWNER",
            format!("{key}: ORFFE owner has no qualified source site table"),
        )
    };
    let mut sites = Vec::new();
    match digest(bytes).as_str() {
        FAPATITE_INS => {
            let (_, table) = text.split_once("\nO1/O-").ok_or_else(wrong)?;
            for (i, row) in table.lines().take(7).enumerate() {
                let mut tokens = row.split_ascii_whitespace();
                let label = if i == 0 {
                    "O1"
                } else {
                    tokens
                        .next()
                        .and_then(|name| name.split_once('/'))
                        .map(|(name, _)| name)
                        .ok_or_else(wrong)?
                };
                tokens.next();
                let fractional = tuple(tokens.take(3).map(finite)).ok_or_else(wrong)?;
                sites.push(Site { label, fractional });
            }
        }
        AGI_LST => {
            let (_, table) = text
                .split_once(
                    "Structure parameters, g, x, y, z, B/Angstrom**2, and U/Angstrom**2, in AgI",
                )
                .ok_or_else(wrong)?;
            for (line, label) in [(4, "Ag1"), (6, "I1")] {
                let row = table.lines().nth(line).ok_or_else(wrong)?;
                let mut tokens = row.split_ascii_whitespace();
                if tokens.next() != Some(label) {
                    return Err(wrong());
                }
                let fractional = tuple(tokens.skip(7).take(3).map(finite)).ok_or_else(wrong)?;
                sites.push(Site { label, fractional });
            }
        }
        ORFFE_FIXTURE_PRIMARY => {
            let (_, table) = text.split_once("\nSTRUC\n").ok_or_else(wrong)?;
            for row in table.lines().step_by(2).take(7) {
                let mut tokens = row.split_ascii_whitespace();
                let label = tokens.nth(2).ok_or_else(wrong)?;
                let fractional = tuple(tokens.skip(1).take(3).map(finite)).ok_or_else(wrong)?;
                sites.push(Site { label, fractional });
            }
        }
        _ => return Err(wrong()),
    }
    Ok(sites)
}

pub(super) fn verify_orffe(
    owner: &[u8],
    table: &[u8],
    key: &str,
    budget: &mut CaptureLimits,
) -> Result<()> {
    parse_orffe(owner, table, key, budget).map(drop)
}

pub(super) struct SelectedOrffeRow<'a> {
    pub serial: usize,
    pub sites: Vec<usize>,
    pub value: &'a str,
    pub uncertainty: &'a str,
    pub native_value: f32,
    pub native_uncertainty: f32,
}

pub(super) fn parse_orffe<'a>(
    owner: &[u8],
    table: &'a [u8],
    key: &str,
    budget: &mut CaptureLimits,
) -> Result<Vec<SelectedOrffeRow<'a>>> {
    let invalid = || format_error(key, "malformed/unqualified identity-only ORFFE result");
    if suffix(key) != "ffe" {
        return Err(invalid());
    }
    let text = ascii(table, key)?;
    consume(&mut budget.records, text.lines().count(), "records", key)?;
    let mut lines = text.lines().map(str::trim);
    while lines.next().ok_or_else(invalid)? != "ATOMIC PARAMETERS FOR PHASE 1" {}
    lines.next().ok_or_else(invalid)?;
    lines.next().ok_or_else(invalid)?;
    let sites = owner_sites(owner, key)?;
    for (i, site) in sites.iter().enumerate() {
        let mut tokens = lines.next().ok_or_else(invalid)?.split_ascii_whitespace();
        let ordinal = tokens
            .next()
            .and_then(|token| token.parse::<usize>().ok())
            .ok_or_else(invalid)?;
        let label = tokens.next().ok_or_else(invalid)?;
        if label.len() > 6 || ordinal != i + 1 {
            return Err(invalid());
        }
        let mut floats = [0.; 6];
        for value in &mut floats {
            *value = tokens.next().and_then(finite).ok_or_else(invalid)?;
            tokens
                .next()
                .and_then(|token| token.parse::<i32>().ok())
                .ok_or_else(invalid)?;
        }
        if tokens.next().is_some() {
            return Err(invalid());
        }
        if label != site.label || floats[1..4] != site.fractional {
            return Err(Error::new(
                "E_REOPEN_OWNER",
                format!("{key}: ORFFE source site identity differs"),
            ));
        }
    }
    let mut lines = lines.filter(|line| !line.is_empty());
    if lines.next() != Some("SYMMETRY INFORMATION")
        || lines.next().is_none()
        || lines.next().is_none()
    {
        return Err(invalid());
    }
    let mut arity = 0;
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    while let Some(row) = lines.next() {
        if row == "INTERATOMIC DISTANCE IN ANGSTROMS" && arity == 0 {
            arity = 2;
            continue;
        }
        if row == "BOND ANGLE IN DEGREES.  CENTRAL ATOM IS VERTEX" && arity == 2 && !rows.is_empty()
        {
            arity = 3;
            continue;
        }
        if arity == 0 {
            return Err(invalid()); // Extra symmetry operators are not qualified in this rule.
        }
        let (serial, mut content) = row.split_once(' ').ok_or_else(invalid)?;
        let serial = serial
            .parse::<usize>()
            .ok()
            .filter(|serial| *serial > 0)
            .ok_or_else(invalid)?;
        if !seen.insert((arity, serial)) {
            return Err(invalid());
        }
        let mut site_ordinals = [0; 3];
        for selected in &mut site_ordinals[..arity] {
            let (pair, tail) = content
                .trim_start()
                .strip_prefix('(')
                .and_then(|pair| pair.split_once(')'))
                .ok_or_else(invalid)?;
            let (ordinal, code) = pair.split_once(',').ok_or_else(invalid)?;
            *selected = ordinal
                .trim()
                .parse::<usize>()
                .ok()
                .filter(|n| *n >= 1 && *n <= sites.len())
                .ok_or_else(invalid)?
                - 1;
            if code.trim().parse::<i32>().ok() != Some(0) {
                return Err(invalid());
            }
            content = tail;
        }
        let mut annotations = content.split(';');
        for &site in &site_ordinals[..arity] {
            let annot = annotations.next().ok_or_else(invalid)?;
            let (label, point) = annot.trim().split_once('(').ok_or_else(invalid)?;
            let coords = point
                .strip_suffix(')')
                .and_then(|tokens| tuple(tokens.split(',').map(|t| finite(t.trim()))))
                .ok_or_else(invalid)?;
            if label.trim() != sites[site].label || coords != sites[site].fractional {
                return Err(Error::new(
                    "E_REOPEN_OWNER",
                    format!("{key}: ORFFE operand annotation differs"),
                ));
            }
        }
        if annotations.next().is_some() {
            return Err(invalid());
        }
        let (value, uncertainty) = lines
            .next()
            .and_then(|line| line.split_once(" +OR- "))
            .ok_or_else(invalid)?;
        let native_value = value
            .parse::<f32>()
            .ok()
            .filter(|n| n.is_finite())
            .ok_or_else(invalid)?;
        let native_uncertainty = uncertainty
            .parse::<f32>()
            .ok()
            .filter(|n| n.is_finite() && *n >= 0.)
            .ok_or_else(invalid)?;
        rows.push(SelectedOrffeRow {
            serial,
            sites: site_ordinals[..arity].to_vec(),
            value,
            uncertainty,
            native_value,
            native_uncertainty,
        });
    }
    if rows.is_empty() {
        Err(invalid())
    } else {
        Ok(rows)
    }
}
