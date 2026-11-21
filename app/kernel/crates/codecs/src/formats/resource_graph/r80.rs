//! Exact R80 VESTA/ORFFE source-to-current conversion.
use super::{CaptureLimits, GraphArchive, selected, verify_artifact_graph};
use vesta_model::source_audit::canonical;
use vesta_model::source7::resource_graph::orffe::{
    R80GeometryMeasurement, R80GeometryTable, R80GeometryValue,
};
use vesta_model::source7::resource_graph::{EdgeKind, GRAPH_GROUP};
use vesta_model::{Cell3, Error, Ids, ImageKey, PersistentResource, Phase, Result, Site, digest};

const R80_TABLE_SHA: &str = "b8b1ed61eaf451b866202ba805f1ac2d5535ca3b6affc44ec4cffa0e7737f088";

pub fn is_r80_source(primary: &[u8]) -> bool {
    digest(primary) == selected::ORFFE_FIXTURE_PRIMARY
}

pub(crate) fn r80_resource_key(archive: &GraphArchive) -> Result<String> {
    Ok(exact_pair(archive)?.logical_key)
}

/// Phase decoded by the selected importer, bound to this immutable graph. An
/// existing phase with coincident geometry cannot be used as a source binding.
#[derive(Debug)]
pub struct R80PhaseImport {
    phase: Phase,
    graph_sha: String,
}

impl R80PhaseImport {
    pub fn phase(&self) -> &Phase {
        &self.phase
    }
}

fn qualification() -> Error {
    Error::new(
        "E_REFERENCE_RULE",
        "R80 phase import requires the exact selected VESTA/ORFFE pair",
    )
}

fn wrong_owner(reason: &str) -> Error {
    Error::new("E_REOPEN_OWNER", reason)
}

struct LockedPair<'a> {
    source: &'a [u8],
    table: &'a [u8],
    table_key: &'a str,
    logical_key: String,
    graph_sha: String,
}

fn exact_pair(archive: &GraphArchive) -> Result<LockedPair<'_>> {
    verify_artifact_graph(&archive.graph, &archive.files, CaptureLimits::default())?;
    let root = archive.graph.root_key.as_str();
    let source = archive.files.get(root).ok_or_else(qualification)?;
    let [edge] = archive.graph.edges.as_slice() else {
        return Err(qualification());
    };
    let target = edge.to_key.as_str();
    let table = archive.files.get(target).ok_or_else(qualification)?;
    if digest(source) != selected::ORFFE_FIXTURE_PRIMARY
        || digest(table) != R80_TABLE_SHA
        || edge.from_key != root
        || edge.kind != EdgeKind::GeometryImport
        || edge.recursive_decode
    {
        return Err(qualification());
    }
    let index = archive
        .graph
        .files
        .iter()
        .position(|file| file.key == target)
        .ok_or_else(qualification)?;
    let value = serde_json::to_value(&archive.graph)
        .map_err(|error| Error::new("E_SCHEMA", error.to_string()))?;
    let graph_sha = digest(&canonical(&value));
    Ok(LockedPair {
        source,
        table,
        table_key: target,
        logical_key: format!("{GRAPH_GROUP}/{graph_sha}/file/{index}"),
        graph_sha,
    })
}

fn section<'a>(text: &'a str, heading: &str) -> Result<&'a str> {
    text.split_once(heading)
        .map(|(_, data)| data)
        .ok_or_else(qualification)
}

fn numbers<const N: usize>(raw: &str) -> Result<[f64; N]> {
    let mut tokens = raw.split_ascii_whitespace();
    let mut values = [0.; N];
    for value in &mut values {
        *value = number(tokens.next())?;
    }
    if tokens.next().is_some() {
        return Err(qualification());
    }
    Ok(values)
}

fn number(raw: Option<&str>) -> Result<f64> {
    raw.and_then(|token| token.parse::<f64>().ok())
        .filter(|n| n.is_finite())
        .ok_or_else(qualification)
}

/// Decode the actual selected Owner phase, including source basis, identity
/// operator, exact ordered site coordinates, occupations and zero displacement.
pub fn import_r80_phase(archive: &GraphArchive, ids: &mut Ids) -> Result<R80PhaseImport> {
    let pair = exact_pair(archive)?;
    let text = selected::ascii(pair.source, &archive.graph.root_key)?;
    Ok(R80PhaseImport {
        phase: import_selected_p1(text, 7, "THERI 0", ids)?,
        graph_sha: pair.graph_sha,
    })
}

pub(super) fn import_selected_p1(
    text: &str,
    site_count: usize,
    thermal_section: &str,
    ids: &mut Ids,
) -> Result<Phase> {
    let title = section(text, "\nTITLE\n")?
        .lines()
        .next()
        .ok_or_else(qualification)?;
    let [a, b, c, alpha, beta, gamma] = numbers(
        section(text, "\nCELLP\n")?
            .lines()
            .next()
            .ok_or_else(qualification)?,
    )?;
    let cell = Cell3 {
        length_a: [a, b, c],
        angles_deg: [alpha, beta, gamma],
    };
    cell.validate()?;
    let [xmin, xmax, ymin, ymax, zmin, zmax] = numbers(
        section(text, "\nBOUND\n")?
            .lines()
            .next()
            .ok_or_else(qualification)?,
    )?;
    if !section(text, "\nGROUP\n")?.starts_with("1 1 P 1\nSYMOP\n0 0 0 1 0 0 0 1 0 0 0 1 1\n") {
        return Err(qualification());
    }
    let mut source_sites = section(text, "\nSTRUC\n")?.lines().step_by(2);
    let mut thermal = section(text, &format!("\n{thermal_section}\n"))?.lines();
    let mut sites = Vec::with_capacity(site_count);
    let mut id_candidate = ids.clone();
    for ordinal in 1..=site_count {
        let row = source_sites.next().ok_or_else(qualification)?;
        let mut words = row.split_ascii_whitespace();
        if words.next().and_then(|token| token.parse::<usize>().ok()) != Some(ordinal) {
            return Err(qualification());
        }
        let element = words
            .next()
            .filter(|e| vesta_model::element(e))
            .ok_or_else(qualification)?;
        let label = words.next().ok_or_else(qualification)?;
        let occupancy = number(words.next())?;
        let frac = [
            number(words.next())?,
            number(words.next())?,
            number(words.next())?,
        ];
        let mut adp = thermal
            .next()
            .ok_or_else(qualification)?
            .split_ascii_whitespace();
        if adp.next().and_then(|token| token.parse::<usize>().ok()) != Some(ordinal)
            || adp.next() != Some(label)
            || number(adp.next())? != 0.
            || adp.next().is_some()
        {
            return Err(qualification());
        }
        sites.push(Site {
            id: id_candidate.next_id().into(),
            label: label.into(),
            element: element.into(),
            frac,
            occupancy,
            u_iso_a2: 0.,
        });
    }
    let phase = Phase {
        id: id_candidate.next_id().into(),
        label: title.into(),
        cell,
        sites,
        symmetry_op_id: id_candidate.next_id().into(),
        boundary_min: [xmin, ymin, zmin],
        boundary_max: [xmax, ymax, zmax],
    };
    *ids = id_candidate;
    Ok(phase)
}

/// Build current typed rows only from the authenticated import phase and its
/// exact logical resource use. The caller installs the complete candidate atomically.
pub fn materialize_r80_orffe(
    archive: &GraphArchive,
    import: &R80PhaseImport,
    phase: &Phase,
    resource: &PersistentResource,
) -> Result<R80GeometryTable> {
    let pair = exact_pair(archive)?;
    if pair.graph_sha != import.graph_sha || phase != &import.phase {
        return Err(wrong_owner(
            "R80 table does not belong to this imported phase, basis and site population",
        ));
    }
    if resource.logical_key != pair.logical_key
        || resource.sha256 != R80_TABLE_SHA
        || resource.byte_length != pair.table.len().to_string()
        || resource.media_type != "application/octet-stream"
        || !vesta_model::valid_id(resource.id.as_str())
    {
        return Err(wrong_owner(
            "R80 table resource is not the captured logical use",
        ));
    }
    let rows = selected::parse_orffe(
        pair.source,
        pair.table,
        pair.table_key,
        &mut CaptureLimits::default(),
    )?;
    let image = |index: usize| ImageKey {
        phase: phase.id.clone(),
        site: phase.sites[index].id.clone(),
        op: phase.symmetry_op_id.clone(),
        shift: [0; 3],
    };
    let rows = geometry_rows(rows, image)?;
    Ok(R80GeometryTable {
        owner: phase.id.clone(),
        resource: resource.id.clone(),
        primary_sha256: selected::ORFFE_FIXTURE_PRIMARY.into(),
        table_sha256: R80_TABLE_SHA.into(),
        rows,
    })
}

pub(super) fn geometry_rows(
    rows: Vec<selected::SelectedOrffeRow<'_>>,
    image: impl Fn(usize) -> ImageKey,
) -> Result<Vec<R80GeometryMeasurement>> {
    rows
        .into_iter()
        .map(|row| {
            let serial = row.serial as u64;
            let result = R80GeometryValue {
                value_lexeme: row.value.into(),
                su_lexeme: row.uncertainty.into(),
                native_value: row.native_value,
                native_su: row.native_uncertainty,
            };
            match row.sites.as_slice() {
                [a, b] => Ok(R80GeometryMeasurement::Distance {
                    serial,
                    images: [image(*a), image(*b)],
                    result,
                }),
                [a, b, c] => Ok(R80GeometryMeasurement::Angle {
                    serial,
                    images: [image(*a), image(*b), image(*c)],
                    result,
                }),
                _ => Err(qualification()),
            }
        })
        .collect()
}

pub(crate) fn verify_r80_binding(
    archive: &GraphArchive,
    phase: &Phase,
    resource: &PersistentResource,
    observed: &R80GeometryTable,
) -> Result<()> {
    let mut import = import_r80_phase(archive, &mut Ids::new([0; 16]))?;
    if import.phase.sites.len() != phase.sites.len() {
        return Err(wrong_owner("restored R80 phase population differs"));
    }
    import.phase.id = phase.id.clone();
    import.phase.symmetry_op_id = phase.symmetry_op_id.clone();
    for (preserved, live) in import.phase.sites.iter_mut().zip(&phase.sites) {
        preserved.id = live.id.clone();
    }
    if materialize_r80_orffe(archive, &import, phase, resource)? != *observed {
        return Err(wrong_owner(
            "R80 active table differs from original rows or current identities",
        ));
    }
    Ok(())
}
