mod base64;
mod frame_io;
mod paths;
mod session_v2;
mod stdio;
use paths::Paths;
use serde_json::{Value, json};
use std::io::Read;
use vesta_codecs::formats::fdat_source::{FdatLayout, is_selected_fdat, open_selected_fdat};
use vesta_codecs::formats::live_state::{NativeStateInput, inspect_native_state, open_native_state};
use vesta_codecs::formats::resource_graph::{
    CaptureLimits, FEFF_VIRTUAL_JOB, GraphArchive, GraphProfile, discover_graph_with_profile,
    is_c1_density_source, is_r80_source, is_rietan_source, is_structure_source,
};
use vesta_codecs::formats::scalar_grid_source::{is_selected_eb, open_selected_eb};
use vesta_commands::{EngineV2, Limits, SourceAttachment};
use vesta_model::source7::resource_graph::{CAPTURE_MEDIA, GRAPH_GROUP};
use vesta_model::{Error, Ids, Result};
fn seed() -> Result<[u8; 16]> {
    let mut bytes = [0; 16];
    std::fs::File::open("/dev/urandom")
        .map_err(|e| Error::new("E_IO", e.to_string()))?
        .read_exact(&mut bytes)
        .map_err(|e| Error::new("E_IO", e.to_string()))?;
    Ok(bytes)
}
fn arg(args: &[String], name: &str) -> Option<String> {
    args.windows(2).find(|v| v[0] == name).map(|v| v[1].clone())
}
fn load(paths: &Paths, input: &str, ids: Ids, limits: Limits) -> Result<EngineV2> {
    load_with_fdat_layout(paths, input, ids, limits, None)
}

fn fdat_layout(args: &[String]) -> Result<Option<FdatLayout>> {
    match graph_arg(args, "--fdat-connection-layout")? {
        None => Ok(None),
        Some("published-count-rule") => Ok(Some(FdatLayout::Published)),
        _ => Err(Error::new("E_SCHEMA", "unknown FDAT connection layout")),
    }
}

fn load_with_fdat_layout(
    paths: &Paths, input: &str, mut ids: Ids, limits: Limits, layout: Option<FdatLayout>,
) -> Result<EngineV2> {
    let primary = paths.read(input)?;
    if layout.is_some() && !is_selected_fdat(&primary) {
        return Err(Error::new("E_FORMAT_UNSUPPORTED", "explicit FDAT layout requires a selected FDAT input"));
    }
    if input.ends_with(".vstate") {
        let state = NativeStateInput::parse(&primary)?;
        let base = state.base_input().map(|base| {
            if base.ends_with(".vstate") {
                return Err(Error::new("E_REOPEN_COVERAGE", "a current-state manifest cannot mount another manifest"));
            }
            load(paths, base, ids.fork(), limits.clone())?.capture()
        }).transpose()?;
        let capture = open_native_state(state, base, |path| {
            paths.read_with_limit(path, 800_000)?
                .ok_or_else(|| Error::new("E_IO", "current grid buffer not found"))
        }, ids.fork())?;
        let mut engine = EngineV2::loaded(capture, ids, limits)?;
        engine.import_dirty();
        return Ok(engine);
    }
    let source_only = if is_selected_eb(&primary) {
        Some(open_selected_eb(&primary, ids.fork())?)
    } else if is_selected_fdat(&primary) {
        Some(open_selected_fdat(&primary, layout.unwrap_or(FdatLayout::Uninterpreted), ids.fork())?)
    } else if input.ends_with(".vcs") {
        Some(vesta_codecs::formats::live_state::open_legacy_vics(&primary, ids.fork())?)
    } else if input.rsplit_once('.').is_some_and(|(_, ext)| ext.eq_ignore_ascii_case("exp")) {
        Some(vesta_codecs::formats::gsas_exp::open(&primary, ids.fork())?)
    } else if vesta_codecs::formats::live_state::is_historical_anatase(&primary) {
        Some(vesta_codecs::formats::live_state::open_historical_anatase(&primary, ids.fork())?)
    } else if input.ends_with(".file06") || input.rsplit('/').next()
        .is_some_and(|leaf| leaf.eq_ignore_ascii_case("file06") || leaf.eq_ignore_ascii_case("file06.dat")) {
        Some(vesta_codecs::formats::mxd_report::open(&primary, ids.fork())?)
    } else {
        None
    };
    if let Some(capture) = source_only {
        let mut engine = EngineV2::loaded(capture, ids, limits)?;
        engine.import_dirty();
        return Ok(engine);
    }
    let density = is_c1_density_source(&primary);
    let rietan = is_rietan_source(&primary);
    if rietan || density || is_r80_source(&primary) {
        let root = paths.graph_key(input)?;
        let mut seen_primary = Some(primary);
        let archive = discover_graph_with_profile(
            &root,
            if rietan { GraphProfile::Rietan } else { GraphProfile::NativeVesta },
            |key, remaining| {
                if key == root {
                    if seen_primary
                        .as_ref()
                        .is_some_and(|raw| raw.len() > remaining)
                    {
                        return Err(Error::new(
                            "E_PARSE_LIMIT",
                            "selected native source exceeds graph budget",
                        ));
                    }
                    Ok(seen_primary.take())
                } else {
                    paths.read_with_limit(key, remaining)
                }
            },
            CaptureLimits::default(),
        )?;
        let capture = if rietan {
            vesta_codecs::open_selected_rietan(&archive, ids.fork())?
        } else if density {
            vesta_codecs::open_selected_density(&archive, ids.fork())?
        } else {
            vesta_codecs::open_selected_orffe(&archive, ids.fork())?
        };
        let mut engine = EngineV2::loaded(capture, ids, limits)?;
        engine.import_dirty();
        return Ok(engine);
    }
    let deps = paths.bundle(input, &primary)?;
    let capture = vesta_codecs::decode(&primary, &deps, ids.fork())?;
    let mut engine = EngineV2::loaded(capture, ids, limits)?;
    engine.loaded_work_ref(input, &vesta_model::digest(&primary));
    Ok(engine)
}
fn preserve_source(
    paths: &Paths,
    engine: &mut EngineV2,
    input: &str,
    profile: Option<&str>,
    producer_job: Option<&str>,
) -> Result<()> {
    let limits = CaptureLimits::default();
    if profile.is_some() || producer_job.is_some() {
        let (Some("feff10-pinned-input"), Some(job)) = (profile, producer_job) else {
            return Err(Error::new(
                "E_REFERENCE_RULE",
                "qualified engineering profile and explicit producer-job root required",
            ));
        };
        let (job_paths, relative) = paths.producer_job(job, input)?;
        let root = format!("{FEFF_VIRTUAL_JOB}/{relative}");
        let prefix = format!("{FEFF_VIRTUAL_JOB}/");
        let archive = discover_graph_with_profile(
            &root,
            GraphProfile::Feff10Engineering,
            |key, remaining| {
                let native = key.strip_prefix(prefix.as_str()).ok_or_else(|| {
                    Error::new("E_READ_ONLY", "file is outside the virtual producer job")
                })?;
                job_paths.read_with_limit(native, remaining)
            },
            limits,
        )?;
        return attach_graph(engine, archive);
    }
    let extension = std::path::Path::new(input)
        .extension()
        .and_then(|part| part.to_str());
    let raw = if matches!(extension, Some("vesta" | "inp" | "mmp" | "ins" | "lst")) {
        paths
            .read_with_limit(input, limits.bytes)?
            .ok_or_else(|| Error::new("E_IO", "source file missing"))?
    } else {
        paths.read(input)?
    };
    let selected = if is_structure_source(&raw) {
        Some(GraphProfile::NativeVesta)
    } else {
        match extension {
            Some("ins" | "lst") => Some(GraphProfile::Rietan),
            Some("mmp") => Some(GraphProfile::GamessPair),
            Some("inp") => {
                if std::str::from_utf8(&raw).is_ok_and(|text| {
                    text.trim_start()
                        .get(..7)
                        .is_some_and(|head| head.eq_ignore_ascii_case("$CONTRL"))
                }) {
                    Some(GraphProfile::GamessPair)
                } else {
                    return Err(Error::new(
                        "E_REFERENCE_RULE",
                        "unqualified .inp source needs a selected producer context",
                    ));
                }
            }
            _ => None,
        }
    };
    let Some(selected) = selected else {
        let leaf = std::path::Path::new(input)
            .file_name()
            .and_then(|p| p.to_str())
            .ok_or_else(|| Error::new("E_SCHEMA", "source name"))?;
        return engine.attach_resource(&format!("source/{leaf}"), raw);
    };
    let root = paths.graph_key(input)?;
    let mut primary = Some(raw);
    let archive = discover_graph_with_profile(
        &root,
        selected,
        |key, remaining| {
            if key == root {
                // GAMESS discovery can acquire the .inp before its .mmp root.
                if primary
                    .as_ref()
                    .is_some_and(|bytes| bytes.len() > remaining)
                {
                    return Err(Error::new("E_PARSE_LIMIT", "native graph bytes"));
                }
                Ok(primary.take())
            } else {
                paths.read_with_limit(key, remaining)
            }
        },
        limits,
    )?;
    attach_graph(engine, archive)
}
fn attach_graph(engine: &mut EngineV2, mut archive: GraphArchive) -> Result<()> {
    let graph = serde_json::to_value(&archive.graph)
        .map_err(|error| Error::new("E_SCHEMA", error.to_string()))?;
    let graph_bytes = vesta_model::source_audit::canonical(&graph);
    let prefix = format!("{GRAPH_GROUP}/{}", vesta_model::digest(&graph_bytes));
    let mut attachments = vec![SourceAttachment {
        logical_key: format!("{prefix}/capture"),
        bytes: graph_bytes,
        media_type: CAPTURE_MEDIA.into(),
    }];
    for (index, file) in archive.graph.files.iter().enumerate() {
        let bytes = archive
            .files
            .remove(&file.key)
            .ok_or_else(|| Error::new("E_GRAPH_FILES", "captured graph file has no bytes"))?;
        attachments.push(SourceAttachment {
            logical_key: format!("{prefix}/file/{index}"),
            bytes,
            media_type: "application/octet-stream".into(),
        });
    }
    engine.attach_resources(attachments)
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("");
    if cmd == "commands" {
        println!("{}", session_v2::registration());
        return Ok(());
    }
    let root = arg(&args, "--root")
        .ok_or_else(|| Error::new("E_READ_ONLY", "explicit --root required"))?;
    let paths = Paths::new(&root)?;
    match cmd {
        "serve" => {
            if !args.iter().any(|a| a == "--stdio") {
                return Err(Error::new("E_SCHEMA", "serve requires --stdio"));
            }
            let initial = arg(&args, "--input");
            let output = arg(&args, "--engineering-output");
            let session = session_v2::Session::new(paths, Ids::new(seed()?), initial, output)?;
            stdio::serve(session)
        }
        "inspect" => {
            let input =
                arg(&args, "--input").ok_or_else(|| Error::new("E_SCHEMA", "--input required"))?;
            let e = load_with_fdat_layout(&paths, &input, Ids::new(seed()?), Limits::default(), fdat_layout(&args)?)?;
            let mut inspection = json!({"context":e.context(),"content":e.content()});
            if !e.active_fields().is_empty() {
                let mut rows = Vec::new();
                for field in e.active_fields() {
                    let buffer = e.field_buffer(&field.field_id)?;
                    let [nx, ny, nz] = buffer.stored_shape();
                    let mut samples = Vec::with_capacity(buffer.sample_count());
                    for z in 0..nz {
                        for y in 0..ny {
                            for x in 0..nx {
                                let bits = buffer.sample_bits([x, y, z]).ok_or_else(|| {
                                    Error::new("E_FIELD_STORAGE", "sample outside layout")
                                })?;
                                samples.push(format!("{bits:016x}"));
                            }
                        }
                    }
                    rows.push(json!({"field":field,"storedShape":[nx,ny,nz],"xFastestSampleBits":samples}));
                }
                inspection["fields"] = json!(rows);
            }
            let captured = e.capture()?;
            if let Some(native) = inspect_native_state(&captured)? {
                inspection["nativeCurrentState"] = native;
            }
            if let Some(magnetic) = vesta_codecs::formats::gsas_exp::inspect_magnetic(&captured)? {
                inspection["gsasMagneticState"] = magnetic;
            }
            if let Some(report) = vesta_codecs::formats::mxd_report::inspect(&captured)? {
                inspection["mxdReport"] = report;
            }
            let tables = e.active_orffe()?;
            if !tables.is_empty() {
                inspection["geometryTables"] = json!(tables);
            }
            println!("{}", inspection);
            Ok(())
        }
        "run" => {
            let profile = graph_arg(&args, "--source-profile")?;
            let producer_job = graph_arg(&args, "--producer-job-root")?;
            let input = arg(&args, "--input").ok_or_else(|| Error::new("E_SCHEMA", "--input"))?;
            let request =
                arg(&args, "--request").ok_or_else(|| Error::new("E_SCHEMA", "--request"))?;
            let output =
                arg(&args, "--output").ok_or_else(|| Error::new("E_SCHEMA", "--output"))?;
            let mut e = load_with_fdat_layout(&paths, &input, Ids::new(seed()?), Limits::default(), fdat_layout(&args)?)?;
            if let Some(source) = arg(&args, "--preserve-source") {
                preserve_source(&paths, &mut e, &source, profile, producer_job)?;
            } else if profile.is_some() || producer_job.is_some() {
                return Err(Error::new(
                    "E_REFERENCE_RULE",
                    "producer capture options require --preserve-source",
                ));
            }
            let request = vesta_model::json::parse(&paths.read(&request)?, 1048576)?;
            if request
                .as_object()
                .is_none_or(|o| o.keys().any(|k| k != "commands" && k != "mode"))
            {
                return Err(Error::new("E_SCHEMA", "run request keys"));
            }
            let mut inv: Vec<Value> = request["commands"]
                .as_array()
                .cloned()
                .ok_or_else(|| Error::new("E_SCHEMA", "commands array"))?;
            for c in &mut inv {
                if c["params"]["phaseId"] == "$first-phase" {
                    let phase = e.content().phases.first().ok_or_else(|| {
                        Error::new("E_OBJECT_NOT_FOUND", "project has no editable first phase")
                    })?;
                    c["params"]["phaseId"] = json!(phase.id);
                }
            }
            let at = e.document();
            let outcomes = if request["mode"] == "atomic" {
                e.batch(&at, &inv)?
            } else if request["mode"] == "sequential" {
                let mut out = vec![];
                for c in inv {
                    out.push(e.execute(&e.document(), &c)?)
                }
                out
            } else {
                return Err(Error::new(
                    "E_SCHEMA",
                    "explicit atomic/sequential mode required",
                ));
            };
            let capture = e.capture()?;
            let leaf = std::path::Path::new(&output)
                .file_name()
                .and_then(|v| v.to_str())
                .ok_or_else(|| Error::new("E_SCHEMA", "output leaf"))?;
            let b = vesta_codecs::encode_verified(&capture, leaf, Ids::new(seed()?))?;
            let baseline = paths.baseline(&output)?;
            let hash = paths.publish(
                &output,
                &b,
                baseline.as_deref(),
                &capture.document.document_id,
            )?;
            println!(
                "{}",
                json!({"engineeringArtifact":output,"primarySha256":hash,"context":e.context(),"outcomes":outcomes,"hostSaved":false})
            );
            Ok(())
        }
        _ => Err(Error::new(
            "E_SCHEMA",
            "commands | serve --stdio --root | inspect --input --root | run --input --request --output --root",
        )),
    }
}

fn graph_arg<'a>(args: &'a [String], flag: &str) -> Result<Option<&'a str>> {
    let mut selected = args
        .iter()
        .enumerate()
        .filter(|(_, arg)| arg.as_str() == flag);
    let Some((index, _)) = selected.next() else {
        return Ok(None);
    };
    if selected.next().is_some() {
        return Err(Error::new("E_SCHEMA", format!("duplicate {flag}")));
    }
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .map(|value| Some(value.as_str()))
        .ok_or_else(|| Error::new("E_SCHEMA", format!("missing value for {flag}")))
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", json!({"code":e.code,"message":e.message}));
        std::process::exit(if e.code == "E_IO" || e.code == "E_READ_ONLY" {
            3
        } else {
            2
        })
    }
}
