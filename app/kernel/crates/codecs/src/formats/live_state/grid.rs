use super::{entity, invalid, payload, tagged};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::source_audit::canonical;
use vesta_model::source7::native_grid_resource_use;
use vesta_model::{Capture, Ids, PersistentResource, Result, digest};

const STEP: &str = "NativeScalarGridCurrent1";
const FRACTIONAL: &str = "NativeFractionalGridCurrent1";
const VIEW: &str = "NativeStoredScalarIsoview1";
const STEP_KIND: &str = "native-current-numerical-grid";
const FRACTIONAL_KIND: &str = "native-fractional-current-grid";
const MASK_RULE: &str = "draft.native-current-grid-mask";
const DESCRIPTOR: &str =
    include_str!("../../../../model/src/source7/frozen/source-descriptor.schema.json");
const BINDING: &str =
    include_str!("../../../../model/src/source7/frozen/bound-descriptor-context.contract.json");
const STEP_FRAME: &str =
    include_str!("../../../contracts/native-scalar-grid-current-frame.schema.json");
const FRACTIONAL_FRAME: &str =
    include_str!("../../../contracts/native-fractional-grid-frame.schema.json");
const FRAME_TARGET: &str = "/fieldBindings/0/frameResource";
const EVIDENCE_TARGET: &str = "/fieldBindings/0/evidence/0/evidenceResource";

pub(super) fn rule_digest() -> String {
    digest(&canonical(&json!([
        STEP_FRAME,
        FRACTIONAL_FRAME,
        STEP_KIND,
        FRACTIONAL_KIND,
        MASK_RULE
    ])))
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Encoding {
    F32,
    F64,
}

impl Encoding {
    fn width(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }
    fn storage(self) -> &'static str {
        match self {
            Self::F32 => "ieee754-little-endian-f32",
            Self::F64 => "ieee754-little-endian-f64",
        }
    }
}

#[derive(Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum Coordinates {
    Step {
        origin_bits: [String; 3],
        step_columns_bits: [[String; 3]; 3],
    },
    Fractional {
        divisions: [usize; 3],
        rows_bits: [[String; 4]; 3],
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GridInput {
    label: String,
    shape: [usize; 3],
    encoding: Encoding,
    buffer: String,
    mask: Option<String>,
    coordinates: Coordinates,
    views: Vec<View>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct View {
    affine4: [f64; 16],
    levels: Vec<Level>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Level {
    stored_value_f32_bits: String,
    sign_mode: SignMode,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum SignMode {
    Positive,
    Negative,
    Both,
}

struct Format {
    tag: &'static str,
    native_kind: &'static str,
    schema: &'static str,
    frame_schema: &'static str,
    role: &'static str,
    rule: &'static str,
}

const STEP_FORMAT: Format = Format {
    tag: STEP,
    native_kind: STEP_KIND,
    schema: "FC-native-scalar-grid-current-1",
    frame_schema: "FC-native-scalar-grid-current-frame-1",
    role: "actual-current-native-numerical-state",
    rule: "draft.native-current-numerical-grid",
};
const FRACTIONAL_FORMAT: Format = Format {
    tag: FRACTIONAL,
    native_kind: FRACTIONAL_KIND,
    schema: "FC-native-fractional-grid-current-1",
    frame_schema: "FC-native-fractional-grid-frame-1",
    role: "actual-current-native-fractional-state",
    rule: "draft.native-fractional-current-grid",
};

fn hex_word(word: &str, width: usize) -> Result<u64> {
    if word.len() != width
        || !word
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("malformed native IEEE word"));
    }
    u64::from_str_radix(word, 16).map_err(|_| invalid("native IEEE word"))
}

fn validate_words<'a>(values: impl IntoIterator<Item = &'a String>, width: usize) -> Result<()> {
    for word in values {
        hex_word(word, width)?;
    }
    Ok(())
}

fn count(shape: [usize; 3]) -> Result<usize> {
    if shape.into_iter().any(|n| n < 2) {
        return Err(invalid("current grid must have three nondegenerate axes"));
    }
    shape
        .into_iter()
        .try_fold(1usize, usize::checked_mul)
        .filter(|&n| n <= 100_000)
        .ok_or_else(|| invalid("current grid sample extent exceeds the host bound"))
}

fn validate_buffer(
    samples: usize,
    encoding: Encoding,
    data: &[u8],
    mask: Option<&[u8]>,
) -> Result<()> {
    if data.len() != samples * encoding.width() {
        return Err(invalid(
            "current grid exact buffer extent differs from shape and encoding",
        ));
    }
    if mask.is_some_and(|b| b.len() != samples || b.iter().any(|&x| x > 1)) {
        return Err(invalid("current grid mask extent or validity byte differs"));
    }
    Ok(())
}

fn materialize_frame(input: &GridInput) -> Result<(&'static Format, Value)> {
    let (format, geometry) = match &input.coordinates {
        Coordinates::Step {
            origin_bits,
            step_columns_bits,
        } => {
            validate_words(
                origin_bits.iter().chain(step_columns_bits.iter().flatten()),
                16,
            )?;
            (
                &STEP_FORMAT,
                json!({"originF64Bits":origin_bits,"stepColumnsF64Bits":step_columns_bits}),
            )
        }
        Coordinates::Fractional {
            divisions,
            rows_bits,
        } => {
            if input.encoding.width() != 4
                || input.mask.is_some()
                || *divisions != input.shape.map(|size| size - 1)
            {
                return Err(invalid(
                    "fractional native grid requires unmasked f32 and exact N-1 divisions",
                ));
            }
            validate_words(rows_bits.iter().flatten(), 16)?;
            (
                &FRACTIONAL_FORMAT,
                json!({"gridDivisions":divisions,"gridToDisplayRowsF64Bits":rows_bits}),
            )
        }
    };
    let mut frame = json!({
        "schemaVersion":format.frame_schema,"pointShape":input.shape,"axisOrder":"x-fastest",
        "coordinateRole":"native-numerical-display-only","coordinatePhysicalUnit":"source-unresolved",
        "scalarRole":"uninterpreted-stored-scalar","scalarPhysicalUnit":"source-unresolved",
        "domainTreatment":"finite-including-every-stored-point","dataEncoding":input.encoding.storage(),
        "ruleId":format.rule,"ruleVersion":"1"
    });
    frame
        .as_object_mut()
        .unwrap()
        .extend(geometry.as_object().unwrap().clone());
    Ok((format, frame))
}

fn native_ref(resource: &PersistentResource) -> Value {
    json!({"kind":"BytesRef","resourceId":resource.id,"sha256":resource.sha256,"byteLength":resource.byte_length})
}

fn install_resource(
    capture: &mut Capture,
    source: &mut Value,
    ids: &mut Ids,
    key: String,
    bytes: Vec<u8>,
    descriptor: &str,
) -> Result<PersistentResource> {
    if capture.content.resources.len() >= 1024
        || capture
            .content
            .resources
            .iter()
            .any(|r| r.logical_key == key)
    {
        return Err(invalid(
            "native current-grid resource population or duplicate logical key",
        ));
    }
    let hash = digest(&bytes);
    let row = PersistentResource {
        id: ids.next_id().into(),
        logical_key: key,
        sha256: hash.clone(),
        byte_length: bytes.len().to_string(),
        media_type: "application/octet-stream".into(),
    };
    if !capture.resources.contains_key(&hash)
        && capture.resources.values().map(Vec::len).sum::<usize>() + bytes.len() > 8 * 1024 * 1024
    {
        return Err(invalid(
            "native current grid exceeds cumulative owned byte bounds",
        ));
    }
    let descriptor = if descriptor.is_empty() {
        super::current_source7::descriptor_digest(&row)
    } else {
        descriptor.into()
    };
    source["resources"]
        .as_array_mut()
        .ok_or_else(|| invalid("typed resource table absent"))?
        .push(
            json!({"id":row.id,"sha256":row.sha256,"byteLength":row.byte_length,
            "storageKind":"owned","sourceDescriptorDigest":descriptor}),
        );
    capture.resources.entry(hash).or_insert(bytes);
    capture.content.resources.push(row.clone());
    Ok(row)
}

fn data_key(index: usize, role: &str) -> String {
    format!("native-current-grid/{index}/{role}")
}

fn descriptor_key(pointer: &str, slot: &str) -> String {
    format!("descriptor-data/{}/{slot}", digest(pointer.as_bytes()))
}

fn target_key(pointer: &str) -> String {
    format!("target/{}", digest(pointer.as_bytes()))
}

struct Descriptor<'a> {
    pointer: String,
    frame: &'a Value,
    bytes: &'a [u8],
    role: &'a str,
    rule: &'a str,
}

impl Descriptor<'_> {
    fn slots(&self) -> BTreeMap<String, Vec<u8>> {
        let frame = canonical(self.frame);
        let raw_ref = |alias: &str, b: &[u8]| json!({"resourceId":alias,"sha256":digest(b),"byteLength":b.len().to_string()});
        let (kind, unit, basis) = if self.role == "maskResource" {
            ("validity-mask", "dimensionless", "producer-explicit")
        } else {
            (
                "uninterpreted-stored-scalar",
                "source-unresolved",
                "source-unresolved",
            )
        };
        let definition = json!({
            "schemaVersion":"FC-source-descriptor-2","producerId":self.rule,"producerVersion":"1",
            "dialect":"native-current-state/1","scopeInputDigest":digest(self.bytes),"unknownSourceFields":[],
            "fieldBindings":[{
                "sourcePointer":format!("/{}",self.role),"physicalKind":kind,"unit":unit,"basis":basis,
                "frameResource":raw_ref("current-frame",&frame),"conversionRuleId":self.rule,"conversionRuleVersion":"1",
                "evidence":[{"evidenceResource":raw_ref("current-buffer",self.bytes),"byteOffset":"0",
                    "byteLength":self.bytes.len().to_string(),"sourceSpanDigest":digest(self.bytes),"sourceVersion":"native-current-state/1"}]
            }],
            "evidence":[]
        });
        BTreeMap::from([
            ("definition".into(), canonical(&definition)),
            ("scope".into(), self.bytes.to_vec()),
            (target_key(EVIDENCE_TARGET), self.bytes.to_vec()),
            (target_key(FRAME_TARGET), frame),
        ])
    }

    fn metadata(&self, owner: &str, slots: &BTreeMap<String, PersistentResource>) -> Result<Value> {
        let reference = |row: &PersistentResource| {
            json!({
                "resourceId":entity("resource",row.id.as_str()),"sha256":row.sha256,"byteLength":row.byte_length
            })
        };
        let tree = tagged(
            "BoundDescriptorContext1",
            &json!({
                "schemaVersion":"BoundDescriptorContext-proposal-1",
                "owner":{"kind":"source-entity","id":entity("source-entity",owner)},
                "sourceTypedPointer":self.pointer,"definition":reference(&slots["definition"]),
                "scopeInput":reference(&slots["scope"]),"descriptorSchemaDigest":digest(DESCRIPTOR.as_bytes()),
                "referenceRuleDigest":digest(BINDING.as_bytes()),
                "bindings":[
                    {"jsonPointer":EVIDENCE_TARGET,"capturedAlias":"current-buffer","target":reference(&slots[&target_key(EVIDENCE_TARGET)])},
                    {"jsonPointer":FRAME_TARGET,"capturedAlias":"current-frame","target":reference(&slots[&target_key(FRAME_TARGET)])}
                ]
            }),
        )?;
        Ok(
            json!({"owner":{"kind":"source-entity","id":owner},"producerId":self.rule,"producerVersion":"1",
            "formatRef":"VM18-I33","dialect":"native-current-state/1","tree":tree,"spans":[]}),
        )
    }
}

pub(super) fn install(
    input: GridInput,
    capture: &mut Capture,
    source: &mut Value,
    read: &mut impl FnMut(&str) -> Result<Vec<u8>>,
    ids: &mut Ids,
) -> Result<()> {
    let samples = count(input.shape)?;
    if input.views.is_empty() || input.views.len() > 16 {
        return Err(invalid(
            "current grid needs an explicit bounded view population",
        ));
    }
    let data = read(&input.buffer)?;
    let mask = input.mask.as_deref().map(&mut *read).transpose()?;
    validate_buffer(samples, input.encoding, &data, mask.as_deref())?;
    let (format, frame) = materialize_frame(&input)?;
    let index = source["sourceEntities"]
        .as_array()
        .ok_or_else(|| invalid("typed source entry table"))?
        .len();
    let owner = ids.next_id();
    let mut metadata = Vec::new();
    let mut refs = BTreeMap::new();
    for (role, buffer, rule) in [
        ("dataResource", Some(data), format.rule),
        ("maskResource", mask, MASK_RULE),
    ] {
        if let Some(buffer) = buffer {
            let d = Descriptor {
                pointer: native_grid_resource_use(index, role),
                frame: &frame,
                bytes: &buffer,
                role,
                rule,
            };
            let slots = d
                .slots()
                .into_iter()
                .map(|(slot, bytes)| {
                    let key = descriptor_key(&d.pointer, &slot);
                    Ok((
                        slot,
                        install_resource(capture, source, ids, key, bytes, "")?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>>>()?;
            metadata.push(d.metadata(&owner, &slots)?);
            let resource = install_resource(
                capture,
                source,
                ids,
                data_key(index, role),
                buffer,
                &slots["definition"].sha256,
            )?;
            refs.insert(role, native_ref(&resource));
        }
    }
    let mut body = frame;
    body["schemaVersion"] = json!(format.schema);
    body["captureRole"] = json!(format.role);
    body["owner"] = entity("source-entity", &owner);
    body["label"] = json!(input.label);
    body["dataResource"] = refs
        .remove("dataResource")
        .ok_or_else(|| invalid("current grid data missing"))?;
    if format.tag == STEP {
        body["maskResource"] = refs.remove("maskResource").unwrap_or(Value::Null);
    }
    let tree = tagged(format.tag, &body)?;
    source["sourceEntities"].as_array_mut().unwrap().push(json!({
        "id":owner,"nativeKind":format.native_kind,"sourceTree":tree,
        "sourceRecords":[],"dependencyIds":[],"scientificProjection":"not-qualified","metadata":metadata
    }));
    source["sourceEntryOrder"]
        .as_array_mut()
        .ok_or_else(|| invalid("typed source order"))?
        .push(json!({"kind":"source-entity","id":owner}));
    let scene = source["scenes"]
        .as_array_mut()
        .and_then(|rows| {
            rows.iter_mut()
                .find(|s| s["id"].as_str() == Some(capture.content.scene_id.as_str()))
        })
        .ok_or_else(|| invalid("grid view requires the actual current scene"))?;
    for view in input.views {
        if view.levels.is_empty()
            || view.levels.len() > 64
            || view.affine4.iter().any(|n| !n.is_finite())
            || view.affine4[12..] != [0., 0., 0., 1.]
        {
            return Err(invalid(
                "grid view requires finite affine and explicit bounded levels",
            ));
        }
        for level in &view.levels {
            if !f32::from_bits(hex_word(&level.stored_value_f32_bits, 8)? as u32).is_finite() {
                return Err(invalid("nonfinite stored-scalar view level"));
            }
        }
        let id = ids.next_id();
        let display = tagged(
            VIEW,
            &json!({
                "schemaVersion":"FC-native-stored-isoview-1","target":entity("source-entity",&owner),
                "levels":view.levels,"levelMeaning":"stored-scalar-only",
                "ruleId":"draft.native-stored-scalar-isoview","ruleVersion":"1"
            }),
        )?;
        scene["nodes"]
            .as_array_mut()
            .ok_or_else(|| invalid("scene node collection"))?
            .push(json!({
                "id":id,"kind":"native-opaque","parent":null,"sourceObjectId":owner,
                "affine4":view.affine4,"display":display,"metadata":[]
            }));
        scene["roots"]
            .as_array_mut()
            .ok_or_else(|| invalid("scene roots collection"))?
            .push(json!(id));
    }
    Ok(())
}

fn with_current_grids(
    capture: &Capture,
    source: &Value,
    mut visit: impl FnMut(Value, &[u8], Option<&[u8]>, Encoding, &[Value]) -> Result<()>,
) -> Result<BTreeSet<String>> {
    let resources: BTreeMap<_, _> = capture
        .content
        .resources
        .iter()
        .map(|resource| (resource.logical_key.as_str(), resource))
        .collect();
    let read = |key: &str| -> Result<(&PersistentResource, &[u8])> {
        let row = *resources
            .get(key)
            .ok_or_else(|| invalid("missing independent current-grid resource"))?;
        let raw = capture
            .resources
            .get(&row.sha256)
            .filter(|raw| row.byte_length == raw.len().to_string() && digest(raw) == row.sha256)
            .ok_or_else(|| invalid("current-grid resource extent or digest differs"))?;
        Ok((row, raw))
    };
    let mut views = BTreeMap::<String, Vec<Value>>::new();
    for scene in source["scenes"].as_array().into_iter().flatten() {
        for node in scene["nodes"].as_array().into_iter().flatten() {
            if node["display"]["tag"] != VIEW {
                continue;
            }
            let display = payload(&node["display"])?;
            let id = node["sourceObjectId"]
                .as_str()
                .ok_or_else(|| invalid("current grid view owner"))?;
            if node["kind"] != "native-opaque" || display["target"] != entity("source-entity", id) {
                return Err(invalid("current-grid view owner and source object differ"));
            }
            let affine: [f64; 16] = serde_json::from_value(node["affine4"].clone())
                .map_err(|_| invalid("current-grid view affine"))?;
            if affine.iter().any(|n| !n.is_finite()) || affine[12..] != [0., 0., 0., 1.] {
                return Err(invalid("current-grid view is not a finite affine"));
            }
            let levels = display["levels"]
                .as_array()
                .ok_or_else(|| invalid("current-grid view levels"))?;
            if levels.is_empty() || levels.len() > 64 {
                return Err(invalid("current-grid view levels count"));
            }
            for level in levels {
                let word = level["storedValueF32Bits"]
                    .as_str()
                    .ok_or_else(|| invalid("current-grid view level word"))?;
                if !f32::from_bits(hex_word(word, 8)? as u32).is_finite() {
                    return Err(invalid("nonfinite stored-scalar view level"));
                }
            }
            views
                .entry(id.into())
                .or_default()
                .push(json!({"id":node["id"],"affine4":affine,"display":display}));
        }
    }
    let mut used = BTreeSet::new();
    for (index, row) in source["sourceEntities"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        let format = match (
            row["nativeKind"].as_str(),
            row["sourceTree"]["tag"].as_str(),
        ) {
            (Some(STEP_KIND), Some(STEP)) => &STEP_FORMAT,
            (Some(FRACTIONAL_KIND), Some(FRACTIONAL)) => &FRACTIONAL_FORMAT,
            (Some(STEP_KIND | FRACTIONAL_KIND), _) | (_, Some(STEP | FRACTIONAL)) => {
                return Err(invalid("current-grid kind and registered type differ"));
            }
            _ => continue,
        };
        let owner = row["id"]
            .as_str()
            .ok_or_else(|| invalid("current-grid owner"))?;
        let body = payload(&row["sourceTree"])?;
        if body["owner"] != entity("source-entity", owner)
            || row["sourceRecords"] != json!([])
            || row["dependencyIds"] != json!([])
            || row["scientificProjection"] != "not-qualified"
        {
            return Err(invalid("current-grid owner or producer scope differs"));
        }
        if source["sourceEntryOrder"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|item| item["kind"] == "source-entity" && item["id"] == owner)
            .count()
            != 1
        {
            return Err(invalid("current-grid owner lacks a unique source entry"));
        }
        let shape: [usize; 3] = serde_json::from_value(body["pointShape"].clone())
            .map_err(|_| invalid("current-grid shape"))?;
        let size = count(shape)?;
        let encoding = match body["dataEncoding"].as_str() {
            Some("ieee754-little-endian-f32") => Encoding::F32,
            Some("ieee754-little-endian-f64") if format.tag == STEP => Encoding::F64,
            _ => return Err(invalid("current-grid encoding")),
        };
        if format.tag == FRACTIONAL && body["gridDivisions"] != json!(shape.map(|n| n - 1)) {
            return Err(invalid("fractional current grid divisions differ from N-1"));
        }
        let frame_schema: Value = serde_json::from_str(if format.tag == STEP {
            STEP_FRAME
        } else {
            FRACTIONAL_FRAME
        })
        .expect("pinned native current-frame schema");
        let mut frame = Value::Object(
            frame_schema["properties"]
                .as_object()
                .unwrap()
                .keys()
                .map(|key| (key.clone(), body[key].clone()))
                .collect(),
        );
        frame["schemaVersion"] = json!(format.frame_schema);
        let mut metadata = Vec::new();
        let mut buffers = BTreeMap::new();
        for (role, rule) in [("dataResource", format.rule), ("maskResource", MASK_RULE)] {
            if role == "maskResource" && body[role].is_null() {
                continue;
            }
            let key = data_key(index, role);
            let (resource, raw) = read(&key)?;
            if body[role] != native_ref(resource) {
                return Err(invalid("typed current-grid resource role differs"));
            }
            used.insert(key);
            buffers.insert(role, raw);
            let descriptor = Descriptor {
                pointer: native_grid_resource_use(index, role),
                frame: &frame,
                bytes: raw,
                role,
                rule,
            };
            let mut slots = BTreeMap::new();
            for (slot, expected) in descriptor.slots() {
                let key = descriptor_key(&descriptor.pointer, &slot);
                let (resource, actual) = read(&key)?;
                if actual != expected {
                    return Err(invalid(
                        "current-grid descriptor leaf differs from actual state",
                    ));
                }
                used.insert(key);
                slots.insert(slot, resource.clone());
            }
            if !source["resources"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|row| {
                    row["id"] == resource.id.as_str()
                        && row["sourceDescriptorDigest"] == slots["definition"].sha256
                })
            {
                return Err(invalid("current-grid descriptor definition differs"));
            }
            metadata.push(descriptor.metadata(owner, &slots)?);
        }
        let data = buffers["dataResource"];
        let mask = buffers.get("maskResource").copied();
        validate_buffer(size, encoding, data, mask)?;
        if row["metadata"] != json!(metadata) {
            return Err(invalid("current-grid descriptor context differs"));
        }
        let owned_views = views
            .remove(owner)
            .filter(|rows| rows.len() <= 16)
            .ok_or_else(|| invalid("current-grid view population missing or excessive"))?;
        visit(body, data, mask, encoding, &owned_views)?;
    }
    if !views.is_empty() {
        return Err(invalid(
            "current grid view targets an unqualified source entity",
        ));
    }
    Ok(used)
}

pub(super) fn verify(capture: &Capture, source: &Value) -> Result<BTreeSet<String>> {
    with_current_grids(capture, source, |_, _, _, _, _| Ok(()))
}

pub(super) fn inspect(capture: &Capture, source: &Value) -> Result<Vec<Value>> {
    let mut rows = Vec::new();
    with_current_grids(capture, source, |body, data, mask, encoding, views| {
        let samples = data
            .chunks_exact(encoding.width())
            .map(|raw| match encoding {
                Encoding::F32 => format!("{:08x}", u32::from_le_bytes(raw.try_into().unwrap())),
                Encoding::F64 => format!("{:016x}", u64::from_le_bytes(raw.try_into().unwrap())),
            })
            .collect::<Vec<_>>();
        rows.push(json!({"ownerId":body["owner"]["objectId"],"record":body,
            "samples":samples,"mask":mask,"views":views}));
        Ok(())
    })?;
    Ok(rows)
}
