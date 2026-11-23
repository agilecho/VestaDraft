use std::collections::BTreeMap;
use vesta_codecs::formats::resource_graph::{
    CaptureLimits, GraphProfile, discover_graph, discover_graph_with_profile, verify_artifact_graph,
};
use vesta_model::digest;
use vesta_model::source7::resource_graph::{EdgeKind, ProbeResult, SourceReference};

const HEADER: &str = "#VESTA_FORMAT_VERSION 3.5.4\n\nCRYSTAL\n\nTITLE\nExample\n\n";

fn native(imports: &[&str]) -> Vec<u8> {
    let mut out = HEADER.to_owned();
    for name in imports {
        out.push_str("IMPORT_STRUCTURE\n");
        out.push_str(name);
        out.push_str("\n\n");
    }
    out.push_str("GROUP\n1 1 P 1\nCOMMENT\n");
    out.into_bytes()
}

fn capture(
    fixture: &BTreeMap<String, Vec<u8>>,
    limits: CaptureLimits,
) -> vesta_model::Result<vesta_codecs::formats::resource_graph::GraphArchive> {
    discover_graph(
        "sample/root.vesta",
        |path, _| Ok(fixture.get(path).cloned()),
        limits,
    )
}

#[test]
fn nested_imports_use_declaring_directory_keep_spans_and_ignore_decoy() {
    let leaf = native(&[]);
    let decoy = [leaf.as_slice(), b"Other source comment\n"].concat();
    let fixture = BTreeMap::from([
        (
            "sample/root.vesta".into(),
            native(&["sub/child.vesta", "alias.vesta"]),
        ),
        ("sample/sub/child.vesta".into(), native(&["leaf.vesta"])),
        ("sample/sub/leaf.vesta".into(), leaf.clone()),
        ("sample/alias.vesta".into(), leaf.clone()),
        ("sample/leaf.vesta".into(), decoy),
    ]);
    let limits = CaptureLimits::default();
    let archive = capture(&fixture, limits).unwrap();
    assert_eq!(archive.files.len(), 4);
    assert_eq!(archive.graph.edges.len(), 3);
    assert!(!archive.files.contains_key("sample/leaf.vesta"));
    let root = archive
        .graph
        .captures
        .iter()
        .find(|row| row.primary_key == "sample/root.vesta")
        .unwrap();
    let SourceReference::Explicit(edge) = &root.captured.references[0] else {
        panic!("native structure reference must retain an explicit span");
    };
    assert_eq!(edge.literal_path_hex, "7375622f6368696c642e7665737461");
    assert_eq!(
        edge.path_span.byte_offset,
        (HEADER.len() + "IMPORT_STRUCTURE\n".len()).to_string()
    );
    assert_eq!(edge.path_span.byte_length, "15");
    let leaf_hash = digest(&leaf);
    assert_eq!(
        archive
            .graph
            .files
            .iter()
            .filter(|file| file.sha256 == leaf_hash)
            .count(),
        2
    );
    verify_artifact_graph(&archive.graph, &archive.files, limits).unwrap();

    let mut false_graph = archive.graph.clone();
    false_graph.edges.remove(1);
    assert_eq!(
        verify_artifact_graph(&false_graph, &archive.files, limits)
            .unwrap_err()
            .code,
        "E_GRAPH_DISCOVERY"
    );
    let mut extra = archive.files.clone();
    extra.insert("sample/leaf.vesta".into(), leaf);
    assert_eq!(
        verify_artifact_graph(&archive.graph, &extra, limits)
            .unwrap_err()
            .code,
        "E_GRAPH_FILES"
    );
}

#[test]
fn cycles_missing_dependencies_and_unknown_sections_fail_before_install() {
    let root = "sample/root.vesta".to_owned();
    let child = "sample/sub/child.vesta".to_owned();
    let mut fixture = BTreeMap::from([
        (root.clone(), native(&["sub/child.vesta"])),
        (child.clone(), native(&["../root.vesta"])),
    ]);
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_GRAPH_CYCLE"
    );
    fixture.remove(&child);
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_REOPEN_REFERENCE"
    );
    fixture.insert(
        child,
        [HEADER.as_bytes(), b"UNKNOWN_LOAD\nmystery\n"].concat(),
    );
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_REFERENCE_RULE"
    );
    fixture.insert(root, native(&["../../outside.vesta"]));
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_READ_ONLY"
    );
    fixture.insert(
        "sample/root.vesta".into(),
        native(&["missing/../sub/child.vesta"]),
    );
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_READ_ONLY"
    );
}

#[test]
fn repeated_occurrences_consume_the_shared_reference_limit() {
    let fixture = BTreeMap::from([
        (
            "sample/root.vesta".into(),
            native(&["leaf.vesta", "leaf.vesta"]),
        ),
        ("sample/leaf.vesta".into(), native(&[])),
    ]);
    let mut limits = CaptureLimits::default();
    limits.references = 2;
    let archive = capture(&fixture, limits).unwrap();
    assert_eq!(archive.files.len(), 2);
    assert_eq!(archive.graph.edges.len(), 2);
    assert_ne!(
        archive.graph.edges[0].reference_id,
        archive.graph.edges[1].reference_id
    );
    limits.references = 1;
    assert_eq!(capture(&fixture, limits).unwrap_err().code, "E_PARSE_LIMIT");
}

#[test]
fn graph_limits_are_cumulative_and_apply_to_a_shared_leaf_reached_at_greater_depth() {
    let fixture = BTreeMap::from([
        ("sample/root.vesta".into(), native(&["leaf.vesta", "mid.vesta"])),
        ("sample/leaf.vesta".into(), native(&[])),
        ("sample/mid.vesta".into(), native(&["leaf.vesta"])),
    ]);
    let byte_count = fixture.values().map(Vec::len).sum();
    let record_count = fixture.values()
        .map(|file| std::str::from_utf8(file).unwrap().lines().count())
        .sum();
    let exact = CaptureLimits {
        bytes: byte_count, records: record_count, nodes: 3,
        references: 3, depth: 2, work: byte_count,
        ..CaptureLimits::default()
    };
    let archive = capture(&fixture, exact).unwrap();
    verify_artifact_graph(&archive.graph, &archive.files, exact).unwrap();
    assert_eq!(archive.graph.edges.iter()
        .filter(|edge| edge.to_key == "sample/leaf.vesta").count(), 2);
    for (limit, rejected) in [
        ("bytes", CaptureLimits { bytes: byte_count - 1, ..exact }),
        ("records", CaptureLimits { records: record_count - 1, ..exact }),
        ("nodes", CaptureLimits { nodes: 2, ..exact }),
        ("depth", CaptureLimits { depth: 1, ..exact }),
        ("work", CaptureLimits { work: byte_count - 1, ..exact }),
    ] {
        let failure = capture(&fixture, rejected).unwrap_err();
        assert_eq!(failure.code, "E_PARSE_LIMIT", "{limit}");
        assert!(failure.message.contains(limit), "{failure:?}");
    }
}

fn flat_grid(periodic: bool) -> Vec<u8> {
    let mut bytes = vec![0; 152];
    for (offset, value) in [
        (0, 3i32),
        (96, if periodic { 1 } else { 0 }),
        (104, 1),
        (108, 3),
        (112, 2),
        (116, 2),
        (120, 2),
        (124, 8),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    for (i, component) in [2f32, 3., 4., 90., 90., 90.].into_iter().enumerate() {
        bytes[128 + 4 * i..132 + 4 * i].copy_from_slice(&component.to_le_bytes());
    }
    for component in [-0f32, 1., 2., 3., 4., 5., 6., 7.] {
        bytes.extend_from_slice(&component.to_le_bytes());
    }
    bytes
}

#[test]
fn density_and_texture_discover_distinct_ordered_path_spans_and_validate_grid_bytes() {
    let source = format!(
        "{HEADER}IMPORT_DENSITY 2\n+1.000000 data grid.pgrid\n\nIMPORT_TEXTURE\n+1.000000E+00 texture.ggrid\n\nGROUP\n1 1 P 1\nCOMMENT\n"
    );
    let mut fixture = BTreeMap::from([
        ("sample/root.vesta".into(), source.as_bytes().to_vec()),
        ("sample/data grid.pgrid".into(), flat_grid(true)),
        ("sample/texture.ggrid".into(), flat_grid(false)),
    ]);
    let limits = CaptureLimits::default();
    let archived = capture(&fixture, limits).unwrap();
    assert_eq!(
        archived
            .graph
            .edges
            .iter()
            .map(|e| &e.kind)
            .collect::<Vec<_>>(),
        [&EdgeKind::DensityImport, &EdgeKind::TextureImport]
    );
    let root = archived
        .graph
        .captures
        .iter()
        .find(|c| c.primary_key == "sample/root.vesta")
        .unwrap();
    let SourceReference::Explicit(density) = &root.captured.references[0] else {
        panic!("density must retain an explicit span");
    };
    assert_eq!(
        density.path_span.byte_offset,
        source.find("data grid.pgrid").unwrap().to_string()
    );
    assert_eq!(density.path_span.sha256, digest(b"data grid.pgrid"));
    verify_artifact_graph(&archived.graph, &archived.files, limits).unwrap();

    fixture.get_mut("sample/texture.ggrid").unwrap().pop();
    assert_eq!(
        capture(&fixture, limits).unwrap_err().code,
        "E_REFERENCE_FORMAT"
    );
    fixture.insert("sample/texture.ggrid".into(), flat_grid(true));
    assert_eq!(
        capture(&fixture, limits).unwrap_err().code,
        "E_REFERENCE_FORMAT"
    );
}

#[test]
fn scalar_reference_coefficients_keep_occurrences_spans_and_fresh_resource_owners() {
    use vesta_codecs::{decode, encode_verified};
    use vesta_commands::{EngineV2, Limits};
    use vesta_model::{Cell3, Ids};
    let source = format!(
        "{HEADER}IMPORT_DENSITY 3\n-0.500000 data grid.pgrid\n+2.5E-1 data grid.pgrid\n\nIMPORT_TEXTURE\n1.25e+00 texture.ggrid\n\nGROUP\n1 1 P 1\nCOMMENT\n"
    );
    let fixture = BTreeMap::from([
        ("sample/root.vesta".into(), source.as_bytes().to_vec()),
        ("sample/data grid.pgrid".into(), flat_grid(true)),
        ("sample/texture.ggrid".into(), flat_grid(false)),
    ]);
    let limits = CaptureLimits::default();
    let archive = capture(&fixture, limits).unwrap();
    assert_eq!(archive.graph.edges.len(), 3);
    assert_eq!(archive.files.len(), 3);
    assert_eq!(archive.graph.edges[0].to_key, archive.graph.edges[1].to_key);
    assert_ne!(archive.graph.edges[0].reference_id, archive.graph.edges[1].reference_id);
    let root = archive.graph.captures.iter().find(|c| c.primary_key == "sample/root.vesta").unwrap();
    let offsets = source.match_indices("data grid.pgrid").map(|(i, _)| i).chain([source.find("texture.ggrid").unwrap()]);
    for (reference, offset) in root.captured.references.iter().zip(offsets) {
        let SourceReference::Explicit(reference) = reference else { panic!("exact source span"); };
        assert_eq!(reference.path_span.byte_offset, offset.to_string());
        let length = reference.path_span.byte_length.parse::<usize>().unwrap();
        assert_eq!(reference.path_span.sha256, digest(&source.as_bytes()[offset..offset + length]));
    }
    verify_artifact_graph(&archive.graph, &archive.files, limits).unwrap();
    let mut engine = EngineV2::new(Cell3 { length_a: [4.; 3], angles_deg: [90.; 3] },
        "coefficient closure", Ids::new([181; 16]), Limits::default()).unwrap();
    let graph = vesta_model::source_audit::canonical(&serde_json::to_value(&archive.graph).unwrap());
    let prefix = format!("source-graph/{}", digest(&graph));
    let mut attachments = vec![vesta_commands::SourceAttachment {
        logical_key: format!("{prefix}/capture"), bytes: graph,
        media_type: vesta_model::source7::resource_graph::CAPTURE_MEDIA.into(),
    }];
    for (i, file) in archive.graph.files.iter().enumerate() {
        attachments.push(vesta_commands::SourceAttachment {
            logical_key: format!("{prefix}/file/{i}"), bytes: archive.files[&file.key].clone(),
            media_type: "application/octet-stream".into(),
        });
    }
    engine.attach_resources(attachments).unwrap();
    let before = engine.capture().unwrap();
    let saved = encode_verified(&before, "coefficients.vesta", Ids::new([182; 16])).unwrap();
    let fresh = decode(&saved.primary, &saved.dependencies, Ids::new([183; 16])).unwrap();
    for old in &before.content.resources {
        let new = fresh.content.resources.iter().find(|r| r.logical_key == old.logical_key).unwrap();
        assert_ne!(new.id, old.id);
        assert_eq!(new.sha256, old.sha256);
        assert_eq!(fresh.resources[&new.sha256], before.resources[&old.sha256]);
    }
}

#[test]
fn invalid_scalar_reference_operations_refuse_before_any_companion_read() {
    for operation in ["NaN", "+inf", "1e100", "1e-100", "x2", "/2", "+1^2", "+1[0]", "+", "1e", "0x1", "1d0"] {
        let source = format!("{HEADER}IMPORT_DENSITY\n{operation} required.pgrid\n\nCOMMENT\n");
        let mut reads = Vec::new();
        let result = discover_graph("root.vesta", |key, _| {
            reads.push(key.to_owned());
            Ok((key == "root.vesta").then(|| source.as_bytes().to_vec()))
        }, CaptureLimits::default());
        assert!(result.is_err(), "operation {operation}");
        assert_eq!(reads, ["root.vesta"], "operation {operation}");
    }
}

#[test]
fn explicit_geometry_missing_file_cannot_be_recorded_absent() {
    let source = format!("{HEADER}IMPORT_ORFFE\nrequired.ffe\n\nGROUP\n1 1 P 1\nCOMMENT\n");
    let fixture = BTreeMap::from([("sample/root.vesta".into(), source.into_bytes())]);
    assert_eq!(
        capture(&fixture, CaptureLimits::default())
            .unwrap_err()
            .code,
        "E_REOPEN_REFERENCE"
    );
}

#[test]
fn gamess_pair_is_two_syntactically_checked_files_with_nonrecursive_return_edge() {
    let inp = b" $CONTRL COORD=UNIQUE UNITS=ANGS $END\n $BASIS GBASIS=STO NGAUSS=3 $END\n $DATA\nPair source\nC1\nH1 1.0 0.0 0.0 0.0\nH2 1.0 0.0 0.0 1.0\n $END\n";
    let surface = b"Small pair fixture\n2 2 2\n0 0 0\n1 1 1\n-0 1 2 3 4 5 6 7\n";
    let mut fixture = BTreeMap::from([
        ("sample/source.inp".to_owned(), inp.to_vec()),
        ("sample/source.mmp".to_owned(), surface.to_vec()),
    ]);
    let limits = CaptureLimits::default();
    let run = |files: &BTreeMap<String, Vec<u8>>| {
        discover_graph_with_profile(
            "sample/source.mmp",
            GraphProfile::GamessPair,
            |key, _| Ok(files.get(key).cloned()),
            limits,
        )
    };
    let archive = run(&fixture).unwrap();
    assert_eq!(archive.graph.edges.len(), 2);
    assert!(
        archive
            .graph
            .edges
            .iter()
            .all(|e| e.kind == EdgeKind::ProducerPair && !e.recursive_decode)
    );
    assert!(archive.graph.captures.iter().all(|c| matches!(
        c.captured.references.as_slice(),
        [SourceReference::Implicit(_)]
    )));
    verify_artifact_graph(&archive.graph, &archive.files, limits).unwrap();
    fixture.insert(
        "sample/source.mmp".into(),
        b"Wrong\n2 2 2\n0 0 0\n1 1 1\n1\n".to_vec(),
    );
    assert_eq!(run(&fixture).unwrap_err().code, "E_REFERENCE_FORMAT");
    fixture.remove("sample/source.inp");
    assert_eq!(run(&fixture).unwrap_err().code, "E_REOPEN_REFERENCE");
}

const FEFF_MAIN: &[u8] = b"TITLE C8 finite FEFF10 engineering graph\nRECIPROCAL\nLATTICE P 1.0\n2 0 0\n0 5 0\n0 0 10\nCOORDINATES 1\nTARGET 1\nInClUdE \"sub/atoms.inc\" ignored-tail.inc\ninclude after-parent-end.inc\n";
const FEFF_ATOMS: &[u8] = b"# include inactive-comment.inc\nPOTENTIALS\n0 6 C\n1 8 O\nload shared.inc\nLOAD [shared.inc] retained-ignored.inc\nATOMS\n0 0 0 0 C\n0.5 1.25 2.5 1 O\nEND\nload after-child-end.inc\n";
const FEFF_SHARED: &[u8] = b"* Selected repeatable comment stream with no extra cards.\n";

fn feff_fixture() -> BTreeMap<String, Vec<u8>> {
    BTreeMap::from([
        ("job/main.inp".into(), FEFF_MAIN.into()),
        ("job/sub/atoms.inc".into(), FEFF_ATOMS.into()),
        ("job/shared.inc".into(), FEFF_SHARED.into()),
        (
            "job/sub/shared.inc".into(),
            b"load wrong-base.inc\n".to_vec(),
        ),
    ])
}

fn feff(
    input: &BTreeMap<String, Vec<u8>>,
    limits: CaptureLimits,
) -> vesta_model::Result<vesta_codecs::formats::resource_graph::GraphArchive> {
    discover_graph_with_profile(
        "job/main.inp",
        GraphProfile::Feff10Engineering,
        |key, _| Ok(input.get(key).cloned()),
        limits,
    )
}

#[test]
fn feff_uses_job_base_and_expands_repeated_calls_in_active_stream_order() {
    let input = feff_fixture();
    let limits = CaptureLimits::default();
    let archive = feff(&input, limits).unwrap();
    let targets: Vec<_> = archive
        .graph
        .edges
        .iter()
        .map(|edge| edge.to_key.as_str())
        .collect();
    assert_eq!(
        targets,
        ["job/sub/atoms.inc", "job/shared.inc", "job/shared.inc"]
    );
    assert_eq!(archive.files.len(), 3);
    assert!(
        archive
            .graph
            .edges
            .iter()
            .all(|edge| edge.kind == EdgeKind::ProducerInclude && edge.recursive_decode)
    );
    let occurrences: Vec<_> = archive
        .graph
        .captures
        .iter()
        .flat_map(|file| &file.captured.references)
        .map(|reference| match reference {
            SourceReference::Explicit(reference) => (
                &reference.path_span.byte_offset,
                &reference.path_span.byte_length,
            ),
            SourceReference::Implicit(_) => panic!("FEFF calls have exact path spans"),
        })
        .map(|(start, len)| (start.as_str(), len.as_str()))
        .collect();
    assert_eq!(occurrences, [("117", "13"), ("59", "10"), ("76", "10")]);
    verify_artifact_graph(&archive.graph, &archive.files, limits).unwrap();
}

#[test]
fn feff_distinguishes_repeated_work_cycle_read_error_and_unqualified_primary() {
    let mut input = feff_fixture();
    let mut limits = CaptureLimits::default();
    let work = FEFF_MAIN.len() + FEFF_ATOMS.len() + 2 * FEFF_SHARED.len();
    limits.work = work;
    feff(&input, limits).unwrap();
    limits.work -= 1;
    assert_eq!(feff(&input, limits).unwrap_err().code, "E_PARSE_LIMIT");

    let limits = CaptureLimits::default();
    let error = discover_graph_with_profile(
        "job/main.inp",
        GraphProfile::Feff10Engineering,
        |key, _| {
            if key == "job/shared.inc" {
                return Err(vesta_model::Error::new("E_READ_ONLY", "permission denied"));
            }
            Ok(input.get(key).cloned())
        },
        limits,
    )
    .unwrap_err();
    assert_eq!(error.code, "E_READ_ONLY");
    input.insert("job/shared.inc".into(), b"load sub/atoms.inc\n".to_vec());
    assert_eq!(feff(&input, limits).unwrap_err().code, "E_GRAPH_CYCLE");
    input.get_mut("job/main.inp").unwrap().push(b'\n');
    assert_eq!(feff(&input, limits).unwrap_err().code, "E_REFERENCE_RULE");
}

#[test]
fn selected_rietan_tables_rebind_real_site_ordinals_and_survive_view_edit() {
    use serde_json::json;
    use std::{fs, path::Path};
    use vesta_commands::{EngineV2, Limits};
    use vesta_model::{Error, Ids, source7::{
        SourceDocument,
        resource_graph::orffe::{R80GeometryMeasurement, RIETAN_OWNER_DIALECT, RIETAN_TABLE_DIALECT},
    }};

    let inputs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors().nth(2).unwrap()
        .join("output/vesta-p2-03/20261121-c8-closure-execution/inputs/graph2");
    for (extension, filename, group, count, distance_sites, angle_sites) in [
        ("ins", "Fapatite.ins", "P 63/m", 12, [3usize, 0], [0usize, 3, 1]),
        ("lst", "AgI.lst", "I m -3 m", 96, [0usize, 1], [0usize, 1, 0]),
    ] {
        for present in [true, false] {
            let folder = if present { "present" } else { "absent" };
            let fixture = inputs.join(format!("rietan/{folder}-{extension}"));
            let graph = discover_graph_with_profile(filename, GraphProfile::Rietan, |key, _| {
                match fs::read(fixture.join(key)) {
                    Ok(raw) => Ok(Some(raw)),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                    Err(e) => Err(Error::new("E_IO", e.to_string())),
                }
            }, CaptureLimits::default()).unwrap();
            let original = vesta_codecs::open_selected_rietan(&graph, Ids::new([111; 16])).unwrap();
            let source = original.capture.source.as_ref().unwrap().to_value().unwrap();
            let phase = &source["phaseGraph"]["phases"][0];
            assert_eq!(phase["structure"]["group"]["symbol"], group);
            assert_eq!(phase["structure"]["group"]["operators"].as_array().unwrap().len(), count);
            let cell = phase["structure"]["cell"].clone();
            assert!(original.content.phases.is_empty());
            let engine = EngineV2::loaded(original, Ids::new([112; 16]), Limits::default()).unwrap();
            let bundle = vesta_codecs::encode_verified(&engine.capture().unwrap(), "rietan.vesta", Ids::new([113; 16])).unwrap();
            let reopened = vesta_codecs::decode(&bundle.primary, &bundle.dependencies, Ids::new([114; 16])).unwrap();
            let value = reopened.capture.source.as_ref().unwrap().to_value().unwrap();
            let candidate = &value["phaseGraph"]["phases"][0];
            assert_eq!(candidate["structure"]["cell"], cell);
            assert_ne!(candidate["id"], phase["id"]);
            let row_index = |dialect: &str| {
                value["metadata"].as_array().unwrap().iter()
                    .position(|row| row["dialect"] == dialect).unwrap()
            };
            let mut counterfeit = vec![
                (row_index(RIETAN_OWNER_DIALECT), vec![("producerVersion", "unregistered")]),
            ];
            if present {
                counterfeit.extend([
                    (row_index(RIETAN_TABLE_DIALECT), vec![
                        ("dialect", "native-r80-orffe-1"), ("formatRef", "VM18-I01"),
                    ]),
                    (row_index(RIETAN_TABLE_DIALECT), vec![("formatRef", "VM18-I01")]),
                ]);
            }
            for (index, replacements) in counterfeit {
                let mut altered = value.clone();
                for (property, replacement) in replacements {
                    altered["metadata"][index][property] = json!(replacement);
                }
                let mut forged = reopened.capture.clone();
                forged.source = Some(SourceDocument::decode(altered).unwrap());
                let error = vesta_codecs::encode_verified(
                    &forged, "wrong-producer.vesta", Ids::new([116; 16]),
                ).unwrap_err();
                assert_eq!(error.code, "E_REOPEN_OWNER");
            }
            let mut engine = EngineV2::loaded(reopened, Ids::new([115; 16]), Limits::default()).unwrap();
            engine.execute(&engine.document(), &json!({"command":"view.setCamera","params":{
                "rotation":[[1,0,0],[0,1,0],[0,0,1]],"projection":"parallel","zoom":2.5
            }})).unwrap();
            let tables = engine.active_orffe().unwrap();
            assert_eq!(tables.len(), present as usize);
            if let [table] = &tables[..] {
                assert_eq!(table.owner.as_str(), candidate["id"].as_str().unwrap());
                let [R80GeometryMeasurement::Distance { images: distance, .. },
                     R80GeometryMeasurement::Angle { images: angle, .. }] = table.rows.as_slice() else {
                    panic!("expected the ordered distance and angle rows");
                };
                let identity = candidate["structure"]["group"]["operators"][0]["id"].as_str().unwrap();
                for (image, ordinal) in distance.iter().zip(distance_sites).chain(angle.iter().zip(angle_sites)) {
                    assert_eq!(image.phase, table.owner);
                    assert_eq!(image.site.as_str(), candidate["structure"]["sites"][ordinal]["id"].as_str().unwrap());
                    assert_eq!(image.op.as_str(), identity);
                    assert_eq!(image.shift, [0; 3]);
                }
            }
            assert_eq!(engine.content().camera.zoom, 2.5);
        }
    }
}

#[test]
fn rietan_optional_probe_distinguishes_not_found_unreadable_invalid_and_forged_absence() {
    use std::{fs, path::Path};
    use vesta_model::Error;

    let inputs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors().nth(2).unwrap()
        .join("output/vesta-p2-03/20261121-c8-closure-execution/inputs/graph2/rietan");
    let limits = CaptureLimits::default();
    for (extension, primary, companion) in [
        ("ins", "Fapatite.ins", "Fapatite.ffe"),
        ("lst", "AgI.lst", "AgI.ffe"),
    ] {
        let dir = inputs.join(format!("present-{extension}"));
        let input = fs::read(dir.join(primary)).unwrap();
        let table = fs::read(dir.join(companion)).unwrap();
        let run = |probe: vesta_model::Result<Option<Vec<u8>>>| {
            discover_graph_with_profile(primary, GraphProfile::Rietan, |key, _| {
                match key {
                    key if key == primary => Ok(Some(input.clone())),
                    key if key == companion => probe.clone(),
                    _ => panic!("unexpected companion lookup: {key}"),
                }
            }, limits)
        };
        let present = run(Ok(Some(table))).unwrap();
        let [probe] = present.graph.optional_probes.as_slice() else {
            panic!("expected a uniquely derived companion probe");
        };
        assert_eq!(probe.from_key, primary);
        assert_eq!(probe.result, ProbeResult::Present {
            to_key: companion.into(), reference_id: present.graph.edges[0].reference_id.clone(),
        });
        let absent = run(Ok(None)).unwrap();
        assert_eq!(absent.graph.optional_probes[0].result, ProbeResult::Absent);
        assert!(!absent.files.contains_key(companion));
        assert_eq!(run(Err(Error::new("E_IO", "permission denied"))).unwrap_err().code, "E_IO");
        assert_eq!(run(Ok(Some(b"bad table bytes\n".to_vec()))).unwrap_err().code, "E_REFERENCE_FORMAT");
        verify_artifact_graph(&present.graph, &present.files, limits).unwrap();
        verify_artifact_graph(&absent.graph, &absent.files, limits).unwrap();
        let mut forged = present.graph.clone();
        forged.optional_probes[0].result = ProbeResult::Absent;
        assert_eq!(verify_artifact_graph(&forged, &present.files, limits).unwrap_err().code,
            "E_GRAPH_DISCOVERY");
    }
}

#[test]
fn active_feff_end_tail_preserves_exact_bytes_and_blocks_scope_changes() {
    use serde_json::{Value, json};
    use vesta_commands::{EngineV2, Limits, SourceAttachment};
    use vesta_model::{Cell3, Ids, source7::SourceDocument, source_audit::canonical};
    use vesta_model::source7::resource_graph::{CAPTURE_MEDIA, GRAPH_GROUP};

    let archive = feff(&feff_fixture(), CaptureLimits::default()).unwrap();
    let raw = canonical(&serde_json::to_value(&archive.graph).unwrap());
    let prefix = format!("{GRAPH_GROUP}/{}", digest(&raw));
    let mut files = vec![SourceAttachment {
        logical_key:format!("{prefix}/capture"),bytes:raw,media_type:CAPTURE_MEDIA.into(),
    }];
    for (i, file) in archive.graph.files.iter().enumerate() {
        files.push(SourceAttachment {
            logical_key:format!("{prefix}/file/{i}"),bytes:archive.files[&file.key].clone(),
            media_type:"application/octet-stream".into(),
        });
    }
    let mut live = EngineV2::new(Cell3 { length_a:[4.;3],angles_deg:[90.;3] },
        "native project", Ids::new([133;16]), Limits::default()).unwrap();
    live.attach_resources(files).unwrap();
    let bundle = vesta_codecs::encode_verified(&live.capture().unwrap(), "opaque.vesta", Ids::new([134;16])).unwrap();
    let restored = vesta_codecs::decode(&bundle.primary, &bundle.dependencies, Ids::new([135;16])).unwrap();
    let value = restored.capture.source.as_ref().unwrap().to_value().unwrap();
    let blocks = value["opaqueBlocks"].as_array().unwrap();
    let [tail] = blocks.as_slice() else { panic!("expected a single bounded opaque span") };
    let deps = &tail["editDependencies"];
    assert_eq!(deps["kind"], "qualified-exact-footprint");
    assert_eq!(deps["referenceClosure"], "complete-no-external-references");
    let points = deps["units"].as_array().unwrap();
    assert_eq!(points.len(), 4);
    for point in points {
        assert!(value.pointer(point["relativePointer"].as_str().unwrap()).is_some());
    }
    let extent = &tail["extent"];
    let owner = restored.content.resources.iter().find(|r|
        extent["sourceResourceId"].as_str() == Some(r.id.as_str())).unwrap();
    let raw = &restored.resources[&owner.sha256];
    let start: usize = extent["byteOffset"].as_str().unwrap().parse().unwrap();
    let tail_bytes = b"load after-child-end.inc\n";
    assert_eq!(&raw[start..], tail_bytes);
    assert_eq!(extent["byteLength"], tail_bytes.len().to_string());
    assert_eq!(extent["sha256"], digest(tail_bytes));

    let mut wrong = value.clone();
    wrong["opaqueBlocks"][0]["editDependencies"]["units"][0]["sourceNodeDigest"] = Value::String("0".repeat(64));
    let mut actor = restored.capture.clone();
    actor.source = Some(SourceDocument::decode(wrong).unwrap());
    assert_eq!(vesta_codecs::encode_verified(&actor, "rejected.vesta", Ids::new([136;16])).unwrap_err().code,
        "E_UNSAFE_SOURCE_REWRITE");

    let mut live = EngineV2::loaded(restored, Ids::new([137;16]), Limits::default()).unwrap();
    let phase = live.content().phases[0].id.to_string();
    live.execute(&live.document(), &json!({"command":"cell.set","params":{
        "phaseId":phase,"cell":{"lengthA":[6,4,4],"anglesDeg":[90,90,90]}
    }})).unwrap();
    let bundle = vesta_codecs::encode_verified(&live.capture().unwrap(), "opaque-edited.vesta", Ids::new([138;16])).unwrap();
    let edited = vesta_codecs::decode(&bundle.primary, &bundle.dependencies, Ids::new([139;16])).unwrap();
    let edited_source = edited.capture.source.as_ref().unwrap().to_value().unwrap();
    assert_eq!(edited.content.phases[0].cell.length_a,[6.,4.,4.]);
    assert_eq!(edited_source["opaqueBlocks"][0]["extent"]["sha256"], extent["sha256"]);
    assert_ne!(edited_source["opaqueBlocks"][0]["id"], tail["id"]);
    assert_eq!(edited_source["opaqueBlocks"][0]["editDependencies"]["baseline"], deps["baseline"]);
}
