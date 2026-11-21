//! Typed source algebra. No Serialize fallback and no property-name role discovery.
use crate::{Error, Ids, Result, digest};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    marker::PhantomData,
    ops::Deref,
};
pub trait Kind: Clone + fmt::Debug + PartialEq + Eq + Ord {
    const NAME: &'static str;
}
macro_rules! kinds {($($k:ident),*)=>{$(#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]pub enum $k{} impl Kind for $k{const NAME:&'static str=stringify!($k);})*}}
kinds!(
    Document,
    Epoch,
    Scene,
    Phase,
    Site,
    SymmetryOp,
    Resource,
    Field,
    Molecule,
    Frame,
    Atom,
    Bond,
    BasisCenter,
    SceneNode,
    SourceEntity,
    Section,
    BondRule,
    Boundary,
    Plane,
    Shape,
    ShapeHalfspace,
    Vector,
    Material,
    Display,
    Camera,
    Mesh,
    Dependency,
    OpaqueBlock,
    MeasurementPoint
);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id<K: Kind> {
    raw: String,
    kind: PhantomData<K>,
}
impl<K: Kind> Id<K> {
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}
impl<K: Kind> From<String> for Id<K> {
    fn from(raw: String) -> Self {
        Self {
            raw,
            kind: PhantomData,
        }
    }
}
impl<K: Kind> Deref for Id<K> {
    type Target = str;
    fn deref(&self) -> &str {
        &self.raw
    }
}
impl<K: Kind> fmt::Display for Id<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.raw)
    }
}
impl<K: Kind> PartialEq<Value> for Id<K> {
    fn eq(&self, v: &Value) -> bool {
        v.as_str() == Some(self.raw.as_str())
    }
}
impl<K: Kind> Serialize for Id<K> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.raw)
    }
}
impl<'de, K: Kind> Deserialize<'de> for Id<K> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        if !crate::valid_id(&s) {
            return Err(serde::de::Error::custom("SOURCE_ID"));
        }
        Ok(s.into())
    }
}
pub type Remap = BTreeMap<(String, String), String>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Definition,
    Reference,
    Alias,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Owned {
        kind: String,
        id: String,
        node: Box<Node>,
    },
    Null,
    Bool(bool),
    Text(String),
    I64(i64),
    U64(u64),
    F64(u64),
    F32(u32),
    Array(Vec<Node>),
    Map(BTreeMap<String, Node>),
    TaggedVariant {
        tag: String,
        payload: Box<Node>,
    },
    Identity {
        kind: String,
        id: String,
        role: Role,
    },
    Resource {
        id: String,
        sha: String,
        len: String,
        role: Role,
    },
}
pub trait AuditSource {
    fn audit(&self) -> Result<Node>;
    fn rebind(&mut self, map: &Remap) -> Result<()>;
    fn descriptor() -> Value;
}
/// Only ordinary persistent values implement this. Id<K> deliberately does not.
pub trait ValueSource: AuditSource {}
pub fn value<T: ValueSource>(v: &T) -> Result<Node> {
    v.audit()
}
pub trait IdentityValue: AuditSource {
    type Kind: Kind;
    fn identity(&self, role: Role) -> Result<Node>;
}
pub fn identity<K: Kind, T: IdentityValue<Kind = K>>(v: &T, role: Role) -> Result<Node> {
    v.identity(role)
}
pub fn rebind_identity<K: Kind, T: IdentityValue<Kind = K>>(v: &mut T, map: &Remap) -> Result<()> {
    v.rebind(map)
}
pub fn owned<K: Kind, T: IdentityValue<Kind = K>>(v: &T, node: Node) -> Result<Node> {
    if let Node::Identity { id, .. } = v.identity(Role::Reference)? {
        Ok(Node::Owned {
            kind: K::NAME.into(),
            id,
            node: Box::new(node),
        })
    } else {
        Err(Error::new("E_SCHEMA", "SOURCE_OWNER"))
    }
}
impl<K: Kind> AuditSource for Id<K> {
    fn audit(&self) -> Result<Node> {
        Err(Error::new("E_SCHEMA", "SOURCE_UNCLASSIFIED_ID"))
    }
    fn rebind(&mut self, map: &Remap) -> Result<()> {
        self.raw = map
            .get(&(K::NAME.into(), self.raw.clone()))
            .ok_or_else(|| Error::new("E_SCHEMA", "unmapped typed identity"))?
            .clone();
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"identity","kind":K::NAME})
    }
}
impl<K: Kind> IdentityValue for Id<K> {
    type Kind = K;
    fn identity(&self, role: Role) -> Result<Node> {
        if !crate::valid_id(&self.raw) {
            return Err(Error::new("E_SCHEMA", "SOURCE_ID"));
        }
        Ok(Node::Identity {
            kind: K::NAME.into(),
            id: self.raw.clone(),
            role,
        })
    }
}
macro_rules! scalar {($t:ty,$variant:ident,$name:literal)=>{impl AuditSource for $t{fn audit(&self)->Result<Node>{Ok(Node::$variant(self.clone().into()))}fn rebind(&mut self,_:&Remap)->Result<()>{Ok(())}fn descriptor()->Value{json!({"type":$name})}}impl ValueSource for $t{}}}
scalar!(bool, Bool, "bool");
scalar!(String, Text, "utf8");
scalar!(i64, I64, "i64");
scalar!(i32, I64, "i32");
scalar!(u64, U64, "u64");
scalar!(u32, U64, "u32");
scalar!(u8, U64, "u8");
impl AuditSource for f64 {
    fn audit(&self) -> Result<Node> {
        Ok(Node::F64(self.to_bits()))
    }
    fn rebind(&mut self, _: &Remap) -> Result<()> {
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"f64-bits"})
    }
}
impl ValueSource for f64 {}
impl AuditSource for f32 {
    fn audit(&self) -> Result<Node> {
        Ok(Node::F32(self.to_bits()))
    }
    fn rebind(&mut self, _: &Remap) -> Result<()> {
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"f32-bits"})
    }
}
impl ValueSource for f32 {}
impl AuditSource for serde_json::Number {
    fn audit(&self) -> Result<Node> {
        if let Some(value) = self.as_u64() {
            Ok(Node::U64(value))
        } else if let Some(value) = self.as_i64() {
            Ok(Node::I64(value))
        } else {
            self.as_f64()
                .map(|value| Node::F64(value.to_bits()))
                .ok_or_else(|| Error::new("E_SCHEMA", "source number"))
        }
    }
    fn rebind(&mut self, _: &Remap) -> Result<()> {
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"json-number","representations":["u64","i64","f64-bits"]})
    }
}
impl ValueSource for serde_json::Number {}
impl<T: AuditSource> AuditSource for Vec<T> {
    fn audit(&self) -> Result<Node> {
        Ok(Node::Array(
            self.iter().map(AuditSource::audit).collect::<Result<_>>()?,
        ))
    }
    fn rebind(&mut self, m: &Remap) -> Result<()> {
        for v in self {
            v.rebind(m)?;
        }
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"array","element":T::descriptor()})
    }
}
impl<T: ValueSource> ValueSource for Vec<T> {}
impl<T: AuditSource, const N: usize> AuditSource for [T; N] {
    fn audit(&self) -> Result<Node> {
        Ok(Node::Array(
            self.iter().map(AuditSource::audit).collect::<Result<_>>()?,
        ))
    }
    fn rebind(&mut self, m: &Remap) -> Result<()> {
        for v in self {
            v.rebind(m)?;
        }
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"fixed-array","length":N,"element":T::descriptor()})
    }
}
impl<T: ValueSource, const N: usize> ValueSource for [T; N] {}
impl<T: AuditSource> AuditSource for Option<T> {
    fn audit(&self) -> Result<Node> {
        match self {
            Some(x) => x.audit(),
            None => Ok(Node::Null),
        }
    }
    fn rebind(&mut self, m: &Remap) -> Result<()> {
        if let Some(x) = self {
            x.rebind(m)?;
        }
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"option","element":T::descriptor()})
    }
}
impl<T: ValueSource> ValueSource for Option<T> {}
impl<T: IdentityValue> IdentityValue for Option<T> {
    type Kind = T::Kind;
    fn identity(&self, r: Role) -> Result<Node> {
        match self {
            Some(x) => x.identity(r),
            None => Ok(Node::Null),
        }
    }
}
impl<T: IdentityValue> IdentityValue for Vec<T> {
    type Kind = T::Kind;
    fn identity(&self, r: Role) -> Result<Node> {
        Ok(Node::Array(
            self.iter().map(|v| v.identity(r)).collect::<Result<_>>()?,
        ))
    }
}
impl<T: IdentityValue, const N: usize> IdentityValue for [T; N] {
    type Kind = T::Kind;
    fn identity(&self, role: Role) -> Result<Node> {
        Ok(Node::Array(
            self.iter()
                .map(|value| value.identity(role))
                .collect::<Result<_>>()?,
        ))
    }
}
impl<T: AuditSource> AuditSource for BTreeMap<String, T> {
    fn audit(&self) -> Result<Node> {
        Ok(Node::Map(
            self.iter()
                .map(|(k, v)| Ok((k.clone(), v.audit()?)))
                .collect::<Result<_>>()?,
        ))
    }
    fn rebind(&mut self, m: &Remap) -> Result<()> {
        for v in self.values_mut() {
            v.rebind(m)?;
        }
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"map","value":T::descriptor()})
    }
}
impl<T: ValueSource> ValueSource for BTreeMap<String, T> {}
pub type ResourceBytes = BTreeMap<String, (String, Vec<u8>)>;
impl Node {
    pub fn bind_resources(&mut self, resources: &ResourceBytes) -> Result<()> {
        match self {
            Self::Owned { node, .. } | Self::TaggedVariant { payload: node, .. } => {
                node.bind_resources(resources)?
            }
            Self::Identity { kind, id, role } if kind == "Resource" => {
                let (sha, b) = resources.get(id).ok_or_else(|| {
                    Error::new("E_RESOURCE_MISSING", "typed resource identity missing")
                })?;
                if digest(b) != *sha {
                    return Err(Error::new("E_SCHEMA", "resource digest"));
                }
                *self = Self::Resource {
                    id: id.clone(),
                    sha: sha.clone(),
                    len: b.len().to_string(),
                    role: *role,
                };
            }
            Self::Resource { id, sha, len, .. } => {
                let (actual_sha, bytes) = resources.get(id).ok_or_else(|| {
                    Error::new(
                        "E_RESOURCE_MISSING",
                        "typed byte use lacks its logical resource",
                    )
                })?;
                if sha != actual_sha || digest(bytes) != *sha || bytes.len().to_string() != *len {
                    return Err(Error::new("E_SCHEMA", "typed byte use digest or length"));
                }
            }
            Self::Map(m) => {
                for n in m.values_mut() {
                    n.bind_resources(resources)?;
                }
            }
            Self::Array(a) => {
                for n in a {
                    n.bind_resources(resources)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn wire(&self) -> Value {
        match self {
            Self::Owned { node, .. } => node.wire(),
            Self::Null => json!({"kind":"Null"}),
            Self::Bool(v) => json!({"kind":"Bool","value":v}),
            Self::Text(v) => json!({"kind":"Utf8","value":v}),
            Self::I64(v) => json!({"kind":"I64","value":v.to_string()}),
            Self::U64(v) => json!({"kind":"U64","value":v.to_string()}),
            Self::F64(v) => json!({"kind":"F64Bits","ieeeHexBE":format!("{v:016x}")}),
            Self::F32(v) => json!({"kind":"F32Bits","ieeeHexBE":format!("{v:08x}")}),
            Self::Identity { kind, id, .. } => {
                json!({"kind":"EntityRef","objectKind":kind,"objectId":id})
            }
            Self::Resource { id, sha, len, .. } => {
                json!({"kind":"BytesRef","resourceId":id,"sha256":sha,"byteLength":len})
            }
            Self::Array(a) => {
                json!({"kind":"Array","items":a.iter().map(Node::wire).collect::<Vec<_>>()})
            }
            Self::Map(m) => {
                json!({"kind":"Map","members":m.iter().map(|(k,v)|(k.clone(),v.wire())).collect::<BTreeMap<_,_>>()})
            }
            Self::TaggedVariant { tag, payload } => {
                json!({"kind":"TaggedVariant","tag":tag,"payload":payload.wire()})
            }
        }
    }
    pub fn data(&self) -> Result<Value> {
        Ok(match self {
            Self::Owned { node, .. } => node.data()?,
            Self::Null => Value::Null,
            Self::Bool(v) => json!(v),
            Self::Text(v) => json!(v),
            Self::I64(v) => json!(v),
            Self::U64(v) => json!(v),
            Self::F64(v) => {
                let n = f64::from_bits(*v);
                if !n.is_finite() {
                    return Err(Error::new("E_SCHEMA", "active nonfinite"));
                }
                json!(n)
            }
            Self::F32(v) => {
                let n = f32::from_bits(*v);
                if !n.is_finite() {
                    return Err(Error::new("E_SCHEMA", "active nonfinite"));
                }
                json!(n)
            }
            Self::Identity { id, .. } | Self::Resource { id, .. } => json!(id),
            Self::Array(a) => json!(a.iter().map(Node::data).collect::<Result<Vec<_>>>()?),
            Self::Map(m) => json!(
                m.iter()
                    .map(|(k, v)| Ok((k.clone(), v.data()?)))
                    .collect::<Result<BTreeMap<_, _>>>()?
            ),
            Self::TaggedVariant { .. } => self.wire(),
        })
    }
    pub fn from_wire(v: &Value) -> Result<Self> {
        Self::from_wire_at(v, 0)
    }
    fn from_wire_at(v: &Value, depth: usize) -> Result<Self> {
        let e = || Error::new("E_SCHEMA", "closed typed audit node");
        if depth > 64 {
            return Err(Error::new("E_PARSE_LIMIT", "audit node nesting"));
        }
        let text = |k: &str| v[k].as_str().map(str::to_owned).ok_or_else(e);
        Ok(match v["kind"].as_str().ok_or_else(e)? {
            "Null" => Self::Null,
            "Bool" => Self::Bool(v["value"].as_bool().ok_or_else(e)?),
            "Utf8" => Self::Text(text("value")?),
            "I64" => Self::I64(text("value")?.parse().map_err(|_| e())?),
            "U64" => Self::U64(text("value")?.parse().map_err(|_| e())?),
            "F64Bits" => Self::F64(u64::from_str_radix(&text("ieeeHexBE")?, 16).map_err(|_| e())?),
            "F32Bits" => Self::F32(u32::from_str_radix(&text("ieeeHexBE")?, 16).map_err(|_| e())?),
            "EntityRef" => Self::Identity {
                kind: text("objectKind")?,
                id: text("objectId")?,
                role: Role::Reference,
            },
            "BytesRef" => Self::Resource {
                id: text("resourceId")?,
                sha: text("sha256")?,
                len: text("byteLength")?,
                role: Role::Reference,
            },
            "Array" => Self::Array(
                v["items"]
                    .as_array()
                    .ok_or_else(e)?
                    .iter()
                    .map(|item| Self::from_wire_at(item, depth + 1))
                    .collect::<Result<_>>()?,
            ),
            "Map" => Self::Map(
                v["members"]
                    .as_object()
                    .ok_or_else(e)?
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), Self::from_wire_at(v, depth + 1)?)))
                    .collect::<Result<_>>()?,
            ),
            "TaggedVariant" if v.as_object().is_some_and(|o| o.len() == 3) => {
                let tag = text("tag")?;
                if tag.is_empty() {
                    return Err(e());
                }
                Self::TaggedVariant {
                    tag,
                    payload: Box::new(Self::from_wire_at(&v["payload"], depth + 1)?),
                }
            }
            _ => return Err(e()),
        })
    }
    pub fn roles(&self) -> Vec<(String, String, String, Role)> {
        fn visit(n: &Node, p: &str, out: &mut Vec<(String, String, String, Role)>) {
            match n {
                Node::Owned { node, .. } => visit(node, p, out),
                Node::Identity { kind, id, role } => {
                    out.push((p.into(), kind.clone(), id.clone(), *role))
                }
                Node::Resource { id, role, .. } => {
                    out.push((p.into(), "Resource".into(), id.clone(), *role))
                }
                Node::Array(a) => {
                    for (i, n) in a.iter().enumerate() {
                        visit(n, &format!("{p}/{i}"), out)
                    }
                }
                Node::Map(m) => {
                    for (k, n) in m {
                        visit(
                            n,
                            &format!("{p}/{}", k.replace('~', "~0").replace('/', "~1")),
                            out,
                        )
                    }
                }
                Node::TaggedVariant { payload, .. } => visit(payload, &format!("{p}/payload"), out),
                _ => {}
            }
        }
        let mut out = vec![];
        visit(self, "", &mut out);
        out
    }
    pub fn definitions(&self) -> Result<BTreeMap<(String, String), String>> {
        let mut defs = BTreeMap::new();
        let mut raw = BTreeSet::new();
        let roles = self.roles();
        for (p, k, id, r) in &roles {
            if *r == Role::Definition {
                if !crate::valid_id(id)
                    || !raw.insert(id.clone())
                    || defs.insert((k.clone(), id.clone()), p.clone()).is_some()
                {
                    return Err(Error::new(
                        "E_SCHEMA",
                        "duplicate/cross-kind source definition",
                    ));
                }
            }
        }
        for (_, k, id, r) in roles {
            if r != Role::Definition && !defs.contains_key(&(k, id)) {
                return Err(Error::new("E_SCHEMA", "dangling typed source reference"));
            }
        }
        Ok(defs)
    }
    pub fn fresh_map(&self, ids: &mut Ids) -> Result<Remap> {
        self.fresh_map_avoiding(ids, &BTreeSet::new())
    }
    pub fn fresh_map_avoiding(&self, ids: &mut Ids, retired: &BTreeSet<String>) -> Result<Remap> {
        let definitions = self.definitions()?;
        let mut used = retired.clone();
        used.extend(definitions.keys().map(|(_, id)| id.clone()));
        let mut map = Remap::new();
        for key in definitions.keys() {
            loop {
                let id = ids.next_id();
                if used.insert(id.clone()) {
                    map.insert(key.clone(), id);
                    break;
                }
            }
        }
        Ok(map)
    }
    /// Every container and scalar is a unit; no codec field inventory is accepted.
    pub fn ledger(&self, snapshot: &Value) -> Value {
        fn walk(n: &Node, p: &str, s: &Value, owner: (&str, &str, &str), out: &mut Vec<Value>) {
            if let Node::Owned { kind, id, node } = n {
                walk(node, p, s, (kind, id, p), out);
                return;
            }
            let payload = match n {
                Node::Map(m) => json!({"kind":"Map","keys":m.keys().collect::<Vec<_>>()}),
                Node::Array(a) => json!({"kind":"Array","count":a.len().to_string()}),
                Node::TaggedVariant { tag, .. } => json!({"kind":"TaggedVariant","tag":tag}),
                _ => n.wire(),
            };
            out.push(json!({"snapshot":s,"ownerKind":owner.0,"ownerId":owner.1,"objectKind":owner.0,"objectId":owner.1,"relativePath":p.strip_prefix(owner.2).unwrap_or(p),"sourceNodeDigest":digest(&canonical(&payload)),"role":if matches!(n,Node::Identity{..}){"identity-only"}else{"required-persistent"}}));
            match n {
                Node::Map(m) => {
                    for (k, v) in m {
                        walk(
                            v,
                            &format!("{p}/{}", k.replace('~', "~0").replace('/', "~1")),
                            s,
                            owner,
                            out,
                        )
                    }
                }
                Node::Array(a) => {
                    for (i, v) in a.iter().enumerate() {
                        walk(v, &format!("{p}/{i}"), s, owner, out)
                    }
                }
                Node::TaggedVariant { payload, .. } => {
                    walk(payload, &format!("{p}/payload"), s, owner, out);
                }
                _ => {}
            }
        }
        let mut rows = vec![];
        walk(
            self,
            "",
            snapshot,
            (
                "document",
                snapshot["documentId"].as_str().unwrap_or(""),
                "",
            ),
            &mut rows,
        );
        json!(rows)
    }
}
pub fn canonical(v: &Value) -> Vec<u8> {
    serde_json::to_vec(v).expect("canonical scalar/map source value")
}
impl Serialize for Node {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        self.wire().serialize(s)
    }
}
impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        Self::from_wire(&Value::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
impl AuditSource for Node {
    fn audit(&self) -> Result<Node> {
        // The algebra is a whitelisted recursive type. Typed identity payloads remain references.
        match self {
            Self::Owned { kind, id, node } => Ok(Node::Owned {
                kind: kind.clone(),
                id: id.clone(),
                node: Box::new(node.audit()?),
            }),
            Self::Identity { kind, id, .. } => Ok(Node::Map(
                [
                    ("kind".into(), Node::Text("EntityRef".into())),
                    ("objectKind".into(), Node::Text(kind.clone())),
                    (
                        "objectId".into(),
                        Node::Identity {
                            kind: kind.clone(),
                            id: id.clone(),
                            role: Role::Reference,
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            )),
            Self::Resource { id, sha, len, .. } => Ok(Node::Map(
                [
                    ("kind".into(), Node::Text("BytesRef".into())),
                    (
                        "resourceId".into(),
                        Node::Resource {
                            id: id.clone(),
                            sha: sha.clone(),
                            len: len.clone(),
                            role: Role::Reference,
                        },
                    ),
                    ("sha256".into(), Node::Text(sha.clone())),
                    ("byteLength".into(), Node::Text(len.clone())),
                ]
                .into_iter()
                .collect(),
            )),
            Self::Array(a) => Ok(Node::Map(
                [
                    ("kind".into(), Node::Text("Array".into())),
                    (
                        "items".into(),
                        Node::Array(a.iter().map(AuditSource::audit).collect::<Result<_>>()?),
                    ),
                ]
                .into_iter()
                .collect(),
            )),
            Self::Map(m) => Ok(Node::Map(
                [
                    ("kind".into(), Node::Text("Map".into())),
                    (
                        "members".into(),
                        Node::Map(
                            m.iter()
                                .map(|(k, v)| Ok((k.clone(), v.audit()?)))
                                .collect::<Result<_>>()?,
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            )),
            Self::TaggedVariant { tag, payload } => Ok(Node::Map(
                [
                    ("kind".into(), Node::Text("TaggedVariant".into())),
                    ("tag".into(), Node::Text(tag.clone())),
                    ("payload".into(), payload.audit()?),
                ]
                .into_iter()
                .collect(),
            )),
            _ => {
                fn data(v: &Value) -> Node {
                    match v {
                        Value::String(s) => Node::Text(s.clone()),
                        Value::Object(m) => {
                            Node::Map(m.iter().map(|(k, v)| (k.clone(), data(v))).collect())
                        }
                        Value::Bool(b) => Node::Bool(*b),
                        _ => unreachable!("scalar audit encoding"),
                    }
                }
                Ok(data(&self.wire()))
            }
        }
    }
    fn rebind(&mut self, m: &Remap) -> Result<()> {
        match self {
            Self::Owned { kind, id, node } => {
                *id = m
                    .get(&(kind.clone(), id.clone()))
                    .ok_or_else(|| Error::new("E_SCHEMA", "owner remap"))?
                    .clone();
                node.rebind(m)?;
            }
            Self::Identity { kind, id, .. } => {
                *id = m
                    .get(&(kind.clone(), id.clone()))
                    .ok_or_else(|| Error::new("E_SCHEMA", "audit reference remap"))?
                    .clone()
            }
            Self::Resource { id, .. } => {
                *id = m
                    .get(&("Resource".into(), id.clone()))
                    .ok_or_else(|| Error::new("E_SCHEMA", "audit resource remap"))?
                    .clone()
            }
            Self::Map(a) => {
                for n in a.values_mut() {
                    n.rebind(m)?;
                }
            }
            Self::Array(a) => {
                for n in a {
                    n.rebind(m)?;
                }
            }
            Self::TaggedVariant { payload, .. } => payload.rebind(m)?,
            _ => {}
        }
        Ok(())
    }
    fn descriptor() -> Value {
        json!({"type":"FC-AuditNode-1","roleVisitor":"closed-tagged-algebra-1"})
    }
}
impl ValueSource for Node {}
