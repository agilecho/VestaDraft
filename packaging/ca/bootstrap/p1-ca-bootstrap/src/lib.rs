//! P2-02 C4 bootstrap source. No worker, models, browser or product implementation.
pub mod copy;
pub mod fs_guard;
pub mod plan;
pub mod verify;

pub use copy::{CopyReceipt, copy_study, create};
pub use plan::{BootstrapPlan, VerifiedInputs, preflight, read_plan};
use serde::{Deserialize, Serialize};
pub use verify::{PreparedReceipt, verify_prepared};

#[derive(Debug, Serialize)]
pub struct BootstrapError {
    pub stage: &'static str,
    pub code: &'static str,
}
impl BootstrapError {
    pub fn new(stage: &'static str, code: &'static str) -> Self {
        Self { stage, code }
    }
}
impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.stage, self.code)
    }
}
impl std::error::Error for BootstrapError {}
pub type Result<T> = std::result::Result<T, BootstrapError>;
pub(crate) fn io<T>(v: std::io::Result<T>, stage: &'static str) -> Result<T> {
    v.map_err(|_| BootstrapError::new(stage, "filesystem-refused"))
}
pub fn require(ok: bool, stage: &'static str, code: &'static str) -> Result<()> {
    if ok { Ok(()) } else { Err(BootstrapError::new(stage, code)) }
}
pub fn sha(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn json_bytes<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(v).map_err(|_| BootstrapError::new("serialize", "json"))
}

/// Exact canonical order for the frozen C4 study tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
    pub bytes: u64,
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyTree {
    pub files: Vec<FileEntry>,
    pub schema: String,
}

/// Serde's Value normally accepts duplicate object keys. Reject them at every level.
pub fn strict_json(bytes: &[u8]) -> Result<serde_json::Value> {
    use serde::de::{self, MapAccess, SeqAccess, Visitor};
    struct Strict(serde_json::Value);
    impl<'de> Deserialize<'de> for Strict {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
            struct V;
            impl<'de> Visitor<'de> for V {
                type Value = Strict;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("strict JSON")
                }
                fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Strict, E> {
                    Ok(Strict(v.into()))
                }
                fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Strict, E> {
                    Ok(Strict(v.into()))
                }
                fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Strict, E> {
                    Ok(Strict(v.into()))
                }
                fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Strict, E> {
                    serde_json::Number::from_f64(v).map(|n| Strict(n.into())).ok_or_else(|| E::custom("nonfinite"))
                }
                fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Strict, E> {
                    Ok(Strict(v.into()))
                }
                fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Strict, E> {
                    Ok(Strict(v.into()))
                }
                fn visit_unit<E: de::Error>(self) -> std::result::Result<Strict, E> {
                    Ok(Strict(serde_json::Value::Null))
                }
                fn visit_none<E: de::Error>(self) -> std::result::Result<Strict, E> {
                    self.visit_unit()
                }
                fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> std::result::Result<Strict, A::Error> {
                    let mut out = Vec::new();
                    while let Some(Strict(v)) = a.next_element::<Strict>()? {
                        out.push(v)
                    }
                    Ok(Strict(out.into()))
                }
                fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> std::result::Result<Strict, A::Error> {
                    let mut out = serde_json::Map::new();
                    while let Some(k) = a.next_key::<String>()? {
                        if out.contains_key(&k) {
                            return Err(de::Error::custom("duplicate-key"));
                        }
                        let Strict(v) = a.next_value::<Strict>()?;
                        out.insert(k, v);
                    }
                    Ok(Strict(out.into()))
                }
            }
            d.deserialize_any(V)
        }
    }
    let mut de = serde_json::Deserializer::from_slice(bytes);
    let v = Strict::deserialize(&mut de).map_err(|_| BootstrapError::new("parse", "invalid-or-duplicate-json"))?;
    de.end().map_err(|_| BootstrapError::new("parse", "trailing-json"))?;
    Ok(v.0)
}
pub(crate) fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    serde_json::from_value(strict_json(bytes)?).map_err(|_| BootstrapError::new("parse", "schema"))
}
