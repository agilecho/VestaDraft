use crate::history::History;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::time::Instant;
use vesta_model::field::{ActiveField, FieldBuffer, FieldOwner, restore_fields};
use vesta_model::source7::SourceDocument;
use vesta_model::source7::resource_graph::orffe::{
    R80GeometryTable, RIETAN_OWNER_DIALECT, active_tables,
};
use vesta_model::*;
#[derive(Clone, Debug)]
pub struct Limits {
    pub candidate_count: usize,
    pub candidate_bytes: usize,
    pub retention_ms: u64,
    pub resident_bytes: usize,
    pub max_field_values: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            candidate_count: 16,
            candidate_bytes: 1048576,
            retention_ms: 60000,
            resident_bytes: 16777216,
            max_field_values: 100_000,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Prepared {
    pub candidate_id: String,
    pub candidate_digest: String,
    pub validated_state: Value,
}
pub struct SourceAttachment {
    pub logical_key: String,
    pub bytes: Vec<u8>,
    pub media_type: String,
}
#[derive(Clone, Debug)]
struct Candidate {
    at: DocumentAt,
    digest: String,
    content: ProjectContent,
    until: u64,
    resource: String,
}
#[derive(Clone)]
pub struct EngineV2 {
    content: ProjectContent,
    source: Option<SourceDocument>,
    source_digest: Option<String>,
    source_bytes: usize,
    fields: Vec<ActiveField>,
    document_id: String,
    epoch: String,
    revision: u64,
    ids: Ids,
    history: History,
    resources: ResourceStore,
    candidates: BTreeMap<String, Candidate>,
    limits: Limits,
    started: Instant,
    baseline: Option<String>,
    work_ref: Option<Value>,
}
fn err(code: &str, message: &str) -> Error {
    Error::new(code, message)
}
fn typed<T: serde::de::DeserializeOwned>(v: &Value) -> Result<T> {
    serde_json::from_value(v.clone()).map_err(|e| Error::new("E_SCHEMA", e.to_string()))
}
impl EngineV2 {
    pub fn new(cell: Cell3, label: &str, mut ids: Ids, limits: Limits) -> Result<Self> {
        cell.volume()?;
        if limits.candidate_count == 0
            || limits.candidate_bytes == 0
            || limits.retention_ms == 0
            || limits.resident_bytes == 0
        {
            return Err(err("E_RESOURCE_LIMIT", "positive host limits required"));
        }
        let document_id = ids.next_id();
        let epoch = ids.next_id();
        let content = ProjectContent {
            native_camera: None,
            phases: vec![Phase {
                id: ids.next_id().into(),
                label: label.into(),
                cell,
                sites: vec![],
                symmetry_op_id: ids.next_id().into(),
                boundary_min: [0.; 3],
                boundary_max: [1.; 3],
            }],
            scene_id: ids.next_id().into(),
            camera: Camera {
                rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
                projection: "parallel".into(),
                zoom: 1.,
            },
            hidden: vec![],
            comment: String::new(),
            resources: vec![],
            legacy_source: None,
        };
        content.validate()?;
        Ok(Self {
            history: History::new(content.clone()),
            content,
            source: None,
            source_digest: None,
            source_bytes: 0,
            fields: Vec::new(),
            document_id,
            epoch,
            revision: 0,
            ids,
            resources: ResourceStore::default(),
            candidates: BTreeMap::new(),
            limits,
            started: Instant::now(),
            baseline: None,
            work_ref: None,
        })
    }
    pub fn document(&self) -> DocumentAt {
        DocumentAt {
            document_id: self.document_id.clone(),
            epoch: self.epoch.clone(),
            revision: self.revision.to_string(),
        }
    }
    pub fn resident_bytes(&self) -> usize {
        self.resources.bytes()
            + self.source_bytes
            + self
                .history
                .snapshots
                .iter()
                .map(|v| {
                    serde_json::to_vec(v)
                        .map(|b| b.len())
                        .unwrap_or(usize::MAX / 1024)
                })
                .sum::<usize>()
            + serde_json::to_vec(&self.content)
                .map(|b| b.len())
                .unwrap_or(usize::MAX / 1024)
            + self
                .candidates
                .values()
                .map(|c| {
                    serde_json::to_vec(&c.content)
                        .map(|b| b.len())
                        .unwrap_or(usize::MAX / 1024)
                })
                .sum::<usize>()
    }
    pub fn content(&self) -> &ProjectContent {
        &self.content
    }
    fn state_digest(&self) -> String {
        Capture::digest_parts(&self.content, self.source_digest.as_deref())
    }
    pub fn active_fields(&self) -> &[ActiveField] {
        &self.fields
    }
    pub fn active_orffe(&self) -> Result<Vec<R80GeometryTable>> {
        self.source.as_ref().map(|source| active_tables(&source.to_value()?))
            .unwrap_or_else(|| Ok(Vec::new()))
    }
    pub fn field_buffer(&self, id: &str) -> Result<FieldBuffer<'_>> {
        let field = self
            .fields
            .iter()
            .find(|field| field.field_id == id)
            .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "field is not installed"))?;
        let raw = self.resources.read(
            &field.descriptor.resource.digest,
            &format!("persistent:{}", field.resource_id),
        )?;
        FieldBuffer::open(&field.descriptor, raw)
    }
    pub fn context(&self) -> Value {
        json!({"documentId":self.document_id,"epoch":self.epoch,"revision":self.revision.to_string(),"stateDigest":self.state_digest(),"dirty":self.baseline.as_ref()!=Some(&self.state_digest()),"workRef":self.work_ref})
    }
    pub fn import_dirty(&mut self) {
        self.baseline = None;
        self.work_ref = None;
    }
    pub fn loaded_work_ref(&mut self, path: &str, hash: &str) {
        self.work_ref = Some(json!({"path":path,"primarySha256":hash}));
    }
    pub fn now_ms(&self) -> u64 {
        u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX)
    }
    pub fn history_summary(&self, effect: &str) -> Value {
        let cursor = Capture::digest_parts(
            &self.history.snapshots[self.history.cursor],
            self.source_digest.as_deref(),
        );
        json!({"documentId":self.document_id,"epoch":self.epoch,"revision":self.revision.to_string(),"liveContentDigest":self.state_digest(),"historyGeneration":self.history.generation.to_string(),"historyCount":self.history.snapshots.len().to_string(),"historyCursor":self.history.cursor.to_string(),"cursorSnapshotDigest":cursor,"historyEffect":effect,"undoAvailable":self.history.cursor>0,"redoAvailable":self.history.cursor+1<self.history.snapshots.len()})
    }
    pub fn check(&self, at: &DocumentAt) -> Result<()> {
        if at.document_id != self.document_id {
            return Err(err("E_DOCUMENT_NOT_FOUND", "document identity"));
        }
        if at.epoch != self.epoch {
            return Err(err("E_STALE_EPOCH", "epoch"));
        }
        if vesta_model::json::u64_str(&json!(at.revision))? != self.revision {
            return Err(err("E_REVISION_CONFLICT", "expected revision differs"));
        }
        Ok(())
    }
    fn install(&mut self, next: ProjectContent) -> Result<bool> {
        next.validate()?;
        if next == self.content {
            return Ok(false);
        }
        for field in &self.fields {
            if let FieldOwner::Phase(owner) = &field.owner
                && field.descriptor.periodic.contains(&true)
                && let Some(current) = self.content.phases.iter().find(|p| p.id.as_str() == owner.as_str())
                && !next.phases.iter().any(|p| p.id == current.id && p.cell == current.cell)
            {
                return Err(err("E_REOPEN_OWNER", "edit invalidates the periodic Field phase basis"));
            }
        }
        for table in self.active_orffe()? {
            let old = self.content.phases.iter().find(|phase| phase.id == table.owner);
            let updated = next.phases.iter().find(|phase| phase.id == table.owner);
            let source_bound = match (old, updated) {
                (Some(old), Some(updated)) => old.cell == updated.cell &&
                    old.label == updated.label && old.sites == updated.sites &&
                    old.symmetry_op_id == updated.symmetry_op_id,
                (None, None) => self.source.as_ref().map(SourceDocument::to_value).transpose()?
                    .is_some_and(|source| source["metadata"].as_array().into_iter().flatten().any(|row| {
                        row["dialect"] == RIETAN_OWNER_DIALECT
                            && row["owner"]["id"] == table.owner.as_str()
                    })),
                _ => false,
            };
            let resource = |project: &ProjectContent| {
                project.resources.iter().any(|r| r.id == table.resource && r.sha256 == table.table_sha256)
            };
            if !source_bound || !resource(&next) {
                return Err(err("E_REOPEN_OWNER", "edit invalidates the selected ORFFE phase or table binding"));
            }
        }
        let retained_history = self
            .history
            .snapshots
            .iter()
            .take(self.history.cursor + 1)
            .map(|v| {
                serde_json::to_vec(v)
                    .map(|b| b.len())
                    .unwrap_or(self.limits.resident_bytes)
            })
            .sum::<usize>();
        let size = serde_json::to_vec(&next)
            .map_err(|e| Error::new("E_SCHEMA", e.to_string()))?
            .len();
        if size
            .checked_mul(2)
            .and_then(|n| n.checked_add(retained_history))
            .and_then(|n| n.checked_add(self.source_bytes))
            .and_then(|n| n.checked_add(self.resources.bytes()))
            .is_none_or(|n| n > self.limits.resident_bytes)
        {
            return Err(err("E_RESOURCE_LIMIT", "current model budget"));
        }
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| err("E_RESOURCE_LIMIT", "revision overflow"))?;
        self.content = next;
        self.history.record(self.content.clone());
        Ok(true)
    }
    fn completion(&mut self, command: &str, before: u64, result: Value, effect: &str) -> Value {
        let changed = self.revision != before;
        json!({"command":command,"requestId":self.ids.next_id(),"documentId":self.document_id,"epoch":self.epoch,"revision":self.revision.to_string(),"stateDigest":self.state_digest(),"changeSet":{"before":before.to_string(),"after":self.revision.to_string(),"digest":self.state_digest(),"changedObjects":if changed{self.content.phases.iter().map(|phase|phase.id.to_string()).chain(std::iter::once(self.content.scene_id.to_string())).collect::<Vec<_>>()}else{vec![]},"invalidated":if changed{vec!["domain","scene","field","render"]}else{vec![]}},"result":result,"warnings":[],"history":self.history_summary(if changed{effect}else{"unchanged"})})
    }
    pub fn execute(&mut self, at: &DocumentAt, inv: &Value) -> Result<Value> {
        self.check(at)?;
        vesta_model::json::schema(inv, "native-session-v2.schema.json", "CoreInvocation")?;
        let command = inv["command"]
            .as_str()
            .ok_or_else(|| err("E_SCHEMA", "command"))?;
        let before = self.revision;
        match command {
            "history.undo" | "history.redo" => {
                let cursor = if command == "history.undo" {
                    self.history.cursor.checked_sub(1)
                } else {
                    self.history
                        .cursor
                        .checked_add(1)
                        .filter(|n| *n < self.history.snapshots.len())
                }
                .ok_or_else(|| err("E_NO_HISTORY", "history end"))?;
                self.revision = self
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| err("E_RESOURCE_LIMIT", "revision overflow"))?;
                self.history.cursor = cursor;
                self.history.generation += 1;
                self.content = self.history.snapshots[cursor].clone();
                Ok(self.completion(command, before, Value::Null, "navigate-history"))
            }
            "domain.state.prepare" => {
                let edit = self.input_json(&inv["params"]["edit"])?;
                let prepared = self.prepare_state(at, &edit, self.now_ms());
                if let Some(id) = inv["params"]["edit"]["sha256"].as_str() {
                    let _ = self.resources.release(id, "input");
                }
                let p = prepared?;
                Ok(self.completion(command,before,json!({"candidateId":p.candidate_id,"candidateDigest":p.candidate_digest,"validatedState":p.validated_state}),"unchanged"))
            }
            "domain.prepare" => {
                let request = self.input_json(&inv["params"]["request"])?;
                let valid = vesta_model::json::schema(
                    &request,
                    "scientific-composition.schema.json",
                    "OperationRequest2",
                );
                if let Some(id) = inv["params"]["request"]["sha256"].as_str() {
                    let _ = self.resources.release(id, "input");
                }
                valid?;
                Err(err(
                    "E_NOT_IMPLEMENTED",
                    "selected scientific operation has no qualified kernel in P2-03",
                ))
            }
            "domain.apply" => self.apply_domain(
                at,
                inv["params"]["candidateId"]
                    .as_str()
                    .ok_or_else(|| err("E_SCHEMA", "candidate"))?,
                inv["params"]["candidateDigest"]
                    .as_str()
                    .ok_or_else(|| err("E_SCHEMA", "candidate digest"))?,
                self.now_ms(),
            ),
            _ => {
                let mut next = self.content.clone();
                let mut ids = self.ids.clone();
                let result = apply_basic(&mut next, &mut ids, command, &inv["params"])?;
                self.install(next)?;
                self.ids = ids;
                Ok(self.completion(command, before, result, "record"))
            }
        }
    }
    pub fn batch(&mut self, at: &DocumentAt, invocations: &[Value]) -> Result<Vec<Value>> {
        self.check(at)?;
        if invocations.is_empty() || invocations.len() > 1024 {
            return Err(err("E_RESOURCE_LIMIT", "batch population"));
        }
        let mut content = self.content.clone();
        let mut ids = self.ids.clone();
        let mut results = vec![];
        for inv in invocations {
            vesta_model::json::schema(inv, "native-session-v2.schema.json", "CoreInvocation")?;
            let c = inv["command"]
                .as_str()
                .ok_or_else(|| err("E_SCHEMA", "command"))?;
            if !crate::transaction::is_atomic_command(c) {
                return Err(err(
                    "E_NONATOMIC_EFFECT",
                    "only pure model edits in atomic batch",
                ));
            }
            results.push((
                c.to_owned(),
                apply_basic(&mut content, &mut ids, c, &inv["params"])?,
            ));
        }
        let before = self.revision;
        self.install(content)?;
        self.ids = ids;
        Ok(results
            .into_iter()
            .map(|(c, v)| self.completion(&c, before, v, "record"))
            .collect())
    }
    pub fn stage_json(&mut self, bytes: Vec<u8>) -> Result<Value> {
        vesta_model::json::parse(&bytes, self.limits.candidate_bytes)?;
        let len = bytes.len();
        let hash = self
            .resources
            .put(bytes, "input", self.limits.resident_bytes)?;
        Ok(json!({"sha256":hash,"byteLength":len.to_string(),"mediaType":"application/json"}))
    }
    fn input_json(&self, desc: &Value) -> Result<Value> {
        let id = desc["sha256"]
            .as_str()
            .ok_or_else(|| err("E_SCHEMA", "input descriptor"))?;
        let bytes = self.resources.read(id, "input")?;
        if vesta_model::json::u64_str(&desc["byteLength"])? != bytes.len() as u64
            || desc["mediaType"] != "application/json"
        {
            return Err(err("E_SCHEMA", "typed input length/media"));
        }
        vesta_model::json::parse(bytes, self.limits.candidate_bytes)
    }
    pub fn prepare_state(&mut self, at: &DocumentAt, edit: &Value, now: u64) -> Result<Prepared> {
        self.check(at)?;
        vesta_model::json::schema(
            edit,
            "scientific-composition.schema.json",
            "ScientificStateEdit2",
        )?;
        self.expire_candidates(now);
        if self.candidates.len() >= self.limits.candidate_count {
            return Err(err("E_RESOURCE_LIMIT", "candidate population"));
        }
        if edit["component"] != "native-camera" {
            return Err(err(
                "E_NOT_IMPLEMENTED",
                "state component is not implemented by this card",
            ));
        }
        if edit["targetSceneId"].as_str() != Some(self.content.scene_id.as_str()) {
            return Err(err("E_OBJECT_NOT_FOUND", "candidate scene owner"));
        }
        validate_native_camera(&edit["state"])?;
        let state = &edit["state"];
        let mut next = self.content.clone();
        let matrix: Vec<Vec<f64>> = typed(&state["sceneMatrixLogicalRows"])?;
        if matrix[3] != [0., 0., 0., 1.] || matrix.iter().take(3).any(|r| r[3] != 0.) {
            return Err(err(
                "E_NOT_IMPLEMENTED",
                "core camera affine translation requires later scene domain",
            ));
        }
        next.camera = Camera {
            rotation: [
                [matrix[0][0], matrix[0][1], matrix[0][2]],
                [matrix[1][0], matrix[1][1], matrix[1][2]],
                [matrix[2][0], matrix[2][1], matrix[2][2]],
            ],
            zoom: state["zoomF32"]
                .as_f64()
                .ok_or_else(|| err("E_SCHEMA", "zoom"))?,
            projection: if state["projectionSelectorBit21"] == 0 {
                "parallel"
            } else {
                "perspective"
            }
            .into(),
        };
        next.native_camera = Some(typed(state)?);
        next.validate()?;
        let bytes = serde_json::to_vec(edit).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
        if bytes.len() > self.limits.candidate_bytes {
            return Err(err("E_RESOURCE_LIMIT", "candidate bytes"));
        }
        let id = self.ids.next_id();
        let digest=digest(&serde_json::to_vec(&json!({"source":at,"sourceDigest":self.state_digest(),"candidate":next,"profile":"I33/native-camera/explicit-canonical/1","limit":self.limits.candidate_bytes})).map_err(|e|Error::new("E_SCHEMA",e.to_string()))?);
        let resource = self.resources.put(
            bytes.clone(),
            &format!("candidate:{id}"),
            self.limits.resident_bytes,
        )?;
        let prepared = Prepared {
            candidate_id: id.clone().into(),
            candidate_digest: digest.clone(),
            validated_state: json!({"sha256":resource,"byteLength":bytes.len().to_string(),"mediaType":"application/json"}),
        };
        self.candidates.insert(
            id,
            Candidate {
                at: at.clone(),
                digest,
                content: next,
                until: now
                    .checked_add(self.limits.retention_ms)
                    .ok_or_else(|| err("E_RESOURCE_LIMIT", "candidate expiry"))?,
                resource,
            },
        );
        Ok(prepared)
    }
    fn expire_candidates(&mut self, now: u64) {
        let ids: Vec<_> = self
            .candidates
            .iter()
            .filter(|(_, c)| now >= c.until || c.at != self.document())
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            if let Some(c) = self.candidates.remove(&id) {
                let _ = self
                    .resources
                    .release(&c.resource, &format!("candidate:{id}"));
            }
        }
    }
    pub fn candidate_bytes(&self, id: &str) -> Result<&[u8]> {
        let c = self
            .candidates
            .get(id)
            .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "candidate"))?;
        self.resources.read(&c.resource, &format!("candidate:{id}"))
    }
    pub fn apply_domain(
        &mut self,
        at: &DocumentAt,
        id: &str,
        expected_digest: &str,
        now: u64,
    ) -> Result<Value> {
        self.check(at)?;
        self.expire_candidates(now);
        let c = self
            .candidates
            .get(id)
            .cloned()
            .ok_or_else(|| err("E_REQUEST_EXPIRED", "candidate absent/expired/consumed"))?;
        if c.at != *at || c.digest != expected_digest {
            return Err(err(
                "E_REVISION_CONFLICT",
                "candidate source/digest mismatch",
            ));
        }
        let before = self.revision;
        let changed = self.install(c.content)?;
        self.candidates.remove(id);
        self.resources
            .release(&c.resource, &format!("candidate:{id}"))?;
        Ok(self.completion(
            "domain.apply",
            before,
            json!({"candidateId":id,"candidateDigest":expected_digest,"applied":changed}),
            "record",
        ))
    }
    pub fn capture(&self) -> Result<Capture> {
        self.content.validate()?;
        let mut resources = BTreeMap::new();
        for r in &self.content.resources {
            let bytes = self
                .resources
                .read(&r.sha256, &format!("persistent:{}", r.id))?;
            if digest(bytes) != r.sha256 || bytes.len().to_string() != r.byte_length {
                return Err(err("E_RESOURCE_MISSING", "persistent bytes mismatch"));
            }
            resources.insert(r.sha256.clone(), bytes.to_vec());
        }
        Ok(Capture {
            document: self.document(),
            content: self.content.clone(),
            resources,
            state_digest: self.state_digest(),
            source: self.source.clone(),
        })
    }
    pub fn loaded(restored: RestoredProject, _ids: Ids, limits: Limits) -> Result<Self> {
        let RestoredProject {
            capture,
            next_ids: ids,
            ..
        } = restored;
        capture.content.validate()?;
        if limits.candidate_count == 0
            || limits.candidate_bytes == 0
            || limits.retention_ms == 0
            || limits.resident_bytes == 0
        {
            return Err(err("E_RESOURCE_LIMIT", "positive host limits required"));
        }
        let mut resources = ResourceStore::default();
        for r in &capture.content.resources {
            let bytes = capture
                .resources
                .get(&r.sha256)
                .ok_or_else(|| err("E_RESOURCE_MISSING", "fresh loader missing source"))?;
            if digest(bytes) != r.sha256 || bytes.len().to_string() != r.byte_length {
                return Err(err("E_RESOURCE_MISSING", "fresh loader bytes mismatch"));
            }
            resources.put(
                bytes.clone(),
                &format!("persistent:{}", r.id),
                limits.resident_bytes,
            )?;
        }
        let (source_digest, source_bytes, fields) = if let Some(source) = &capture.source {
            let value = source.to_value()?;
            let installed_bytes = capture
                .content
                .resources
                .iter()
                .map(|resource| {
                    let bytes = resources.read(
                        &resource.sha256,
                        &format!("persistent:{}", resource.id),
                    )?;
                    Ok((resource.id.to_string(), (resource.sha256.clone(), bytes.to_vec())))
                })
                .collect::<Result<_>>()?;
            source.validate_installed_resources(&installed_bytes, &capture.content.resources)?;
            let fields = restore_fields(
                &value,
                &capture.content.resources,
                &capture.resources,
                limits.max_field_values,
            )?;
            let canonical = vesta_model::source_audit::canonical(&value);
            (Some(digest(&canonical)), canonical.len(), fields)
        } else {
            (None, 0, Vec::new())
        };
        let baseline = Some(Capture::digest_parts(
            &capture.content,
            source_digest.as_deref(),
        ));
        let history = History::new(capture.content.clone());
        let loaded = Self {
            content: capture.content,
            source: capture.source,
            source_digest,
            source_bytes,
            fields,
            document_id: capture.document.document_id,
            epoch: capture.document.epoch,
            revision: 0,
            ids,
            resources,
            history,
            candidates: BTreeMap::new(),
            limits,
            started: Instant::now(),
            baseline,
            work_ref: None,
        };
        if loaded.resident_bytes() > loaded.limits.resident_bytes {
            return Err(err(
                "E_RESOURCE_LIMIT",
                "restored engine exceeds resident budget",
            ));
        }
        Ok(loaded)
    }
    pub fn attach_resource(&mut self, key: &str, bytes: Vec<u8>) -> Result<()> {
        self.attach_resources(vec![SourceAttachment {
            logical_key: key.into(),
            bytes,
            media_type: "application/octet-stream".into(),
        }])
    }
    pub fn attach_resources(&mut self, attachments: Vec<SourceAttachment>) -> Result<()> {
        let mut store = self.resources.clone();
        let mut ids = self.ids.clone();
        let mut next = self.content.clone();
        for attachment in attachments {
            if !safe_key(&attachment.logical_key)
                || next
                    .resources
                    .iter()
                    .any(|r| r.logical_key == attachment.logical_key)
            {
                return Err(err("E_SCHEMA", "resource logical key"));
            }
            let resource_id = ids.next_id();
            let length = attachment.bytes.len();
            let hash = store.put(
                attachment.bytes,
                &format!("persistent:{resource_id}"),
                self.limits.resident_bytes,
            )?;
            next.resources.push(PersistentResource {
                id: resource_id.into(),
                logical_key: attachment.logical_key,
                sha256: hash,
                byte_length: length.to_string(),
                media_type: attachment.media_type,
            });
        }
        next.validate()?;
        if serde_json::to_vec(&next)
            .map_err(|e| Error::new("E_SCHEMA", e.to_string()))?
            .len()
            .checked_add(store.bytes())
            .is_none_or(|n| n > self.limits.resident_bytes)
        {
            return Err(err("E_RESOURCE_LIMIT", "full model/resource budget"));
        }
        self.install(next)?;
        self.resources = store;
        self.ids = ids;
        Ok(())
    }
}
fn apply_basic(
    content: &mut ProjectContent,
    ids: &mut Ids,
    command: &str,
    p: &Value,
) -> Result<Value> {
    let mut result = Value::Null;
    match command {
        "cell.set" => {
            let phase = content
                .phases
                .iter_mut()
                .find(|v| v.id == p["phaseId"])
                .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "phase"))?;
            phase.cell = typed(&p["cell"])?;
        }
        "site.add" => {
            let phase = content
                .phases
                .iter_mut()
                .find(|v| v.id == p["phaseId"])
                .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "phase"))?;
            let id = ids.next_id();
            let site = Site {
                id: id.clone().into(),
                label: p["label"]
                    .as_str()
                    .ok_or_else(|| err("E_SCHEMA", "label"))?
                    .into(),
                element: p["species"]["element"]
                    .as_str()
                    .ok_or_else(|| err("E_SCHEMA", "element"))?
                    .into(),
                frac: typed(&p["frac"])?,
                occupancy: p["occupancy"]
                    .as_f64()
                    .ok_or_else(|| err("E_SCHEMA", "occupancy"))?,
                u_iso_a2: p["uIsoA2"]
                    .as_f64()
                    .ok_or_else(|| err("E_SCHEMA", "uIsoA2"))?,
            };
            phase.sites.push(site);
            result = json!({"siteId":id});
        }
        "site.setFractional" => {
            let phase = content
                .phases
                .iter_mut()
                .find(|v| v.id == p["phaseId"])
                .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "phase"))?;
            let site = phase
                .sites
                .iter_mut()
                .find(|s| s.id == p["siteId"])
                .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "site"))?;
            site.frac = typed(&p["frac"])?;
        }
        "boundary.set" => {
            let phase = content
                .phases
                .iter_mut()
                .find(|v| v.id == p["phaseId"])
                .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "phase"))?;
            phase.boundary_min = typed(&p["min"])?;
            phase.boundary_max = typed(&p["max"])?;
        }
        "display.setHidden" => {
            let images: Vec<ImageKey> = typed(&p["images"])?;
            let hidden = p["hidden"]
                .as_bool()
                .ok_or_else(|| err("E_SCHEMA", "hidden"))?;
            for k in images {
                let phase = content
                    .phases
                    .iter()
                    .find(|a| a.id == k.phase)
                    .ok_or_else(|| err("E_OBJECT_NOT_FOUND", "image phase"))?;
                if phase.symmetry_op_id != k.op || !phase.sites.iter().any(|s| s.id == k.site) {
                    return Err(err("E_OBJECT_NOT_FOUND", "image owner"));
                }
                if hidden {
                    if !content.hidden.contains(&k) {
                        content.hidden.push(k)
                    }
                } else {
                    content.hidden.retain(|v| v != &k)
                }
            }
        }
        "view.setCamera" => {
            content.camera = typed(p)?;
            content.native_camera = None;
        }
        _ => {
            return Err(err(
                "E_NOT_IMPLEMENTED",
                "command has no qualified implementation",
            ));
        }
    }
    content.validate()?;
    Ok(result)
}
