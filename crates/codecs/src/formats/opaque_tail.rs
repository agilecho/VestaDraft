//! Opaque preservation for the inert suffix of the release-pinned FEFF job.
use super::graph_provenance::VerifiedGraph;
use super::native_vesta_v2::pointer;
use super::resource_graph::{FEFF_END_CHILD, feff_end_tail};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use vesta_model::source_audit::{AuditSource, Role, canonical};
use vesta_model::source7::SourceDocument;
use vesta_model::{Capture, Error, PersistentResource, Result, digest};

const CONTRACT: &str = include_str!("../../contracts/opaque-edit-dependency.contract.json");
const RULE: &str = include_str!("../../contracts/feff10-opaque-end-tail.json");
const DIALECT: &str = "native-feff10-post-end-tail-1";

pub(super) fn rule_digest() -> String {
    digest(&canonical(&json!([
        CONTRACT,
        RULE,
        include_str!("../../contracts/feff10-lexer.contract.json")
    ])))
}

fn unsafe_rewrite(message: &str) -> Error {
    Error::new("E_UNSAFE_SOURCE_REWRITE", message)
}

struct Tail<'a> {
    context: &'a PersistentResource,
    child: &'a PersistentResource,
    extent: Value,
    units: Vec<Value>,
    namespace: String,
}

fn tail<'a>(capture: &Capture, source: &Value, graph: &VerifiedGraph<'a>) -> Result<Tail<'a>> {
    let child = *graph
        .files
        .get(FEFF_END_CHILD)
        .ok_or_else(|| unsafe_rewrite("qualified FEFF graph has no terminal source stream"))?;
    let bytes = capture
        .resources
        .get(&child.sha256)
        .ok_or_else(|| unsafe_rewrite("terminal FEFF stream has no owned bytes"))?;
    let offset = feff_end_tail(bytes)?;
    let span = &bytes[offset..];
    let expected = graph.audit_node().wire();
    let metadata = source["metadata"]
        .as_array()
        .ok_or_else(|| unsafe_rewrite("expected complete source metadata"))?;
    let (index, _) = metadata
        .iter()
        .enumerate()
        .find(|(_, row)| row["tree"] == expected)
        .ok_or_else(|| unsafe_rewrite("terminal FEFF stream has no authoritative graph use"))?;
    let base = format!("/metadata/{index}/tree/payload/members");
    let unit = |at: String, resource: &PersistentResource| {
        json!({
            "owner":{"kind":"document","id":capture.document.document_id},
            "relativePointer":at,
            "sourceNodeDigest":digest(&canonical(&json!({"kind":"BytesRef",
                "logicalResourceKey":resource.logical_key,"sha256":resource.sha256,"byteLength":resource.byte_length})))
        })
    };
    let mut units = vec![unit(format!("{base}/graph"), graph.context)];
    let file_members = format!("{base}/files/members");
    for (name, resource) in &graph.files {
        units.push(unit(pointer(&file_members, name), resource));
    }
    let namespace = digest(&canonical(&json!({
        "root":graph.root_key(),"graph":graph.context.sha256,"rule":rule_digest()
    })));
    Ok(Tail {
        context: graph.context,
        child,
        extent: json!({"sourceResourceId":child.id,"byteOffset":offset.to_string(),
            "byteLength":span.len().to_string(),"sha256":digest(span)}),
        units,
        namespace,
    })
}

fn resource_closure(capture: &Capture, source: &Value, tails: &[Tail<'_>]) -> Result<String> {
    let mut base = source.clone();
    base["opaqueBlocks"] = json!([]);
    let audit = SourceDocument::decode(base)?.audit()?;
    let resources: BTreeMap<_, _> = capture
        .content
        .resources
        .iter()
        .map(|r| (r.id.as_str(), r))
        .collect();
    let uses = audit
        .roles()
        .into_iter()
        .filter(|(_, kind, _, _)| kind == "Resource")
        .map(|(path, _, id, role)| {
            let row = resources
                .get(id.as_str())
                .ok_or_else(|| unsafe_rewrite("resource absent from opaque baseline"))?;
            let role = match role {
                Role::Definition => "definition",
                Role::Alias => "alias",
                Role::Reference => "reference",
            };
            Ok(json!([
                path,
                role,
                row.logical_key,
                row.sha256,
                row.byte_length,
                row.media_type
            ]))
        })
        .collect::<Result<Vec<_>>>()?;
    // Opaque RawSpans are accounted for independently without hashing their
    // derived baseline fields back into the resource closure.
    let extents: Vec<_> = tails
        .iter()
        .map(|tail| {
            json!([
                tail.child.logical_key,
                tail.extent["byteOffset"],
                tail.extent["byteLength"],
                tail.extent["sha256"],
            ])
        })
        .collect();
    Ok(digest(&canonical(
        &json!({"domain":"logical-opaque-resource-closure/1","uses":uses,"extents":extents}),
    )))
}

pub(super) fn bind(
    capture: &Capture,
    source: &mut Value,
    graphs: &[VerifiedGraph<'_>],
) -> Result<()> {
    let actual = source["opaqueBlocks"]
        .as_array()
        .ok_or_else(|| unsafe_rewrite("expected opaque source collection"))?;
    let tails = graphs
        .iter()
        .filter(|g| g.is_selected_feff())
        .map(|g| tail(capture, source, g))
        .collect::<Result<Vec<_>>>()?;
    if tails.is_empty() {
        return if actual.is_empty() {
            Ok(())
        } else {
            Err(unsafe_rewrite(
                "opaque blocks lack an independently qualified rule",
            ))
        };
    }
    if !actual.is_empty() && actual.len() != tails.len() {
        return Err(unsafe_rewrite(
            "opaque block population differs from trusted extents",
        ));
    }
    let closure = resource_closure(capture, source, &tails)?;
    let mut expected = Vec::with_capacity(tails.len());
    for (index, tail) in tails.iter().enumerate() {
        let existing = actual.get(index);
        let minted = digest(&canonical(&json!([
            "native-opaque-identity/1",
            capture.document.document_id,
            tail.context.logical_key,
            tail.extent["byteOffset"],
            tail.extent["byteLength"]
        ])));
        let id = existing
            .and_then(|block| block["id"].as_str())
            .unwrap_or(&minted[..32]);
        let new_row = json!({
            "id":id,"owner":{"kind":"document","id":capture.document.document_id},
            "formatRef":"VM18-I11","dialect":DIALECT,"extent":tail.extent,
            "editOverlapPolicy":"refuse-unqualified-overlap","metadata":[],
            "editDependencies":{
                "kind":"qualified-exact-footprint",
                "baseline":{
                    "persistentSourceDigest":"0".repeat(64),
                    "sourceTypeDigest":source["identity"]["sourceTypeDigest"],
                    "policyDigest":source["identity"]["policyDigest"],
                    "resourceGraphDigest":closure,"referenceNamespaceDigest":tail.namespace
                },
                "ruleDigest":rule_digest(),"units":tail.units,"referenceClosure":"complete-no-external-references"
            }
        });
        expected.push(new_row);
    }
    if actual.is_empty() {
        let mut initial = source.clone();
        initial["identity"] = Value::Null;
        initial["opaqueBlocks"] = json!(expected);
        let original = digest(&canonical(&initial));
        for row in &mut expected {
            row["editDependencies"]["baseline"]["persistentSourceDigest"] = json!(original);
        }
        source["opaqueBlocks"] = json!(expected);
    } else {
        for (row, observed) in expected.iter_mut().zip(actual) {
            let historical = &observed["editDependencies"]["baseline"]["persistentSourceDigest"];
            // The original projection hash remains provenance. The current
            // authority comes from rederiving all footprint units and context
            // commitments against the independently verified archive.
            row["editDependencies"]["baseline"]["persistentSourceDigest"] = historical.clone();
            if &*row != observed {
                return Err(unsafe_rewrite(
                    "opaque footprint, exact extent or resource context changed",
                ));
            }
        }
    }
    Ok(())
}
