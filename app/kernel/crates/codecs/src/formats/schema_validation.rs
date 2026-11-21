//! Local strict schema consumer; shared generated bindings remain frozen.
use serde_json::Value;
pub(super) fn ref_target<'a>(r: &str, f: &'a str, c: &'a Value) -> Result<(&'a Value, &'a str), String> {
    let (name, p) = r.split_once('#').unwrap_or((r, ""));
    let key = if name.is_empty() {
        f.to_string()
    } else {
        format!("{}/{}", f.rsplit_once('/').ok_or("schema path")?.0, name)
    };
    let mut parts = Vec::new();
    for a in key.split('/') {
        match a {
            ".." => {
                parts.pop();
            }
            "" | "." => {}
            _ => parts.push(a),
        }
    }
    let key = format!("/{}", parts.join("/"));
    let (file, root) = c
        .as_object()
        .ok_or("catalog")?
        .get_key_value(&key)
        .ok_or("missing ref")?;
    Ok((root.pointer(p).ok_or("schema pointer")?, file.as_str()))
}
fn pattern(p: &str, v: &str) -> bool {
    match p {
        "^(0|[1-9][0-9]*)$" => {
            v == "0"
                || v.as_bytes()
                    .first()
                    .is_some_and(|b| matches!(b, b'1'..=b'9'))
                    && v.bytes().all(|b| b.is_ascii_digit())
        }
        "^[1-9][0-9]*$" => {
            v.as_bytes()
                .first()
                .is_some_and(|b| matches!(b, b'1'..=b'9'))
                && v.bytes().all(|b| b.is_ascii_digit())
        }
        "^(0|-?[1-9][0-9]*)$" | "^(0|-[1-9][0-9]*|[1-9][0-9]*)$" => {
            pattern("^(0|[1-9][0-9]*)$", v.strip_prefix('-').unwrap_or(v)) && v != "-0"
        }
        "^[0-9a-f]{32}$" | "^[0-9a-f]{64}$" | "^[0-9a-f]{16}$" | "^[0-9a-f]{8}$" => {
            let n = p
                .split('{')
                .nth(1)
                .and_then(|s| s.split('}').next())
                .and_then(|n| n.parse::<usize>().ok())
                .unwrap_or(0);
            v.len() == n
                && v.bytes()
                    .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
        }
        "^[0-9a-f]+$" => {
            !v.is_empty()
                && v.bytes()
                    .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
        }
        "^[A-Za-z]$" => v.len() == 1 && v.bytes().all(|b| b.is_ascii_alphabetic()),
        "^VF-[0-9]{4}$" => {
            v.starts_with("VF-") && v.len() == 7 && v[3..].bytes().all(|b| b.is_ascii_digit())
        }
        "^[!-~]{1,2}$" => (1..=2).contains(&v.len()) && v.bytes().all(|b| (33..=126).contains(&b)),
        "^[!-~]{1,10}$" => (1..=10).contains(&v.len()) && v.bytes().all(|b| (33..=126).contains(&b)),
        "^[0-9]+[.][0-9]+$" => v.split_once('.').is_some_and(|(a, b)| {
            !a.is_empty() && !b.is_empty() && a.bytes().chain(b.bytes()).all(|c| c.is_ascii_digit())
        }),
        "^raw-header:[0-9]+:[0-9]+:[0-9]+:[0-9]+:[0-9]+$" => {
            let a: Vec<_> = v.split(':').collect();
            a.len() == 6
                && a[0] == "raw-header"
                && a[1..]
                    .iter()
                    .all(|a| !a.is_empty() && a.bytes().all(|b| b.is_ascii_digit()))
        }
        "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$" => {
            let a = v
                .strip_prefix('+')
                .or_else(|| v.strip_prefix('-'))
                .unwrap_or(v);
            let parts: Vec<_> = a.split(['e', 'E']).collect();
            if parts.len() > 2 {
                return false;
            }
            let m = parts[0];
            let dots = m.bytes().filter(|b| *b == b'.').count();
            let valid = dots <= 1
                && m.bytes().any(|b| b.is_ascii_digit())
                && m.bytes().all(|b| b == b'.' || b.is_ascii_digit());
            valid
                && (parts.len() == 1 || {
                    let e = parts[1].trim_start_matches(['+', '-']);
                    !e.is_empty() && e.bytes().all(|b| b.is_ascii_digit())
                })
        }
        "^(|(/([^~]|~[01])*))$" | "^(/([^~]|~[01])*)*$" => pointer(v),
        _ => false,
    }
}
pub fn validate(v: &Value, s: &Value, f: &str, c: &Value, d: usize) -> Result<(), String> {
    if let Some(o) = s.as_object() {
        for k in o.keys() {
            if ![
                "$schema",
                "$id",
                "$ref",
                "$defs",
                "title",
                "description",
                "$comment",
                "examples",
                "default",
                "deprecated",
                "x-semanticConstraints",
                "type",
                "const",
                "enum",
                "allOf",
                "oneOf",
                "anyOf",
                "properties",
                "required",
                "additionalProperties",
                "items",
                "prefixItems",
                "minItems",
                "maxItems",
                "uniqueItems",
                "minLength",
                "maxLength",
                "pattern",
                "minimum",
                "maximum",
                "exclusiveMinimum",
                "exclusiveMaximum",
            ]
            .contains(&k.as_str())
            {
                return Err(format!("unqualified schema constraint {k}"));
            }
        }
    }
    if d > 256 {
        return Err("schema depth".into());
    }
    if s == &Value::Bool(true) {
        return Ok(());
    }
    if s == &Value::Bool(false) {
        return Err("false schema".into());
    }
    if let Some(r) = s.get("$ref").and_then(Value::as_str) {
        let (a, ff) = ref_target(r, f, c)?;
        validate(v, a, ff, c, d + 1)?;
    }
    if let Some(a) = s.get("allOf").and_then(Value::as_array) {
        for x in a {
            validate(v, x, f, c, d + 1)?;
        }
    }
    for k in ["oneOf", "anyOf"] {
        if let Some(a) = s.get(k).and_then(Value::as_array) {
            let n = a
                .iter()
                .filter(|x| validate(v, x, f, c, d + 1).is_ok())
                .count();
            if n == 0 || k == "oneOf" && n != 1 {
                return Err(k.into());
            }
        }
    }
    if s.get("const").is_some_and(|a| a != v)
        || s.get("enum")
            .and_then(Value::as_array)
            .is_some_and(|a| !a.contains(v))
    {
        return Err("const/enum".into());
    }
    if let Some(t) = s.get("type") {
        let valid = |t: &str| match t {
            "null" => v.is_null(),
            "boolean" => v.is_boolean(),
            "object" => v.is_object(),
            "array" => v.is_array(),
            "string" => v.is_string(),
            "number" => v.is_number(),
            "integer" => v.is_i64() || v.is_u64(),
            _ => false,
        };
        if !t.as_str().is_some_and(valid)
            && !t
                .as_array()
                .is_some_and(|a| a.iter().any(|x| x.as_str().is_some_and(valid)))
        {
            return Err("type".into());
        }
    }
    if let Some(a) = v.as_object() {
        if let Some(req) = s.get("required").and_then(Value::as_array) {
            for k in req {
                if !a.contains_key(k.as_str().ok_or("required key")?) {
                    return Err("missing field".into());
                }
            }
        }
        for (k, v) in a {
            if let Some(ss) = s.get("properties").and_then(|p| p.get(k)) {
                validate(v, ss, f, c, d + 1)?;
            } else if s.get("additionalProperties") == Some(&Value::Bool(false)) {
                return Err("unknown field".into());
            } else if let Some(ss) = s.get("additionalProperties").filter(|a| a.is_object()) {
                validate(v, ss, f, c, d + 1)?;
            }
        }
    }
    if let Some(a) = v.as_array() {
        if s.get("uniqueItems") == Some(&Value::Bool(true)) {
            for (i, x) in a.iter().enumerate() {
                if a[..i].contains(x) {
                    return Err("uniqueItems".into());
                }
            }
        }
        if s.get("minItems")
            .and_then(Value::as_u64)
            .is_some_and(|n| a.len() < (n as usize))
            || s.get("maxItems")
                .and_then(Value::as_u64)
                .is_some_and(|n| a.len() > (n as usize))
        {
            return Err("array size".into());
        }
        for (i, x) in a.iter().enumerate() {
            if let Some(ss) = s
                .get("prefixItems")
                .and_then(|a| a.get(i))
                .or_else(|| s.get("items"))
            {
                validate(x, ss, f, c, d + 1)?;
            }
        }
    }
    if let Some(a) = v.as_str() {
        let n = a.chars().count();
        if s.get("minLength")
            .and_then(Value::as_u64)
            .is_some_and(|m| n < (m as usize))
            || s.get("maxLength")
                .and_then(Value::as_u64)
                .is_some_and(|m| n > (m as usize))
        {
            return Err("string size".into());
        }
        if let Some(p) = s.get("pattern").and_then(Value::as_str) {
            if !pattern(p, a) {
                return Err("pattern".into());
            }
            if p == "^(0|[1-9][0-9]*)$"
                && s.get("maxLength").and_then(Value::as_u64) == Some(20)
                && a.parse::<u64>().is_err()
            {
                return Err("u64 range".into());
            }
        }
    }
    if let Some(n) = v.as_f64() {
        for (k, ok) in [
            ("minimum", 0),
            ("maximum", 1),
            ("exclusiveMinimum", 2),
            ("exclusiveMaximum", 3),
        ] {
            if let Some(m) = s.get(k).and_then(Value::as_f64) {
                if !match ok {
                    0 => n >= m,
                    1 => n <= m,
                    2 => n > m,
                    _ => n < m,
                } {
                    return Err(k.into());
                }
            }
        }
    }
    Ok(())
}

fn pointer(v: &str) -> bool {
    if !v.is_empty() && !v.starts_with('/') {
        return false;
    }
    let mut chars = v.chars();
    while let Some(c) = chars.next() {
        if c == '~' && !matches!(chars.next(), Some('0' | '1')) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn refuses_unknown_constraint_and_duplicate_unique_items() {
        let catalog = json!({});
        assert!(
            validate(
                &json!(1),
                &json!({"type":"integer","multipleOf":2}),
                "/schemas/test.json",
                &catalog,
                0
            )
            .is_err()
        );
        assert!(
            validate(
                &json!([1, 1]),
                &json!({"type":"array","uniqueItems":true}),
                "/schemas/test.json",
                &catalog,
                0
            )
            .is_err()
        );
    }
}
