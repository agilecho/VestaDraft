//! Strict bounded JSON plus the frozen generated I33 schema validator.
use crate::{Error, Result};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::fmt;
struct Seed<'a> {
    left: &'a mut usize,
    depth: usize,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> std::result::Result<Value, D::Error> {
        if self.depth > 64 || *self.left == 0 {
            return Err(de::Error::custom("JSON node/depth budget"));
        }
        *self.left -= 1;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "strict bounded JSON")
    }
    fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Value, E> {
        Ok(v.into())
    }
    fn visit_unit<E: de::Error>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite"))
    }
    fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Value, E> {
        Ok(v.into())
    }
    fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Value, E> {
        Ok(v.into())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> std::result::Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            left: self.left,
            depth: self.depth + 1,
        })? {
            out.push(v)
        }
        Ok(out.into())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> std::result::Result<Value, A::Error> {
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(de::Error::custom("duplicate JSON key"));
            }
            let v = a.next_value_seed(Seed {
                left: self.left,
                depth: self.depth + 1,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
pub fn parse(bytes: &[u8], limit: usize) -> Result<Value> {
    if bytes.len() > limit {
        return Err(Error::new("E_RESOURCE_LIMIT", "JSON bytes"));
    }
    let mut d = serde_json::Deserializer::from_slice(bytes);
    let v = Seed {
        left: &mut 200000,
        depth: 0,
    }
    .deserialize(&mut d)
    .map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    d.end().map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
    Ok(v)
}
pub fn schema(v: &Value, file: &str, definition: &str) -> Result<()> {
    use std::sync::OnceLock;
    static C: OnceLock<Value> = OnceLock::new();
    let c = C.get_or_init(|| {
        let mut catalog: Value = serde_json::from_str(vesta_bindings::resource_roles::CATALOG)
            .expect("compiled schema catalog");
        // The inherited validator resolves file references relative to the current file.
        // Preserve exact targets of frozen absolute references using lexical relative URIs.
        // This changes no frozen schema, CPJ byte or generated shared-contract source.
        let keys: std::collections::BTreeSet<String> = catalog
            .as_object()
            .expect("compiled catalog map")
            .keys()
            .cloned()
            .collect();
        for (file, root) in catalog.as_object_mut().expect("compiled catalog map") {
            normalize_absolute_refs(root, file, &keys);
        }
        catalog
    });
    let (key, root) = c
        .as_object()
        .and_then(|c| c.iter().find(|(k, _)| k.ends_with(&format!("/{file}"))))
        .ok_or_else(|| Error::new("E_SCHEMA", "schema root missing"))?;
    let s = root
        .get("$defs")
        .and_then(|d| d.get(definition))
        .ok_or_else(|| Error::new("E_SCHEMA", "definition missing"))?;
    vesta_bindings::decoder::validate(v, s, key, c, 0).map_err(|s| Error::new("E_SCHEMA", s))
}
fn normalize_absolute_refs(v: &mut Value, file: &str, keys: &std::collections::BTreeSet<String>) {
    match v {
        Value::Object(o) => {
            if let Some(Value::String(reference)) = o.get_mut("$ref") {
                let (target, fragment) = reference
                    .split_once('#')
                    .unwrap_or((reference.as_str(), ""));
                if target.starts_with('/') && keys.contains(target) {
                    let base: Vec<_> = file
                        .rsplit_once('/')
                        .map(|(p, _)| p)
                        .unwrap_or("")
                        .split('/')
                        .filter(|s| !s.is_empty())
                        .collect();
                    let target_parts: Vec<_> =
                        target.split('/').filter(|s| !s.is_empty()).collect();
                    let common = base
                        .iter()
                        .zip(&target_parts)
                        .take_while(|(a, b)| a == b)
                        .count();
                    let mut parts = vec![".."; base.len() - common];
                    parts.extend_from_slice(&target_parts[common..]);
                    *reference = format!("{}#{}", parts.join("/"), fragment);
                }
            }
            for child in o.values_mut() {
                normalize_absolute_refs(child, file, keys)
            }
        }
        Value::Array(a) => {
            for child in a {
                normalize_absolute_refs(child, file, keys)
            }
        }
        _ => {}
    }
}
pub fn u64_str(v: &Value) -> Result<u64> {
    let s = v
        .as_str()
        .ok_or_else(|| Error::new("E_SCHEMA", "u64 string"))?;
    if s.is_empty() || (s.len() > 1 && s.starts_with('0')) || !s.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Error::new("E_SCHEMA", "canonical u64"));
    }
    s.parse()
        .map_err(|_| Error::new("E_SCHEMA", "u64 overflow"))
}
