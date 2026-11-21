//! Active FS1 data uses and checked immutable storage for restored fields.
use crate::{Error, Result, digest};
use serde::{Deserialize, Serialize};

pub const FIELD_ENCODING: &str = "ieee754-little-endian";
pub const FIELD_STORAGE_USE: &str = "FieldStorageUse1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AxisOrder {
    XFastest,
    ZFastest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SampleType {
    F32,
    F64,
}

impl SampleType {
    fn width(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldKind {
    ElectronDensity,
    NuclearDensity,
    ElectronPatterson,
    NuclearPatterson,
    Generic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldUnit {
    #[serde(rename = "ePerA3")]
    EPerA3,
    #[serde(rename = "fmPerA3")]
    FmPerA3,
    #[serde(rename = "e2PerA3")]
    E2PerA3,
    #[serde(rename = "fm2PerA3")]
    Fm2PerA3,
    #[serde(rename = "declared")]
    Declared,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldResource {
    pub digest: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldDescriptor {
    pub resource: FieldResource,
    pub kind: FieldKind,
    pub unit: FieldUnit,
    pub declared_unit: Option<String>,
    pub core_shape: [u64; 3],
    pub closure: [bool; 3],
    pub periodic: [bool; 3],
    pub axis_order: AxisOrder,
    pub origin_a: [f64; 3],
    pub dtype: SampleType,
    pub step_basis_a: [[f64; 3]; 3],
}

impl FieldDescriptor {
    fn checked_shape(&self) -> Result<[usize; 3]> {
        if self.core_shape.contains(&0) {
            return Err(Error::new("E_FIELD_STORAGE", "core shape must be positive"));
        }
        let mut stored = [0; 3];
        for (axis, dimension) in stored.iter_mut().enumerate() {
            if self.closure[axis] && !self.periodic[axis] {
                return Err(Error::new(
                    "E_FIELD_CLOSURE",
                    "closure requires periodic axis",
                ));
            }
            let count = self.core_shape[axis]
                .checked_add(self.closure[axis] as u64)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| Error::new("E_FIELD_STORAGE", "stored shape overflow"))?;
            *dimension = count;
        }
        Ok(stored)
    }

    fn validate_axes_and_quantity(&self) -> Result<()> {
        let unit = match self.kind {
            FieldKind::ElectronDensity => FieldUnit::EPerA3,
            FieldKind::NuclearDensity => FieldUnit::FmPerA3,
            FieldKind::ElectronPatterson => FieldUnit::E2PerA3,
            FieldKind::NuclearPatterson => FieldUnit::Fm2PerA3,
            FieldKind::Generic => FieldUnit::Declared,
        };
        if self.unit != unit
            || if unit == FieldUnit::Declared {
                self.declared_unit
                    .as_deref()
                    .is_none_or(|s| s.trim().is_empty())
            } else {
                self.declared_unit.is_some()
            }
        {
            return Err(Error::new(
                "E_FIELD_DESCRIPTOR",
                "field quantity and unit differ",
            ));
        }
        if !self.origin_a.iter().all(|v| v.is_finite()) {
            return Err(Error::new("E_FIELD_DESCRIPTOR", "non-finite field origin"));
        }
        let mut basis = self.step_basis_a;
        for row in &mut basis {
            let scale = row.iter().fold(0_f64, |a, v| a.max(v.abs()));
            if scale == 0.0 || !row.iter().all(|v| v.is_finite()) {
                return Err(Error::new("E_FIELD_DESCRIPTOR", "invalid field basis"));
            }
            for x in row {
                *x /= scale;
            }
        }
        let [a, b, c] = basis;
        let det = a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
        if det == 0.0 {
            return Err(Error::new("E_FIELD_DESCRIPTOR", "singular field basis"));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FieldBuffer<'a> {
    bytes: &'a [u8],
    stored: [usize; 3],
    dtype: SampleType,
    order: AxisOrder,
}

impl<'a> FieldBuffer<'a> {
    pub fn open(descriptor: &FieldDescriptor, bytes: &'a [u8]) -> Result<Self> {
        descriptor.validate_axes_and_quantity()?;
        let stored = descriptor.checked_shape()?;
        let size = stored
            .into_iter()
            .try_fold(descriptor.dtype.width(), usize::checked_mul)
            .ok_or_else(|| Error::new("E_FIELD_STORAGE", "field byte count overflow"))?;
        if size != bytes.len()
            || size as u64 != descriptor.resource.bytes
            || digest(bytes) != descriptor.resource.digest
        {
            return Err(Error::new(
                "E_FIELD_STORAGE",
                "declared storage and actual field bytes differ",
            ));
        }
        let buffer = Self {
            bytes,
            stored,
            dtype: descriptor.dtype,
            order: descriptor.axis_order,
        };
        let core =
            std::array::from_fn::<_, 3, _>(|axis| stored[axis] - descriptor.closure[axis] as usize);
        for i in 0..stored[0] {
            for j in 0..stored[1] {
                for k in 0..stored[2] {
                    let at = [i, j, k];
                    let sample = buffer.bits_at(at);
                    let finite = match buffer.dtype {
                        SampleType::F32 => f32::from_bits(sample as u32).is_finite(),
                        SampleType::F64 => f64::from_bits(sample).is_finite(),
                    };
                    if !finite {
                        return Err(Error::new(
                            "E_FIELD_VALUE",
                            format!("non-finite sample at {at:?}"),
                        ));
                    }
                    let source =
                        std::array::from_fn(
                            |axis| if at[axis] == core[axis] { 0 } else { at[axis] },
                        );
                    if at != source && sample != buffer.bits_at(source) {
                        return Err(Error::new(
                            "E_FIELD_CLOSURE",
                            format!("different periodic closure sample at {at:?}"),
                        ));
                    }
                }
            }
        }
        Ok(buffer)
    }

    pub fn stored_shape(&self) -> [usize; 3] {
        self.stored
    }

    pub fn sample_count(&self) -> usize {
        self.bytes.len() / self.dtype.width()
    }

    pub fn sample_bits(&self, at: [usize; 3]) -> Option<u64> {
        at.into_iter()
            .zip(self.stored)
            .all(|(i, len)| i < len)
            .then(|| self.bits_at(at))
    }

    fn bits_at(&self, [x, y, z]: [usize; 3]) -> u64 {
        let [nx, ny, nz] = self.stored;
        let index = match self.order {
            AxisOrder::XFastest => (z * ny + y) * nx + x,
            AxisOrder::ZFastest => (x * ny + y) * nz + z,
        };
        let start = index * self.dtype.width();
        match self.dtype {
            SampleType::F32 => u64::from(u32::from_le_bytes(
                self.bytes[start..start + 4]
                    .try_into()
                    .expect("validated sample size"),
            )),
            SampleType::F64 => u64::from_le_bytes(
                self.bytes[start..start + 8]
                    .try_into()
                    .expect("validated sample size"),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "kebab-case",
    deny_unknown_fields
)]
pub enum FieldOwner {
    Phase(String),
    Molecule(String),
    Scene(String),
}

impl FieldOwner {
    fn entity(&self) -> (&'static str, &str) {
        match self {
            Self::Phase(id) => ("Phase", id),
            Self::Molecule(id) => ("Molecule", id),
            Self::Scene(id) => ("Scene", id),
        }
    }

    fn use_key(&self, source: &serde_json::Value, ordinal: usize) -> Result<String> {
        let (_, id) = self.entity();
        let (kind, owners) = match self {
            Self::Phase(_) => ("phase", &source["phaseGraph"]["phases"]),
            Self::Molecule(_) => ("molecule", &source["molecules"]),
            Self::Scene(_) => ("scene", &source["scenes"]),
        };
        let index = owners
            .as_array()
            .and_then(|rows| rows.iter().position(|row| row["id"] == id))
            .ok_or_else(|| coverage("field owner is missing from typed state"))?;
        Ok(format!("field-data/{ordinal}/{kind}/{index}"))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveField {
    pub field_id: String,
    pub owner: FieldOwner,
    pub resource_id: String,
    pub descriptor: FieldDescriptor,
}

fn coverage(message: impl Into<String>) -> Error {
    Error::new("E_REOPEN_COVERAGE", message)
}

fn extent(message: impl Into<String>) -> Error {
    Error::new("E_RESOURCE_LIMIT", message)
}

fn entity(value: &serde_json::Value, kind: &str, id: &str) -> bool {
    value == &serde_json::json!({"kind":"EntityRef","objectKind":kind,"objectId":id})
}

fn literal(value: &serde_json::Value, expected: &str) -> bool {
    value == &serde_json::json!({"kind":"Utf8","value":expected})
}

/// Reconstruct the active field table from actual typed owner/data-use bindings.
/// SourceResource rows describe logical identities; stored bytes are keyed by digest only.
pub fn restore_fields(
    source: &serde_json::Value,
    resources: &[crate::PersistentResource],
    bytes: &std::collections::BTreeMap<String, Vec<u8>>,
    max_samples: usize,
) -> Result<Vec<ActiveField>> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut installed: BTreeMap<_, _> = resources
        .iter()
        .filter(|r| r.logical_key.starts_with("field-data/"))
        .map(|r| (r.logical_key.as_str(), r))
        .collect();
    let mut active = Vec::new();
    let mut count = 0usize;
    let mut identities = BTreeSet::new();
    let mut storage_ids = BTreeSet::new();
    for (ordinal, row) in source["fields"]
        .as_array()
        .ok_or_else(|| coverage("source requires a field collection"))?
        .iter()
        .enumerate()
    {
        let id = row["id"]
            .as_str()
            .filter(|id| crate::valid_id(id))
            .ok_or_else(|| coverage("field requires a valid identity"))?;
        if !identities.insert(id) {
            return Err(coverage("duplicate Field identity"));
        }
        let owner: FieldOwner = serde_json::from_value(row["owner"].clone())
            .map_err(|_| coverage("field owner kind is not qualified"))?;
        let (kind, owner_id) = owner.entity();
        let use_key = owner.use_key(source, ordinal)?;
        let descriptor: FieldDescriptor = serde_json::from_value(row["field"].clone())
            .map_err(|e| coverage(format!("field descriptor: {e}")))?;
        let mut bindings = row["metadata"]
            .as_array()
            .ok_or_else(|| coverage("field metadata collection is missing"))?
            .iter()
            .filter(|m| m["tree"]["tag"] == FIELD_STORAGE_USE);
        let binding = bindings
            .next()
            .ok_or_else(|| coverage("missing registered FieldStorageUse1"))?;
        if bindings.next().is_some()
            || binding["owner"] != serde_json::json!({"kind":"field","id":id})
            || binding["tree"]["kind"] != "TaggedVariant"
            || binding["tree"]["payload"]["kind"] != "Map"
        {
            return Err(coverage("ambiguous or invalid Field storage owner"));
        }
        let payload = binding["tree"]["payload"]["members"]
            .as_object()
            .ok_or_else(|| coverage("field storage binding lacks members"))?;
        let hash = digest(&crate::source_audit::canonical(&row["field"]));
        let value = |k| payload.get(k).unwrap_or(&serde_json::Value::Null);
        if payload.len() != 6
            || !entity(value("field"), "Field", id)
            || !entity(value("owner"), kind, owner_id)
            || !literal(
                value("usePointer"),
                &format!("/fields/{ordinal}/field/resource"),
            )
            || !literal(value("encoding"), FIELD_ENCODING)
            || !literal(value("descriptorDigest"), &hash)
        {
            return Err(coverage(
                "field storage use does not match actual owner and descriptor",
            ));
        }
        let binding_bytes = value("bytes");
        let resource_id = binding_bytes["resourceId"]
            .as_str()
            .ok_or_else(|| coverage("field storage use lacks a logical resource identity"))?;
        let resource = installed.remove(use_key.as_str()).ok_or_else(|| {
            Error::new("E_REOPEN_OWNER", format!("missing installed field use {use_key}"))
        })?;
        if resource.id.as_str() != resource_id {
            return Err(Error::new(
                "E_REOPEN_OWNER",
                format!("Field storage points to a different logical resource than {use_key}"),
            ));
        }
        if !storage_ids.insert(resource_id)
            || binding_bytes
                != &serde_json::json!({
                    "kind":"BytesRef","resourceId":resource_id,"sha256":resource.sha256,"byteLength":resource.byte_length
                })
            || resource.sha256 != descriptor.resource.digest
            || resource.byte_length != descriptor.resource.bytes.to_string()
        {
            return Err(coverage(
                "field storage logical resource disagrees with descriptor",
            ));
        }
        let raw = bytes
            .get(&resource.sha256)
            .ok_or_else(|| coverage("field storage bytes absent"))?;
        let stored = descriptor.checked_shape()?;
        count = stored
            .into_iter()
            .try_fold(1usize, usize::checked_mul)
            .and_then(|n| count.checked_add(n))
            .filter(|n| *n <= max_samples)
            .ok_or_else(|| extent("cumulative active field samples exceed host bounds"))?;
        FieldBuffer::open(&descriptor, raw)?;
        active.push(ActiveField {
            field_id: id.into(),
            owner,
            resource_id: resource_id.into(),
            descriptor,
        });
    }
    if !installed.is_empty() {
        return Err(coverage("installed Field resource has no typed use"));
    }
    Ok(active)
}
