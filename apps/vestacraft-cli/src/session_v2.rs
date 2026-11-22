//! NS2 transport owner. No public resource, digest or role confers host Saved authority.
use crate::paths::Paths;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::time::Instant;
use vesta_bindings::resource_roles::ResourceRole2;
use vesta_commands::{
    EngineV2, Limits,
    registry_v2::{CORE_COMMANDS, METHODS},
    save_tracker::SaveTracker,
};
use vesta_model::{DocumentAt, Error, Ids, ResourceStore, Result, digest};
const PROTOCOL: &str = "vestacraft.native-session/2";
const MAX: usize = 16777216;
fn err(code: &str, s: impl Into<String>) -> Error {
    Error::new(code, s)
}
fn n(v: &Value) -> Result<u64> {
    vesta_model::json::u64_str(v).map_err(|e| err("N_SCHEMA", e.message))
}
fn string(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| err("N_SCHEMA", "string expected"))
}
fn bytes(v: &Value) -> Result<Vec<u8>> {
    serde_json::to_vec(v).map_err(|e| err("N_INTERNAL", e.to_string()))
}
pub fn profiles() -> Value {
    vesta_bindings::profile_digests::PROFILE_DIGESTS
        .iter()
        .map(|(k, v)| (k.to_string(), json!(v)))
        .collect()
}
pub fn registration() -> Value {
    json!({"protocol":PROTOCOL,"methods":METHODS,"coreCommands":CORE_COMMANDS,"profiles":profiles(),"implementedStateComponents":["native-camera/explicit-canonical-camera"],"scientificDomains":"unsupported until qualified domain kernels are installed","engineeringArtifactsAreNotHostSaved":true})
}
#[derive(Clone)]
struct Upload {
    expected: Value,
    bytes: Vec<u8>,
}
#[derive(Clone)]
struct Cached {
    raw: Vec<u8>,
    replies: Vec<Value>,
    terminal: bool,
}
struct Target {
    output: String,
    token: String,
    expected: Option<String>,
    expires: u64,
}
pub struct Session {
    paths: Paths,
    initial: Option<String>,
    ids: Ids,
    id: Option<String>,
    cv: u64,
    highest: u64,
    engine: Option<EngineV2>,
    store: ResourceStore,
    uploads: BTreeMap<u64, Upload>,
    cache: BTreeMap<u64, Cached>,
    cache_limit: usize,
    limits: Limits,
    policy: Value,
    pending: Option<Value>,
    snapshots: SaveTracker,
    target: Option<Target>,
    started: Instant,
    pub shutdown: bool,
    max_uploads: usize,
    max_snapshots: usize,
    max_output: usize,
}
impl Session {
    pub fn new(
        paths: Paths,
        mut ids: Ids,
        initial: Option<String>,
        output: Option<String>,
    ) -> Result<Self> {
        let target = if let Some(output) = output {
            paths.resolve(&output, true)?;
            let expected = paths.baseline(&output)?;
            let token = format!("engineering:{}", ids.next_id());
            eprintln!(
                "{}",
                json!({"scope":"engineering-candidate-only","targetToken":token,"output":output,"hostSavedAuthority":false})
            );
            Some(Target {
                output,
                token,
                expected,
                expires: 60000,
            })
        } else {
            None
        };
        Ok(Self {
            paths,
            initial,
            ids,
            id: None,
            cv: 0,
            highest: 0,
            engine: None,
            store: ResourceStore::default(),
            uploads: BTreeMap::new(),
            cache: BTreeMap::new(),
            cache_limit: 4194304,
            limits: Limits::default(),
            policy: Value::Null,
            pending: None,
            snapshots: SaveTracker::default(),
            target,
            started: Instant::now(),
            shutdown: false,
            max_uploads: 4,
            max_snapshots: 4,
            max_output: MAX,
        })
    }
    fn now(&self) -> u64 {
        u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX)
    }
    fn owner(&self) -> &str {
        self.id.as_deref().unwrap_or("uninitialized")
    }
    fn response(&self, seq: Option<u64>, kind: &str, op: &str, result: Value) -> Value {
        json!({"protocol":PROTOCOL,"sessionId":self.id,"contextVersion":self.cv.to_string(),"seq":seq.map(|n|n.to_string()),"kind":kind,"op":op,"result":result})
    }
    fn failure(&self, seq: Option<u64>, e: Error, admitted: bool) -> Value {
        json!({"protocol":PROTOCOL,"kind":"error","sessionId":self.id,"contextVersion":self.id.as_ref().map(|_|self.cv.to_string()),"seq":seq.map(|n|n.to_string()),"code":e.code,"admitted":admitted,"fatal":false,"highestAdmittedSeq":self.id.as_ref().map(|_|self.highest.to_string()),"message":e.message.chars().take(1024).collect::<String>()})
    }
    pub fn malformed(&self, code: &str, message: &str) -> Value {
        self.failure(None, err(code, message), false)
    }
    fn validate_reply(v: &Value) -> Result<()> {
        vesta_model::json::schema(v, "native-session-v2.schema.json", "Response")
            .map_err(|e| err("N_INTERNAL", format!("response schema: {}", e.message)))
    }
    pub fn handle(&mut self, raw: &[u8]) -> Vec<Value> {
        let v = match vesta_model::json::parse(raw, 1048576) {
            Ok(v) => v,
            Err(e) => {
                return vec![self.failure(
                    None,
                    err(
                        if e.code == "E_RESOURCE_LIMIT" {
                            "N_FRAME"
                        } else {
                            "N_SCHEMA"
                        },
                        e.message,
                    ),
                    false,
                )];
            }
        };
        let seq = v.get("seq").and_then(|v| n(v).ok());
        let op = v["op"].as_str().unwrap_or("");
        if v["protocol"] != PROTOCOL {
            return vec![self.failure(seq, err("N_PROTOCOL", "NS2 required"), false)];
        }
        if let Err(e) = vesta_model::json::schema(&v, "native-session-v2.schema.json", "Request") {
            return vec![self.failure(seq, err("N_SCHEMA", e.message), false)];
        }
        let result = if op == "hello" {
            self.hello(&v).map(|r| vec![r])
        } else {
            self.admit(&v, raw)
        };
        let replies = match result {
            Ok(v) => v,
            Err(e) => vec![self.failure(seq, e, false)],
        };
        for reply in &replies {
            if let Err(e) = Self::validate_reply(reply) {
                return vec![self.failure(seq, e, true)];
            }
        }
        replies
    }
    fn hello(&mut self, v: &Value) -> Result<Value> {
        if self.id.is_some() {
            return Err(err("N_ALREADY_INITIALIZED", "hello already completed"));
        }
        if v["expectedProfiles"] != profiles() {
            return Err(err(
                "N_PROFILE_UNAVAILABLE",
                "all nine actual profile digests must match",
            ));
        }
        let p = &v["policy"];
        let resident = n(&p["budget"]["maxResidentBytes"])?;
        let input = n(&p["budget"]["maxInputBytes"])?;
        let output = n(&p["budget"]["maxOutputBytes"])?;
        let fields = n(&p["budget"]["maxFieldValues"])?;
        let candidate = n(&p["maxDomainResultBytes"])?;
        let retention = n(&p["domainCandidateRetentionMs"])?;
        let cache = n(&p["maxResponseCacheBytes"])?;
        let count = p["maxPendingDomainCandidates"]
            .as_u64()
            .ok_or_else(|| err("N_POLICY", "candidate count"))?;
        let up = p["maxPendingUploads"]
            .as_u64()
            .ok_or_else(|| err("N_POLICY", "uploads"))?;
        let snapshots = p["maxPendingSnapshots"]
            .as_u64()
            .ok_or_else(|| err("N_POLICY", "snapshots"))?;
        if [
            resident, input, output, candidate, cache, retention, count, up, snapshots, fields,
        ]
        .contains(&0)
            || [resident, input, output, candidate, cache]
                .iter()
                .any(|n| *n > MAX as u64)
            || count > 16
            || up > 16
            || snapshots > 16
            || candidate > resident
            || p["hostCapabilitiesDigest"] != profiles()["hostCapabilitiesDigest"]
        {
            return Err(err(
                "N_POLICY",
                "policy exceeds bounded native implementation or has invalid limits",
            ));
        }
        self.limits = Limits {
            candidate_count: count as usize,
            candidate_bytes: candidate as usize,
            retention_ms: retention,
            resident_bytes: resident as usize,
            max_field_values: fields.min(100_000) as usize,
        };
        self.cache_limit = cache as usize;
        self.max_uploads = up as usize;
        self.max_snapshots = snapshots as usize;
        self.max_output = output as usize;
        self.policy = p.clone();
        if let Some(input) = self.initial.clone() {
            self.engine = Some(
                crate::load(&self.paths, &input, self.ids.fork(), self.limits.clone())
                    .map_err(|e| err("N_SCOPE_UNSUPPORTED", e.to_string()))?,
            );
            self.cv = 1;
        }
        self.id = Some(self.ids.next_id());
        Ok(self.response(None,"ok","hello",json!({"policyDigest":digest(&bytes(p)?),"profiles":profiles(),"maxFrameBytes":1048576,"maxRawChunkBytes":65536,"coreCommands":CORE_COMMANDS,"methods":METHODS})))
    }
    fn admit(&mut self, v: &Value, raw: &[u8]) -> Result<Vec<Value>> {
        if self.id.is_none() {
            return Err(err("N_UNINITIALIZED", "hello first"));
        }
        if v["sessionId"] != self.id.as_deref().unwrap_or("") {
            return Err(err("N_STALE_SESSION", "session identity"));
        }
        let op = string(&v["op"])?;
        if op == "status" {
            return Ok(vec![self.status(v)?]);
        }
        if op == "cancel" {
            return self.cancel(v);
        }
        let seq = n(&v["seq"])?;
        if seq <= self.highest {
            let cached = self
                .cache
                .get(&seq)
                .ok_or_else(|| err("N_REQUEST_EXPIRED", "terminal cache no longer retained"))?;
            if cached.raw != raw {
                return Err(err(
                    "N_REPLAY_MISMATCH",
                    "same seq requires identical original JSON bytes",
                ));
            }
            return Ok(cached.replies.clone());
        }
        if seq
            != self
                .highest
                .checked_add(1)
                .ok_or_else(|| err("N_SEQUENCE_GAP", "sequence exhausted"))?
        {
            return Err(err("N_SEQUENCE_GAP", "next monotonic seq required"));
        }
        if n(&v["contextVersion"])? != self.cv {
            return Err(err(
                "N_CONTEXT_CONFLICT",
                "current context version required",
            ));
        }
        if self.pending.is_some() {
            return Err(err("N_BUSY", "one native job at a time"));
        }
        if matches!(op, "new" | "open") && self.engine.is_some() {
            return Err(err("N_CLOSE_REQUIRED", "close existing document first"));
        }
        if matches!(op, "save.confirm" | "save.reject") {
            return Err(err(
                "N_HOST_ONLY",
                "engineering fixture has no authenticated HS4 settlement authority",
            ));
        }
        if op == "snapshot" && self.target.is_none() {
            return Err(err(
                "N_HOST_ONLY",
                "snapshot requires authenticated host or explicit engineering fixture target",
            ));
        }
        if op == "shutdown" && (self.engine.is_some() || self.snapshots.count() > 0) {
            return Err(err(
                "N_CLOSE_REQUIRED",
                "document or unsettled snapshots remain",
            ));
        }
        self.highest = seq;
        let replies = if matches!(op, "execute" | "snapshot") {
            let job = self.ids.next_id();
            self.pending = Some(v.clone());
            vec![
                json!({"protocol":PROTOCOL,"sessionId":self.id,"contextVersion":self.cv.to_string(),"seq":seq.to_string(),"kind":"accepted","op":op,"jobId":job}),
            ]
        } else {
            match self.sync(v) {
                Ok(r) => vec![self.response(Some(seq), "ok", op, r)],
                Err(e) => vec![self.failure(Some(seq), transport_error(e), true)],
            }
        };
        self.cache.insert(
            seq,
            Cached {
                raw: raw.to_vec(),
                replies: replies.clone(),
                terminal: self.pending.is_none(),
            },
        );
        self.trim_cache();
        Ok(replies)
    }
    fn trim_cache(&mut self) {
        loop {
            let size: usize = self
                .cache
                .values()
                .map(|v| {
                    v.raw.len()
                        + serde_json::to_vec(&v.replies)
                            .map(|b| b.len())
                            .unwrap_or(MAX)
                })
                .sum();
            if size <= self.cache_limit {
                break;
            }
            let Some(key) = self.cache.iter().find(|(_, v)| v.terminal).map(|(k, _)| *k) else {
                break;
            };
            self.cache.remove(&key);
        }
    }
    fn engine(&self) -> Result<&EngineV2> {
        self.engine
            .as_ref()
            .ok_or_else(|| err("E_DOCUMENT_NOT_FOUND", "no current document"))
    }
    fn document(v: &Value) -> Result<DocumentAt> {
        serde_json::from_value(v.clone()).map_err(|e| err("E_SCHEMA", e.to_string()))
    }
    fn read_ref(&self, desc: &Value) -> Result<Vec<u8>> {
        let id = string(&desc["sha256"])?;
        let b = self
            .store
            .read(id, self.owner())
            .map_err(|_| err("N_RESOURCE_MISSING", "resource not owned by this session"))?;
        if n(&desc["byteLength"])? != b.len() as u64 || digest(b) != id {
            return Err(err("N_DIGEST", "descriptor byte binding"));
        }
        Ok(b.to_vec())
    }
    fn role(&self, desc: &Value, role: ResourceRole2) -> Result<Value> {
        let b = self.read_ref(desc)?;
        vesta_bindings::decoder::decode_resource(
            role,
            desc,
            &b,
            vesta_bindings::decoder::DecodeBudget::default(),
        )
        .map_err(|e| err("N_SCHEMA", e))?;
        vesta_model::json::parse(&b, MAX)
    }
    fn put(&mut self, b: Vec<u8>, media: &str) -> Result<Value> {
        let len = b.len();
        let owner = self.owner().to_owned();
        let other = self
            .engine
            .as_ref()
            .map(EngineV2::resident_bytes)
            .unwrap_or(0)
            + self.snapshots.resident_bytes()
            + self
                .uploads
                .values()
                .map(|u| numeric_len(&u.expected) as usize)
                .sum::<usize>();
        let available = self.limits.resident_bytes.saturating_sub(other);
        let sha = self
            .store
            .put(b, &owner, available)
            .map_err(|e| err("N_RESOURCE_LIMIT", e.message))?;
        Ok(json!({"sha256":sha,"byteLength":len.to_string(),"mediaType":media}))
    }
    fn put_role(&mut self, v: Value, role: ResourceRole2) -> Result<Value> {
        let media = match role {
            ResourceRole2::OpenBundle => "application/vnd.vestacraft.native-open+json",
            ResourceRole2::InspectionPage => "application/vnd.vestacraft.native-inspection+json",
            ResourceRole2::HostAck => "application/vnd.vestacraft.host-ack+json",
            ResourceRole2::HostNack => "application/vnd.vestacraft.host-nack+json",
            ResourceRole2::CommandCompletion => "application/vnd.vestacraft.command-outcome+json",
            ResourceRole2::FailureNotice => "application/vnd.vestacraft.engine-failure+json",
        };
        let b = bytes(&v)?;
        let d = json!({"sha256":digest(&b),"byteLength":b.len().to_string(),"mediaType":media});
        vesta_bindings::decoder::decode_resource(
            role,
            &d,
            &b,
            vesta_bindings::decoder::DecodeBudget::default(),
        )
        .map_err(|e| err("N_INTERNAL", format!("produced role payload invalid: {e}")))?;
        self.put(b, media)
    }
    fn sync(&mut self, v: &Value) -> Result<Value> {
        let body = &v["body"];
        let op = string(&v["op"])?;
        match op {
            "resource.begin" => {
                if self.uploads.len() >= self.max_uploads {
                    return Err(err("N_RESOURCE_LIMIT", "pending uploads"));
                }
                let expected = body["expected"].clone();
                let size = n(&expected["byteLength"])?;
                let reserved = self
                    .uploads
                    .values()
                    .try_fold(0u64, |n, u| n.checked_add(numeric_len(&u.expected)))
                    .ok_or_else(|| err("N_RESOURCE_LIMIT", "upload accounting overflow"))?;
                if size > n(&self.policy["budget"]["maxInputBytes"])?
                    || size
                        .checked_add(reserved)
                        .and_then(|n| n.checked_add(self.store.bytes() as u64))
                        .is_none_or(|n| n > self.limits.resident_bytes as u64)
                {
                    return Err(err(
                        "N_RESOURCE_LIMIT",
                        "input/resident bytes reserved before allocation",
                    ));
                }
                let seq = n(&v["seq"])?;
                self.uploads.insert(
                    seq,
                    Upload {
                        expected,
                        bytes: vec![],
                    },
                );
                Ok(json!({"uploadSeq":seq.to_string(),"receivedBytes":"0"}))
            }
            "resource.chunk" => {
                let up = n(&body["uploadSeq"])?;
                let chunk = crate::base64::decode(string(&body["bytesB64"])?)?;
                let u = self
                    .uploads
                    .get_mut(&up)
                    .ok_or_else(|| err("N_UPLOAD_STATE", "unknown upload"))?;
                if n(&body["offset"])? != u.bytes.len() as u64 {
                    return Err(err("N_UPLOAD_OFFSET", "next exact offset required"));
                }
                if (u.bytes.len() as u64)
                    .checked_add(chunk.len() as u64)
                    .is_none_or(|n| n > numeric_len(&u.expected))
                {
                    return Err(err("N_RESOURCE_LIMIT", "upload exceeds declared bytes"));
                }
                u.bytes.extend(chunk);
                Ok(json!({"uploadSeq":up.to_string(),"receivedBytes":u.bytes.len().to_string()}))
            }
            "resource.commit" => {
                let up = n(&body["uploadSeq"])?;
                let u = self
                    .uploads
                    .get(&up)
                    .ok_or_else(|| err("N_UPLOAD_STATE", "unknown upload"))?;
                if u.bytes.len() as u64 != n(&u.expected["byteLength"])?
                    || digest(&u.bytes) != u.expected["sha256"]
                {
                    return Err(err("N_DIGEST", "complete actual length/SHA required"));
                }
                let u = self
                    .uploads
                    .remove(&up)
                    .ok_or_else(|| err("N_UPLOAD_STATE", "upload"))?;
                let d = self.put(u.bytes, string(&u.expected["mediaType"])?)?;
                Ok(json!({"resource":d}))
            }
            "resource.read" => {
                let id = string(&body["resource"])?;
                let b = self
                    .store
                    .read(id, self.owner())
                    .map_err(|_| err("N_RESOURCE_MISSING", "owned retained resource required"))?;
                let off = usize::try_from(n(&body["offset"])?)
                    .map_err(|_| err("N_UPLOAD_OFFSET", "offset"))?;
                if off > b.len() {
                    return Err(err("N_UPLOAD_OFFSET", "read beyond end"));
                }
                let size = body["maxBytes"]
                    .as_u64()
                    .ok_or_else(|| err("N_SCHEMA", "maxBytes"))?
                    as usize;
                let end = off.saturating_add(size).min(b.len());
                Ok(
                    json!({"resource":id,"offset":off.to_string(),"bytesB64":crate::base64::encode(&b[off..end]),"eof":end==b.len()}),
                )
            }
            "resource.release" => {
                let id = string(&body["resource"])?;
                let owner = self.owner().to_owned();
                let released = self
                    .store
                    .release(id, &owner)
                    .map_err(|_| err("N_RESOURCE_MISSING", "transport lease missing"))?;
                Ok(json!({"resource":id,"transportLeaseReleased":released}))
            }
            "new" => {
                let cell = serde_json::from_value(body["cell"].clone())
                    .map_err(|e| err("E_SCHEMA", e.to_string()))?;
                let engine = EngineV2::new(
                    cell,
                    string(&body["label"])?,
                    self.ids.fork(),
                    self.limits.clone(),
                )?;
                let phase = engine.content().phases[0].id.clone();
                self.engine = Some(engine);
                self.cv += 1;
                Ok(
                    json!({"context":self.engine()?.context(),"nextContextVersion":self.cv.to_string(),"phaseId":phase}),
                )
            }
            "open" => {
                let b = self.role(&body["bundle"], ResourceRole2::OpenBundle)?;
                if b["format"] != "vesta" || !b["workRef"].is_null() {
                    return Err(err(
                        "N_HOST_ONLY",
                        "untrusted upload has no current-source work reference authority",
                    ));
                }
                let primary = self.read_ref(&b["primary"])?;
                let mut dependencies = BTreeMap::new();
                for x in b["dependencies"]
                    .as_array()
                    .ok_or_else(|| err("N_SCHEMA", "dependencies"))?
                {
                    let key = string(&x["logicalPath"])?;
                    if dependencies
                        .insert(key.into(), self.read_ref(&x["resource"])?)
                        .is_some()
                    {
                        return Err(err("N_SCHEMA", "duplicate dependency key"));
                    }
                }
                let capture = vesta_codecs::decode(&primary, &dependencies, self.ids.fork())?;
                let mut e = EngineV2::loaded(capture, self.ids.fork(), self.limits.clone())?;
                e.import_dirty();
                self.engine = Some(e);
                self.cv += 1;
                Ok(
                    json!({"context":self.engine()?.context(),"nextContextVersion":self.cv.to_string(),"editable":true,"diagnostics":null}),
                )
            }
            "inspect" => {
                let at = Self::document(&body["document"])?;
                let e = self.engine()?;
                e.check(&at)?;
                let q = &body["query"];
                let kind = string(&q["kind"])?;
                let limit = q["limit"]
                    .as_u64()
                    .ok_or_else(|| err("N_SCHEMA", "limit"))? as usize;
                let off = if q["cursor"].is_null() {
                    0
                } else {
                    let token = string(&q["cursor"])?;
                    let prefix = format!("{}:{}:{}:{kind}:", at.document_id, at.epoch, at.revision);
                    token
                        .strip_prefix(&prefix)
                        .and_then(|n| n.parse::<usize>().ok())
                        .ok_or_else(|| err("E_STALE_CURSOR", "inspection cursor"))?
                };
                let all: Vec<Value> = if kind == "phases" {
                    e.content().phases.iter().map(|p|Ok(json!({"phaseId":p.id,"label":p.label,"structurePresent":true,"cell":p.cell,"volumeA3":p.cell.volume()?,"cellEditable":true,"siteCount":p.sites.len().to_string()}))).collect::<Result<_>>()?
                } else {
                    let p = e
                        .content()
                        .phases
                        .iter()
                        .find(|p| p.id == q["phaseId"])
                        .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "site query phase"))?;
                    p.sites
                        .iter()
                        .map(|s| json!({"phaseId":p.id,"siteId":s.id,"label":s.label}))
                        .collect()
                };
                if off > all.len() {
                    return Err(err("E_STALE_CURSOR", "cursor offset"));
                }
                let end = off.saturating_add(limit).min(all.len());
                let next = if end < all.len() {
                    Some(format!(
                        "{}:{}:{}:{kind}:{end}",
                        at.document_id, at.epoch, at.revision
                    ))
                } else {
                    None
                };
                let page=self.put_role(json!({"query":kind,"document":at,"contextVersion":self.cv.to_string(),"records":&all[off..end]}),ResourceRole2::InspectionPage)?;
                Ok(json!({"document":at,"queryKind":kind,"page":page,"nextCursor":next}))
            }
            "close" => {
                let at = Self::document(&body["document"])?;
                self.engine()?.check(&at)?;
                if self.snapshots.count() > 0 {
                    return Err(err(
                        "N_INCOMPLETE_WORK",
                        "unsettled snapshots remain; cannot assert safe handoff",
                    ));
                }
                if body["disposition"] == "require-saved"
                    && (self.engine()?.context()["dirty"] != false
                        || body["proof"]["kind"] != "loaded-baseline")
                {
                    return Err(err(
                        "N_INCOMPLETE_WORK",
                        "no authentic captured Saved proof",
                    ));
                }
                self.engine = None;
                self.cv += 1;
                Ok(
                    json!({"closedDocumentId":at.document_id,"closedEpoch":at.epoch,"nextContextVersion":self.cv.to_string()}),
                )
            }
            "shutdown" => {
                self.shutdown = true;
                Ok(json!({"exitCode":0}))
            }
            _ => Err(err(
                "N_SCOPE_UNSUPPORTED",
                "not a synchronous NS2 operation",
            )),
        }
    }
    pub fn has_pending(&self) -> bool {
        self.pending.is_some()
    }
    pub fn has_unsettled(&self) -> bool {
        self.snapshots.count() > 0
    }
    pub fn advance(&mut self) -> Vec<Value> {
        let Some(v) = self.pending.take() else {
            return vec![];
        };
        let seq = n(&v["seq"]).unwrap_or(self.highest);
        let op = v["op"].as_str().unwrap_or("");
        let result = if op == "execute" {
            self.execute(&v)
        } else {
            self.snapshot(&v)
        };
        let reply = match result {
            Ok(r) => self.response(Some(seq), "ok", op, r),
            Err(e) => self.terminal_failure(seq, op, e),
        };
        let reply = match Self::validate_reply(&reply) {
            Ok(()) => reply,
            Err(e) => self.failure(Some(seq), e, true),
        };
        if let Some(c) = self.cache.get_mut(&seq) {
            c.replies.push(reply.clone());
            c.terminal = true;
        }
        self.trim_cache();
        vec![reply]
    }
    fn execute(&mut self, v: &Value) -> Result<Value> {
        let body = &v["body"];
        let at = Self::document(&body["document"])?;
        self.engine()?.check(&at)?;
        let mut inv = body["invocation"].clone();
        let command = string(&inv["command"])?.to_owned();
        if matches!(command.as_str(), "domain.prepare" | "domain.state.prepare") {
            let field = if command == "domain.prepare" {
                "request"
            } else {
                "edit"
            };
            let raw = self.read_ref(&inv["params"][field])?;
            let e = self
                .engine
                .as_mut()
                .ok_or_else(|| err("E_DOCUMENT_NOT_FOUND", "no current document"))?;
            let staged = e.stage_json(raw)?;
            if staged != inv["params"][field] {
                return Err(err(
                    "E_SCHEMA",
                    "typed scientific resource descriptor differs",
                ));
            }
            inv["params"][field] = staged;
        }
        let e = self
            .engine
            .as_mut()
            .ok_or_else(|| err("E_DOCUMENT_NOT_FOUND", "no document"))?;
        let outcome = e.execute(&at, &inv)?;
        let context = e.context();
        if command == "domain.state.prepare" {
            let id = string(&outcome["result"]["candidateId"])?;
            let b = e.candidate_bytes(id)?.to_vec();
            let d = self.put(b, "application/json")?;
            if d != outcome["result"]["validatedState"] {
                return Err(err("N_INTERNAL", "candidate resource binding"));
            }
        }
        let request_id = outcome["requestId"].clone();
        let descriptor = self.put_role(outcome, ResourceRole2::CommandCompletion)?;
        Ok(json!({"context":context,"engineRequestId":request_id,"outcome":descriptor}))
    }
    fn snapshot(&mut self, v: &Value) -> Result<Value> {
        let body = &v["body"];
        let at = Self::document(&body["document"])?;
        self.engine()?.check(&at)?;
        let target = self
            .target
            .as_ref()
            .ok_or_else(|| err("E_TARGET_TOKEN_INVALID", "no engineering capability"))?;
        if body["options"]["targetToken"] != target.token {
            return Err(err(
                "E_TARGET_TOKEN_INVALID",
                "target belongs to another process/operator",
            ));
        }
        if self.now() >= target.expires {
            return Err(err("E_TARGET_TOKEN_EXPIRED", "engineering target expired"));
        }
        if body["options"]["format"] != "vesta" || body["options"]["purpose"] == "export" {
            return Err(err(
                "E_FORMAT_UNSUPPORTED",
                "P2-03 native project bytes only",
            ));
        }
        if self.snapshots.count() >= self.max_snapshots {
            return Err(err("E_RESOURCE_LIMIT", "pending snapshots"));
        }
        let capture = self.engine()?.capture()?;
        let context = self.engine()?.context();
        let id = self.ids.next_id();
        let output = target.output.clone();
        let token = target.token.clone();
        let baseline = target.expected.clone();
        let leaf = std::path::Path::new(&output)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| err("E_SCHEMA", "target leaf"))?;
        let bundle = vesta_codecs::encode_verified(&capture, leaf, self.ids.fork())?;
        let total = bundle
            .dependencies
            .values()
            .map(Vec::len)
            .sum::<usize>()
            .checked_add(bundle.primary.len())
            .ok_or_else(|| err("E_RESOURCE_LIMIT", "snapshot bytes"))?;
        if total > self.max_output {
            return Err(err("E_RESOURCE_LIMIT", "snapshot output"));
        }
        let old_store = self.store.clone();
        let result = (|| {
            let primary = self.put(bundle.primary.clone(), "application/vnd.vesta.project")?;
            let mut dependencies = vec![];
            let mut manifest_deps = vec![];
            for (path, b) in &bundle.dependencies {
                let d = self.put(b.clone(), "application/octet-stream")?;
                dependencies.push(json!({"relativePath":path,"bytesResource":d["sha256"],"sha256":d["sha256"],"byteLength":d["byteLength"]}));
                manifest_deps.push(
                    json!({"relativePath":path,"sha256":d["sha256"],"byteLength":d["byteLength"]}),
                );
            }
            let descriptor = json!({"snapshotId":id,"sessionId":self.id,"documentId":at.document_id,"epoch":at.epoch,"revision":at.revision,"stateDigest":capture.state_digest,"purpose":body["options"]["purpose"],"targetToken":token,"expectedPrimarySha256":baseline,"primary":{"bytesResource":primary["sha256"],"sha256":primary["sha256"],"byteLength":primary["byteLength"],"format":"vesta"},"dependencies":dependencies,"manifest":{"primarySha256":primary["sha256"],"dependencies":manifest_deps}});
            self.put(
                bytes(&descriptor)?,
                "application/vnd.vestacraft.save-bundle+json",
            )
        })();
        let descriptor = match result {
            Ok(d) => d,
            Err(e) => {
                self.store = old_store;
                return Err(e);
            }
        };
        let until = self
            .now()
            .checked_add(60000)
            .ok_or_else(|| err("E_RESOURCE_LIMIT", "snapshot lifetime"))?;
        let owner = self.owner().to_owned();
        self.snapshots
            .record(&id, capture.clone(), &owner, &token, until)?;
        let owner = self.owner().to_owned();
        let snapshot_owner = format!("snapshot:{id}");
        for blob in std::iter::once(&bundle.primary).chain(bundle.dependencies.values()) {
            self.store.retain(&digest(blob), &owner, &snapshot_owner)?;
        }
        self.store.retain(
            descriptor["sha256"]
                .as_str()
                .ok_or_else(|| err("N_INTERNAL", "snapshot bundle"))?,
            &owner,
            &snapshot_owner,
        )?;
        // Explicit engineering mode materializes candidate artifacts under the operator's --root.
        // This does not run HS4, issue HostAck, rebind a WorkRef or clear dirty.
        let hash =
            self.paths
                .publish(&output, &bundle, baseline.as_deref(), &self.ids.next_id())?;
        if let Some(t) = self.target.as_mut() {
            t.expected = Some(hash)
        }
        Ok(
            json!({"snapshotId":id,"captured":context,"purpose":body["options"]["purpose"],"targetTokenDigest":digest(token.as_bytes()),"bundle":descriptor,"sourceExpectationDigest":capture.state_digest}),
        )
    }
    fn terminal_failure(&mut self, seq: u64, op: &str, e: Error) -> Value {
        let code = if e.code.starts_with("E_") {
            e.code
        } else {
            "E_SCHEMA".into()
        };
        let stage = if [
            "E_DOCUMENT_NOT_FOUND",
            "E_STALE_EPOCH",
            "E_REVISION_CONFLICT",
        ]
        .contains(&code.as_str())
        {
            "identity"
        } else if op == "snapshot" {
            "encode"
        } else {
            "execute"
        };
        let notice = json!({"requestId":self.ids.next_id(),"code":code,"stage":stage,"message":e.message.chars().take(1024).collect::<String>(),"retryable":false,"committedRevision":null,"recovery":"none"});
        match self.put_role(notice, ResourceRole2::FailureNotice) {
            Ok(d) => {
                json!({"protocol":PROTOCOL,"sessionId":self.id,"contextVersion":self.cv.to_string(),"seq":seq.to_string(),"kind":"failed","op":op,"code":code,"committedRevision":null,"engineFailure":d})
            }
            Err(e) => self.failure(Some(seq), e, true),
        }
    }
    fn status(&self, v: &Value) -> Result<Value> {
        let target = if v["targetSeq"].is_null() {
            None
        } else {
            Some(n(&v["targetSeq"])?)
        };
        let mut state = "session";
        let mut stage = Value::Null;
        let mut result = Value::Null;
        let mut received = Value::Null;
        let mut revision = Value::Null;
        if let Some(seq) = target {
            state = if seq > self.highest {
                "not-admitted"
            } else {
                "expired"
            };
            if let Some(p) = &self.pending {
                if n(&p["seq"])? == seq {
                    state = "active";
                    stage = json!("queued")
                }
            }
            if let Some(c) = self.cache.get(&seq) {
                if c.terminal {
                    state = "finished";
                    if let Some(last) = c.replies.last() {
                        stage = json!(if last["kind"] == "failed" || last["kind"] == "error" {
                            "failed"
                        } else if last["op"] == "snapshot" {
                            "snapshot-ready"
                        } else {
                            "committed"
                        });
                        result = last["result"]
                            .get("outcome")
                            .or_else(|| last["result"].get("bundle"))
                            .cloned()
                            .unwrap_or(Value::Null);
                        revision = last["result"]["context"]["revision"].clone();
                    }
                }
            }
            if let Some(up) = self.uploads.get(&seq) {
                state = "active";
                stage = json!("upload");
                received = json!(up.bytes.len().to_string())
            }
        }
        Ok(self.response(None,"ok","status",json!({"highestAdmittedSeq":self.highest.to_string(),"contextVersion":self.cv.to_string(),"context":self.engine.as_ref().map(EngineV2::context),"targetState":state,"stage":stage,"result":result,"receivedBytes":received,"committedRevision":revision,"pendingSnapshots":self.snapshots.count().to_string()})))
    }
    fn cancel(&mut self, v: &Value) -> Result<Vec<Value>> {
        let seq = n(&v["targetSeq"])?;
        if self
            .pending
            .as_ref()
            .is_some_and(|p| n(&p["seq"]).ok() == Some(seq))
        {
            let p = self
                .pending
                .take()
                .ok_or_else(|| err("N_INTERNAL", "pending disappeared"))?;
            let failed = self.terminal_failure(
                seq,
                string(&p["op"])?,
                err("E_CANCELLED", "cancelled before commit"),
            );
            if let Some(c) = self.cache.get_mut(&seq) {
                c.replies.push(failed.clone());
                c.terminal = true
            }
            let mut reply = self.response(
                None,
                "ok",
                "cancel",
                json!({"outcome":"cancelled-before-commit","committedRevision":null}),
            );
            reply["targetSeq"] = json!(seq.to_string());
            return Ok(vec![reply, failed]);
        }
        let (outcome, revision) = if seq > self.highest {
            ("not-admitted", Value::Null)
        } else if let Some(c) = self.cache.get(&seq) {
            let rev = c
                .replies
                .last()
                .and_then(|r| r["result"]["context"].get("revision"))
                .cloned()
                .unwrap_or(Value::Null);
            if rev.is_null() {
                ("already-finished", Value::Null)
            } else {
                ("too-late", rev)
            }
        } else {
            ("expired", Value::Null)
        };
        let mut reply = self.response(
            None,
            "ok",
            "cancel",
            json!({"outcome":outcome,"committedRevision":revision}),
        );
        reply["targetSeq"] = json!(seq.to_string());
        Ok(vec![reply])
    }
}
fn numeric_len(v: &Value) -> u64 {
    v["byteLength"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .unwrap_or(u64::MAX)
}
fn transport_error(e: Error) -> Error {
    if e.code.starts_with("N_") {
        e
    } else {
        let code = match e.code.as_str() {
            "E_RESOURCE_LIMIT" => "N_RESOURCE_LIMIT",
            "E_RESOURCE_MISSING" => "N_RESOURCE_MISSING",
            "E_DOCUMENT_NOT_FOUND" | "E_STALE_EPOCH" | "E_REVISION_CONFLICT" | "E_STALE_CURSOR" => {
                "N_CONTEXT_CONFLICT"
            }
            "E_DOMAIN" | "E_NOT_IMPLEMENTED" | "E_FORMAT_UNSUPPORTED" | "E_REFERENCE_FAILURE" => {
                "N_SCOPE_UNSUPPORTED"
            }
            _ => "N_SCHEMA",
        };
        err(code, e.to_string())
    }
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod lifecycle_tests;
