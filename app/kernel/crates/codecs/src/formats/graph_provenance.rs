//! Artifact-only reconstruction of captured native file graphs.
use super::resource_graph::{
    CaptureLimits, GraphArchive, GraphProfile, expanded_rule_digest, graph_rule, profile_for_graph,
    verify_artifact_graph,
};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::source_audit::{Node, Role, canonical};
use vesta_model::source7::resource_graph::{CAPTURE_MEDIA, CapturedGraph, GRAPH_GROUP};
use vesta_model::{Capture, Error, PersistentResource, Result, digest};

const TAG: &str = "NativeGraphCapture2";

pub(super) fn has_graph(source: &Capture) -> bool {
    source
        .content
        .resources
        .iter()
        .any(|r| r.media_type == CAPTURE_MEDIA)
}

pub(super) fn type_digest(core_type: &str) -> String {
    digest(format!("{core_type}\n{TAG}\nstructure source closure 1").as_bytes())
}

pub(super) fn rule_digest(core_rule: &str) -> String {
    digest(
        format!(
            "{core_rule}\n{TAG}\n{}",
            super::resource_graph::graph_rule()
        )
        .as_bytes(),
    )
}

pub(super) fn expanded_digests(core_type: &str, core_rule: &str) -> (String, String) {
    (
        digest(format!("{core_type}\n{TAG}\nselected file closure 2").as_bytes()),
        digest(format!("{core_rule}\n{TAG}\n{}", expanded_rule_digest()).as_bytes()),
    )
}

pub(super) fn snapshot_digests(
    source: &Capture,
    core_type: &str,
    core_rule: &str,
) -> Result<(String, String)> {
    let contexts = source
        .content
        .resources
        .iter()
        .filter(|r| r.media_type == CAPTURE_MEDIA);
    let mut present = false;
    let mut expanded = false;
    let legacy = graph_rule();
    for context in contexts {
        present = true;
        let raw = source
            .resources
            .get(&context.sha256)
            .ok_or_else(|| graph_error("missing graph context bytes"))?;
        let value = vesta_model::json::parse(raw, 16 * 1024 * 1024)?;
        let policy = value["policyDigest"]
            .as_str()
            .ok_or_else(|| graph_error("missing graph context policy"))?;
        expanded |= policy != legacy;
    }
    Ok(if expanded {
        expanded_digests(core_type, core_rule)
    } else if present {
        (type_digest(core_type), rule_digest(core_rule))
    } else {
        (core_type.into(), core_rule.into())
    })
}

fn graph_error(message: &str) -> Error {
    Error::new("E_GRAPH_FILES", message)
}

pub(super) struct VerifiedGraph<'a> {
    pub context: &'a PersistentResource,
    pub files: BTreeMap<String, &'a PersistentResource>,
    graph: CapturedGraph,
    profile: GraphProfile,
}

impl VerifiedGraph<'_> {
    pub(super) fn is_selected_feff(&self) -> bool {
        self.profile == GraphProfile::Feff10Engineering
    }

    pub(super) fn root_key(&self) -> &str {
        &self.graph.root_key
    }

    pub(super) fn archive(&self, source: &Capture) -> Result<GraphArchive> {
        let files = self
            .files
            .iter()
            .map(|(key, resource)| {
                source
                    .resources
                    .get(&resource.sha256)
                    .cloned()
                    .map(|raw| (key.clone(), raw))
                    .ok_or_else(|| graph_error("verified native source resource is absent"))
            })
            .collect::<Result<_>>()?;
        Ok(GraphArchive {
            graph: self.graph.clone(),
            files,
        })
    }

    pub(super) fn format_ref(&self) -> &'static str {
        match self.profile {
            GraphProfile::NativeVesta => "VM18-I01",
            GraphProfile::GamessPair => "VM18-I44",
            GraphProfile::Rietan if self.graph.root_key.ends_with(".ins") => "VM18-I21",
            GraphProfile::Rietan => "VM18-I22",
            GraphProfile::Feff10Engineering => "VM18-I11",
        }
    }

    pub(super) fn dialect(&self) -> &'static str {
        match self.profile {
            GraphProfile::NativeVesta if self.graph.policy_digest == graph_rule() => {
                "native-graph2-structure1"
            }
            GraphProfile::NativeVesta => "native-graph2-vesta354-selected",
            GraphProfile::GamessPair => "native-graph2-gamess-source-only",
            GraphProfile::Rietan => "native-graph2-rietan-source-table",
            GraphProfile::Feff10Engineering => "native-graph2-feff10-engineering-fixture",
        }
    }

    pub(super) fn audit_node(&self) -> Node {
        fn resource_ref(resource: &PersistentResource) -> Node {
            Node::Resource {
                id: resource.id.to_string(),
                sha: resource.sha256.clone(),
                len: resource.byte_length.clone(),
                role: Role::Reference,
            }
        }
        Node::TaggedVariant {
            tag: TAG.into(),
            payload: Box::new(Node::Map(BTreeMap::from([
                ("graph".into(), resource_ref(self.context)),
                (
                    "files".into(),
                    Node::Map(
                        self.files
                            .iter()
                            .map(|(key, resource)| (key.clone(), resource_ref(resource)))
                            .collect(),
                    ),
                ),
            ]))),
        }
    }
}

pub(super) fn verify(source: &Capture) -> Result<Vec<VerifiedGraph<'_>>> {
    let by_key: BTreeMap<_, _> = source
        .content
        .resources
        .iter()
        .map(|r| (r.logical_key.as_str(), r))
        .collect();
    let mut expected = BTreeSet::new();
    let mut output = Vec::new();
    let bytes = |resource: &PersistentResource| -> Result<&Vec<u8>> {
        let bytes = source
            .resources
            .get(&resource.sha256)
            .ok_or_else(|| graph_error("missing graph resource bytes"))?;
        if resource.byte_length != bytes.len().to_string() || resource.sha256 != digest(bytes) {
            return Err(graph_error("graph resource digest mismatch"));
        }
        Ok(bytes)
    };
    for context in source
        .content
        .resources
        .iter()
        .filter(|r| r.media_type == CAPTURE_MEDIA)
    {
        let prefix = format!("{GRAPH_GROUP}/{}", context.sha256);
        let key = format!("{prefix}/capture");
        if context.logical_key != key {
            return Err(graph_error("invalid graph context resource key"));
        }
        expected.insert(key);
        let raw = bytes(context)?;
        let graph: CapturedGraph =
            serde_json::from_value(vesta_model::json::parse(raw, 16 * 1024 * 1024)?)
                .map_err(|err| Error::new("E_SCHEMA", err.to_string()))?;
        let serialized =
            serde_json::to_value(&graph).map_err(|err| Error::new("E_SCHEMA", err.to_string()))?;
        if *raw != canonical(&serialized) {
            return Err(graph_error("noncanonical graph context"));
        }
        let mut files = BTreeMap::new();
        let mut bindings = BTreeMap::new();
        let limits = CaptureLimits::default();
        if graph.files.len() > limits.nodes {
            return Err(Error::new("E_PARSE_LIMIT", "graph file count"));
        }
        let mut remaining = limits.bytes;
        for (i, file) in graph.files.iter().enumerate() {
            let key = format!("{prefix}/file/{i}");
            let resource = *by_key
                .get(key.as_str())
                .ok_or_else(|| graph_error("graph file has no logical resource"))?;
            expected.insert(key);
            if resource.sha256 != file.sha256
                || resource.byte_length != file.byte_length
                || resource.media_type != "application/octet-stream"
            {
                return Err(graph_error("graph file resource identity differs"));
            }
            let actual = bytes(resource)?;
            remaining = remaining
                .checked_sub(actual.len())
                .ok_or_else(|| Error::new("E_PARSE_LIMIT", "graph artifact bytes"))?;
            if files.insert(file.key.clone(), actual.clone()).is_some() {
                return Err(graph_error("duplicate graph file key"));
            }
            bindings.insert(file.key.clone(), resource);
        }
        verify_artifact_graph(&graph, &files, limits)?;
        output.push(VerifiedGraph {
            context,
            files: bindings,
            profile: profile_for_graph(&graph)?,
            graph,
        });
    }
    let graph_keys: BTreeSet<_> = by_key
        .keys()
        .filter(|key| key.starts_with("source-graph/"))
        .map(|key| (*key).to_owned())
        .collect();
    if expected != graph_keys {
        return Err(graph_error("orphan graph provenance resource"));
    }
    Ok(output)
}
