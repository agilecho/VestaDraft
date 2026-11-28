//! Independent discovery under the selected native source rules.
//! The reader is a host capability during capture and an immutable map on reopen.
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::source7::resource_graph::*;
use vesta_model::{Error, Result, digest};

mod r80;
mod density;
mod rietan;
mod selected;
pub use density::is_c1_density_source;
pub(super) use density::read_c1_density;
pub use rietan::is_rietan_source;
pub(super) use rietan::read_rietan_phase;
pub(super) use rietan::binding as rietan_binding;
pub use r80::{R80PhaseImport, import_r80_phase, is_r80_source, materialize_r80_orffe};
pub(super) use r80::{r80_resource_key, verify_r80_binding};

const HEADER: &str = "#VESTA_FORMAT_VERSION 3.5.4";
const FROZEN_RULES: &str = include_str!("../../contracts/resource-graph.contract.json");
const FROZEN_WIRE: &str = include_str!("../../contracts/resource-graph.schema.json");
const GRAPH_RULES: &str = include_str!("../../contracts/resource-graph-source-bindings.json");
const PARSE_BUDGET: &str = include_str!("../../contracts/parse-budget.contract.json");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphProfile {
    NativeVesta,
    /// Selected 1 July 2016 C1 Cartesian/global-basis source pair. Association is source-only.
    GamessPair,
    /// Exact selected RIETAN input/report identities and their optional result.
    Rietan,
    /// Exact local engineering fixture for the frozen FEFF10 input lexer.
    Feff10Engineering,
}

impl GraphProfile {
    fn policy(self) -> String {
        let (id, inputs): (&str, &[&str]) = match self {
            Self::NativeVesta => (
                "native-vesta-354-references/2",
                &[
                    selected::VESTA_RULE,
                    selected::DENSITY_RULE,
                    selected::GRID_RULE,
                    selected::ORFFE_RULE,
                    selected::ORFFE_FIXTURE_PRIMARY,
                ],
            ),
            Self::GamessPair => ("gamess-c1-source-pair/1", &[selected::GAMESS_RULE]),
            Self::Rietan => (
                "rietan-selected-source-companion/1",
                &[
                    selected::RIETAN_RULE,
                    selected::ORFFE_RULE,
                    selected::FAPATITE_INS,
                    selected::AGI_LST,
                ],
            ),
            Self::Feff10Engineering => (
                "feff10-d861cf8-engineering-closure/1",
                &[
                    selected::FEFF_RULE,
                    selected::FEFF_LEXER,
                    selected::FEFF_FIXTURE_PRIMARY,
                    selected::FEFF_FIXTURE_CHILD,
                    selected::FEFF_FIXTURE_SHARED,
                ],
            ),
        };
        digest(
            format!(
                "{id}\n{FROZEN_RULES}\n{FROZEN_WIRE}\n{GRAPH_RULES}\n{PARSE_BUDGET}\n{}",
                inputs.join("\n")
            )
            .as_bytes(),
        )
    }
}

pub const FEFF_VIRTUAL_JOB: &str = "job";

/// The release-qualified engineering job stops in this exact child stream.
/// The graph verifier has already replayed expansion, including active END.
pub(super) const FEFF_END_CHILD: &str = "job/sub/atoms.inc";

pub(super) fn feff_end_tail(bytes: &[u8]) -> Result<usize> {
    let key = FEFF_END_CHILD;
    if digest(bytes) != selected::FEFF_FIXTURE_CHILD {
        return Err(selected::format_error(key, "unqualified FEFF END stream"));
    }
    let mut start = 0;
    for raw in selected::ascii(bytes, key)?.split_inclusive('\n') {
        let end = start + raw.len();
        if matches!(selected::feff_line(raw.trim_end_matches(['\r','\n']), start, key)?, selected::FeffLine::End)
            && end < bytes.len()
        {
            return Ok(end);
        }
        start = end;
    }
    Err(selected::format_error(key, "expected active END with a retained suffix"))
}

#[derive(Clone, Copy, Debug)]
pub struct CaptureLimits {
    pub bytes: usize,
    pub records: usize,
    pub nodes: usize,
    pub references: usize,
    pub depth: usize,
    pub work: usize,
    pub samples: usize,
}
impl Default for CaptureLimits {
    fn default() -> Self {
        Self {
            bytes: 8 * 1024 * 1024,
            records: 300_000,
            nodes: 128,
            references: 2048,
            depth: 32,
            work: 32 * 1024 * 1024,
            samples: 2 * 1024 * 1024,
        }
    }
}

#[derive(Debug)]
pub struct GraphArchive {
    pub graph: CapturedGraph,
    pub files: BTreeMap<String, Vec<u8>>,
}

fn parse_limit(name: &str, key: &str) -> Error {
    Error::new("E_PARSE_LIMIT", format!("native graph {name} at {key}"))
}

fn consume(available: &mut usize, used: usize, name: &str, key: &str) -> Result<()> {
    *available = available
        .checked_sub(used)
        .ok_or_else(|| parse_limit(name, key))?;
    Ok(())
}

pub(super) fn graph_rule() -> String {
    digest(format!("{AVAILABLE_RULES}\n{FROZEN_RULES}\n{FROZEN_WIRE}").as_bytes())
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8] = b"0123456789abcdef";
    let mut value = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        value.push(DIGITS[usize::from(byte >> 4)] as char);
        value.push(DIGITS[usize::from(byte & 15)] as char);
    }
    value
}

fn resolve_relative(parent: &str, literal: &str) -> Result<String> {
    resolve_reference(parent, literal, false)
}

fn resolve_reference(parent: &str, literal: &str, legacy_artifact: bool) -> Result<String> {
    let failure = || Error::new("E_READ_ONLY", format!("unqualified path in {parent}"));
    if literal.len() > 1024
        || literal.starts_with('/')
        || literal.contains(['\\', ':'])
        || literal.chars().any(char::is_control)
    {
        return Err(failure());
    }
    let mut path: Vec<_> = parent
        .rsplit_once('/')
        .map_or_else(Vec::new, |(dir, _)| dir.split('/').collect());
    let mut descended = false;
    for part in literal.split('/') {
        match part {
            "." => {}
            ".." => {
                // Native reads never erase a previously named component. Its
                // real file type or symlink target would otherwise be ignored.
                if descended && !legacy_artifact {
                    return Err(failure());
                }
                path.pop().ok_or_else(failure)?;
            }
            "" => return Err(failure()),
            other => {
                descended = true;
                path.push(other);
            }
        }
    }
    if path.is_empty() || matches!(literal.rsplit('/').next(), Some("." | "..")) {
        return Err(failure());
    }
    Ok(path.join("/"))
}

struct Occurrence {
    offset: usize,
    literal: String,
    kind: EdgeKind,
    rule_id: &'static str,
}

fn data_line(line: &str) -> bool {
    line.as_bytes()
        .first()
        .is_some_and(|b| b.is_ascii_digit() || b"+-.".contains(b))
}

fn occurrences(bytes: &[u8], key: &str, budget: &mut CaptureLimits) -> Result<Vec<Occurrence>> {
    if digest(bytes) == selected::ORFFE_FIXTURE_PRIMARY {
        let path = "controlled-identity0.ffe";
        let offset = selected::native_orffe_path_offset(bytes, path, key)?;
        consume(
            &mut budget.records,
            bytes.iter().filter(|&&b| b == b'\n').count(),
            "records",
            key,
        )?;
        consume(&mut budget.references, 1, "references", key)?;
        return Ok(vec![Occurrence {
            offset,
            literal: path.into(),
            kind: EdgeKind::GeometryImport,
            rule_id: "native-vesta-354-explicit-orffe/1",
        }]);
    }
    if digest(bytes) == selected::C1_DENSITY_PRIMARY {
        let path = "temporary-C1-density.pgrid";
        let prefix = "\nIMPORT_DENSITY 1\n+1.000000 ";
        let literal = format!("{prefix}{path}\n\n");
        let offset = selected::ascii(bytes, key)?
            .find(&literal)
            .ok_or_else(|| selected::format_error(key, "selected density import row missing"))?
            + prefix.len();
        consume(&mut budget.records, bytes.split(|&b| b == b'\n').count(), "records", key)?;
        consume(&mut budget.references, 1, "references", key)?;
        return Ok(vec![Occurrence {
            offset, literal: path.into(), kind: EdgeKind::DensityImport,
            rule_id: "native-vesta-354-density-flat-grid/1",
        }]);
    }
    let source = std::str::from_utf8(bytes).map_err(|_| {
        Error::new(
            "E_REFERENCE_RULE",
            format!("native graph source is not UTF-8: {key}"),
        )
    })?;
    let mut offset = 0;
    let mut lines = source.split_inclusive('\n');
    let mut next_line = || -> Result<Option<(usize, &str)>> {
        let Some(line) = lines.next() else {
            return Ok(None);
        };
        budget.records = budget
            .records
            .checked_sub(1)
            .ok_or_else(|| parse_limit("records", key))?;
        let start = offset;
        offset += line.len();
        Ok(Some((start, line.trim_end_matches(['\r', '\n']))))
    };
    let rule = |detail: &str| Error::new("E_REFERENCE_RULE", format!("{key}: {detail}"));
    if next_line()?.map(|(_, s)| s) != Some(HEADER) {
        return Err(rule("selected unstamped VESTA 3.5.4 header required"));
    }
    let mut found = vec![];
    let mut started = false;
    let mut numeric = false;
    while let Some((_, line)) = next_line()? {
        let head = line.trim_start();
        if head.trim_end().is_empty() {
            numeric = false;
            continue;
        }
        if !started {
            if head != "CRYSTAL" {
                return Err(rule("selected CRYSTAL framing required"));
            }
            started = true;
            continue;
        }
        let section = head.split_ascii_whitespace().next().unwrap_or("");
        match section {
            "COMMENT" if head == "COMMENT" => {
                // The selected native projection uses COMMENT as the final text block.
                while next_line()?.is_some() {}
                break;
            }
            "TITLE" if head == "TITLE" => {
                while let Some((_, body)) = next_line()? {
                    if body.trim().is_empty() {
                        break;
                    }
                }
                numeric = false;
            }
            "IMPORT_STRUCTURE" | "IMPORT_ORFFE" if head == section => {
                let (start, path) = next_line()?.ok_or_else(|| rule("structure path missing"))?;
                if path.trim().is_empty()
                    || next_line()?.is_none_or(|(_, line)| !line.trim().is_empty())
                {
                    return Err(rule(
                        "structure import requires one path and a blank delimiter",
                    ));
                }
                budget.references = budget
                    .references
                    .checked_sub(1)
                    .ok_or_else(|| parse_limit("references", key))?;
                found.push(Occurrence {
                    offset: start,
                    literal: path.into(),
                    kind: if section == "IMPORT_STRUCTURE" {
                        EdgeKind::StructureImport
                    } else {
                        EdgeKind::GeometryImport
                    },
                    rule_id: if section == "IMPORT_STRUCTURE" {
                        AVAILABLE_RULES
                    } else {
                        "native-vesta-354-explicit-orffe/1"
                    },
                });
                numeric = false;
            }
            "IMPORT_DENSITY" | "IMPORT_TEXTURE" => {
                let density = section == "IMPORT_DENSITY";
                if if density {
                    let mut tokens = head.split_ascii_whitespace();
                    tokens.next();
                    tokens
                        .next()
                        .is_some_and(|mode| mode.parse::<i32>().is_err())
                        || tokens.next().is_some()
                } else {
                    head != section
                } {
                    return Err(rule("unsupported resource section header"));
                }
                let before = found.len();
                loop {
                    let (start, row) = next_line()?
                        .ok_or_else(|| rule("resource list missing blank delimiter"))?;
                    if row.trim().is_empty() {
                        break;
                    }
                    let (operation, path) = row
                        .split_once(' ')
                        .ok_or_else(|| rule("resource operation and literal path required"))?;
                    if !selected::scalar_reference_operation(operation) || path.is_empty() {
                        return Err(rule("unqualified resource operation or empty path"));
                    }
                    consume(&mut budget.references, 1, "references", key)?;
                    found.push(Occurrence {
                        offset: start + operation.len() + 1,
                        literal: path.into(),
                        kind: if density {
                            EdgeKind::DensityImport
                        } else {
                            EdgeKind::TextureImport
                        },
                        rule_id: if density {
                            "native-vesta-354-density-flat-grid/1"
                        } else {
                            "native-vesta-354-texture-flat-grid/1"
                        },
                    });
                }
                if before == found.len() {
                    return Err(rule("empty resource list"));
                }
                numeric = false;
            }
            "GROUP" | "SYMOP" | "CELLP" | "STRUC" | "THERI" | "BOUND" | "SCENE" | "STYLE"
            | "DISPF" | "MODEL" | "PROJT" => {
                numeric = true;
            }
            _ if numeric && data_line(head) => {}
            _ => return Err(rule(&format!("unsupported source record {section}"))),
        }
    }
    if !started {
        return Err(rule("empty native project source"));
    }
    Ok(found)
}

struct Discovery<F> {
    read: F,
    budget: CaptureLimits,
    depth: usize,
    raw: BTreeMap<String, Vec<u8>>,
    files: BTreeMap<String, GraphFile>,
    captures: BTreeMap<String, NodeCapture>,
    active: BTreeSet<String>,
    heights: BTreeMap<String, usize>,
    profile: GraphProfile,
    policy: String,
    extended: bool,
    legacy_artifact: bool,
    edges: Vec<GraphEdge>,
    probes: Vec<OptionalProbe>,
}

impl<F: FnMut(&str, usize) -> Result<Option<Vec<u8>>>> Discovery<F> {
    fn load(&mut self, key: &str, rule: &str, optional: bool) -> Result<bool> {
        let rule_digest = digest(format!("{}\n{rule}", self.policy).as_bytes());
        if let Some(old) = self.files.get(key) {
            if old.reference_rule_digest != rule_digest {
                return Err(Error::new(
                    "E_REFERENCE_RULE",
                    format!("incompatible file roles at {key}"),
                ));
            }
            return Ok(true);
        }
        if self.budget.nodes == 0 && !optional {
            return Err(parse_limit("nodes", key));
        }
        let allowance = if self.budget.nodes == 0 {
            0
        } else {
            self.budget.bytes
        };
        let Some(raw) = (self.read)(key, allowance)? else {
            return if optional {
                Ok(false)
            } else {
                Err(Error::new("E_REOPEN_REFERENCE", key))
            };
        };
        consume(&mut self.budget.nodes, 1, "nodes", key)?;
        consume(&mut self.budget.bytes, raw.len(), "bytes", key)?;
        let main = GraphFile {
            key: key.into(),
            sha256: digest(&raw),
            byte_length: raw.len().to_string(),
            original_name_hex: hex(key.as_bytes()),
            reference_rule_digest: rule_digest,
        };
        self.captures.insert(
            key.into(),
            NodeCapture {
                primary_key: key.into(),
                captured: CapturedInput {
                    primary_sha256: main.sha256.clone(),
                    primary_name_hex: main.original_name_hex.clone(),
                    references: Vec::new(),
                    files: Vec::new(),
                },
            },
        );
        self.files.insert(key.into(), main);
        self.raw.insert(key.into(), raw);
        Ok(true)
    }

    fn work(&mut self, key: &str) -> Result<()> {
        consume(&mut self.budget.work, self.raw[key].len(), "work", key)
    }

    fn explicit(&mut self, key: &str, target: String, reference: Occurrence, recursive: bool) {
        let index = self.captures[key].captured.references.len();
        let prefix = match reference.kind {
            EdgeKind::StructureImport => "structure",
            EdgeKind::DensityImport => "density",
            EdgeKind::TextureImport => "texture",
            EdgeKind::GeometryImport => "orffe",
            EdgeKind::ProducerInclude => reference.rule_id,
            EdgeKind::ProducerPair => unreachable!("producer pair has an implicit name"),
        };
        let reference_id = if reference.kind == EdgeKind::ProducerInclude {
            format!("{prefix}/{}/{index}", reference.offset)
        } else {
            format!("{prefix}/{}", reference.offset)
        };
        let resource = &self.files[&target];
        let source = SourceReference::Explicit(ExplicitReference {
            reference_id: reference_id.clone(),
            source_primary_sha256: self.files[key].sha256.clone(),
            path_span: Span {
                byte_offset: reference.offset.to_string(),
                byte_length: reference.literal.len().to_string(),
                sha256: digest(reference.literal.as_bytes()),
            },
            literal_path_hex: hex(reference.literal.as_bytes()),
            resource_key: target.clone(),
            resource_sha256: resource.sha256.clone(),
            resource_byte_length: resource.byte_length.clone(),
            owner_locator: format!("/references/{index}"),
            rule_id: reference.rule_id.into(),
            rule_digest: digest(format!("{}\n{}", self.policy, reference.rule_id).as_bytes()),
            edge_kind: Explicit::Explicit,
        });
        self.bind(key, target, reference_id, reference.kind, recursive, source);
    }

    fn implicit(
        &mut self,
        key: &str,
        target: String,
        reference_id: &str,
        rule: &str,
        kind: EdgeKind,
    ) {
        let index = self.captures[key].captured.references.len();
        let file = &self.files[&target];
        let source = SourceReference::Implicit(ImplicitReference {
            edge_kind: Implicit::Implicit,
            reference_id: reference_id.into(),
            source_primary_sha256: self.files[key].sha256.clone(),
            seed_reference_id: "primary-name".into(),
            derived_name_hex: hex(leaf(&target).as_bytes()),
            resource_key: target.clone(),
            resource_sha256: file.sha256.clone(),
            resource_byte_length: file.byte_length.clone(),
            owner_locator: format!("/references/{index}"),
            rule_id: rule.into(),
            rule_digest: digest(format!("{}\n{rule}", self.policy).as_bytes()),
        });
        self.bind(key, target, reference_id.into(), kind, false, source);
    }

    fn bind(
        &mut self,
        key: &str,
        target: String,
        reference_id: String,
        kind: EdgeKind,
        recursive: bool,
        source: SourceReference,
    ) {
        let resource = &self.files[&target];
        let capture = &mut self
            .captures
            .get_mut(key)
            .expect("captured declaring file")
            .captured;
        match capture.files.binary_search_by(|file| file.key.cmp(&target)) {
            Ok(_) => {}
            Err(index) => capture.files.insert(
                index,
                FileBinding {
                    key: target.clone(),
                    sha256: resource.sha256.clone(),
                    byte_length: resource.byte_length.clone(),
                },
            ),
        }
        capture.references.push(source);
        self.edges.push(GraphEdge {
            from_key: key.into(),
            to_key: target,
            reference_id,
            kind,
            recursive_decode: recursive,
        });
    }

    fn visit_native(&mut self, key: &str, depth: usize) -> Result<usize> {
        if self.active.contains(key) {
            return Err(Error::new("E_GRAPH_CYCLE", key));
        }
        let height = self.heights.get(key).copied().unwrap_or_default();
        if depth.checked_add(height).is_none_or(|n| n > self.depth) {
            return Err(parse_limit("depth", key));
        }
        if let Some(&height) = self.heights.get(key) {
            return Ok(height);
        }
        self.load(key, AVAILABLE_RULES, false)?;
        self.work(key)?;
        let refs = occurrences(&self.raw[key], key, &mut self.budget)?;
        self.active.insert(key.into());
        let mut height = 0;
        for reference in refs {
            let target = resolve_reference(key, &reference.literal, self.legacy_artifact)?;
            let recursive = reference.kind == EdgeKind::StructureImport;
            if recursive {
                height = height.max(1 + self.visit_native(&target, depth + 1)?);
            } else {
                self.extended = true;
                match reference.kind {
                    EdgeKind::DensityImport | EdgeKind::TextureImport => {
                        if !self.files.contains_key(&target) {
                            self.load(&target, "grid3.0-flat-one-scalar-le32/1", false)?;
                            self.work(&target)?;
                            selected::grid(&self.raw[&target], &target, &mut self.budget)?;
                        } else {
                            self.load(&target, "grid3.0-flat-one-scalar-le32/1", false)?;
                        }
                    }
                    EdgeKind::GeometryImport => {
                        let is_new = !self.files.contains_key(&target);
                        self.load(&target, "orffe-result-identity0/1", false)?;
                        if is_new {
                            self.work(&target)?;
                        }
                        // Each declaring owner is checked even when sharing the same table bytes.
                        selected::verify_orffe(
                            &self.raw[key],
                            &self.raw[&target],
                            &target,
                            &mut self.budget,
                        )?;
                    }
                    _ => unreachable!("native VESTA parser emitted a producer reference"),
                }
            }
            self.explicit(key, target, reference, recursive);
        }
        self.active.remove(key);
        self.heights.insert(key.into(), height);
        Ok(height)
    }

    fn gamess_pair(&mut self, root: &str) -> Result<()> {
        let extension = suffix(root);
        if !matches!(extension, "inp" | "mmp") {
            return Err(selected::format_error(
                root,
                "exact .inp/.mmp spelling required",
            ));
        }
        let (inp, mmp) = if extension == "inp" {
            (root.into(), sibling(root, "mmp")?)
        } else {
            (sibling(root, "inp")?, root.into())
        };
        self.load(&inp, "gamess-2016-c1-input/1", false)?;
        self.load(&mmp, "macmolplt-small-flat-surface/1", false)?;
        self.work(&inp)?;
        self.work(&mmp)?;
        selected::gamess(&self.raw[&inp], &inp, &mut self.budget)?;
        selected::surface(&self.raw[&mmp], &mmp, &mut self.budget)?;
        consume(&mut self.budget.references, 2, "references", root)?;
        let rule = "gamess-native-same-stem-association/1";
        self.implicit(&inp, mmp.clone(), "pair/mmp", rule, EdgeKind::ProducerPair);
        self.implicit(&mmp, inp, "pair/inp", rule, EdgeKind::ProducerPair);
        Ok(())
    }

    fn rietan(&mut self, root: &str) -> Result<()> {
        self.load(root, "rietan-selected-input-or-report/1", false)?;
        self.work(root)?;
        selected::rietan(&self.raw[root], root, &mut self.budget)?;
        consume(&mut self.budget.references, 1, "optional probes", root)?;
        let target = sibling(root, "ffe")?;
        let rule = "rietan-native-same-stem-optional-ffe/1";
        let id = "optional/ffe";
        let result = if self.load(&target, "orffe-result-identity0/1", true)? {
            self.work(&target)?;
            selected::verify_orffe(
                &self.raw[root],
                &self.raw[&target],
                &target,
                &mut self.budget,
            )?;
            self.implicit(root, target.clone(), id, rule, EdgeKind::GeometryImport);
            ProbeResult::Present {
                to_key: target.clone(),
                reference_id: id.into(),
            }
        } else {
            ProbeResult::Absent
        };
        self.probes.push(OptionalProbe {
            id: id.into(),
            from_key: root.into(),
            derived_name_hex: hex(leaf(&target).as_bytes()),
            rule_id: rule.into(),
            rule_digest: digest(format!("{}\n{rule}", self.policy).as_bytes()),
            result,
        });
        Ok(())
    }

    fn check_feff_depth(&self, key: &str, depth: usize) -> Result<()> {
        if self.active.contains(key) {
            return Err(Error::new("E_GRAPH_CYCLE", key));
        }
        // Frozen FEFF's 10 streams include the primary.
        if depth >= 10 || depth > self.depth {
            return Err(parse_limit("FEFF stream depth", key));
        }
        Ok(())
    }

    /// Returns whether the shared producer stream encountered an active END.
    fn visit_feff(&mut self, key: &str, depth: usize) -> Result<bool> {
        self.check_feff_depth(key, depth)?;
        self.load(key, "feff10-selected-input-stream/1", false)?;
        if depth == 0
            && (self.files[key].sha256 != selected::FEFF_FIXTURE_PRIMARY || key != "job/main.inp")
        {
            return Err(Error::new(
                "E_REFERENCE_RULE",
                "FEFF10 source lacks the pinned engineering job identity",
            ));
        }
        self.work(key)?;
        let stream = self.raw[key].clone();
        let text = selected::ascii(&stream, key)?;
        self.active.insert(key.into());
        let mut offset = 0;
        for raw in text.split_inclusive('\n') {
            consume(&mut self.budget.records, 1, "records", key)?;
            let line = raw.strip_suffix('\n').unwrap_or(raw);
            let line = line.strip_suffix('\r').unwrap_or(line);
            match selected::feff_line(line, offset, key)? {
                selected::FeffLine::End => {
                    self.active.remove(key);
                    return Ok(true);
                }
                selected::FeffLine::Reference(reference) => {
                    consume(&mut self.budget.references, 1, "references", key)?;
                    let target = format!(
                        "{FEFF_VIRTUAL_JOB}/{}",
                        resolve_relative("", &reference.literal)?
                    );
                    // Bind each call before descending so the edge stream records expansion order.
                    self.check_feff_depth(&target, depth + 1)?;
                    self.load(&target, "feff10-selected-input-stream/1", false)?;
                    self.explicit(key, target.clone(), reference, true);
                    if self.visit_feff(&target, depth + 1)? {
                        self.active.remove(key);
                        return Ok(true);
                    }
                }
                selected::FeffLine::Record => {}
            }
            offset += raw.len();
        }
        self.active.remove(key);
        Ok(false)
    }

    fn finish(mut self, root: &str) -> Result<GraphArchive> {
        if self.profile == GraphProfile::NativeVesta {
            // Native lists are source-ordered within each file. The C7 shape also
            // fixed order between files and a different digest grammar.
            self.edges.sort_by(|a, b| a.from_key.cmp(&b.from_key));
            if !self.extended {
                let legacy = graph_rule();
                for file in self.files.values_mut() {
                    file.reference_rule_digest = legacy.clone();
                }
                for capture in self.captures.values_mut() {
                    for reference in &mut capture.captured.references {
                        if let SourceReference::Explicit(reference) = reference {
                            reference.rule_digest = legacy.clone();
                        }
                    }
                }
                self.policy = legacy;
            }
        }
        let mut graph = CapturedGraph {
            root_key: root.into(),
            files: self.files.into_values().collect(),
            captures: self.captures.into_values().collect(),
            edges: self.edges,
            optional_probes: self.probes,
            policy_digest: self.policy,
            host_snapshot_id: String::new(),
        };
        // Content identity only. The capture reader's filesystem permission is process-local.
        graph.host_snapshot_id = digest(
            &serde_json::to_vec(&graph)
                .map_err(|error| Error::new("E_SCHEMA", error.to_string()))?,
        );
        Ok(GraphArchive {
            graph,
            files: self.raw,
        })
    }
}

fn leaf(key: &str) -> &str {
    key.rsplit('/').next().expect("nonempty virtual key")
}

fn suffix(key: &str) -> &str {
    leaf(key)
        .rsplit_once('.')
        .map_or("", |(_, extension)| extension)
}

fn sibling(key: &str, extension: &str) -> Result<String> {
    let (stem, _) = leaf(key)
        .rsplit_once('.')
        .filter(|(stem, _)| !stem.is_empty())
        .ok_or_else(|| selected::format_error(key, "same-stem filename required"))?;
    resolve_relative(key, &format!("{stem}.{extension}"))
}

pub fn discover_graph(
    root: &str,
    read: impl FnMut(&str, usize) -> Result<Option<Vec<u8>>>,
    limits: CaptureLimits,
) -> Result<GraphArchive> {
    discover_graph_with_profile(root, GraphProfile::NativeVesta, read, limits)
}

pub fn discover_graph_with_profile(
    root: &str,
    profile: GraphProfile,
    read: impl FnMut(&str, usize) -> Result<Option<Vec<u8>>>,
    limits: CaptureLimits,
) -> Result<GraphArchive> {
    discover(root, profile, read, limits, false)
}

fn discover(
    root: &str,
    profile: GraphProfile,
    read: impl FnMut(&str, usize) -> Result<Option<Vec<u8>>>,
    limits: CaptureLimits,
    legacy_artifact: bool,
) -> Result<GraphArchive> {
    if resolve_relative("", root)? != root {
        return Err(Error::new(
            "E_READ_ONLY",
            "native graph root key must be canonical",
        ));
    }
    let mut discovery = Discovery {
        read,
        budget: limits,
        depth: limits.depth.min(64),
        raw: BTreeMap::new(),
        files: BTreeMap::new(),
        captures: BTreeMap::new(),
        active: BTreeSet::new(),
        heights: BTreeMap::new(),
        profile,
        policy: profile.policy(),
        extended: false,
        legacy_artifact,
        edges: Vec::new(),
        probes: Vec::new(),
    };
    match profile {
        GraphProfile::NativeVesta => {
            discovery.visit_native(root, 0)?;
        }
        GraphProfile::GamessPair => discovery.gamess_pair(root)?,
        GraphProfile::Rietan => discovery.rietan(root)?,
        GraphProfile::Feff10Engineering => {
            if !discovery.visit_feff(root, 0)? {
                return Err(selected::format_error(root, "missing active FEFF END"));
            }
            if discovery.files.len() != 3
                || discovery
                    .files
                    .get("job/sub/atoms.inc")
                    .is_none_or(|file| file.sha256 != selected::FEFF_FIXTURE_CHILD)
                || discovery
                    .files
                    .get("job/shared.inc")
                    .is_none_or(|file| file.sha256 != selected::FEFF_FIXTURE_SHARED)
            {
                return Err(Error::new(
                    "E_REFERENCE_RULE",
                    "FEFF10 engineering job closure identity differs",
                ));
            }
        }
    }
    discovery.finish(root)
}

pub fn is_structure_source(bytes: &[u8]) -> bool {
    bytes
        .split(|byte| *byte == b'\n')
        .next()
        .is_some_and(|line| line.strip_suffix(b"\r").unwrap_or(line) == HEADER.as_bytes())
}

pub fn verify_artifact_graph(
    graph: &CapturedGraph,
    files: &BTreeMap<String, Vec<u8>>,
    limits: CaptureLimits,
) -> Result<()> {
    let profile = profile_for_graph(graph)?;
    let checked = discover(
        &graph.root_key,
        profile,
        |key, remaining| match files.get(key) {
            Some(bytes) if bytes.len() > remaining => Err(parse_limit("bytes", key)),
            bytes => Ok(bytes.cloned()),
        },
        limits,
        graph.policy_digest == graph_rule(),
    )?;
    if files.keys().ne(checked.files.keys()) {
        return Err(Error::new(
            "E_GRAPH_FILES",
            "artifact graph has unexpected files",
        ));
    }
    if *graph != checked.graph {
        return Err(Error::new(
            "E_GRAPH_DISCOVERY",
            "graph differs from actual reference discovery",
        ));
    }
    Ok(())
}

pub(super) fn profile_for_graph(graph: &CapturedGraph) -> Result<GraphProfile> {
    if graph.policy_digest == graph_rule() {
        return Ok(GraphProfile::NativeVesta);
    }
    [
        GraphProfile::NativeVesta,
        GraphProfile::GamessPair,
        GraphProfile::Rietan,
        GraphProfile::Feff10Engineering,
    ]
    .into_iter()
    .find(|profile| graph.policy_digest == profile.policy())
    .ok_or_else(|| Error::new("E_GRAPH_DISCOVERY", "unknown artifact graph rule"))
}

pub(super) fn expanded_rule_digest() -> String {
    let rules = [
        GraphProfile::NativeVesta,
        GraphProfile::GamessPair,
        GraphProfile::Rietan,
        GraphProfile::Feff10Engineering,
    ];
    digest(rules.map(GraphProfile::policy).join("\n").as_bytes())
}
