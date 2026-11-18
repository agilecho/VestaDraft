// Generated schema wire types.
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)] pub enum Never {}
pub type S14Id = String;

pub type S14Digest = String;

pub type S14U64 = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14Cell3 { #[serde(rename="lengthA")] pub f_0: Vec<f64>,
#[serde(rename="anglesDeg")] pub f_1: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14WorkRef { #[serde(rename="path")] pub f_0: String,
#[serde(rename="primarySha256")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14DocumentAt { #[serde(rename="documentId")] pub f_0: String,
#[serde(rename="epoch")] pub f_1: String,
#[serde(rename="revision")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ByteRef { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14Budget { #[serde(rename="maxInputBytes")] pub f_0: String,
#[serde(rename="maxResidentBytes")] pub f_1: String,
#[serde(rename="maxFieldValues")] pub f_2: String,
#[serde(rename="maxMeshVertices")] pub f_3: String,
#[serde(rename="maxOutputBytes")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14Policy { #[serde(rename="budget")] pub f_0: S14Budget,
#[serde(rename="maxResponseCacheBytes")] pub f_1: String,
#[serde(rename="maxPendingUploads")] pub f_2: i64,
#[serde(rename="maxPendingSnapshots")] pub f_3: i64,
#[serde(rename="advanceWorkUnits")] pub f_4: i64,
#[serde(rename="cancelGraceMs")] pub f_5: i64,
#[serde(rename="hostCapabilitiesDigest")] pub f_6: String,
#[serde(rename="maxPendingDomainCandidates")] pub f_7: i64,
#[serde(rename="maxDomainResultBytes")] pub f_8: String,
#[serde(rename="domainCandidateRetentionMs")] pub f_9: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14Context { #[serde(rename="documentId")] pub f_0: String,
#[serde(rename="epoch")] pub f_1: String,
#[serde(rename="revision")] pub f_2: String,
#[serde(rename="stateDigest")] pub f_3: String,
#[serde(rename="dirty")] pub f_4: bool,
#[serde(rename="workRef")] pub f_5: S14ContextF5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ContextF5 { V0(S14WorkRef), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14OpenBundle { #[serde(rename="sourcePurpose")] pub f_0: String,
#[serde(rename="primary")] pub f_1: S14ByteRef,
#[serde(rename="format")] pub f_2: String,
#[serde(rename="dependencies")] pub f_3: Vec<S14OpenBundleF3Item>,
#[serde(rename="workRef")] pub f_4: S14OpenBundleF4,
#[serde(rename="allowDiagnosticImport")] pub f_5: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14OpenBundleF3Item { #[serde(rename="logicalPath")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S14ByteRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14OpenBundleF4 { V0(S14WorkRef), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14PhaseRow { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="label")] pub f_1: String,
#[serde(rename="structurePresent")] pub f_2: bool,
#[serde(rename="cell")] pub f_3: S14PhaseRowF3,
#[serde(rename="volumeA3")] pub f_4: S14PhaseRowF4,
#[serde(rename="cellEditable")] pub f_5: bool,
#[serde(rename="siteCount")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14PhaseRowF3 { V0(S14Cell3), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14PhaseRowF4 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14SiteRow { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="siteId")] pub f_1: String,
#[serde(rename="label")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14InspectionPage { V0(S14InspectionPageV0), V1(S14InspectionPageV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14InspectionPageV0 { #[serde(rename="query")] pub f_0: String,
#[serde(rename="document")] pub f_1: S14DocumentAt,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="records")] pub f_3: Vec<S14PhaseRow> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14InspectionPageV1 { #[serde(rename="query")] pub f_0: String,
#[serde(rename="document")] pub f_1: S14DocumentAt,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="records")] pub f_3: Vec<S14SiteRow> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14Query { V0(S14QueryV0), V1(S14QueryV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14QueryV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="limit")] pub f_1: i64,
#[serde(rename="cursor")] pub f_2: S14QueryV0F2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14QueryV0F2 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14QueryV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: String,
#[serde(rename="limit")] pub f_2: i64,
#[serde(rename="cursor")] pub f_3: S14QueryV1F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14QueryV1F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14CoreInvocation { V0(S14CoreInvocationV0), V1(S14CoreInvocationV1), V2(S14CoreInvocationV2), V3(S14CoreInvocationV3), V4(S14CoreInvocationV4), V5(S14CoreInvocationV5), V6(S14CoreInvocationV6), V7(S14CoreInvocationV7), V8(S14CoreInvocationV8), V9(S14CoreInvocationV9), V10(S14CoreInvocationV10) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV0 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV0F1 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="cell")] pub f_1: S14Cell3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV1 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV1F1 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="label")] pub f_1: String,
#[serde(rename="species")] pub f_2: S14CoreInvocationV1F1F2,
#[serde(rename="frac")] pub f_3: Vec<f64>,
#[serde(rename="occupancy")] pub f_4: f64,
#[serde(rename="uIsoA2")] pub f_5: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV1F1F2 { #[serde(rename="element")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV2 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV2F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV2F1 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="siteId")] pub f_1: String,
#[serde(rename="frac")] pub f_2: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV3 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV3F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV3F1 {  }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV4 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV4F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV4F1 {  }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV5 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV5F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV5F1 { #[serde(rename="images")] pub f_0: Vec<S14CoreInvocationV5F1F0Item>,
#[serde(rename="hidden")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV5F1F0Item { #[serde(rename="phase")] pub f_0: String,
#[serde(rename="site")] pub f_1: String,
#[serde(rename="op")] pub f_2: String,
#[serde(rename="shift")] pub f_3: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV6 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV6F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV6F1 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="min")] pub f_1: Vec<f64>,
#[serde(rename="max")] pub f_2: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV7 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV7F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV7F1 { #[serde(rename="rotation")] pub f_0: Vec<Vec<f64>>,
#[serde(rename="projection")] pub f_1: String,
#[serde(rename="zoom")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV8 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV8F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV8F1 { #[serde(rename="request")] pub f_0: S15TypedJsonResource2,
#[serde(rename="resultMode")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV9 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV9F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV9F1 { #[serde(rename="edit")] pub f_0: S15TypedJsonResource2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV10 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="params")] pub f_1: S14CoreInvocationV10F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CoreInvocationV10F1 { #[serde(rename="candidateId")] pub f_0: String,
#[serde(rename="candidateDigest")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14SnapshotOptions { V0(S14SnapshotOptionsV0), V1(S14SnapshotOptionsV1), V2(S14SnapshotOptionsV2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14SnapshotOptionsV0 { #[serde(rename="purpose")] pub f_0: String,
#[serde(rename="format")] pub f_1: String,
#[serde(rename="targetToken")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14SnapshotOptionsV1 { #[serde(rename="purpose")] pub f_0: String,
#[serde(rename="format")] pub f_1: String,
#[serde(rename="targetToken")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14SnapshotOptionsV2 { #[serde(rename="purpose")] pub f_0: String,
#[serde(rename="format")] pub f_1: String,
#[serde(rename="targetToken")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14OperationRef { #[serde(rename="registryGeneration")] pub f_0: String,
#[serde(rename="operationId")] pub f_1: String,
#[serde(rename="recordSequence")] pub f_2: String,
#[serde(rename="publicationSequence")] pub f_3: String,
#[serde(rename="targetDigest")] pub f_4: String,
#[serde(rename="capabilityDigest")] pub f_5: String,
#[serde(rename="semanticEvidenceDigest")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14HostAck { #[serde(rename="snapshotId")] pub f_0: String,
#[serde(rename="documentId")] pub f_1: String,
#[serde(rename="epoch")] pub f_2: String,
#[serde(rename="revision")] pub f_3: String,
#[serde(rename="stateDigest")] pub f_4: String,
#[serde(rename="purpose")] pub f_5: String,
#[serde(rename="targetToken")] pub f_6: String,
#[serde(rename="actualPrimarySha256")] pub f_7: String,
#[serde(rename="verifiedFiles")] pub f_8: Vec<S14HostAckF8Item>,
#[serde(rename="output")] pub f_9: S14WorkRef,
#[serde(rename="currentWorkRef")] pub f_10: S14HostAckF10,
#[serde(rename="operation")] pub f_11: S14OperationRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14HostAckF8Item { #[serde(rename="path")] pub f_0: String,
#[serde(rename="sha256")] pub f_1: String,
#[serde(rename="byteLength")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14HostAckF10 { V0(S14WorkRef), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14HostNack { #[serde(rename="snapshotId")] pub f_0: String,
#[serde(rename="documentId")] pub f_1: String,
#[serde(rename="epoch")] pub f_2: String,
#[serde(rename="revision")] pub f_3: String,
#[serde(rename="purpose")] pub f_4: String,
#[serde(rename="code")] pub f_5: String,
#[serde(rename="settlement")] pub f_6: String,
#[serde(rename="operation")] pub f_7: S14HostNackF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14HostNackF7 { V0(S14OperationRef), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14CloseProof { V0(S14CloseProofV0), V1(S14CloseProofV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CloseProofV0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CloseProofV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="snapshotId")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14Request { V0(S14RequestV0), V1(S14RequestV1), V2(S14RequestV2), V3(S14RequestV3), V4(S14RequestV4), V5(S14RequestV5), V6(S14RequestV6), V7(S14RequestV7), V8(S14RequestV8), V9(S14RequestV9), V10(S14RequestV10), V11(S14RequestV11), V12(S14RequestV12), V13(S14RequestV13), V14(S14RequestV14), V15(S14RequestV15), V16(S14RequestV16) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV0 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="op")] pub f_1: String,
#[serde(rename="policy")] pub f_2: S14Policy,
#[serde(rename="expectedProfiles")] pub f_3: S14RequestV0F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV0F3 { #[serde(rename="nativeSchemaDigest")] pub f_0: String,
#[serde(rename="coreRegistryDigest")] pub f_1: String,
#[serde(rename="saveBundleProfileDigest")] pub f_2: String,
#[serde(rename="hostCapabilitiesDigest")] pub f_3: String,
#[serde(rename="scientificCatalogDigest")] pub f_4: String,
#[serde(rename="assetCatalogDigest")] pub f_5: String,
#[serde(rename="historyPolicyDigest")] pub f_6: String,
#[serde(rename="preferencesSchemaDigest")] pub f_7: String,
#[serde(rename="externalWorkflowSchemaDigest")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV1 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="op")] pub f_2: String,
#[serde(rename="targetSeq")] pub f_3: S14RequestV1F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14RequestV1F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV2 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="op")] pub f_2: String,
#[serde(rename="targetSeq")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV3 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV3F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV3F5 { #[serde(rename="expected")] pub f_0: S14ByteRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV4 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV4F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV4F5 { #[serde(rename="uploadSeq")] pub f_0: String,
#[serde(rename="offset")] pub f_1: String,
#[serde(rename="bytesB64")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV5 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV5F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV5F5 { #[serde(rename="uploadSeq")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV6 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV6F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV6F5 { #[serde(rename="resource")] pub f_0: String,
#[serde(rename="offset")] pub f_1: String,
#[serde(rename="maxBytes")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV7 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV7F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV7F5 { #[serde(rename="resource")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV8 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV8F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV8F5 { #[serde(rename="bundle")] pub f_0: S14OpenBundleResourceRef2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV9 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV9F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV9F5 { #[serde(rename="cell")] pub f_0: S14Cell3,
#[serde(rename="label")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV10 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV10F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV10F5 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="query")] pub f_1: S14Query }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV11 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV11F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV11F5 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="invocation")] pub f_1: S14CoreInvocation }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV12 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV12F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV12F5 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="options")] pub f_1: S14SnapshotOptions }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV13 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV13F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV13F5 { #[serde(rename="ack")] pub f_0: S14HostAckResourceRef2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV14 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV14F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV14F5 { #[serde(rename="nack")] pub f_0: S14HostNackResourceRef2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV15 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV15F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14RequestV15F5 { V0(S14RequestV15F5V0), V1(S14RequestV15F5V1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV15F5V0 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="disposition")] pub f_1: String,
#[serde(rename="proof")] pub f_2: S14CloseProof }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV15F5V1 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="disposition")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV16 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="seq")] pub f_2: String,
#[serde(rename="contextVersion")] pub f_3: String,
#[serde(rename="op")] pub f_4: String,
#[serde(rename="body")] pub f_5: S14RequestV16F5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14RequestV16F5 {  }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14Response { V0(S14ResponseV0), V1(S14ResponseV1), V2(S14ResponseV2), V3(S14ResponseV3), V4(S14ResponseV4), V5(S14ResponseV5), V6(S14ResponseV6), V7(S14ResponseV7), V8(S14ResponseV8), V9(S14ResponseV9), V10(S14ResponseV10), V11(S14ResponseV11), V12(S14ResponseV12), V13(S14ResponseV13), V14(S14ResponseV14), V15(S14ResponseV15), V16(S14ResponseV16), V17(S14ResponseV17), V18(S14ResponseV18), V19(S14ResponseV19) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV0 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: (),
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV0F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV0F6 { #[serde(rename="policyDigest")] pub f_0: String,
#[serde(rename="profiles")] pub f_1: S14ResponseV0F6F1,
#[serde(rename="maxFrameBytes")] pub f_2: f64,
#[serde(rename="maxRawChunkBytes")] pub f_3: f64,
#[serde(rename="coreCommands")] pub f_4: Vec<String>,
#[serde(rename="methods")] pub f_5: Vec<String> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV0F6F1 { #[serde(rename="nativeSchemaDigest")] pub f_0: String,
#[serde(rename="coreRegistryDigest")] pub f_1: String,
#[serde(rename="saveBundleProfileDigest")] pub f_2: String,
#[serde(rename="hostCapabilitiesDigest")] pub f_3: String,
#[serde(rename="scientificCatalogDigest")] pub f_4: String,
#[serde(rename="assetCatalogDigest")] pub f_5: String,
#[serde(rename="historyPolicyDigest")] pub f_6: String,
#[serde(rename="preferencesSchemaDigest")] pub f_7: String,
#[serde(rename="externalWorkflowSchemaDigest")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV1 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV1F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV1F6 { #[serde(rename="uploadSeq")] pub f_0: String,
#[serde(rename="receivedBytes")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV2 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV2F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV2F6 { #[serde(rename="uploadSeq")] pub f_0: String,
#[serde(rename="receivedBytes")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV3 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV3F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV3F6 { #[serde(rename="resource")] pub f_0: S14ByteRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV4 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV4F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV4F6 { #[serde(rename="resource")] pub f_0: String,
#[serde(rename="offset")] pub f_1: String,
#[serde(rename="bytesB64")] pub f_2: String,
#[serde(rename="eof")] pub f_3: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV5 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV5F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV5F6 { #[serde(rename="resource")] pub f_0: String,
#[serde(rename="transportLeaseReleased")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV6 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV6F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV6F6 { #[serde(rename="context")] pub f_0: S14Context,
#[serde(rename="nextContextVersion")] pub f_1: String,
#[serde(rename="editable")] pub f_2: bool,
#[serde(rename="diagnostics")] pub f_3: S14ResponseV6F6F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV6F6F3 { V0(S14FailureNoticeResourceRef2), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV7 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV7F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV7F6 { #[serde(rename="context")] pub f_0: S14Context,
#[serde(rename="nextContextVersion")] pub f_1: String,
#[serde(rename="phaseId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV8 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV8F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV8F6 { #[serde(rename="document")] pub f_0: S14DocumentAt,
#[serde(rename="queryKind")] pub f_1: String,
#[serde(rename="page")] pub f_2: S14InspectionPageResourceRef2,
#[serde(rename="nextCursor")] pub f_3: S14ResponseV8F6F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV8F6F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV9 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV9F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV9F6 { #[serde(rename="context")] pub f_0: S14Context,
#[serde(rename="engineRequestId")] pub f_1: String,
#[serde(rename="outcome")] pub f_2: S14CommandCompletionResourceRef2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV10 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV10F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV10F6 { #[serde(rename="snapshotId")] pub f_0: String,
#[serde(rename="captured")] pub f_1: S14Context,
#[serde(rename="purpose")] pub f_2: String,
#[serde(rename="targetTokenDigest")] pub f_3: String,
#[serde(rename="bundle")] pub f_4: S14ByteRef,
#[serde(rename="sourceExpectationDigest")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV11 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV11F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV11F6 { #[serde(rename="snapshotId")] pub f_0: String,
#[serde(rename="savedDocument")] pub f_1: S14DocumentAt,
#[serde(rename="output")] pub f_2: S14WorkRef,
#[serde(rename="current")] pub f_3: S14Context,
#[serde(rename="publicationSequence")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV12 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV12F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV12F6 { #[serde(rename="snapshotId")] pub f_0: String,
#[serde(rename="settlement")] pub f_1: String,
#[serde(rename="requiresResolution")] pub f_2: bool,
#[serde(rename="current")] pub f_3: S14Context }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV13 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV13F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV13F6 { #[serde(rename="closedDocumentId")] pub f_0: String,
#[serde(rename="closedEpoch")] pub f_1: String,
#[serde(rename="nextContextVersion")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV14 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV14F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV14F6 { #[serde(rename="exitCode")] pub f_0: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV15 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: (),
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="result")] pub f_6: S14ResponseV15F6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV15F6 { #[serde(rename="highestAdmittedSeq")] pub f_0: String,
#[serde(rename="contextVersion")] pub f_1: String,
#[serde(rename="context")] pub f_2: S14ResponseV15F6F2,
#[serde(rename="targetState")] pub f_3: String,
#[serde(rename="stage")] pub f_4: S14ResponseV15F6F4,
#[serde(rename="result")] pub f_5: S14ResponseV15F6F5,
#[serde(rename="receivedBytes")] pub f_6: S14ResponseV15F6F6,
#[serde(rename="committedRevision")] pub f_7: S14ResponseV15F6F7,
#[serde(rename="pendingSnapshots")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F2 { V0(S14Context), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F4 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F5 { V0(S14ResponseV15F6F5V0), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F5V0 { V0(S14CommandCompletionResourceRef2), V1(S14ResponseV15F6F5V0V1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV15F6F5V0V1 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F6 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV15F6F7 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV16 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: (),
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="targetSeq")] pub f_6: String,
#[serde(rename="result")] pub f_7: S14ResponseV16F7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV16F7 { #[serde(rename="outcome")] pub f_0: String,
#[serde(rename="committedRevision")] pub f_1: S14ResponseV16F7F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV16F7F1 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV17 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="jobId")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV18 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="sessionId")] pub f_2: S14ResponseV18F2,
#[serde(rename="contextVersion")] pub f_3: S14ResponseV18F3,
#[serde(rename="seq")] pub f_4: S14ResponseV18F4,
#[serde(rename="code")] pub f_5: String,
#[serde(rename="admitted")] pub f_6: bool,
#[serde(rename="fatal")] pub f_7: bool,
#[serde(rename="highestAdmittedSeq")] pub f_8: S14ResponseV18F8,
#[serde(rename="message")] pub f_9: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV18F2 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV18F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV18F4 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV18F8 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ResponseV19 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="sessionId")] pub f_1: String,
#[serde(rename="contextVersion")] pub f_2: String,
#[serde(rename="seq")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="code")] pub f_6: String,
#[serde(rename="committedRevision")] pub f_7: S14ResponseV19F7,
#[serde(rename="engineFailure")] pub f_8: S14FailureNoticeResourceRef2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14ResponseV19F7 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14Warning { #[serde(rename="code")] pub f_0: String,
#[serde(rename="featureId")] pub f_1: String,
#[serde(rename="referenceVersion")] pub f_2: String,
#[serde(rename="evidenceRef")] pub f_3: String,
#[serde(rename="ruleId")] pub f_4: String,
#[serde(rename="impact")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14ChangeSet { #[serde(rename="before")] pub f_0: String,
#[serde(rename="after")] pub f_1: String,
#[serde(rename="digest")] pub f_2: String,
#[serde(rename="changedObjects")] pub f_3: Vec<String>,
#[serde(rename="invalidated")] pub f_4: Vec<String> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14CommandCompletion { V0(S14CommandCompletionV0), V1(S14CommandCompletionV1), V2(S14CommandCompletionV2), V3(S14CommandCompletionV3), V4(S14CommandCompletionV4), V5(S14CommandCompletionV5), V6(S14CommandCompletionV6), V7(S14CommandCompletionV7), V8(S14CommandCompletionV8), V9(S14CommandCompletionV9), V10(S14CommandCompletionV10) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV0 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV1 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: S14CommandCompletionV1F7,
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV1F7 { #[serde(rename="siteId")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV2 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV3 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV4 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV5 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV6 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV7 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: (),
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV8 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: S15PrepareDomainResult2,
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV9 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: S15PrepareStateResult2,
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionV10 { #[serde(rename="command")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="documentId")] pub f_2: String,
#[serde(rename="epoch")] pub f_3: String,
#[serde(rename="revision")] pub f_4: String,
#[serde(rename="stateDigest")] pub f_5: String,
#[serde(rename="changeSet")] pub f_6: S14ChangeSet,
#[serde(rename="result")] pub f_7: S15AppliedDomainResult2,
#[serde(rename="warnings")] pub f_8: Vec<S14Warning>,
#[serde(rename="history")] pub f_9: S14HistorySummary2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14FailureNotice { #[serde(rename="requestId")] pub f_0: String,
#[serde(rename="code")] pub f_1: String,
#[serde(rename="stage")] pub f_2: String,
#[serde(rename="message")] pub f_3: String,
#[serde(rename="retryable")] pub f_4: bool,
#[serde(rename="committedRevision")] pub f_5: S14FailureNoticeF5,
#[serde(rename="recovery")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S14FailureNoticeF5 { V0(String), V1(()) }

pub type S14HistorySummary2 = S13HistoryAwareCommitSummary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14OpenBundleResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14InspectionPageResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14HostAckResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14HostNackResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14CommandCompletionResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S14FailureNoticeResourceRef2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

pub type S14Root = S14Request;

pub type S15Id = String;

pub type S15Digest = String;

pub type S15U64 = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S15OperationRequest2 { V0(S15OperationRequest2V0), V1(S15OperationRequest2V1), V2(S15OperationRequest2V2), V3(S15OperationRequest2V3), V4(S15OperationRequest2V4), V5(S15OperationRequest2V5), V6(S15OperationRequest2V6), V7(S15OperationRequest2V7), V8(S15OperationRequest2V8), V9(S15OperationRequest2V9), V10(S15OperationRequest2V10), V11(S15OperationRequest2V11), V12(S15OperationRequest2V12), V13(S15OperationRequest2V13), V14(S15OperationRequest2V14), V15(S15OperationRequest2V15), V16(S15OperationRequest2V16), V17(S15OperationRequest2V17), V18(S15OperationRequest2V18), V19(S15OperationRequest2V19), V20(S15OperationRequest2V20), V21(S15OperationRequest2V21), V22(S15OperationRequest2V22), V23(S15OperationRequest2V23), V24(S15OperationRequest2V24), V25(S15OperationRequest2V25), V26(S15OperationRequest2V26), V27(S15OperationRequest2V27), V28(S15OperationRequest2V28), V29(S15OperationRequest2V29), V30(S15OperationRequest2V30), V31(S15OperationRequest2V31), V32(S15OperationRequest2V32), V33(S15OperationRequest2V33), V34(S15OperationRequest2V34), V35(S15OperationRequest2V35), V36(S15OperationRequest2V36), V37(S15OperationRequest2V37), V38(S15OperationRequest2V38), V39(S15OperationRequest2V39) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V0 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11ResolveNativePlacementsRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V1 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeGroupRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V2 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeWyckoffRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V3 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeDimensionDRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V4 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11ApplyNativeDomainParametersRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V5 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeADPAndUncertaintyRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V6 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeMergeRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V7 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeBVRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V8 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativePlaneRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V9 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeShapeRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V10 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S11NativeCameraRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V11 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeFourierUiInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V12 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeGridNoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V13 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeTchWidthInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V14 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativePowderAccumulateInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V15 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8OrderedExpressionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V16 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8MathFSResampleAffineInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V17 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8MathFSLinearCombineFieldsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V18 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8MathFSCartesianLineInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V19 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8MathFSLinearCombineCurvesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V20 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeDispersionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V21 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeScatteringInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V22 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeFourierFieldInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V23 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeModelFieldInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V24 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeVolumeTransformInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V25 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativePeakSearchInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V26 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeVoronoiInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V27 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeMeshInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V28 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeConversionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V29 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativePowderFullInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V30 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeDefaultIsoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V31 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8NativeReflectionReadInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V32 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SamplingNativePointSampleInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V33 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SamplingNativeLineProfileInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V34 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SamplingNativeContourLevelsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V35 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SamplingNativeContourSegmentsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V36 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SamplingNativePolyIntegralInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V37 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SliceNativeObliqueSliceInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V38 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S8SliceNativePeriodicSliceInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationRequest2V39 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="input")] pub f_1: S10RenderNativeIsosurfacePacketRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S15OperationResult2 { V0(S15OperationResult2V0), V1(S15OperationResult2V1), V2(S15OperationResult2V2), V3(S15OperationResult2V3), V4(S15OperationResult2V4), V5(S15OperationResult2V5), V6(S15OperationResult2V6), V7(S15OperationResult2V7), V8(S15OperationResult2V8), V9(S15OperationResult2V9), V10(S15OperationResult2V10), V11(S15OperationResult2V11), V12(S15OperationResult2V12), V13(S15OperationResult2V13), V14(S15OperationResult2V14), V15(S15OperationResult2V15), V16(S15OperationResult2V16), V17(S15OperationResult2V17), V18(S15OperationResult2V18), V19(S15OperationResult2V19), V20(S15OperationResult2V20), V21(S15OperationResult2V21), V22(S15OperationResult2V22), V23(S15OperationResult2V23), V24(S15OperationResult2V24), V25(S15OperationResult2V25), V26(S15OperationResult2V26), V27(S15OperationResult2V27), V28(S15OperationResult2V28), V29(S15OperationResult2V29), V30(S15OperationResult2V30), V31(S15OperationResult2V31), V32(S15OperationResult2V32), V33(S15OperationResult2V33), V34(S15OperationResult2V34), V35(S15OperationResult2V35), V36(S15OperationResult2V36), V37(S15OperationResult2V37), V38(S15OperationResult2V38), V39(S15OperationResult2V39) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V0 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11ResolveNativePlacementsResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V1 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeGroupResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V2 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeWyckoffResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V3 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeDimensionDResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V4 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11ApplyNativeDomainParametersResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V5 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeADPAndUncertaintyResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V6 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeMergeResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V7 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeBVResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V8 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativePlaneResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V9 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeShapeResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V10 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S11NativeCameraResponse }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V11 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativePreparedFourierOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V12 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeGridNoOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V13 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeTchWidthOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V14 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativePowderAccumulateOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V15 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8OrderedExpressionOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V16 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8FS1FSFieldOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V17 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8FS1FSFieldOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V18 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8MathFSCartesianLineOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V19 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8MathFSLinearCombineCurvesOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V20 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeDispersionOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V21 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeScatteringOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V22 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeFullFieldOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V23 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeModelFieldOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V24 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeVolumeTransformOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V25 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativePeakVoronoiOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V26 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativePeakVoronoiOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V27 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeMeshOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V28 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeConversionOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V29 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativePowderFullOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V30 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeDefaultIsoOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V31 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8NativeReflectionReadOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V32 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SamplingNativePointSampleOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V33 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SamplingNativeLineProfileOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V34 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SamplingNativeContourLevelsOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V35 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SamplingNativeContourSegmentsOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V36 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SamplingNativePolyIntegralOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V37 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SliceNativeObliqueSliceOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V38 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S8SliceNativePeriodicSliceOutputOrFailure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15OperationResult2V39 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S10RenderNativeIsosurfacePacketResponse }

pub type S15HistoryAwareCommitSummaryWire2 = S13HistoryAwareCommitSummary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15TypedJsonResource2 { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String,
#[serde(rename="mediaType")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S15ScientificStateEdit2 { V0(S15ScientificStateEdit2V0), V1(S15ScientificStateEdit2V1), V2(S15ScientificStateEdit2V2), V3(S15ScientificStateEdit2V3), V4(S15ScientificStateEdit2V4), V5(S15ScientificStateEdit2V5), V6(S15ScientificStateEdit2V6) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V0 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetDocumentId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11PhasePlacementSet }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V1 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetPhaseId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11NativeDimensionDState }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V2 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetPhaseId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11GroupSettingState }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V3 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetSceneId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11NativeCamera }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V4 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetPhaseId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11ApplyNativeDomainParametersRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V5 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetPhaseId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11NativeShapeRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15ScientificStateEdit2V6 { #[serde(rename="component")] pub f_0: String,
#[serde(rename="targetPhaseId")] pub f_1: String,
#[serde(rename="state")] pub f_2: S11NativePlaneRequest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15PrepareDomainResult2 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="output")] pub f_1: S15TypedJsonResource2,
#[serde(rename="candidateId")] pub f_2: S15PrepareDomainResult2F2,
#[serde(rename="candidateDigest")] pub f_3: S15PrepareDomainResult2F3,
#[serde(rename="complete")] pub f_4: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S15PrepareDomainResult2F2 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S15PrepareDomainResult2F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15PrepareStateResult2 { #[serde(rename="candidateId")] pub f_0: String,
#[serde(rename="candidateDigest")] pub f_1: String,
#[serde(rename="validatedState")] pub f_2: S15TypedJsonResource2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S15AppliedDomainResult2 { #[serde(rename="candidateId")] pub f_0: String,
#[serde(rename="candidateDigest")] pub f_1: String,
#[serde(rename="applied")] pub f_2: bool }

pub type S15Root = serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S13InternalHistoryEffectDecision { V0(S13InternalHistoryEffectDecisionV0), V1(S13InternalHistoryEffectDecisionV1), V2(S13InternalHistoryEffectDecisionV2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S13InternalHistoryEffectDecisionV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="registryRowDigest")] pub f_1: String,
#[serde(rename="producerProfileDigest")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S13InternalHistoryEffectDecisionV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="registryRowDigest")] pub f_1: String,
#[serde(rename="producerProfileDigest")] pub f_2: String,
#[serde(rename="animationSessionId")] pub f_3: String,
#[serde(rename="tickSequence")] pub f_4: String,
#[serde(rename="mutationMaskDigest")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S13InternalHistoryEffectDecisionV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="registryRowDigest")] pub f_1: String,
#[serde(rename="producerProfileDigest")] pub f_2: String,
#[serde(rename="direction")] pub f_3: String,
#[serde(rename="steps")] pub f_4: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S13HistoryAwareCommitSummary { #[serde(rename="documentId")] pub f_0: String,
#[serde(rename="epoch")] pub f_1: String,
#[serde(rename="revision")] pub f_2: String,
#[serde(rename="liveContentDigest")] pub f_3: String,
#[serde(rename="historyGeneration")] pub f_4: String,
#[serde(rename="historyCount")] pub f_5: String,
#[serde(rename="historyCursor")] pub f_6: String,
#[serde(rename="cursorSnapshotDigest")] pub f_7: String,
#[serde(rename="historyEffect")] pub f_8: String,
#[serde(rename="undoAvailable")] pub f_9: bool,
#[serde(rename="redoAvailable")] pub f_10: bool }

pub type S13Root = serde_json::Value;

pub type S11Id = String;

pub type S11SHA256 = String;

pub type S11Float32Word = String;

pub type S11Vec3 = Vec<f64>;

pub type S11Vec3A = Vec<f64>;

pub type S11Vec6A2 = Vec<f64>;

pub type S11Matrix3Rows = Vec<S11Vec3>;

pub type S11Vec4 = Vec<f64>;

pub type S11Matrix4Rows = Vec<S11Vec4>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11Rational { #[serde(rename="numerator")] pub f_0: String,
#[serde(rename="denominator")] pub f_1: String }

pub type S11Rational3 = Vec<S11Rational>;

pub type S11RationalMatrix3 = Vec<S11Rational3>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11SourceSpan { #[serde(rename="resourceSHA256")] pub f_0: S11SHA256,
#[serde(rename="byteOffset")] pub f_1: i64,
#[serde(rename="byteLength")] pub f_2: i64,
#[serde(rename="spanSHA256")] pub f_3: S11SHA256 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11Frame { V0(S11FrameV0), V1(S11FrameV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FrameV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="basisId")] pub f_1: S11Id,
#[serde(rename="basisColumnsInCartesianA")] pub f_2: S11Matrix3Rows }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FrameV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="basisId")] pub f_1: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11WorldPose { #[serde(rename="rotationRows")] pub f_0: S11Matrix3Rows,
#[serde(rename="translationA")] pub f_1: S11Vec3A }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11Reference { V0(S11ReferenceV0), V1(S11ReferenceV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ReferenceV0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ReferenceV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11Anchor { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="value")] pub f_1: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11AxisPair { #[serde(rename="firstKind")] pub f_0: String,
#[serde(rename="first")] pub f_1: S11Vec3,
#[serde(rename="second")] pub f_2: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11OrientationConstraint { V0(S11OrientationConstraintV0), V1(S11OrientationConstraintV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11OrientationConstraintV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="rotationRows")] pub f_1: S11Matrix3Rows }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11OrientationConstraintV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reference")] pub f_1: S11Reference,
#[serde(rename="currentAxes")] pub f_2: S11AxisPair,
#[serde(rename="referenceAxes")] pub f_3: S11AxisPair }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11PositionConstraint { V0(S11PositionConstraintV0), V1(S11PositionConstraintV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PositionConstraintV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="translationA")] pub f_1: S11Vec3A }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PositionConstraintV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reference")] pub f_1: S11Reference,
#[serde(rename="currentAnchor")] pub f_2: S11Anchor,
#[serde(rename="referenceAnchor")] pub f_3: S11Anchor }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeLayerCurrent { #[serde(rename="ltranslSelectorI32")] pub f_0: i64,
#[serde(rename="lorientSelectorI32")] pub f_1: i64,
#[serde(rename="lorientRawFlagsU32")] pub f_2: i64,
#[serde(rename="currentPositionAnchor6")] pub f_3: Vec<f64>,
#[serde(rename="firstOrientationVectors6")] pub f_4: Vec<f64>,
#[serde(rename="secondOrientationVectors6")] pub f_5: Vec<f64>,
#[serde(rename="lmatrixLogicalRows")] pub f_6: S11Matrix4Rows,
#[serde(rename="separateTranslationF32A")] pub f_7: Vec<f64>,
#[serde(rename="source")] pub f_8: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PhasePlacement { #[serde(rename="phaseId")] pub f_0: S11Id,
#[serde(rename="frame")] pub f_1: S11Frame,
#[serde(rename="orientation")] pub f_2: S11OrientationConstraint,
#[serde(rename="position")] pub f_3: S11PositionConstraint,
#[serde(rename="worldPose")] pub f_4: S11WorldPose,
#[serde(rename="nativeCurrent")] #[serde(default, skip_serializing_if = "Option::is_none")] pub f_5: Option<S11NativeLayerCurrent>,
#[serde(rename="nativeSourceObservation")] #[serde(default, skip_serializing_if = "Option::is_none")] pub f_6: Option<S11PhasePlacementF6> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PhasePlacementF6 { #[serde(rename="record")] pub f_0: S11NativeLayerSourceObservation,
#[serde(rename="source")] pub f_1: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PhasePlacementSet { #[serde(rename="evaluationProfile")] pub f_0: String,
#[serde(rename="phases")] pub f_1: Vec<S11PhasePlacement>,
#[serde(rename="nativeSourceOrder")] pub f_2: Vec<S11Id> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11SymmetryOp3 { #[serde(rename="operatorId")] pub f_0: S11Id,
#[serde(rename="fractionalRotationRows")] pub f_1: S11RationalMatrix3,
#[serde(rename="fractionalTranslation")] pub f_2: S11Rational3,
#[serde(rename="timeSign")] pub f_3: f64,
#[serde(rename="basisId")] pub f_4: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11OrdinarySettingKey { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="assetSHA256")] pub f_1: S11SHA256,
#[serde(rename="groupNumber")] pub f_2: i64,
#[serde(rename="settingOrdinal1")] pub f_3: i64,
#[serde(rename="rawHeaderKey")] pub f_4: String,
#[serde(rename="rawHeaderFiveI32")] pub f_5: Vec<i64>,
#[serde(rename="nativeName")] pub f_6: String,
#[serde(rename="source")] pub f_7: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11MagneticSettingKey { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="assetSHA256")] pub f_1: S11SHA256,
#[serde(rename="bnsMajor")] pub f_2: i64,
#[serde(rename="bnsMinor")] pub f_3: i64,
#[serde(rename="bnsText")] pub f_4: String,
#[serde(rename="nativeName")] pub f_5: String,
#[serde(rename="rawHeader")] pub f_6: String,
#[serde(rename="ogToBnsRawExpression")] pub f_7: String,
#[serde(rename="source")] pub f_8: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11GroupSettingState { #[serde(rename="selectedSetting")] pub f_0: S11GroupSettingStateF0,
#[serde(rename="basisId")] pub f_1: S11Id,
#[serde(rename="listedSourceOperatorExpressions")] pub f_2: Vec<String>,
#[serde(rename="qualifiedAffineOps")] pub f_3: Vec<S11SymmetryOp3>,
#[serde(rename="nativeCurrentCellTransformLogicalRows")] pub f_4: S11Matrix4Rows,
#[serde(rename="nativeCurrentOperatorMatrices")] pub f_5: Vec<S11Matrix4Rows>,
#[serde(rename="wyckoffPolicy")] pub f_6: S11GroupSettingStateF6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11GroupSettingStateF0 { V0(S11OrdinarySettingKey), V1(S11MagneticSettingKey) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11GroupSettingStateF6 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="baseFractionalTolerance")] pub f_1: f64,
#[serde(rename="comparison")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeProbability99 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="tableSHA256")] pub f_1: String,
#[serde(rename="percent")] pub f_2: i64,
#[serde(rename="sourceWordLE")] pub f_3: S11Float32Word,
#[serde(rename="radiusScaleF32")] pub f_4: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11MathProbabilityChi3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="profileId")] pub f_1: String,
#[serde(rename="probability")] pub f_2: f64,
#[serde(rename="radiusScale")] pub f_3: f64,
#[serde(rename="absoluteErrorBound")] pub f_4: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11AdpGeometry { V0(S11AdpGeometryV0), V1(S11AdpGeometryV1), V2(S11AdpGeometryV2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11AdpGeometryV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="radiusA")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11AdpGeometryV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="axisDirectionsColumns")] pub f_1: S11Matrix3Rows,
#[serde(rename="semiAxesA")] pub f_2: S11Vec3A }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11AdpGeometryV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="axisDirectionsColumns")] pub f_1: S11Matrix3Rows,
#[serde(rename="halfAxesA")] pub f_2: S11Vec3A,
#[serde(rename="nonPositiveEigenAxes")] pub f_3: Vec<bool>,
#[serde(rename="unitVertices")] pub f_4: S11AdpGeometryV2F4 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct S11AdpGeometryV2F4 {  }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11AdpRenderPlan { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="currentNativeScalar")] pub f_1: f64,
#[serde(rename="nativeScalarKind")] pub f_2: String,
#[serde(rename="currentNativeTensor6")] pub f_3: Vec<f64>,
#[serde(rename="nativeTensorKind")] pub f_4: String,
#[serde(rename="canonicalCartesianU6A2")] pub f_5: S11Vec6A2,
#[serde(rename="probability")] pub f_6: S11AdpRenderPlanF6,
#[serde(rename="nativeZero6Policy")] pub f_7: String,
#[serde(rename="sourceExactSixZero")] pub f_8: bool,
#[serde(rename="eigenvaluesA2")] pub f_9: Vec<f64>,
#[serde(rename="nativeMarkerColumn3")] pub f_10: Vec<f64>,
#[serde(rename="nativeThermalMatrixLogicalRows")] pub f_11: S11Matrix4Rows,
#[serde(rename="geometry")] pub f_12: S11AdpGeometry,
#[serde(rename="nativeSiteFlagsU16")] pub f_13: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11AdpRenderPlanF6 { V0(S11NativeProbability99), V1(S11MathProbabilityChi3) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11MergeSiteInput { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="nativeSpeciesToken")] pub f_1: String,
#[serde(rename="coordinate")] pub f_2: S11Vec3,
#[serde(rename="coordinateSU")] pub f_3: S11Vec3,
#[serde(rename="sourceMetadataEntityId")] pub f_4: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11MergeGroupResult { #[serde(rename="representativeSiteId")] pub f_0: S11Id,
#[serde(rename="matchedSiteIds")] pub f_1: Vec<S11Id>,
#[serde(rename="matchedImageCount")] pub f_2: i64,
#[serde(rename="meanCoordinate")] pub f_3: S11Vec3,
#[serde(rename="legacyArithmeticMeanCoordinateSU")] pub f_4: S11Vec3,
#[serde(rename="retainedMetadataFromSiteId")] pub f_5: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PublicBondRule { #[serde(rename="ruleId")] pub f_0: S11Id,
#[serde(rename="searchMode")] pub f_1: String,
#[serde(rename="selectorKind")] pub f_2: String,
#[serde(rename="selectorA")] pub f_3: String,
#[serde(rename="selectorB")] pub f_4: String,
#[serde(rename="minimumA")] pub f_5: f64,
#[serde(rename="maximumA")] pub f_6: f64,
#[serde(rename="boundaryMode")] pub f_7: String,
#[serde(rename="showPolyhedra")] pub f_8: bool,
#[serde(rename="nativeRuleSourceId")] pub f_9: S11Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11MergePlan { #[serde(rename="phaseId")] pub f_0: S11Id,
#[serde(rename="frame")] pub f_1: S11Frame,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="thresholdA")] pub f_3: f64,
#[serde(rename="sourceOrder")] pub f_4: Vec<S11Id>,
#[serde(rename="sites")] pub f_5: Vec<S11MergeSiteInput>,
#[serde(rename="symmetryOps")] pub f_6: Vec<S11SymmetryOp3>,
#[serde(rename="groups")] pub f_7: Vec<S11MergeGroupResult>,
#[serde(rename="bondRules")] pub f_8: Vec<S11PublicBondRule> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PlaneFitInput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="pointsCartesianA")] pub f_1: Vec<S11Vec3A>,
#[serde(rename="inputNormalForNativeSmallN")] pub f_2: S11Vec3,
#[serde(rename="sourceSelectionIds")] pub f_3: Vec<S11Id> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PlaneFitResolved { #[serde(rename="status")] pub f_0: String,
#[serde(rename="sourceReturnCode")] pub f_1: i64,
#[serde(rename="pointCount")] pub f_2: i64,
#[serde(rename="sourceHomogeneous4")] pub f_3: S11Vec4,
#[serde(rename="unitNormalCartesian")] pub f_4: S11Vec3,
#[serde(rename="offsetA")] pub f_5: f64,
#[serde(rename="signPolicy")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PlaneFitUnderdetermined { #[serde(rename="status")] pub f_0: String,
#[serde(rename="sourceReturnCode")] pub f_1: i64,
#[serde(rename="pointCount")] pub f_2: i64,
#[serde(rename="affineRank")] pub f_3: i64,
#[serde(rename="reason")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11PlaneFit { #[serde(rename="input")] pub f_0: S11PlaneFitInput,
#[serde(rename="result")] pub f_1: S11PlaneFitF1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11PlaneFitF1 { V0(S11PlaneFitResolved), V1(S11PlaneFitUnderdetermined) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeCamera { #[serde(rename="sceneMatrixLogicalRows")] pub f_0: S11Matrix4Rows,
#[serde(rename="shiftViewX")] pub f_1: f64,
#[serde(rename="shiftViewY")] pub f_2: f64,
#[serde(rename="thirdSceneStoredF32")] pub f_3: f64,
#[serde(rename="zoomF32")] pub f_4: f64,
#[serde(rename="projectionSelectorBit21")] pub f_5: f64,
#[serde(rename="projectionZScaleF32")] pub f_6: f64,
#[serde(rename="localTranslationA")] pub f_7: Vec<f64>,
#[serde(rename="normalizationF64")] pub f_8: f64,
#[serde(rename="viewportLogicalWidth")] pub f_9: i64,
#[serde(rename="viewportLogicalHeight")] pub f_10: i64,
#[serde(rename="deviceScaleF64")] pub f_11: f64,
#[serde(rename="aspectF32")] pub f_12: f64,
#[serde(rename="aspectScaleF32")] pub f_13: f64,
#[serde(rename="projectionContext")] pub f_14: String,
#[serde(rename="nearView")] pub f_15: f64,
#[serde(rename="farView")] pub f_16: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FC1Entry { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="id")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FC1SourceScalar { #[serde(rename="literal")] pub f_0: String,
#[serde(rename="nativeF64Bits")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11FC1Reference { V0(S11FC1ReferenceV0), V1(S11FC1ReferenceV1), V2(S11FC1ReferenceV2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FC1ReferenceV0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FC1ReferenceV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="entry")] pub f_1: S11FC1Entry }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11FC1ReferenceV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="rawSelector")] pub f_1: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeLayerSourceObservation { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="owner")] pub f_1: S11FC1Entry,
#[serde(rename="position")] pub f_2: S11NativeLayerSourceObservationF2,
#[serde(rename="orientation")] pub f_3: S11NativeLayerSourceObservationF3,
#[serde(rename="sourceProjection")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeLayerSourceObservationF2 { #[serde(rename="rawSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S11FC1Reference,
#[serde(rename="currentAnchor")] pub f_2: Vec<S11FC1SourceScalar>,
#[serde(rename="referenceAnchor")] pub f_3: Vec<S11FC1SourceScalar> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeLayerSourceObservationF3 { #[serde(rename="rawSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S11FC1Reference,
#[serde(rename="currentFirst")] pub f_2: Vec<S11FC1SourceScalar>,
#[serde(rename="referenceFirst")] pub f_3: Vec<S11FC1SourceScalar>,
#[serde(rename="currentSecond")] pub f_4: Vec<S11FC1SourceScalar>,
#[serde(rename="referenceSecond")] pub f_5: Vec<S11FC1SourceScalar>,
#[serde(rename="rawFlags")] pub f_6: Vec<i64> }

pub type S11Vec6 = Vec<f64>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeOccupationDomainDefinition { #[serde(rename="domainId")] pub f_0: S11Id,
#[serde(rename="sourceShapeOrdinal")] pub f_1: i64,
#[serde(rename="nativeCoordinateWidth")] pub f_2: f64,
#[serde(rename="verticesNative6")] pub f_3: Vec<S11Vec6>,
#[serde(rename="nativeNthIndices0based")] pub f_4: Vec<i64>,
#[serde(rename="nativeVertexCount")] pub f_5: i64,
#[serde(rename="nativeNthCount")] pub f_6: i64,
#[serde(rename="externalComponentNames")] pub f_7: S11NativeOccupationDomainDefinitionF7,
#[serde(rename="externalDisplacementLengthsA")] pub f_8: Vec<f64>,
#[serde(rename="externalDirectionRowsNative6")] pub f_9: Vec<S11Vec6>,
#[serde(rename="internalShiftDirectionNative6")] pub f_10: S11Vec6,
#[serde(rename="internalShiftScale")] pub f_11: f64,
#[serde(rename="currentDerivedTranslationNative6")] pub f_12: S11Vec6,
#[serde(rename="externalComponentSU")] pub f_13: Vec<f64>,
#[serde(rename="externalComponentStep")] pub f_14: Vec<f64>,
#[serde(rename="nativeParameterFlagsI32")] pub f_15: i64,
#[serde(rename="source")] pub f_16: S11SourceSpan,
#[serde(rename="nativeSymmetryOperatorOrdinal0based")] pub f_17: i64,
#[serde(rename="nativeDomainScale")] pub f_18: f64,
#[serde(rename="currentMaximumInternalRadiusSquared")] pub f_19: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct S11NativeOccupationDomainDefinitionF7 {  }

pub type S11VectorD3 = Vec<f64>;

pub type S11MatrixD3Rows = Vec<S11VectorD3>;

pub type S11MatrixD4HomogeneousRows = Vec<Vec<f64>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSiteD3 { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="nativeSpeciesToken")] pub f_1: String,
#[serde(rename="nativeElementIndexI16")] pub f_2: i64,
#[serde(rename="fractionalCoordinatesD")] pub f_3: S11VectorD3,
#[serde(rename="occupancySlotD")] pub f_4: f64,
#[serde(rename="nativeScalarSlotD1")] pub f_5: f64,
#[serde(rename="thermal6SlotsD2ToD7")] pub f_6: Vec<f64>,
#[serde(rename="coordinateSUD")] pub f_7: S11VectorD3,
#[serde(rename="nativeSiteArrayBlocks3ByDPlus8")] pub f_8: Vec<Vec<f64>>,
#[serde(rename="nativeFlagsU16")] pub f_9: i64,
#[serde(rename="occupationDomainId")] pub f_10: S11NativeSiteD3F10,
#[serde(rename="nativeOrbitNumeratorI16")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeSiteD3F10 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSymmetryOpD3 { #[serde(rename="operatorId")] pub f_0: S11Id,
#[serde(rename="currentNativeMatrixRows")] pub f_1: S11MatrixD4HomogeneousRows,
#[serde(rename="timeSign")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionD3State { #[serde(rename="modelId")] pub f_0: S11Id,
#[serde(rename="dimensionD")] pub f_1: f64,
#[serde(rename="nativeMatrixDimensionWord")] pub f_2: f64,
#[serde(rename="nativeModelKind")] pub f_3: String,
#[serde(rename="sourceGroupNumberI32")] pub f_4: i64,
#[serde(rename="sourceSettingOrdinalI32")] pub f_5: i64,
#[serde(rename="crystallographicToOrthoRowsD")] pub f_6: S11MatrixD3Rows,
#[serde(rename="orthoToCrystallographicRowsD")] pub f_7: S11MatrixD3Rows,
#[serde(rename="phasonRowsD")] pub f_8: S11MatrixD3Rows,
#[serde(rename="layerWorldMatrix4Rows")] pub f_9: S11Matrix4Rows,
#[serde(rename="currentC2XYZRowsDp1")] pub f_10: S11MatrixD4HomogeneousRows,
#[serde(rename="currentXYZ2CRowsDp1")] pub f_11: S11MatrixD4HomogeneousRows,
#[serde(rename="nativeAuxInverse1A0RowsDp1")] pub f_12: S11MatrixD4HomogeneousRows,
#[serde(rename="qcorigNative6")] pub f_13: S11Vec6,
#[serde(rename="nativeCIFWaveVectorRows3")] pub f_14: Vec<S11Vec3>,
#[serde(rename="drawingBoundsMinMaxD")] pub f_15: Vec<Vec<f64>>,
#[serde(rename="symmetryOpsD")] pub f_16: Vec<S11NativeSymmetryOpD3>,
#[serde(rename="sites")] pub f_17: Vec<S11NativeSiteD3>,
#[serde(rename="occupationDomains")] pub f_18: Vec<S11NativeOccupationDomainDefinition>,
#[serde(rename="nativeSourceOrder")] pub f_19: Vec<S11Id>,
#[serde(rename="source")] pub f_20: S11SourceSpan,
#[serde(rename="occupationDomainTemplates")] pub f_21: Vec<S11NativeOccupationDomainTemplate> }

pub type S11VectorD4 = Vec<f64>;

pub type S11MatrixD4Rows = Vec<S11VectorD4>;

pub type S11MatrixD5HomogeneousRows = Vec<Vec<f64>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSiteD4 { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="nativeSpeciesToken")] pub f_1: String,
#[serde(rename="nativeElementIndexI16")] pub f_2: i64,
#[serde(rename="fractionalCoordinatesD")] pub f_3: S11VectorD4,
#[serde(rename="occupancySlotD")] pub f_4: f64,
#[serde(rename="nativeScalarSlotD1")] pub f_5: f64,
#[serde(rename="thermal6SlotsD2ToD7")] pub f_6: Vec<f64>,
#[serde(rename="coordinateSUD")] pub f_7: S11VectorD4,
#[serde(rename="nativeSiteArrayBlocks3ByDPlus8")] pub f_8: Vec<Vec<f64>>,
#[serde(rename="nativeFlagsU16")] pub f_9: i64,
#[serde(rename="occupationDomainId")] pub f_10: S11NativeSiteD4F10,
#[serde(rename="nativeOrbitNumeratorI16")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeSiteD4F10 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSymmetryOpD4 { #[serde(rename="operatorId")] pub f_0: S11Id,
#[serde(rename="currentNativeMatrixRows")] pub f_1: S11MatrixD5HomogeneousRows,
#[serde(rename="timeSign")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionD4State { #[serde(rename="modelId")] pub f_0: S11Id,
#[serde(rename="dimensionD")] pub f_1: f64,
#[serde(rename="nativeMatrixDimensionWord")] pub f_2: f64,
#[serde(rename="nativeModelKind")] pub f_3: String,
#[serde(rename="sourceGroupNumberI32")] pub f_4: i64,
#[serde(rename="sourceSettingOrdinalI32")] pub f_5: i64,
#[serde(rename="crystallographicToOrthoRowsD")] pub f_6: S11MatrixD4Rows,
#[serde(rename="orthoToCrystallographicRowsD")] pub f_7: S11MatrixD4Rows,
#[serde(rename="phasonRowsD")] pub f_8: S11MatrixD4Rows,
#[serde(rename="layerWorldMatrix4Rows")] pub f_9: S11Matrix4Rows,
#[serde(rename="currentC2XYZRowsDp1")] pub f_10: S11MatrixD5HomogeneousRows,
#[serde(rename="currentXYZ2CRowsDp1")] pub f_11: S11MatrixD5HomogeneousRows,
#[serde(rename="nativeAuxInverse1A0RowsDp1")] pub f_12: S11MatrixD5HomogeneousRows,
#[serde(rename="qcorigNative6")] pub f_13: S11Vec6,
#[serde(rename="nativeCIFWaveVectorRows3")] pub f_14: Vec<S11Vec3>,
#[serde(rename="drawingBoundsMinMaxD")] pub f_15: Vec<Vec<f64>>,
#[serde(rename="symmetryOpsD")] pub f_16: Vec<S11NativeSymmetryOpD4>,
#[serde(rename="sites")] pub f_17: Vec<S11NativeSiteD4>,
#[serde(rename="occupationDomains")] pub f_18: Vec<S11NativeOccupationDomainDefinition>,
#[serde(rename="nativeSourceOrder")] pub f_19: Vec<S11Id>,
#[serde(rename="source")] pub f_20: S11SourceSpan,
#[serde(rename="occupationDomainTemplates")] pub f_21: Vec<S11NativeOccupationDomainTemplate> }

pub type S11VectorD5 = Vec<f64>;

pub type S11MatrixD5Rows = Vec<S11VectorD5>;

pub type S11MatrixD6HomogeneousRows = Vec<Vec<f64>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSiteD5 { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="nativeSpeciesToken")] pub f_1: String,
#[serde(rename="nativeElementIndexI16")] pub f_2: i64,
#[serde(rename="fractionalCoordinatesD")] pub f_3: S11VectorD5,
#[serde(rename="occupancySlotD")] pub f_4: f64,
#[serde(rename="nativeScalarSlotD1")] pub f_5: f64,
#[serde(rename="thermal6SlotsD2ToD7")] pub f_6: Vec<f64>,
#[serde(rename="coordinateSUD")] pub f_7: S11VectorD5,
#[serde(rename="nativeSiteArrayBlocks3ByDPlus8")] pub f_8: Vec<Vec<f64>>,
#[serde(rename="nativeFlagsU16")] pub f_9: i64,
#[serde(rename="occupationDomainId")] pub f_10: S11NativeSiteD5F10,
#[serde(rename="nativeOrbitNumeratorI16")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeSiteD5F10 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSymmetryOpD5 { #[serde(rename="operatorId")] pub f_0: S11Id,
#[serde(rename="currentNativeMatrixRows")] pub f_1: S11MatrixD6HomogeneousRows,
#[serde(rename="timeSign")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionD5State { #[serde(rename="modelId")] pub f_0: S11Id,
#[serde(rename="dimensionD")] pub f_1: f64,
#[serde(rename="nativeMatrixDimensionWord")] pub f_2: f64,
#[serde(rename="nativeModelKind")] pub f_3: String,
#[serde(rename="sourceGroupNumberI32")] pub f_4: i64,
#[serde(rename="sourceSettingOrdinalI32")] pub f_5: i64,
#[serde(rename="crystallographicToOrthoRowsD")] pub f_6: S11MatrixD5Rows,
#[serde(rename="orthoToCrystallographicRowsD")] pub f_7: S11MatrixD5Rows,
#[serde(rename="phasonRowsD")] pub f_8: S11MatrixD5Rows,
#[serde(rename="layerWorldMatrix4Rows")] pub f_9: S11Matrix4Rows,
#[serde(rename="currentC2XYZRowsDp1")] pub f_10: S11MatrixD6HomogeneousRows,
#[serde(rename="currentXYZ2CRowsDp1")] pub f_11: S11MatrixD6HomogeneousRows,
#[serde(rename="nativeAuxInverse1A0RowsDp1")] pub f_12: S11MatrixD6HomogeneousRows,
#[serde(rename="qcorigNative6")] pub f_13: S11Vec6,
#[serde(rename="nativeCIFWaveVectorRows3")] pub f_14: Vec<S11Vec3>,
#[serde(rename="drawingBoundsMinMaxD")] pub f_15: Vec<Vec<f64>>,
#[serde(rename="symmetryOpsD")] pub f_16: Vec<S11NativeSymmetryOpD5>,
#[serde(rename="sites")] pub f_17: Vec<S11NativeSiteD5>,
#[serde(rename="occupationDomains")] pub f_18: Vec<S11NativeOccupationDomainDefinition>,
#[serde(rename="nativeSourceOrder")] pub f_19: Vec<S11Id>,
#[serde(rename="source")] pub f_20: S11SourceSpan,
#[serde(rename="occupationDomainTemplates")] pub f_21: Vec<S11NativeOccupationDomainTemplate> }

pub type S11VectorD6 = Vec<f64>;

pub type S11MatrixD6Rows = Vec<S11VectorD6>;

pub type S11MatrixD7HomogeneousRows = Vec<Vec<f64>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSiteD6 { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="nativeSpeciesToken")] pub f_1: String,
#[serde(rename="nativeElementIndexI16")] pub f_2: i64,
#[serde(rename="fractionalCoordinatesD")] pub f_3: S11VectorD6,
#[serde(rename="occupancySlotD")] pub f_4: f64,
#[serde(rename="nativeScalarSlotD1")] pub f_5: f64,
#[serde(rename="thermal6SlotsD2ToD7")] pub f_6: Vec<f64>,
#[serde(rename="coordinateSUD")] pub f_7: S11VectorD6,
#[serde(rename="nativeSiteArrayBlocks3ByDPlus8")] pub f_8: Vec<Vec<f64>>,
#[serde(rename="nativeFlagsU16")] pub f_9: i64,
#[serde(rename="occupationDomainId")] pub f_10: S11NativeSiteD6F10,
#[serde(rename="nativeOrbitNumeratorI16")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeSiteD6F10 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeSymmetryOpD6 { #[serde(rename="operatorId")] pub f_0: S11Id,
#[serde(rename="currentNativeMatrixRows")] pub f_1: S11MatrixD7HomogeneousRows,
#[serde(rename="timeSign")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionD6State { #[serde(rename="modelId")] pub f_0: S11Id,
#[serde(rename="dimensionD")] pub f_1: f64,
#[serde(rename="nativeMatrixDimensionWord")] pub f_2: f64,
#[serde(rename="nativeModelKind")] pub f_3: String,
#[serde(rename="sourceGroupNumberI32")] pub f_4: i64,
#[serde(rename="sourceSettingOrdinalI32")] pub f_5: i64,
#[serde(rename="crystallographicToOrthoRowsD")] pub f_6: S11MatrixD6Rows,
#[serde(rename="orthoToCrystallographicRowsD")] pub f_7: S11MatrixD6Rows,
#[serde(rename="phasonRowsD")] pub f_8: S11MatrixD6Rows,
#[serde(rename="layerWorldMatrix4Rows")] pub f_9: S11Matrix4Rows,
#[serde(rename="currentC2XYZRowsDp1")] pub f_10: S11MatrixD7HomogeneousRows,
#[serde(rename="currentXYZ2CRowsDp1")] pub f_11: S11MatrixD7HomogeneousRows,
#[serde(rename="nativeAuxInverse1A0RowsDp1")] pub f_12: S11MatrixD7HomogeneousRows,
#[serde(rename="qcorigNative6")] pub f_13: S11Vec6,
#[serde(rename="nativeCIFWaveVectorRows3")] pub f_14: Vec<S11Vec3>,
#[serde(rename="drawingBoundsMinMaxD")] pub f_15: Vec<Vec<f64>>,
#[serde(rename="symmetryOpsD")] pub f_16: Vec<S11NativeSymmetryOpD6>,
#[serde(rename="sites")] pub f_17: Vec<S11NativeSiteD6>,
#[serde(rename="occupationDomains")] pub f_18: Vec<S11NativeOccupationDomainDefinition>,
#[serde(rename="nativeSourceOrder")] pub f_19: Vec<S11Id>,
#[serde(rename="source")] pub f_20: S11SourceSpan,
#[serde(rename="occupationDomainTemplates")] pub f_21: Vec<S11NativeOccupationDomainTemplate> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeDimensionDState { V0(S11NativeDimensionD3State), V1(S11NativeDimensionD4State), V2(S11NativeDimensionD5State), V3(S11NativeDimensionD6State) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ProjectedDImage { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="operatorId")] pub f_1: S11Id,
#[serde(rename="translationIntegerD")] pub f_2: Vec<i64>,
#[serde(rename="fractionalImageD")] pub f_3: Vec<f64>,
#[serde(rename="nativeXYZImageD")] pub f_4: Vec<f64>,
#[serde(rename="physicalCartesianA")] pub f_5: S11Vec3A,
#[serde(rename="insideDrawingBoundary")] pub f_6: bool,
#[serde(rename="occupationDomainId")] pub f_7: S11ProjectedDImageF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11ProjectedDImageF7 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11DomainDisplayMesh { #[serde(rename="domainId")] pub f_0: S11Id,
#[serde(rename="siteId")] pub f_1: S11Id,
#[serde(rename="operatorId")] pub f_2: S11Id,
#[serde(rename="verticesCartesianA")] pub f_3: Vec<S11Vec3A>,
#[serde(rename="facesVertexIndices")] pub f_4: Vec<Vec<i64>>,
#[serde(rename="sourceVertexOrbits")] pub f_5: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionDRequest { #[serde(rename="state")] pub f_0: S11NativeDimensionDState,
#[serde(rename="action")] pub f_1: String,
#[serde(rename="maximumImages")] pub f_2: i64,
#[serde(rename="maximumDomainVertices")] pub f_3: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeDimensionDResponse { #[serde(rename="currentState")] pub f_0: S11NativeDimensionDState,
#[serde(rename="images")] pub f_1: Vec<S11ProjectedDImage>,
#[serde(rename="domainMeshes")] pub f_2: Vec<S11DomainDisplayMesh>,
#[serde(rename="completion")] pub f_3: String,
#[serde(rename="remainingFrontierCount")] pub f_4: i64,
#[serde(rename="diagnostics")] pub f_5: Vec<String> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11Diagnostic { #[serde(rename="code")] pub f_0: String,
#[serde(rename="entityId")] pub f_1: S11DiagnosticF1,
#[serde(rename="detail")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11DiagnosticF1 { V0(S11Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ResolveNativePlacementsRequest { #[serde(rename="state")] pub f_0: S11PhasePlacementSet }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ResolveNativePlacementsResponse { #[serde(rename="resolved")] pub f_0: S11PhasePlacementSet,
#[serde(rename="diagnostics")] pub f_1: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeGroupRequest { #[serde(rename="state")] pub f_0: S11GroupSettingState,
#[serde(rename="sitesFractional3")] pub f_1: Vec<S11NativeGroupRequestF1Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeGroupRequestF1Item { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="position")] pub f_1: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeGroupResponse { #[serde(rename="state")] pub f_0: S11GroupSettingState,
#[serde(rename="expanded")] pub f_1: Vec<S11NativeGroupResponseF1Item>,
#[serde(rename="diagnostics")] pub f_2: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeGroupResponseF1Item { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="operatorId")] pub f_1: S11Id,
#[serde(rename="fractionalImageModulo1")] pub f_2: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeWyckoffRequest { #[serde(rename="state")] pub f_0: S11GroupSettingState,
#[serde(rename="siteId")] pub f_1: S11Id,
#[serde(rename="positionFractional")] pub f_2: S11Vec3,
#[serde(rename="coordinateSUFractional")] pub f_3: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeWyckoffResponse { #[serde(rename="siteId")] pub f_0: S11Id,
#[serde(rename="assignment")] pub f_1: S11NativeWyckoffResponseF1,
#[serde(rename="diagnostics")] pub f_2: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeWyckoffResponseF1 { V0(S11NativeWyckoffResponseF1V0), V1(S11NativeWyckoffResponseF1V1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeWyckoffResponseF1V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="multiplicity")] pub f_1: i64,
#[serde(rename="letter")] pub f_2: String,
#[serde(rename="sourceRow")] pub f_3: S11SourceSpan,
#[serde(rename="effectiveToleranceFractional")] pub f_4: S11Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeWyckoffResponseF1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11CovarianceParameter { #[serde(rename="parameterId")] pub f_0: String,
#[serde(rename="unit")] pub f_1: String,
#[serde(rename="centralValue")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11UncertaintyModel { #[serde(rename="authority")] pub f_0: String,
#[serde(rename="parametersInOrder")] pub f_1: Vec<S11CovarianceParameter>,
#[serde(rename="covarianceRows")] pub f_2: Vec<Vec<f64>>,
#[serde(rename="independentParameterIds")] pub f_3: Vec<String>,
#[serde(rename="fullParameterByIndependentJacobianRows")] pub f_4: Vec<Vec<f64>>,
#[serde(rename="source")] pub f_5: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeMeasurement { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="selectedCartesianPointsA")] pub f_1: Vec<S11Vec3A>,
#[serde(rename="selectedImageIds")] pub f_2: Vec<S11Id>,
#[serde(rename="parameterToSelectedCartesianJacobianRows")] pub f_3: Vec<Vec<f64>> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeADPAndUncertaintyRequest { #[serde(rename="adpPlan")] pub f_0: S11AdpRenderPlan,
#[serde(rename="measurement")] pub f_1: S11NativeMeasurement,
#[serde(rename="uncertainty")] pub f_2: S11UncertaintyModel }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeADPAndUncertaintyResponse { #[serde(rename="adpPlan")] pub f_0: S11AdpRenderPlan,
#[serde(rename="measurementValue")] pub f_1: S11NativeADPAndUncertaintyResponseF1,
#[serde(rename="measurementUnit")] pub f_2: String,
#[serde(rename="uncertaintyResult")] pub f_3: S11NativeADPAndUncertaintyResponseF3,
#[serde(rename="diagnostics")] pub f_4: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeADPAndUncertaintyResponseF1 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeADPAndUncertaintyResponseF3 { V0(S11NativeADPAndUncertaintyResponseF3V0), V1(S11NativeADPAndUncertaintyResponseF3V1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeADPAndUncertaintyResponseF3V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="jacobianInParameterOrder")] pub f_1: Vec<f64>,
#[serde(rename="variance")] pub f_2: f64,
#[serde(rename="standardUncertainty")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeADPAndUncertaintyResponseF3V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeMergeRequest { #[serde(rename="phaseId")] pub f_0: S11Id,
#[serde(rename="frame")] pub f_1: S11Frame,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="thresholdA")] pub f_3: f64,
#[serde(rename="sourceOrder")] pub f_4: Vec<S11Id>,
#[serde(rename="sites")] pub f_5: Vec<S11MergeSiteInput>,
#[serde(rename="symmetryOps")] pub f_6: Vec<S11SymmetryOp3>,
#[serde(rename="bondRules")] pub f_7: Vec<S11PublicBondRule> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeMergeResponse { #[serde(rename="groups")] pub f_0: Vec<S11MergeGroupResult>,
#[serde(rename="mutatesSites")] pub f_1: bool,
#[serde(rename="diagnostics")] pub f_2: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11BondValenceRow { #[serde(rename="rowKey")] pub f_0: String,
#[serde(rename="assetSHA256")] pub f_1: S11SHA256,
#[serde(rename="speciesA")] pub f_2: String,
#[serde(rename="speciesB")] pub f_3: String,
#[serde(rename="oxidationA")] pub f_4: i64,
#[serde(rename="oxidationB")] pub f_5: i64,
#[serde(rename="r0A")] pub f_6: f64,
#[serde(rename="bA")] pub f_7: f64,
#[serde(rename="source")] pub f_8: S11SourceSpan }

pub type S11NativePlaneRequest = S11PlaneFitInput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativePlaneResponse { #[serde(rename="result")] pub f_0: S11NativePlaneResponseF0,
#[serde(rename="diagnostics")] pub f_1: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativePlaneResponseF0 { V0(S11PlaneFitResolved), V1(S11PlaneFitUnderdetermined) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11Halfspace3 { #[serde(rename="halfspaceId")] pub f_0: S11Id,
#[serde(rename="normalInverseA")] pub f_1: S11Vec3,
#[serde(rename="offsetDimensionless")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ExplicitMesh3 { #[serde(rename="verticesCartesianA")] pub f_0: Vec<S11Vec3A>,
#[serde(rename="facesVertexIndices")] pub f_1: Vec<Vec<i64>> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeShapeRequest { V0(S11NativeShapeRequestV0), V1(S11NativeShapeRequestV1), V2(S11NativeShapeRequestV2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeShapeRequestV0 { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="halfspaces")] pub f_1: Vec<S11Halfspace3> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeShapeRequestV1 { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="mesh")] pub f_1: S11ExplicitMesh3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeShapeRequestV2 { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="neighborImageIds")] pub f_1: Vec<S11Id>,
#[serde(rename="neighborCartesianA")] pub f_2: Vec<S11Vec3A> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeShapeResponse { #[serde(rename="classification")] pub f_0: String,
#[serde(rename="affineDimension")] pub f_1: i64,
#[serde(rename="verticesCartesianA")] pub f_2: Vec<S11Vec3A>,
#[serde(rename="exactVerticesA")] pub f_3: Vec<S11Rational3>,
#[serde(rename="facesVertexIndices")] pub f_4: Vec<Vec<i64>>,
#[serde(rename="farkasMultipliers")] pub f_5: Vec<S11Rational>,
#[serde(rename="feasiblePointA")] pub f_6: S11NativeShapeResponseF6,
#[serde(rename="recessionDirections")] pub f_7: Vec<S11Rational3>,
#[serde(rename="diagnostics")] pub f_8: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeShapeResponseF6 { V0(S11Rational3), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeCameraRequest { #[serde(rename="state")] pub f_0: S11NativeCamera,
#[serde(rename="pointsCartesianA")] pub f_1: Vec<S11Vec3A> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeCameraResponse { #[serde(rename="state")] pub f_0: S11NativeCamera,
#[serde(rename="modelviewRows")] pub f_1: S11Matrix4Rows,
#[serde(rename="projectedNDC")] pub f_2: Vec<S11Vec3>,
#[serde(rename="physicalViewportPixels")] pub f_3: Vec<i64>,
#[serde(rename="diagnostics")] pub f_4: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ApplyNativeDomainParametersRequest { #[serde(rename="state")] pub f_0: S11NativeDimensionDState,
#[serde(rename="domainId")] pub f_1: S11Id,
#[serde(rename="xeA")] pub f_2: Vec<f64>,
#[serde(rename="parameterFlagsI32")] pub f_3: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11ApplyNativeDomainParametersResponse { #[serde(rename="state")] pub f_0: S11NativeDimensionDState,
#[serde(rename="domainId")] pub f_1: S11Id,
#[serde(rename="derivedDomainTranslationNative6")] pub f_2: S11Vec6,
#[serde(rename="diagnostics")] pub f_3: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeOccupationDomainTemplate { #[serde(rename="templateOrdinal0based")] pub f_0: i64,
#[serde(rename="coordinateWidth")] pub f_1: f64,
#[serde(rename="verticesNative6")] pub f_2: Vec<S11Vec6>,
#[serde(rename="nativeNthVertexIndices0based")] pub f_3: Vec<i64>,
#[serde(rename="source")] pub f_4: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeTableBVRequest { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="candidateRows")] pub f_1: Vec<S11BondValenceRow>,
#[serde(rename="selectedRowKeys")] pub f_2: Vec<String>,
#[serde(rename="distanceA")] pub f_3: f64,
#[serde(rename="nativeCutoffDenominator")] pub f_4: i64,
#[serde(rename="denominatorSource")] pub f_5: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativePolyhedronBVSRequest { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="centerSiteId")] pub f_1: S11Id,
#[serde(rename="centerOccupancyF64")] pub f_2: f64,
#[serde(rename="userR0F32")] pub f_3: f64,
#[serde(rename="sourceOutputFlagsU32")] pub f_4: i64,
#[serde(rename="neighborsInSourceOrder")] pub f_5: Vec<S11NativePolyhedronBVSRequestF5Item>,
#[serde(rename="source")] pub f_6: S11SourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativePolyhedronBVSRequestF5Item { #[serde(rename="imageId")] pub f_0: S11Id,
#[serde(rename="neighborSiteId")] pub f_1: S11Id,
#[serde(rename="distanceA")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeBVRequest { V0(S11NativeTableBVRequest), V1(S11NativePolyhedronBVSRequest) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativeTableBVResponse { #[serde(rename="selectedRows")] pub f_0: Vec<S11BondValenceRow>,
#[serde(rename="terms")] pub f_1: Vec<f64>,
#[serde(rename="sum")] pub f_2: f64,
#[serde(rename="nativeAutoCutoffA")] pub f_3: S11NativeTableBVResponseF3,
#[serde(rename="diagnostics")] pub f_4: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeTableBVResponseF3 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativePolyhedronBVSResponse { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="centerSiteId")] pub f_1: S11Id,
#[serde(rename="bF64")] pub f_2: f64,
#[serde(rename="neighborsInSourceOrder")] pub f_3: Vec<S11NativePolyhedronBVSResponseF3Item>,
#[serde(rename="centerOccupancyWeightedSum")] pub f_4: f64,
#[serde(rename="diagnostics")] pub f_5: Vec<S11Diagnostic> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11NativePolyhedronBVSResponseF3Item { #[serde(rename="imageId")] pub f_0: S11Id,
#[serde(rename="neighborSiteId")] pub f_1: S11Id,
#[serde(rename="distanceA")] pub f_2: f64,
#[serde(rename="positiveTerm")] pub f_3: f64,
#[serde(rename="printedNegativeTerm")] pub f_4: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11NativeBVResponse { V0(S11NativeTableBVResponse), V1(S11NativePolyhedronBVSResponse) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S11Root { V0(S11RootV0), V1(S11RootV1), V2(S11RootV2), V3(S11RootV3), V4(S11RootV4), V5(S11RootV5), V6(S11RootV6) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV0 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11PhasePlacementSet }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV1 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11GroupSettingState }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV2 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11AdpRenderPlan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV3 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11MergePlan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV4 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11PlaneFit }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV5 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11NativeCamera }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S11RootV6 { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="component")] pub f_1: String,
#[serde(rename="ownerId")] pub f_2: S11Id,
#[serde(rename="revision")] pub f_3: i64,
#[serde(rename="payload")] pub f_4: S11NativeDimensionDState }

pub type S8FS1Id = S0Id;

pub type S8FS1Digest = S0Digest;

pub type S8FS1FSContext = S0FSContext;

pub type S8FS1FSByteSpan = S0FSByteSpan;

pub type S8FS1FSFieldBinding = S0FSFieldBinding;

pub type S8FS1FSFieldOutput = S0FSFieldOutput;

pub type S8FS1FSSpectrumInput = S0FSSpectrumInput;

pub type S8FS1FSSpectrumResult = S0FSSpectrumResult;

pub type S8FS1FSProjectionInput = S0FSProjectionInput;

pub type S8FS1FSProjectionResult = S0FSProjectionResult;

pub type S8FS1FSSliceResult = S0FSSliceResult;

pub type S8FS1FSProvenance = S0FSProvenance;

pub type S8FS1FSTableUse = S0FSTableUse;

pub type S8FS1Species = S0Species;

pub type S8FS1SGTableIdentity = S0SGTableIdentity;

pub type S8FS1Cell = S0Cell;

pub type S8FS1Int3 = S0Int3;

pub type S8FS1Vec3 = S0Vec3;

pub type S8FS1Mat3 = S0Mat3;

pub type S8FS1Curve = S0Curve;

pub type S8FS1Diagnostic = S0Diagnostic;

pub type S8FCNativeFractionalGridCurrent1 = S1Root;

pub type S8FCNativeScalarGridCurrent1 = S3Root;

pub type S8FCNativeStoredScalarIsoview1 = S4Root;

pub type S8MathFSResampleAffineInput = S9FSResampleAffineInput;

pub type S8MathFSLinearCombineFieldsInput = S9FSLinearCombineFieldsInput;

pub type S8MathFSCartesianLineInput = S9FSCartesianLineInput;

pub type S8MathFSCartesianLineOutput = S9FSCartesianLineOutput;

pub type S8MathFSLinearCombineCurvesInput = S9FSLinearCombineCurvesInput;

pub type S8MathFSLinearCombineCurvesOutput = S9FSLinearCombineCurvesOutput;

pub type S8F32Bits = String;

pub type S8F64Bits = String;

pub type S8ComplexF64Bits = Vec<S8F64Bits>;

pub type S8Vec3F32Bits = Vec<S8F32Bits>;

pub type S8Vec3F64Bits = Vec<S8F64Bits>;

pub type S8Mat4ColumnsF64Bits = Vec<Vec<S8F64Bits>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8TableSourceIdentity { #[serde(rename="resourceDigest")] pub f_0: S8FS1Digest,
#[serde(rename="sourceId")] pub f_1: String,
#[serde(rename="sourceVersion")] pub f_2: String,
#[serde(rename="edition")] pub f_3: String,
#[serde(rename="tableKey")] pub f_4: String,
#[serde(rename="tableContentDigest")] pub f_5: S8FS1Digest,
#[serde(rename="producerKind")] pub f_6: String,
#[serde(rename="producerName")] pub f_7: String,
#[serde(rename="consumerRole")] pub f_8: String,
#[serde(rename="licenseDisposition")] pub f_9: String,
#[serde(rename="licenseTextDigest")] pub f_10: S8TableSourceIdentityF10,
#[serde(rename="writtenPermissionEvidenceDigest")] pub f_11: S8TableSourceIdentityF11,
#[serde(rename="grantsRedistribution")] pub f_12: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8TableSourceIdentityF10 { V0(S8FS1Digest), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8TableSourceIdentityF11 { V0(S8FS1Digest), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8ScientificTableUse { #[serde(rename="identity")] pub f_0: S8TableSourceIdentity,
#[serde(rename="species")] pub f_1: S8FS1Species,
#[serde(rename="purpose")] pub f_2: String,
#[serde(rename="rowSource")] pub f_3: S8FS1FSByteSpan,
#[serde(rename="scientificQualificationRecordDigest")] pub f_4: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeSettingBinding { #[serde(rename="groupNumberOneBased")] pub f_0: i64,
#[serde(rename="settingNumberOneBased")] pub f_1: i64,
#[serde(rename="registryTable")] pub f_2: S8TableSourceIdentity,
#[serde(rename="registryRecord")] pub f_3: S8FS1FSByteSpan,
#[serde(rename="completeWyckoffPopulationDigest")] pub f_4: S8FS1Digest,
#[serde(rename="completeCurrentSymmetryPopulationDigest")] pub f_5: S8FS1Digest,
#[serde(rename="qualificationRecordDigest")] pub f_6: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8WyckoffAffine { #[serde(rename="sourceOrdinalOneBased")] pub f_0: i64,
#[serde(rename="affineColumnsF64Bits")] pub f_1: S8Mat4ColumnsF64Bits,
#[serde(rename="source")] pub f_2: S8FS1FSByteSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8CurrentSymmetryMatrix { #[serde(rename="sourceOrdinalZeroBased")] pub f_0: i64,
#[serde(rename="affineColumnsF64Bits")] pub f_1: S8Mat4ColumnsF64Bits,
#[serde(rename="qualificationRecordDigest")] pub f_2: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeGridNoInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="dimension")] pub f_3: f64,
#[serde(rename="axisLengthsF32Bits")] pub f_4: S8Vec3F32Bits,
#[serde(rename="resolutionF32Bits")] pub f_5: S8F32Bits,
#[serde(rename="setting")] pub f_6: S8NativeSettingBinding,
#[serde(rename="currentBasisColumnsF64Bits")] pub f_7: S8Mat4ColumnsF64Bits,
#[serde(rename="wyckoffPopulation")] pub f_8: Vec<S8WyckoffAffine>,
#[serde(rename="currentSymmetryPopulation")] pub f_9: Vec<S8CurrentSymmetryMatrix>,
#[serde(rename="maxOutputBytes")] pub f_10: i64,
#[serde(rename="maxResidentBytes")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeGridNoOutput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="requiredAxisMultiples")] pub f_3: Vec<i64>,
#[serde(rename="divisionsBeforeFallback")] pub f_4: Vec<i64>,
#[serde(rename="coreDivisions")] pub f_5: Vec<i64>,
#[serde(rename="reflectionAxisBounds")] pub f_6: Vec<i64>,
#[serde(rename="parameterDigest")] pub f_7: S8FS1Digest,
#[serde(rename="sourceSetting")] pub f_8: S8NativeSettingBinding }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8AmplitudeUnitBinding { #[serde(rename="unit")] pub f_0: String,
#[serde(rename="physicalQualificationDigest")] pub f_1: S8FS1Digest,
#[serde(rename="tableUses")] pub f_2: Vec<S8ScientificTableUse> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8EquivalentReflection { #[serde(rename="hkl")] pub f_0: S8FS1Int3,
#[serde(rename="phasePairF64Bits")] pub f_1: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8MaterializedReflection { #[serde(rename="representativeHkl")] pub f_0: S8FS1Int3,
#[serde(rename="physicalReciprocalMagnitudePerAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="coefficientF64Bits")] pub f_2: S8ComplexF64Bits,
#[serde(rename="equivalents")] pub f_3: Vec<S8EquivalentReflection>,
#[serde(rename="sourceOrdinalZeroBased")] pub f_4: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8ReflectionSource { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="source")] pub f_2: S8FS1FSByteSpan,
#[serde(rename="sourceFormat")] pub f_3: String,
#[serde(rename="readerSelection")] pub f_4: i64,
#[serde(rename="materializedRows")] pub f_5: Vec<S8MaterializedReflection>,
#[serde(rename="actualF000AfterReadF64Bits")] pub f_6: S8F64Bits,
#[serde(rename="amplitude")] pub f_7: S8AmplitudeUnitBinding,
#[serde(rename="readerRuleDigest")] pub f_8: S8FS1Digest,
#[serde(rename="materializedCurrentDigest")] pub f_9: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FourierUi0Input { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="dimension")] pub f_3: f64,
#[serde(rename="uiSelection")] pub f_4: f64,
#[serde(rename="uiLabel")] pub f_5: String,
#[serde(rename="firstSource")] pub f_6: S8ReflectionSource,
#[serde(rename="actualCurrentF000F64Bits")] pub f_7: S8F64Bits,
#[serde(rename="f000Policy")] pub f_8: String,
#[serde(rename="grid")] pub f_9: S8NativeGridNoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FourierUi1Input { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="dimension")] pub f_3: f64,
#[serde(rename="uiSelection")] pub f_4: f64,
#[serde(rename="uiLabel")] pub f_5: String,
#[serde(rename="firstSource")] pub f_6: S8ReflectionSource,
#[serde(rename="actualCurrentF000F64Bits")] pub f_7: S8F64Bits,
#[serde(rename="f000Policy")] pub f_8: String,
#[serde(rename="grid")] pub f_9: S8NativeGridNoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FourierUi2Input { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="dimension")] pub f_3: f64,
#[serde(rename="uiSelection")] pub f_4: f64,
#[serde(rename="uiLabel")] pub f_5: String,
#[serde(rename="firstSource")] pub f_6: S8ReflectionSource,
#[serde(rename="actualCurrentF000F64Bits")] pub f_7: S8F64Bits,
#[serde(rename="f000Policy")] pub f_8: String,
#[serde(rename="grid")] pub f_9: S8NativeGridNoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FourierUi3Input { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="dimension")] pub f_3: f64,
#[serde(rename="uiSelection")] pub f_4: f64,
#[serde(rename="uiLabel")] pub f_5: String,
#[serde(rename="firstSource")] pub f_6: S8ReflectionSource,
#[serde(rename="actualCurrentF000F64Bits")] pub f_7: S8F64Bits,
#[serde(rename="f000Policy")] pub f_8: String,
#[serde(rename="grid")] pub f_9: S8NativeGridNoInput,
#[serde(rename="secondSource")] pub f_10: S8ReflectionSource,
#[serde(rename="secondScaleF64Bits")] pub f_11: S8F64Bits,
#[serde(rename="differencePolicy")] pub f_12: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeFourierUiInput { V0(S8FourierUi0Input), V1(S8FourierUi1Input), V2(S8FourierUi2Input), V3(S8FourierUi3Input) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FourierSourceDiagnostic { #[serde(rename="code")] pub f_0: String,
#[serde(rename="sourceOrdinalZeroBased")] pub f_1: i64,
#[serde(rename="hkl")] pub f_2: S8FS1Int3,
#[serde(rename="source")] pub f_3: S8FS1FSByteSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePreparedFourierOutput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="uiSelection")] pub f_2: i64,
#[serde(rename="rawBackendMode")] pub f_3: f64,
#[serde(rename="actualCurrentF000F64Bits")] pub f_4: S8F64Bits,
#[serde(rename="rows")] pub f_5: Vec<S8MaterializedReflection>,
#[serde(rename="sourceBindings")] pub f_6: Vec<S8ReflectionSource>,
#[serde(rename="secondScaleF64Bits")] pub f_7: S8NativePreparedFourierOutputF7,
#[serde(rename="grid")] pub f_8: S8NativeGridNoOutput,
#[serde(rename="diagnostics")] pub f_9: Vec<S8FourierSourceDiagnostic>,
#[serde(rename="parameterDigest")] pub f_10: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativePreparedFourierOutputF7 { V0(S8F64Bits), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13Parameters { #[serde(rename="nameCount")] pub f_0: f64,
#[serde(rename="fitParameterCount")] pub f_1: f64,
#[serde(rename="parameterNames")] pub f_2: [String;13],
#[serde(rename="U")] pub f_3: S8PowderTch13ParametersF3,
#[serde(rename="V")] pub f_4: S8PowderTch13ParametersF4,
#[serde(rename="W")] pub f_5: S8PowderTch13ParametersF5,
#[serde(rename="P")] pub f_6: S8PowderTch13ParametersF6,
#[serde(rename="X")] pub f_7: S8PowderTch13ParametersF7,
#[serde(rename="Xe")] pub f_8: S8PowderTch13ParametersF8,
#[serde(rename="Y")] pub f_9: S8PowderTch13ParametersF9,
#[serde(rename="Ye")] pub f_10: S8PowderTch13ParametersF10,
#[serde(rename="rs")] pub f_11: S8PowderTch13ParametersF11,
#[serde(rename="rd")] pub f_12: S8PowderTch13ParametersF12,
#[serde(rename="h")] pub f_13: S8PowderTch13ParametersF13,
#[serde(rename="k")] pub f_14: S8PowderTch13ParametersF14,
#[serde(rename="l")] pub f_15: S8PowderTch13ParametersF15,
#[serde(rename="parameterTableSource")] pub f_16: S8FS1FSByteSpan,
#[serde(rename="parameterRuleDigest")] pub f_17: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF3 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF4 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF5 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF6 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF7 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF8 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF9 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF10 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF11 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF12 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF13 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF14 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8PowderTch13ParametersF15 { #[serde(rename="valueF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="unit")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeTchWidthInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="parameters")] pub f_3: S8PowderTch13Parameters,
#[serde(rename="twoThetaDegreeF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="reflectionRawAxisScalarF64Bits")] pub f_5: S8F64Bits,
#[serde(rename="rawAxisScalarRule")] pub f_6: String,
#[serde(rename="currentReciprocalMatrixColumnsF64Bits")] pub f_7: Vec<S8Vec3F64Bits> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeTchWidthOutput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="gaussianWidthDegreeF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="lorentzWidthDegreeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="combinedWidthDegreeF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="mixingF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="parameterDigest")] pub f_5: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderReflectionInput { #[serde(rename="coefficientF64Bits")] pub f_0: S8ComplexF64Bits,
#[serde(rename="coefficientUnit")] pub f_1: String,
#[serde(rename="multiplicity")] pub f_2: i64,
#[serde(rename="wavelengthAF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="beamWeightF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="reflectionCorrectionF64Bits")] pub f_5: S8F64Bits,
#[serde(rename="perObservationFactorsF64Bits")] pub f_6: Vec<S8F64Bits>,
#[serde(rename="materializedProfileF64Bits")] pub f_7: Vec<S8F64Bits>,
#[serde(rename="profileUnit")] pub f_8: String,
#[serde(rename="profileProducer")] pub f_9: String,
#[serde(rename="profileRuleDigest")] pub f_10: S8FS1Digest,
#[serde(rename="source")] pub f_11: S8FS1FSByteSpan,
#[serde(rename="materializedProfileStage")] pub f_12: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderPhaseInput { #[serde(rename="phaseId")] pub f_0: S8FS1Id,
#[serde(rename="phaseScaleF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="reflections")] pub f_2: Vec<S8NativePowderReflectionInput>,
#[serde(rename="scientificTableUses")] pub f_3: Vec<S8ScientificTableUse> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativePowderNormalization { V0(S8NativePowderNormalizationV0), V1(S8NativePowderNormalizationV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderNormalizationV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="target")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderNormalizationV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="observed")] pub f_1: S8FS1Curve,
#[serde(rename="actualStoredObservedMaxF64Bits")] pub f_2: S8F64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderAccumulateInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="profile")] pub f_1: String,
#[serde(rename="sampleTwoThetaDegreeF64Bits")] pub f_2: Vec<S8F64Bits>,
#[serde(rename="phases")] pub f_3: Vec<S8NativePowderPhaseInput>,
#[serde(rename="backgroundF64Bits")] pub f_4: Vec<S8F64Bits>,
#[serde(rename="normalization")] pub f_5: S8NativePowderNormalization,
#[serde(rename="policy")] pub f_6: String,
#[serde(rename="maxOutputBytes")] pub f_7: i64,
#[serde(rename="maxResidentBytes")] pub f_8: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderAccumulateOutput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="sampleTwoThetaDegreeF64Bits")] pub f_1: Vec<S8F64Bits>,
#[serde(rename="rawPhaseCurvesF64Bits")] pub f_2: Vec<Vec<S8F64Bits>>,
#[serde(rename="rawAggregateF64Bits")] pub f_3: Vec<S8F64Bits>,
#[serde(rename="maxRawYcF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="targetF64Bits")] pub f_5: S8F64Bits,
#[serde(rename="commonScaleF64Bits")] pub f_6: S8F64Bits,
#[serde(rename="scaledPhaseCurvesF64Bits")] pub f_7: Vec<Vec<S8F64Bits>>,
#[serde(rename="backgroundF64Bits")] pub f_8: Vec<S8F64Bits>,
#[serde(rename="totalF64Bits")] pub f_9: Vec<S8F64Bits>,
#[serde(rename="unit")] pub f_10: String,
#[serde(rename="parameterDigest")] pub f_11: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8QualifiedFieldSource { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="field")] pub f_1: S8FS1FSFieldBinding,
#[serde(rename="sourceFormatDescriptorDigest")] pub f_2: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFractionalSource { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="current")] pub f_1: S8FCNativeFractionalGridCurrent1,
#[serde(rename="source")] pub f_2: S8FS1FSByteSpan,
#[serde(rename="sourceRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeAffineSource { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="current")] pub f_1: S8FCNativeScalarGridCurrent1,
#[serde(rename="source")] pub f_2: S8FS1FSByteSpan,
#[serde(rename="sourceRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8ScalarSource { V0(S8QualifiedFieldSource), V1(S8NativeFractionalSource), V2(S8NativeAffineSource) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NoComponentSelector { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8FlatIgnoredSelector { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="selectionIndexI32")] pub f_1: i64,
#[serde(rename="group0Count")] pub f_2: i64,
#[serde(rename="group1Count")] pub f_3: i64,
#[serde(rename="remapFlagI32")] pub f_4: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8IndexedNativeSelector { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="selectionIndexI32")] pub f_1: i64,
#[serde(rename="group0Count")] pub f_2: i64,
#[serde(rename="group1Count")] pub f_3: i64,
#[serde(rename="remapFlagI32")] pub f_4: i64,
#[serde(rename="secondGroupRule")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8IndexedCorrectedSelector { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="selectionIndexI32")] pub f_1: i64,
#[serde(rename="group0Count")] pub f_2: i64,
#[serde(rename="group1Count")] pub f_3: i64,
#[serde(rename="remapFlagI32")] pub f_4: i64,
#[serde(rename="secondGroupRule")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8ComponentSelector { V0(S8NoComponentSelector), V1(S8FlatIgnoredSelector), V2(S8IndexedNativeSelector), V3(S8IndexedCorrectedSelector) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8OrderedExpressionEntry { #[serde(rename="source")] pub f_0: S8ScalarSource,
#[serde(rename="originalReference")] pub f_1: S8FS1FSByteSpan,
#[serde(rename="sourceFormat")] pub f_2: String,
#[serde(rename="selector")] pub f_3: S8ComponentSelector,
#[serde(rename="prefix")] pub f_4: String,
#[serde(rename="coefficientF32Bits")] pub f_5: S8F32Bits,
#[serde(rename="powerF32Bits")] pub f_6: S8F32Bits,
#[serde(rename="formatRuleDigest")] pub f_7: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8ResultQuantityBinding { #[serde(rename="meaning")] pub f_0: String,
#[serde(rename="physicalKind")] pub f_1: String,
#[serde(rename="unit")] pub f_2: String,
#[serde(rename="declaredUnit")] pub f_3: S8ResultQuantityBindingF3,
#[serde(rename="resultQuantityRuleDigest")] pub f_4: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8ResultQuantityBindingF3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8OrderedExpressionInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="entries")] pub f_3: Vec<S8OrderedExpressionEntry>,
#[serde(rename="initialization")] pub f_4: String,
#[serde(rename="geometryPolicy")] pub f_5: String,
#[serde(rename="resultQuantity")] pub f_6: S8ResultQuantityBinding,
#[serde(rename="arithmetic")] pub f_7: String,
#[serde(rename="reopen")] pub f_8: String,
#[serde(rename="maxResidentBytes")] pub f_9: i64,
#[serde(rename="maxOutputBytes")] pub f_10: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8OrderedExpressionOutput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="materialized")] pub f_2: S8ScalarSource,
#[serde(rename="ownership")] pub f_3: String,
#[serde(rename="orderedSources")] pub f_4: Vec<S8OrderedExpressionEntry>,
#[serde(rename="resultQuantity")] pub f_5: S8ResultQuantityBinding,
#[serde(rename="materializedDataDigest")] pub f_6: S8FS1Digest,
#[serde(rename="materializedDescriptorDigest")] pub f_7: S8FS1Digest,
#[serde(rename="operationRuleDigest")] pub f_8: S8FS1Digest,
#[serde(rename="parameterDigest")] pub f_9: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8ExactComparison { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="domain")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundedComparison { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="absoluteBound")] pub f_1: f64,
#[serde(rename="unit")] pub f_2: String,
#[serde(rename="scope")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8OracleBinding { #[serde(rename="id")] pub f_0: String,
#[serde(rename="classification")] pub f_1: String,
#[serde(rename="expectedPath")] pub f_2: String,
#[serde(rename="expectedSha256")] pub f_3: S8FS1Digest,
#[serde(rename="expectedPointers")] pub f_4: Vec<String>,
#[serde(rename="requiredOutputFields")] pub f_5: Vec<String>,
#[serde(rename="comparison")] pub f_6: S8OracleBindingF6,
#[serde(rename="completeArrayCounts")] pub f_7: Vec<i64>,
#[serde(rename="nativeExecutionClaim")] pub f_8: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8OracleBindingF6 { V0(S8ExactComparison), V1(S8BoundedComparison) }

pub type S8SG2NativeDimensionDState = S11NativeDimensionDState;

pub type S8FCNativeLayerCurrent1 = S2Root;

pub type S8FS1Mesh = S0Mesh;

pub type S8FS1FSNonfiniteDiagnostic = S0FSNonfiniteDiagnostic;

pub type S8FS1FSConversionResult = S0FSConversionResult;

pub type S8FS1FSPeakResult = S0FSPeakResult;

pub type S8FS1Field = S0Field;

pub type S8FS1Plane = S0Plane;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFailure { #[serde(rename="status")] pub f_0: String,
#[serde(rename="code")] pub f_1: String,
#[serde(rename="sourceProcedure")] pub f_2: String,
#[serde(rename="sourceAddress")] pub f_3: String,
#[serde(rename="diagnostic")] pub f_4: String,
#[serde(rename="preservedInputDigest")] pub f_5: S8FS1Digest,
#[serde(rename="candidateReleased")] pub f_6: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8ScientificSourceRow { #[serde(rename="speciesLabel")] pub f_0: String,
#[serde(rename="atomicNumberI32")] pub f_1: i64,
#[serde(rename="actualElementIndexI16")] pub f_2: i64,
#[serde(rename="asfdcSource")] pub f_3: S8TableSourceIdentity,
#[serde(rename="selectedFourLineSpan")] pub f_4: S8ScientificSourceRowF4,
#[serde(rename="selection")] pub f_5: String,
#[serde(rename="candidateSpansInActualScanOrder")] pub f_6: Vec<S8FS1FSByteSpan>,
#[serde(rename="fiveGaussianAF64Bits")] pub f_7: Vec<S8F64Bits>,
#[serde(rename="fiveGaussianBA2F64Bits")] pub f_8: Vec<S8F64Bits>,
#[serde(rename="constantElectronF64Bits")] pub f_9: S8F64Bits,
#[serde(rename="coherentBcFmF64Bits")] pub f_10: S8F64Bits,
#[serde(rename="printedMassF64Bits")] pub f_11: S8F64Bits,
#[serde(rename="sourceRuleDigest")] pub f_12: S8FS1Digest,
#[serde(rename="printedMassUnit")] pub f_13: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8ScientificSourceRowF4 { V0(S8FS1FSByteSpan), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8DispersionEightColumnRow { #[serde(rename="source")] pub f_0: S8FS1FSByteSpan,
#[serde(rename="valuesF32Bits")] pub f_1: Vec<S8F32Bits> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeDispersionInput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="sourceIdentity")] pub f_1: S8TableSourceIdentity,
#[serde(rename="atomicNumberI32")] pub f_2: i64,
#[serde(rename="wavelengthNmF32Bits")] pub f_3: S8F32Bits,
#[serde(rename="relativisticCorrectionF32Bits")] pub f_4: S8F32Bits,
#[serde(rename="nuclearThomsonF32Bits")] pub f_5: S8F32Bits,
#[serde(rename="completeRows")] pub f_6: Vec<S8DispersionEightColumnRow>,
#[serde(rename="sourceEdgeHeaderSpans")] pub f_7: Vec<S8FS1FSByteSpan> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeDispersionOutput { #[serde(rename="energyKeVF32Bits")] pub f_0: S8F32Bits,
#[serde(rename="lowerSourceOrdinal")] pub f_1: S8NativeDispersionOutputF1,
#[serde(rename="upperSourceOrdinal")] pub f_2: S8NativeDispersionOutputF2,
#[serde(rename="lowerWasInitializedZero")] pub f_3: bool,
#[serde(rename="upperWasResetZeroAtEof")] pub f_4: bool,
#[serde(rename="fpElectronF32Bits")] pub f_5: S8F32Bits,
#[serde(rename="fppElectronF32Bits")] pub f_6: S8F32Bits,
#[serde(rename="massAttenuationCm2PerGF32Bits")] pub f_7: S8F32Bits,
#[serde(rename="parameterDigest")] pub f_8: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeDispersionOutputF1 { V0(i64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeDispersionOutputF2 { V0(i64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeEquivalentD3 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="phasePairF64Bits")] pub f_1: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionD3 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="physicalReciprocalMagnitudePerAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="internalReciprocalMagnitudeNativeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="integerPhaseMultiplicityI32")] pub f_3: i64,
#[serde(rename="friedelFlagsU32")] pub f_4: i64,
#[serde(rename="equivalents")] pub f_5: Vec<S8NativeEquivalentD3>,
#[serde(rename="coefficientF64Bits")] pub f_6: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_7: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionPopulationD3 { #[serde(rename="dimensionD")] pub f_0: f64,
#[serde(rename="rows")] pub f_1: Vec<S8NativeReflectionD3>,
#[serde(rename="populationDigest")] pub f_2: S8FS1Digest,
#[serde(rename="producerRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeEquivalentD4 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="phasePairF64Bits")] pub f_1: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionD4 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="physicalReciprocalMagnitudePerAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="internalReciprocalMagnitudeNativeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="integerPhaseMultiplicityI32")] pub f_3: i64,
#[serde(rename="friedelFlagsU32")] pub f_4: i64,
#[serde(rename="equivalents")] pub f_5: Vec<S8NativeEquivalentD4>,
#[serde(rename="coefficientF64Bits")] pub f_6: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_7: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionPopulationD4 { #[serde(rename="dimensionD")] pub f_0: f64,
#[serde(rename="rows")] pub f_1: Vec<S8NativeReflectionD4>,
#[serde(rename="populationDigest")] pub f_2: S8FS1Digest,
#[serde(rename="producerRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeEquivalentD5 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="phasePairF64Bits")] pub f_1: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionD5 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="physicalReciprocalMagnitudePerAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="internalReciprocalMagnitudeNativeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="integerPhaseMultiplicityI32")] pub f_3: i64,
#[serde(rename="friedelFlagsU32")] pub f_4: i64,
#[serde(rename="equivalents")] pub f_5: Vec<S8NativeEquivalentD5>,
#[serde(rename="coefficientF64Bits")] pub f_6: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_7: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionPopulationD5 { #[serde(rename="dimensionD")] pub f_0: f64,
#[serde(rename="rows")] pub f_1: Vec<S8NativeReflectionD5>,
#[serde(rename="populationDigest")] pub f_2: S8FS1Digest,
#[serde(rename="producerRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeEquivalentD6 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="phasePairF64Bits")] pub f_1: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionD6 { #[serde(rename="hklD")] pub f_0: Vec<i64>,
#[serde(rename="physicalReciprocalMagnitudePerAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="internalReciprocalMagnitudeNativeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="integerPhaseMultiplicityI32")] pub f_3: i64,
#[serde(rename="friedelFlagsU32")] pub f_4: i64,
#[serde(rename="equivalents")] pub f_5: Vec<S8NativeEquivalentD6>,
#[serde(rename="coefficientF64Bits")] pub f_6: S8ComplexF64Bits,
#[serde(rename="sourceOrdinalZeroBased")] pub f_7: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionPopulationD6 { #[serde(rename="dimensionD")] pub f_0: f64,
#[serde(rename="rows")] pub f_1: Vec<S8NativeReflectionD6>,
#[serde(rename="populationDigest")] pub f_2: S8FS1Digest,
#[serde(rename="producerRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeReflectionPopulation { V0(S8NativeReflectionPopulationD3), V1(S8NativeReflectionPopulationD4), V2(S8NativeReflectionPopulationD5), V3(S8NativeReflectionPopulationD6) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeThermalSelection { #[serde(rename="anisotropicEnabledI32")] pub f_0: i64,
#[serde(rename="anisotropicRepresentation")] pub f_1: String,
#[serde(rename="isotropicRepresentation")] pub f_2: String,
#[serde(rename="perSiteFlagsPolicy")] pub f_3: String,
#[serde(rename="covariancePolicy")] pub f_4: String,
#[serde(rename="sourceRuleDigest")] pub f_5: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeScatteringInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="model")] pub f_3: S8SG2NativeDimensionDState,
#[serde(rename="radiation")] pub f_4: String,
#[serde(rename="thermal")] pub f_5: S8NativeThermalSelection,
#[serde(rename="completeElementRows")] pub f_6: Vec<S8ScientificSourceRow>,
#[serde(rename="completeDispersionByElement")] pub f_7: Vec<S8NativeDispersionOutput>,
#[serde(rename="actualSymmetryOperatorCount")] pub f_8: i64,
#[serde(rename="reflections")] pub f_9: S8NativeReflectionPopulation,
#[serde(rename="maxOutputBytes")] pub f_10: i64,
#[serde(rename="maxResidentBytes")] pub f_11: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8SiteReflectionContribution { #[serde(rename="siteId")] pub f_0: S8FS1Id,
#[serde(rename="reflectionSourceOrdinalZeroBased")] pub f_1: i64,
#[serde(rename="equivalentSourceOrdinalZeroBased")] pub f_2: i64,
#[serde(rename="coefficientF64Bits")] pub f_3: S8ComplexF64Bits,
#[serde(rename="thermalFactorF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="weightedAmplitudeF64Bits")] pub f_5: S8ComplexF64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeScatteringOutput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="reflections")] pub f_1: S8NativeReflectionPopulation,
#[serde(rename="allSiteEquivalentContributions")] pub f_2: Vec<S8SiteReflectionContribution>,
#[serde(rename="amplitudeUnit")] pub f_3: String,
#[serde(rename="elementRowsUsed")] pub f_4: Vec<S8ScientificSourceRow>,
#[serde(rename="diagnostics")] pub f_5: Vec<S8FS1Diagnostic>,
#[serde(rename="parameterDigest")] pub f_6: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFieldGrid { #[serde(rename="actualDimensionD")] pub f_0: f64,
#[serde(rename="nativeThreeDivisions")] pub f_1: Vec<i64>,
#[serde(rename="currentC2OColumnsF64Bits")] pub f_2: Vec<S8Vec3F64Bits>,
#[serde(rename="physicalFieldQuantity")] pub f_3: String,
#[serde(rename="sourceLayoutRule")] pub f_4: String,
#[serde(rename="gridProducerRuleDigest")] pub f_5: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFourierFieldInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="grid")] pub f_3: S8NativeFieldGrid,
#[serde(rename="reflections")] pub f_4: S8NativeReflectionPopulation,
#[serde(rename="actualCurrentF000F64Bits")] pub f_5: S8F64Bits,
#[serde(rename="rawModeI32")] pub f_6: f64,
#[serde(rename="amplitudeUnit")] pub f_7: String,
#[serde(rename="packingPolicy")] pub f_8: String,
#[serde(rename="maxOutputBytes")] pub f_9: i64,
#[serde(rename="maxResidentBytes")] pub f_10: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFullFieldOutput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="grid")] pub f_2: S8NativeFieldGrid,
#[serde(rename="coreValuesF32Bits")] pub f_3: Vec<S8F32Bits>,
#[serde(rename="periodicClosureValuesF32Bits")] pub f_4: Vec<S8F32Bits>,
#[serde(rename="packedSpectrumF64Bits")] pub f_5: Vec<S8ComplexF64Bits>,
#[serde(rename="field")] pub f_6: S8ScalarSource,
#[serde(rename="quantityUnit")] pub f_7: String,
#[serde(rename="actualF000F64Bits")] pub f_8: S8F64Bits,
#[serde(rename="sourcePopulationDigest")] pub f_9: S8FS1Digest,
#[serde(rename="parameterDigest")] pub f_10: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeModelFieldInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="model")] pub f_3: S8SG2NativeDimensionDState,
#[serde(rename="gridSelection")] pub f_4: S8NativeGridNoInput,
#[serde(rename="rawModelModeI32")] pub f_5: f64,
#[serde(rename="thermal")] pub f_6: S8NativeThermalSelection,
#[serde(rename="completeElementRows")] pub f_7: Vec<S8ScientificSourceRow>,
#[serde(rename="anomalousPolicy")] pub f_8: String,
#[serde(rename="maxOutputBytes")] pub f_9: i64,
#[serde(rename="maxResidentBytes")] pub f_10: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeModelFieldOutput { #[serde(rename="fullField")] pub f_0: S8NativeFullFieldOutput,
#[serde(rename="fullScattering")] pub f_1: S8NativeScatteringOutput,
#[serde(rename="F000PositiveF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="F000NegativeF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="F000BeforePattersonSquareF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="allSourceRows")] pub f_5: Vec<S8ScientificSourceRow> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeVolumeTransformInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="source")] pub f_3: S8ScalarSource,
#[serde(rename="grid")] pub f_4: S8NativeFieldGrid,
#[serde(rename="requestedReflections")] pub f_5: S8NativeReflectionPopulation,
#[serde(rename="maxOutputBytes")] pub f_6: i64,
#[serde(rename="maxResidentBytes")] pub f_7: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeVolumeTransformOutput { #[serde(rename="fullField")] pub f_0: S8NativeVolumeTransformOutputF0,
#[serde(rename="reflections")] pub f_1: S8NativeReflectionPopulation,
#[serde(rename="sourceDescriptorDigest")] pub f_2: S8FS1Digest,
#[serde(rename="transformScaleVolumePerCorePointF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="packedSpectrumF64Bits")] pub f_4: Vec<S8ComplexF64Bits>,
#[serde(rename="parameterDigest")] pub f_5: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeVolumeTransformOutputF0 { V0(S8NativeFullFieldOutput), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePeakRecord { #[serde(rename="sourceCoreIndex")] pub f_0: i64,
#[serde(rename="fractionalPositionF64Bits")] pub f_1: S8Vec3F64Bits,
#[serde(rename="peakValueF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="integralF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="volumeA3F64Bits")] pub f_4: S8F64Bits,
#[serde(rename="metricWeightF64Bits")] pub f_5: S8F64Bits,
#[serde(rename="metricOffsetF64Bits")] pub f_6: S8F64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePeakSearchInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="source")] pub f_3: S8ScalarSource,
#[serde(rename="actualC2OColumnsF64Bits")] pub f_4: Vec<S8Vec3F64Bits>,
#[serde(rename="requestedCountI32")] pub f_5: i64,
#[serde(rename="includeNegative")] pub f_6: bool,
#[serde(rename="automaticIntegration")] pub f_7: String,
#[serde(rename="maxOutputBytes")] pub f_8: i64,
#[serde(rename="maxResidentBytes")] pub f_9: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeVoronoiInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="source")] pub f_3: S8ScalarSource,
#[serde(rename="actualC2OColumnsF64Bits")] pub f_4: Vec<S8Vec3F64Bits>,
#[serde(rename="orderedPeaks")] pub f_5: Vec<S8NativePeakRecord>,
#[serde(rename="subdivisionsPerAxis")] pub f_6: i64,
#[serde(rename="metricMethodI32")] pub f_7: i64,
#[serde(rename="boundaryToleranceF64Bits")] pub f_8: String,
#[serde(rename="candidateRule")] pub f_9: String,
#[serde(rename="densityRule")] pub f_10: String,
#[serde(rename="maxOutputBytes")] pub f_11: i64,
#[serde(rename="maxResidentBytes")] pub f_12: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeNodeAllocation { #[serde(rename="coreIndex")] pub f_0: i64,
#[serde(rename="subsampleOrdinalZeroBased")] pub f_1: i64,
#[serde(rename="tiedOwnerOrdinals")] pub f_2: Vec<i64>,
#[serde(rename="fractionF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="signedDensityContributionF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="volumeContributionA3F64Bits")] pub f_5: S8F64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePeakVoronoiOutput { #[serde(rename="counterU16")] pub f_0: Vec<i64>,
#[serde(rename="orderedPeaks")] pub f_1: Vec<S8NativePeakRecord>,
#[serde(rename="initialCoreOwners")] pub f_2: Vec<i64>,
#[serde(rename="initialCoreScoresF64Bits")] pub f_3: Vec<S8F64Bits>,
#[serde(rename="allAllocations")] pub f_4: Vec<S8NativeNodeAllocation>,
#[serde(rename="boundaryVoxelCount")] pub f_5: i64,
#[serde(rename="totalSignedIntegralF64Bits")] pub f_6: S8F64Bits,
#[serde(rename="totalAssignedVolumeA3F64Bits")] pub f_7: S8F64Bits,
#[serde(rename="parameterDigest")] pub f_8: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="profile")] pub f_1: String,
#[serde(rename="orderedActualPhases")] pub f_2: Vec<S8NativeMeshPhase>,
#[serde(rename="actualLayerPopulation")] pub f_3: S8FCNativeLayerCurrent1,
#[serde(rename="storedLevels")] pub f_4: S8FCNativeStoredScalarIsoview1,
#[serde(rename="textureSettings")] pub f_5: S8NativeMeshTextureSettings,
#[serde(rename="sourceConsumerRuleDigest")] pub f_6: S8FS1Digest,
#[serde(rename="maxOutputBytes")] pub f_7: i64,
#[serde(rename="maxResidentBytes")] pub f_8: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshTriangle { #[serde(rename="levelOrdinal")] pub f_0: i64,
#[serde(rename="sign")] pub f_1: f64,
#[serde(rename="coreCellIndex")] pub f_2: i64,
#[serde(rename="nativeCaseId")] pub f_3: i64,
#[serde(rename="sourceRowTriangleOrdinal")] pub f_4: i64,
#[serde(rename="edgeIds")] pub f_5: Vec<i64>,
#[serde(rename="vertexIndices")] pub f_6: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshOutput { #[serde(rename="verticesCartesianF32Bits")] pub f_0: Vec<S8Vec3F32Bits>,
#[serde(rename="triangles")] pub f_1: Vec<S8NativeMeshTriangle>,
#[serde(rename="faceNormalsF32Bits")] pub f_2: Vec<S8Vec3F32Bits>,
#[serde(rename="vertexNormalsF32Bits")] pub f_3: Vec<S8Vec3F32Bits>,
#[serde(rename="levelBindings")] pub f_4: S8FCNativeStoredScalarIsoview1,
#[serde(rename="layerBindings")] pub f_5: S8FCNativeLayerCurrent1,
#[serde(rename="sourceDescriptorDigest")] pub f_6: S8FS1Digest,
#[serde(rename="nativeNonfiniteTriangleOrdinals")] pub f_7: Vec<i64>,
#[serde(rename="parameterDigest")] pub f_8: S8FS1Digest,
#[serde(rename="vertexColorScalarsF32Bits")] pub f_9: Vec<S8F32Bits>,
#[serde(rename="vertexTextureCoordinatesF32Bits")] pub f_10: Vec<S8F32Bits>,
#[serde(rename="completeClippedPolygonVertexIndices")] pub f_11: Vec<Vec<i64>>,
#[serde(rename="sourcePhaseByTriangle")] pub f_12: Vec<S8FS1Id>,
#[serde(rename="stage")] pub f_13: String,
#[serde(rename="levelBranchRanges")] pub f_14: Vec<S8NativeMeshBranchRange>,
#[serde(rename="nativeNonfiniteUsedVertexOrdinals")] pub f_15: Vec<i64>,
#[serde(rename="nativeNonfiniteUnusedVertexOrdinals")] pub f_16: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeConversionInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="source")] pub f_3: S8ScalarSource,
#[serde(rename="actualC2OColumnsF64Bits")] pub f_4: Vec<S8Vec3F64Bits>,
#[serde(rename="nativeCellSixF32Bits")] pub f_5: Vec<S8F32Bits>,
#[serde(rename="inputDensityUnit")] pub f_6: String,
#[serde(rename="argumentLengthScaleF64Bits")] pub f_7: String,
#[serde(rename="unitProducerRuleDigest")] pub f_8: S8FS1Digest,
#[serde(rename="kineticSelectorI32")] pub f_9: f64,
#[serde(rename="requestedOutputs")] pub f_10: Vec<String>,
#[serde(rename="maxOutputBytes")] pub f_11: i64,
#[serde(rename="maxResidentBytes")] pub f_12: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeConversionOutput { #[serde(rename="stencil27F64Bits")] pub f_0: Vec<S8F64Bits>,
#[serde(rename="includedNeighbourOffsets")] pub f_1: Vec<S8FS1Int3>,
#[serde(rename="laplacianF64Bits")] pub f_2: Vec<S8F64Bits>,
#[serde(rename="centralDifferenceStencilSurrogateF64Bits")] pub f_3: Vec<S8F64Bits>,
#[serde(rename="kineticGF64Bits")] pub f_4: Vec<S8F64Bits>,
#[serde(rename="potentialVF64Bits")] pub f_5: Vec<S8F64Bits>,
#[serde(rename="totalHF64Bits")] pub f_6: Vec<S8F64Bits>,
#[serde(rename="logAbsDensityF64Bits")] pub f_7: Vec<S8F64Bits>,
#[serde(rename="laplacianUnit")] pub f_8: String,
#[serde(rename="energyDensityUnit")] pub f_9: String,
#[serde(rename="logarithmArgument")] pub f_10: String,
#[serde(rename="energyScaleF64Bits")] pub f_11: S8F64Bits,
#[serde(rename="nonfiniteDiagnostics")] pub f_12: Vec<S8FS1FSNonfiniteDiagnostic>,
#[serde(rename="nativeExportSuccessDoesNotCertifyWrite")] pub f_13: bool,
#[serde(rename="parameterDigest")] pub f_14: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeDispersionOutputOrFailure { V0(S8NativeDispersionOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeScatteringOutputOrFailure { V0(S8NativeScatteringOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeFullFieldOutputOrFailure { V0(S8NativeFullFieldOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeModelFieldOutputOrFailure { V0(S8NativeModelFieldOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeVolumeTransformOutputOrFailure { V0(S8NativeVolumeTransformOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativePeakVoronoiOutputOrFailure { V0(S8NativePeakVoronoiOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeMeshOutputOrFailure { V0(S8NativeMeshOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeConversionOutputOrFailure { V0(S8NativeConversionOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderBeam { #[serde(rename="sourceIndexU16")] pub f_0: i64,
#[serde(rename="wavelengthAF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="relativeWeightF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="radiation")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderShift { #[serde(rename="p0TwoThetaDegreeF64Bits")] pub f_0: S8F64Bits,
#[serde(rename="p1TwoThetaDegreeF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="p2TwoThetaDegreeF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="rule")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderFullPhase { #[serde(rename="phaseId")] pub f_0: S8FS1Id,
#[serde(rename="model")] pub f_1: S8SG2NativeDimensionDState,
#[serde(rename="thermal")] pub f_2: S8NativeThermalSelection,
#[serde(rename="elementRows")] pub f_3: Vec<S8ScientificSourceRow>,
#[serde(rename="dispersionByBeamElement")] pub f_4: Vec<Vec<S8NativeDispersionOutput>>,
#[serde(rename="actualSymmetryOperatorCount")] pub f_5: i64,
#[serde(rename="phaseScaleF64Bits")] pub f_6: S8F64Bits,
#[serde(rename="parameters")] pub f_7: S8PowderTch13Parameters,
#[serde(rename="profileKind")] pub f_8: String,
#[serde(rename="xShift")] pub f_9: S8NativePowderShift,
#[serde(rename="lorentzParameterM_F64Bits")] pub f_10: S8F64Bits,
#[serde(rename="lorentzParameterP_F64Bits")] pub f_11: S8F64Bits,
#[serde(rename="perObservationFactorsF64Bits")] pub f_12: Vec<S8F64Bits>,
#[serde(rename="sourcePopulation")] pub f_13: S8NativeReflectionPopulation,
#[serde(rename="actualGlobalReferenceMaxIntensityF64Bits")] pub f_14: S8F64Bits,
#[serde(rename="cutRatioDimensionlessF64Bits")] pub f_15: S8F64Bits,
#[serde(rename="cutoffProducerRuleDigest")] pub f_16: S8FS1Digest,
#[serde(rename="enumerationRecipe")] pub f_17: S8NativePowderEnumerationRecipe }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderFullInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="profile")] pub f_1: String,
#[serde(rename="sampleTwoThetaDegreeF64Bits")] pub f_2: Vec<S8F64Bits>,
#[serde(rename="beams")] pub f_3: Vec<S8NativePowderBeam>,
#[serde(rename="phases")] pub f_4: Vec<S8NativePowderFullPhase>,
#[serde(rename="normalization")] pub f_5: S8NativePowderNormalization,
#[serde(rename="backgroundF64Bits")] pub f_6: Vec<S8F64Bits>,
#[serde(rename="maxOutputBytes")] pub f_7: i64,
#[serde(rename="maxResidentBytes")] pub f_8: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderProfileWitness { #[serde(rename="phaseId")] pub f_0: S8FS1Id,
#[serde(rename="reflectionSourceOrdinalZeroBased")] pub f_1: i64,
#[serde(rename="beamSourceIndexU16")] pub f_2: i64,
#[serde(rename="twoThetaDegreeF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="width")] pub f_4: S8NativeTchWidthOutput,
#[serde(rename="equivalentMultiplicity")] pub f_5: i64,
#[serde(rename="coefficientF64Bits")] pub f_6: S8ComplexF64Bits,
#[serde(rename="startSampleOrdinal")] pub f_7: i64,
#[serde(rename="endSampleOrdinalExclusive")] pub f_8: i64,
#[serde(rename="completeProfileF64Bits")] pub f_9: Vec<S8F64Bits>,
#[serde(rename="firstSamplePolicy")] pub f_10: String,
#[serde(rename="completeProfileStage")] pub f_11: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderFullOutput { #[serde(rename="pattern")] pub f_0: S8NativePowderAccumulateOutput,
#[serde(rename="reflectionPopulations")] pub f_1: Vec<S8NativeReflectionPopulation>,
#[serde(rename="allProfileWitnesses")] pub f_2: Vec<S8NativePowderProfileWitness>,
#[serde(rename="actualElementRows")] pub f_3: Vec<S8ScientificSourceRow>,
#[serde(rename="parameterDigest")] pub f_4: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeIsoPhaseBuffer { #[serde(rename="phaseId")] pub f_0: S8FS1Id,
#[serde(rename="source")] pub f_1: S8ScalarSource,
#[serde(rename="actualStoredVectorF32Bits")] pub f_2: Vec<S8F32Bits>,
#[serde(rename="includesActualStoredClosure")] pub f_3: bool,
#[serde(rename="sourceDescriptorDigest")] pub f_4: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeDefaultIsoInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="profile")] pub f_1: String,
#[serde(rename="orderedActualPhaseBuffers")] pub f_2: Vec<S8NativeIsoPhaseBuffer>,
#[serde(rename="parameterNF32Bits")] pub f_3: S8F32Bits,
#[serde(rename="preferenceLabel")] pub f_4: String,
#[serde(rename="preferenceProducerRuleDigest")] pub f_5: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeDefaultIsoOutput { #[serde(rename="countAllStoredValues")] pub f_0: i64,
#[serde(rename="signedMinF32Bits")] pub f_1: S8F32Bits,
#[serde(rename="signedMaxF32Bits")] pub f_2: S8F32Bits,
#[serde(rename="absoluteMeanF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="absolutePopulationStdDevF64Bits")] pub f_4: S8F64Bits,
#[serde(rename="storedLevelF32Bits")] pub f_5: S8F32Bits,
#[serde(rename="nativeSignModeI32")] pub f_6: f64,
#[serde(rename="signMode")] pub f_7: String,
#[serde(rename="parameterDigest")] pub f_8: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeFractionalCutoffPlane { #[serde(rename="normalFractionalF64Bits")] pub f_0: S8Vec3F64Bits,
#[serde(rename="distanceFractionalF64Bits")] pub f_1: S8F64Bits,
#[serde(rename="sourcePhaseId")] pub f_2: S8FS1Id,
#[serde(rename="sourceSliceId")] pub f_3: S8FS1Id,
#[serde(rename="producerRuleDigest")] pub f_4: S8FS1Digest,
#[serde(rename="orthogonalGeometry")] pub f_5: S8SliceNativeOrthoCutoffPlaneGeometry }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshPhase { #[serde(rename="phaseId")] pub f_0: S8FS1Id,
#[serde(rename="source")] pub f_1: S8ScalarSource,
#[serde(rename="colorSource")] pub f_2: S8NativeMeshPhaseF2,
#[serde(rename="nativeVisibility878U8")] pub f_3: i64,
#[serde(rename="fractionalBoundsInterleavedF32Bits")] pub f_4: Vec<S8F32Bits>,
#[serde(rename="cutoffPlanes")] pub f_5: Vec<S8NativeFractionalCutoffPlane>,
#[serde(rename="effectiveNativeMatrix4")] pub f_6: S8Mat4ColumnsF64Bits,
#[serde(rename="effectiveNativeTranslationF32Bits")] pub f_7: S8Vec3F32Bits,
#[serde(rename="layerQualificationDigest")] pub f_8: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeMeshPhaseF2 { V0(S8ScalarSource), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshTextureSettings { #[serde(rename="lowerRangeFractionF32Bits")] pub f_0: S8F32Bits,
#[serde(rename="upperRangeFractionF32Bits")] pub f_1: S8F32Bits,
#[serde(rename="signedTextureFactorI8")] pub f_2: i64,
#[serde(rename="sourceRuleDigest")] pub f_3: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativePowderFullOutputOrFailure { V0(S8NativePowderFullOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeDefaultIsoOutputOrFailure { V0(S8NativeDefaultIsoOutput), V1(S8NativeFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionReaderSettings { #[serde(rename="formatCodeI32")] pub f_0: f64,
#[serde(rename="coefficientSelectionI32")] pub f_1: i64,
#[serde(rename="calculatedF000ElectronF64Bits")] pub f_2: S8F64Bits,
#[serde(rename="calculatedF000NeutronF64Bits")] pub f_3: S8F64Bits,
#[serde(rename="incomingCurrentF000F64Bits")] pub f_4: S8F64Bits,
#[serde(rename="lineBufferBytes")] pub f_5: f64,
#[serde(rename="sourceRuleDigest")] pub f_6: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionReadInput { #[serde(rename="base")] pub f_0: S8FS1FSContext,
#[serde(rename="phaseId")] pub f_1: S8FS1Id,
#[serde(rename="profile")] pub f_2: String,
#[serde(rename="source")] pub f_3: S8FS1FSByteSpan,
#[serde(rename="settings")] pub f_4: S8NativeReflectionReaderSettings,
#[serde(rename="model")] pub f_5: S8SG2NativeDimensionDState,
#[serde(rename="completeElementRows")] pub f_6: Vec<S8ScientificSourceRow>,
#[serde(rename="maxOutputBytes")] pub f_7: i64,
#[serde(rename="maxResidentBytes")] pub f_8: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeSkippedReflectionRow { #[serde(rename="source")] pub f_0: S8FS1FSByteSpan,
#[serde(rename="sourceLineOrdinalZeroBased")] pub f_1: i64,
#[serde(rename="actualScanfAssignmentCountI32")] pub f_2: i64,
#[serde(rename="requiredAssignmentCountI32")] pub f_3: i64,
#[serde(rename="reason")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeReflectionReadOutput { #[serde(rename="source")] pub f_0: S8ReflectionSource,
#[serde(rename="completeRows")] pub f_1: S8NativeReflectionPopulation,
#[serde(rename="actualF000F64Bits")] pub f_2: S8F64Bits,
#[serde(rename="sourceOrderedSkippedRows")] pub f_3: Vec<S8NativeSkippedReflectionRow>,
#[serde(rename="nativeReturnI32")] pub f_4: i64,
#[serde(rename="nativeReadDoesNotCertifyAllRowsValid")] pub f_5: bool,
#[serde(rename="parameterDigest")] pub f_6: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8NativeReflectionReadOutputOrFailure { V0(S8NativeReflectionReadOutput), V1(S8NativeFailure) }

pub type S8SamplingNativePointSampleInput = S5NativePointSampleInput;

pub type S8SamplingNativePointSampleOutput = S5NativePointSampleOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SamplingNativePointSampleOutputOrFailure { V0(S8SamplingNativePointSampleOutput), V1(S8NativeFailure) }

pub type S8SamplingNativeLineProfileInput = S5NativeLineProfileInput;

pub type S8SamplingNativeLineProfileOutput = S5NativeLineProfileOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SamplingNativeLineProfileOutputOrFailure { V0(S8SamplingNativeLineProfileOutput), V1(S8NativeFailure) }

pub type S8SamplingNativeContourLevelsInput = S5NativeContourLevelsInput;

pub type S8SamplingNativeContourLevelsOutput = S5NativeContourLevelsOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SamplingNativeContourLevelsOutputOrFailure { V0(S8SamplingNativeContourLevelsOutput), V1(S8NativeFailure) }

pub type S8SamplingNativeContourSegmentsInput = S5NativeContourSegmentsInput;

pub type S8SamplingNativeContourSegmentsOutput = S5NativeContourSegmentsOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SamplingNativeContourSegmentsOutputOrFailure { V0(S8SamplingNativeContourSegmentsOutput), V1(S8NativeFailure) }

pub type S8SamplingNativePolyIntegralInput = S5NativePolyIntegralInput;

pub type S8SamplingNativePolyIntegralOutput = S5NativePolyIntegralOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SamplingNativePolyIntegralOutputOrFailure { V0(S8SamplingNativePolyIntegralOutput), V1(S8NativeFailure) }

pub type S8SliceNativeObliqueSliceInput = S12NativeObliqueSliceInput;

pub type S8SliceNativeObliqueSliceOutput = S12NativeObliqueSliceOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SliceNativeObliqueSliceOutputOrFailure { V0(S8SliceNativeObliqueSliceOutput), V1(S8NativeFailure) }

pub type S8SliceNativePeriodicSliceInput = S12NativePeriodicSliceInput;

pub type S8SliceNativePeriodicSliceOutput = S12NativePeriodicSliceOutput;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8SliceNativePeriodicSliceOutputOrFailure { V0(S8SliceNativePeriodicSliceOutput), V1(S8NativeFailure) }

pub type S8SliceNativeOrthoCutoffPlaneGeometry = S12NativeOrthoCutoffPlaneGeometry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativeMeshBranchRange { #[serde(rename="levelOrdinal")] pub f_0: i64,
#[serde(rename="sign")] pub f_1: f64,
#[serde(rename="firstVertexIndex")] pub f_2: i64,
#[serde(rename="endVertexIndexExclusive")] pub f_3: i64,
#[serde(rename="firstTriangleOrdinal")] pub f_4: i64,
#[serde(rename="endTriangleOrdinalExclusive")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8NativePowderEnumerationRecipe { #[serde(rename="sourceTwoThetaRangeDegreeF64Bits")] pub f_0: Vec<S8F64Bits>,
#[serde(rename="allBeamWavelengthsAF64Bits")] pub f_1: Vec<S8F64Bits>,
#[serde(rename="actualDRangeAF64Bits")] pub f_2: Vec<S8F64Bits>,
#[serde(rename="actualHklAxisBoundsI32")] pub f_3: Vec<i64>,
#[serde(rename="prototypeReflectionDimension")] pub f_4: f64,
#[serde(rename="rangeRule")] pub f_5: String,
#[serde(rename="populationAuthority")] pub f_6: String,
#[serde(rename="sourceRuleDigest")] pub f_7: S8FS1Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S8BoundaryRequest { V0(S8BoundaryRequestV0), V1(S8BoundaryRequestV1), V2(S8BoundaryRequestV2), V3(S8BoundaryRequestV3), V4(S8BoundaryRequestV4), V5(S8BoundaryRequestV5), V6(S8BoundaryRequestV6), V7(S8BoundaryRequestV7), V8(S8BoundaryRequestV8), V9(S8BoundaryRequestV9), V10(S8BoundaryRequestV10), V11(S8BoundaryRequestV11), V12(S8BoundaryRequestV12), V13(S8BoundaryRequestV13), V14(S8BoundaryRequestV14), V15(S8BoundaryRequestV15), V16(S8BoundaryRequestV16), V17(S8BoundaryRequestV17), V18(S8BoundaryRequestV18), V19(S8BoundaryRequestV19), V20(S8BoundaryRequestV20), V21(S8BoundaryRequestV21), V22(S8BoundaryRequestV22), V23(S8BoundaryRequestV23), V24(S8BoundaryRequestV24), V25(S8BoundaryRequestV25), V26(S8BoundaryRequestV26), V27(S8BoundaryRequestV27) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV0 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeFourierUiInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV1 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeGridNoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV2 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeTchWidthInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV3 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativePowderAccumulateInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV4 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8OrderedExpressionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV5 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8MathFSResampleAffineInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV6 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8MathFSLinearCombineFieldsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV7 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8MathFSCartesianLineInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV8 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8MathFSLinearCombineCurvesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV9 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeDispersionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV10 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeScatteringInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV11 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeFourierFieldInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV12 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeModelFieldInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV13 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeVolumeTransformInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV14 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativePeakSearchInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV15 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeVoronoiInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV16 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeMeshInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV17 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeConversionInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV18 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativePowderFullInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV19 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeDefaultIsoInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV20 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8NativeReflectionReadInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV21 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SamplingNativePointSampleInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV22 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SamplingNativeLineProfileInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV23 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SamplingNativeContourLevelsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV24 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SamplingNativeContourSegmentsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV25 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SamplingNativePolyIntegralInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV26 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SliceNativeObliqueSliceInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S8BoundaryRequestV27 { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="input")] pub f_2: S8SliceNativePeriodicSliceInput }

pub type S8Root = S8BoundaryRequest;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10FSNativeMeshOutput { #[serde(rename="verticesCartesianF32Bits")] pub f_0: Vec<S10FSVec3F32Bits>,
#[serde(rename="triangles")] pub f_1: Vec<S10FSNativeMeshTriangle>,
#[serde(rename="faceNormalsF32Bits")] pub f_2: Vec<S10FSVec3F32Bits>,
#[serde(rename="vertexNormalsF32Bits")] pub f_3: Vec<S10FSVec3F32Bits>,
#[serde(rename="levelBindings")] pub f_4: S10FSFCNativeStoredScalarIsoview1,
#[serde(rename="layerBindings")] pub f_5: S10FSFCNativeLayerCurrent1,
#[serde(rename="sourceDescriptorDigest")] pub f_6: S10FSFS1Digest,
#[serde(rename="nativeNonfiniteTriangleOrdinals")] pub f_7: Vec<i64>,
#[serde(rename="parameterDigest")] pub f_8: S10FSFS1Digest,
#[serde(rename="vertexColorScalarsF32Bits")] pub f_9: Vec<S10FSF32Bits>,
#[serde(rename="vertexTextureCoordinatesF32Bits")] pub f_10: Vec<S10FSF32Bits>,
#[serde(rename="completeClippedPolygonVertexIndices")] pub f_11: Vec<Vec<i64>>,
#[serde(rename="sourcePhaseByTriangle")] pub f_12: Vec<S10FSFS1Id>,
#[serde(rename="stage")] pub f_13: String,
#[serde(rename="levelBranchRanges")] pub f_14: Vec<S10FSNativeMeshBranchRange>,
#[serde(rename="nativeNonfiniteUsedVertexOrdinals")] pub f_15: Vec<i64>,
#[serde(rename="nativeNonfiniteUnusedVertexOrdinals")] pub f_16: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10FSNativeMeshBranchRange { #[serde(rename="levelOrdinal")] pub f_0: i64,
#[serde(rename="sign")] pub f_1: f64,
#[serde(rename="firstVertexIndex")] pub f_2: i64,
#[serde(rename="endVertexIndexExclusive")] pub f_3: i64,
#[serde(rename="firstTriangleOrdinal")] pub f_4: i64,
#[serde(rename="endTriangleOrdinalExclusive")] pub f_5: i64 }

pub type S10FSFS1Id = S10IM21Id;

pub type S10IM21Id = String;

pub type S10FSF32Bits = String;

pub type S10FSFS1Digest = S10IM21Digest;

pub type S10IM21Digest = String;

pub type S10FSFCNativeLayerCurrent1 = S10IM72ROOT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOT { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="captureRole")] pub f_1: String,
#[serde(rename="document")] pub f_2: S10IM72ROOTF2,
#[serde(rename="orderedEntries")] pub f_3: Vec<S10IM72ROOTF3Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3Item { #[serde(rename="owner")] pub f_0: S10IM72ROOTF3ItemF0,
#[serde(rename="position")] pub f_1: S10IM72ROOTF3ItemF1,
#[serde(rename="orientation")] pub f_2: S10IM72ROOTF3ItemF2,
#[serde(rename="effectiveNativeMatrix4")] pub f_3: Vec<S10IM72ROOTF3ItemF3Item>,
#[serde(rename="effectiveNativeTranslation")] pub f_4: Vec<S10IM72ROOTF3ItemF4Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10IM72ROOTF3ItemF0 { V0(S10IM72ROOTF3ItemF0V0), V1(S10IM72ROOTF3ItemF0V1), V2(S10IM72ROOTF3ItemF0V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF0V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF0V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF0V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1 { #[serde(rename="selector")] pub f_0: S10IM72ROOTF3ItemF1F0,
#[serde(rename="currentAnchor")] pub f_1: Vec<S10IM72ROOTF3ItemF1F1Item>,
#[serde(rename="referenceAnchor")] pub f_2: Vec<S10IM72ROOTF3ItemF1F2Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0 { #[serde(rename="capturedSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S10IM72ROOTF3ItemF1F0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10IM72ROOTF3ItemF1F0F1 { V0(S10IM72ROOTF3ItemF1F0F1V0), V1(S10IM72ROOTF3ItemF1F0F1V1), V2(S10IM72ROOTF3ItemF1F0F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="target")] pub f_1: S10IM72ROOTF3ItemF1F0F1V1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10IM72ROOTF3ItemF1F0F1V1F1 { V0(S10IM72ROOTF3ItemF1F0F1V1F1V0), V1(S10IM72ROOTF3ItemF1F0F1V1F1V1), V2(S10IM72ROOTF3ItemF1F0F1V1F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V1F1V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V1F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V1F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F0F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F1Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF1F2Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2 { #[serde(rename="selector")] pub f_0: S10IM72ROOTF3ItemF2F0,
#[serde(rename="storedFlagBits")] pub f_1: String,
#[serde(rename="currentFirst")] pub f_2: Vec<S10IM72ROOTF3ItemF2F2Item>,
#[serde(rename="referenceFirst")] pub f_3: Vec<S10IM72ROOTF3ItemF2F3Item>,
#[serde(rename="currentSecond")] pub f_4: Vec<S10IM72ROOTF3ItemF2F4Item>,
#[serde(rename="referenceSecond")] pub f_5: Vec<S10IM72ROOTF3ItemF2F5Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0 { #[serde(rename="capturedSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S10IM72ROOTF3ItemF2F0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10IM72ROOTF3ItemF2F0F1 { V0(S10IM72ROOTF3ItemF2F0F1V0), V1(S10IM72ROOTF3ItemF2F0F1V1), V2(S10IM72ROOTF3ItemF2F0F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="target")] pub f_1: S10IM72ROOTF3ItemF2F0F1V1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10IM72ROOTF3ItemF2F0F1V1F1 { V0(S10IM72ROOTF3ItemF2F0F1V1F1V0), V1(S10IM72ROOTF3ItemF2F0F1V1F1V1), V2(S10IM72ROOTF3ItemF2F0F1V1F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V1F1V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V1F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V1F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F0F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F2Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F3Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F4Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF2F5Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF3Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM72ROOTF3ItemF4Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

pub type S10FSFCNativeStoredScalarIsoview1 = S10IM93ROOT;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM93ROOT { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="target")] pub f_1: S10IM93ROOTF1,
#[serde(rename="levels")] pub f_2: Vec<S10IM93ROOTF2Item>,
#[serde(rename="levelMeaning")] pub f_3: String,
#[serde(rename="ruleId")] pub f_4: String,
#[serde(rename="ruleVersion")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM93ROOTF1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10IM93ROOTF2Item { #[serde(rename="storedValueF32Bits")] pub f_0: String,
#[serde(rename="signMode")] pub f_1: String }

pub type S10FSVec3F32Bits = Vec<S10FSF32Bits>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10FSNativeMeshTriangle { #[serde(rename="levelOrdinal")] pub f_0: i64,
#[serde(rename="sign")] pub f_1: f64,
#[serde(rename="coreCellIndex")] pub f_2: i64,
#[serde(rename="nativeCaseId")] pub f_3: i64,
#[serde(rename="sourceRowTriangleOrdinal")] pub f_4: i64,
#[serde(rename="edgeIds")] pub f_5: Vec<i64>,
#[serde(rename="vertexIndices")] pub f_6: Vec<i64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10SGNativeCamera { #[serde(rename="sceneMatrixLogicalRows")] pub f_0: S10SGMatrix4Rows,
#[serde(rename="shiftViewX")] pub f_1: f64,
#[serde(rename="shiftViewY")] pub f_2: f64,
#[serde(rename="thirdSceneStoredF32")] pub f_3: f64,
#[serde(rename="zoomF32")] pub f_4: f64,
#[serde(rename="projectionSelectorBit21")] pub f_5: f64,
#[serde(rename="projectionZScaleF32")] pub f_6: f64,
#[serde(rename="localTranslationA")] pub f_7: Vec<f64>,
#[serde(rename="normalizationF64")] pub f_8: f64,
#[serde(rename="viewportLogicalWidth")] pub f_9: i64,
#[serde(rename="viewportLogicalHeight")] pub f_10: i64,
#[serde(rename="deviceScaleF64")] pub f_11: f64,
#[serde(rename="aspectF32")] pub f_12: f64,
#[serde(rename="aspectScaleF32")] pub f_13: f64,
#[serde(rename="projectionContext")] pub f_14: String,
#[serde(rename="nearView")] pub f_15: f64,
#[serde(rename="farView")] pub f_16: f64 }

pub type S10SGMatrix4Rows = Vec<S10SGVec4>;

pub type S10SGVec4 = Vec<f64>;

pub type S10Digest = String;

pub type S10F32Bits = String;

pub type S10F64Bits = String;

pub type S10Vec3F32Bits = Vec<S10F32Bits>;

pub type S10Vec3F64Bits = Vec<S10F64Bits>;

pub type S10RGBAU8 = Vec<i64>;

pub type S10RGBU8 = Vec<i64>;

pub type S10Triangle = Vec<i64>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10NativeLevelAppearance { #[serde(rename="levelOrdinal")] pub f_0: i64,
#[serde(rename="rgbSourceU8")] pub f_1: S10RGBU8,
#[serde(rename="alphaFacingU8")] pub f_2: i64,
#[serde(rename="alphaGrazingU8")] pub f_3: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10NativeSurfaceDisplay { #[serde(rename="scene130U8")] pub f_0: i64,
#[serde(rename="scene5B0U32")] pub f_1: i64,
#[serde(rename="scene354U16")] pub f_2: i64,
#[serde(rename="scene356U16")] pub f_3: i64,
#[serde(rename="scene2F2SelectorI8")] pub f_4: i64,
#[serde(rename="surfaceSpecularRGBAU8")] pub f_5: S10RGBAU8,
#[serde(rename="surfaceShininessF32Bits")] pub f_6: S10F32Bits,
#[serde(rename="exportSpecularRGB3DCU8")] pub f_7: S10RGBU8,
#[serde(rename="exportShininess3E0F32Bits")] pub f_8: S10F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10OwnedPalette { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="sourceSlot")] pub f_2: S10OwnedPaletteF2,
#[serde(rename="format")] pub f_3: String,
#[serde(rename="width")] pub f_4: f64,
#[serde(rename="bytesHex")] pub f_5: String,
#[serde(rename="contentDigest")] pub f_6: S10Digest,
#[serde(rename="sourceRuleDigest")] pub f_7: S10Digest,
#[serde(rename="samplerWrapS")] pub f_8: String,
#[serde(rename="minFilter")] pub f_9: String,
#[serde(rename="magFilter")] pub f_10: String,
#[serde(rename="nativeBindingQualificationDigest")] pub f_11: S10OwnedPaletteF11 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10OwnedPaletteF2 { V0(i64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10OwnedPaletteF11 { V0(S10Digest), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10NativeBoundaryDisplayMesh { #[serde(rename="meshId")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: String,
#[serde(rename="sliceId")] pub f_2: String,
#[serde(rename="kind")] pub f_3: String,
#[serde(rename="sourceNormalF64Bits")] pub f_4: S10Vec3F64Bits,
#[serde(rename="verticesCartesianF32Bits")] pub f_5: Vec<S10Vec3F32Bits>,
#[serde(rename="triangles")] pub f_6: Vec<S10Triangle>,
#[serde(rename="textureCoordinatesF32Bits")] pub f_7: Vec<S10F32Bits>,
#[serde(rename="rgbaSourceU8")] pub f_8: S10RGBAU8,
#[serde(rename="sourceRuleDigest")] pub f_9: S10Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10DepthState { #[serde(rename="testEnabled")] pub f_0: bool,
#[serde(rename="writeEnabled")] pub f_1: bool,
#[serde(rename="comparison")] pub f_2: String,
#[serde(rename="sourceRuleDigest")] pub f_3: S10Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10RenderNativeIsosurfacePacketRequest { #[serde(rename="namespace")] pub f_0: String,
#[serde(rename="profile")] pub f_1: String,
#[serde(rename="preCameraMesh")] pub f_2: S10FSNativeMeshOutput,
#[serde(rename="camera")] pub f_3: S10SGNativeCamera,
#[serde(rename="depthState")] pub f_4: S10DepthState,
#[serde(rename="inheritedPolygonMode")] pub f_5: String,
#[serde(rename="levelAppearanceInSourceOrder")] pub f_6: Vec<S10NativeLevelAppearance>,
#[serde(rename="display")] pub f_7: S10NativeSurfaceDisplay,
#[serde(rename="palette")] pub f_8: S10RenderNativeIsosurfacePacketRequestF8,
#[serde(rename="boundaryDisplayMeshes")] pub f_9: Vec<S10NativeBoundaryDisplayMesh>,
#[serde(rename="ownerId")] pub f_10: String,
#[serde(rename="revision")] pub f_11: i64,
#[serde(rename="sourceRuleDigest")] pub f_12: S10Digest,
#[serde(rename="maximumPacketBytes")] pub f_13: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10RenderNativeIsosurfacePacketRequestF8 { V0(S10OwnedPalette), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10BoundaryPass { #[serde(rename="passValue")] pub f_0: f64,
#[serde(rename="position")] pub f_1: String,
#[serde(rename="admittedMeshIds")] pub f_2: Vec<String>,
#[serde(rename="rejectedMeshIds")] pub f_3: Vec<String>,
#[serde(rename="normalZeroInBothPasses")] pub f_4: bool,
#[serde(rename="lighting")] pub f_5: String,
#[serde(rename="textureEnvironment")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10SurfaceMaterialPlan { #[serde(rename="specularNormalizedRGBAF32Bits")] pub f_0: Vec<S10F32Bits>,
#[serde(rename="shininessF32Bits")] pub f_1: S10F32Bits,
#[serde(rename="faces")] pub f_2: String,
#[serde(rename="polygonMode")] pub f_3: String,
#[serde(rename="lineWidthF32Bits")] pub f_4: S10F32Bits,
#[serde(rename="pointSizeF32Bits")] pub f_5: S10F32Bits,
#[serde(rename="blendSrc")] pub f_6: String,
#[serde(rename="blendDst")] pub f_7: String,
#[serde(rename="depthState")] pub f_8: S10DepthState }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10PacketDiagnostic { #[serde(rename="code")] pub f_0: String,
#[serde(rename="fieldPath")] pub f_1: String,
#[serde(rename="sourceIndices")] pub f_2: Vec<i64>,
#[serde(rename="detail")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10RenderNativeIsosurfacePacketSuccess { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="namespace")] pub f_1: String,
#[serde(rename="stage")] pub f_2: String,
#[serde(rename="sourceMeshParameterDigest")] pub f_3: S10Digest,
#[serde(rename="ownerId")] pub f_4: String,
#[serde(rename="revision")] pub f_5: i64,
#[serde(rename="surfaceVisible")] pub f_6: bool,
#[serde(rename="vertexNormalViewZF32Bits")] pub f_7: Vec<S10RenderNativeIsosurfacePacketSuccessF7Item>,
#[serde(rename="vertexRGBAU8")] pub f_8: Vec<S10RenderNativeIsosurfacePacketSuccessF8Item>,
#[serde(rename="sourceTriangleOrdinalsInDrawOrder")] pub f_9: Vec<i64>,
#[serde(rename="triangleIndicesInDrawOrder")] pub f_10: Vec<S10Triangle>,
#[serde(rename="palette")] pub f_11: S10RenderNativeIsosurfacePacketSuccessF11,
#[serde(rename="material")] pub f_12: S10SurfaceMaterialPlan,
#[serde(rename="filledBoundaryPasses")] pub f_13: Vec<S10BoundaryPass>,
#[serde(rename="diagnostics")] pub f_14: Vec<S10PacketDiagnostic>,
#[serde(rename="parameterDigest")] pub f_15: S10Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10RenderNativeIsosurfacePacketSuccessF7Item { V0(S10F32Bits), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10RenderNativeIsosurfacePacketSuccessF8Item { V0(S10RGBAU8), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10RenderNativeIsosurfacePacketSuccessF11 { V0(S10OwnedPalette), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S10RenderNativeIsosurfacePacketFailure { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="namespace")] pub f_1: String,
#[serde(rename="diagnostics")] pub f_2: Vec<S10PacketDiagnostic>,
#[serde(rename="sourceMeshParameterDigest")] pub f_3: S10Digest,
#[serde(rename="ownerId")] pub f_4: String,
#[serde(rename="revision")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10RenderNativeIsosurfacePacketResponse { V0(S10RenderNativeIsosurfacePacketSuccess), V1(S10RenderNativeIsosurfacePacketFailure) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S10Root { V0(S10RenderNativeIsosurfacePacketRequest), V1(S10RenderNativeIsosurfacePacketResponse) }

pub type S6Id = String;

pub type S6Digest = String;

pub type S6U64 = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6ArtifactChoice { #[serde(rename="artifactId")] pub f_0: String,
#[serde(rename="manifestDigest")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6FontChoice { #[serde(rename="fontResourceDigest")] pub f_0: String,
#[serde(rename="faceIndex")] pub f_1: i64,
#[serde(rename="sizePt")] pub f_2: f64,
#[serde(rename="fontCapabilityDigest")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6NumericDraft { #[serde(rename="rawText")] pub f_0: String,
#[serde(rename="domainProfileDigest")] pub f_1: String,
#[serde(rename="parseProfileDigest")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6PreferenceChange { V0(S6PreferenceChangeV0), V1(S6PreferenceChangeV1), V2(S6PreferenceChangeV2), V3(S6PreferenceChangeV3), V4(S6PreferenceChangeV4), V5(S6PreferenceChangeV5), V6(S6PreferenceChangeV6), V7(S6PreferenceChangeV7), V8(S6PreferenceChangeV8), V9(S6PreferenceChangeV9), V10(S6PreferenceChangeV10), V11(S6PreferenceChangeV11) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV0 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6PreferenceChangeV0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6PreferenceChangeV0F1 { V0(S6ArtifactChoice), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV1 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6PreferenceChangeV1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6PreferenceChangeV1F1 { V0(S6ArtifactChoice), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV2 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6PreferenceChangeV2F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6PreferenceChangeV2F1 { V0(S6ArtifactChoice), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV3 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6FontChoice }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV4 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV5 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6NumericDraft }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV6 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6NumericDraft }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV7 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV8 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV9 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6NumericDraft }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV10 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6NumericDraft }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6PreferenceChangeV11 { #[serde(rename="field")] pub f_0: String,
#[serde(rename="value")] pub f_1: S6NumericDraft }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6ValidateRequest { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="basePreferenceVersion")] pub f_2: String,
#[serde(rename="baseEffectiveDigest")] pub f_3: String,
#[serde(rename="ownerProfileDigest")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="changes")] pub f_6: Vec<S6PreferenceChange> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6ApplyRequest { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="basePreferenceVersion")] pub f_2: String,
#[serde(rename="baseEffectiveDigest")] pub f_3: String,
#[serde(rename="ownerProfileDigest")] pub f_4: String,
#[serde(rename="op")] pub f_5: String,
#[serde(rename="validatedPlanDigest")] pub f_6: String,
#[serde(rename="trimConsent")] pub f_7: S6ApplyRequestF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6ApplyRequestF7 { V0(S6ApplyRequestF7V0), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6ApplyRequestF7V0 { #[serde(rename="lostSnapshotIds")] pub f_0: Vec<String>,
#[serde(rename="planDigest")] pub f_1: String,
#[serde(rename="explicit")] pub f_2: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6QueryRequest { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="op")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6Request { V0(S6ValidateRequest), V1(S6ApplyRequest), V2(S6QueryRequest) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6AppliedReply { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="outcome")] pub f_2: String,
#[serde(rename="effectivePreferenceVersion")] pub f_3: String,
#[serde(rename="actualGenerationDigest")] pub f_4: String,
#[serde(rename="installedEffectiveDigest")] pub f_5: String,
#[serde(rename="changedFields")] pub f_6: Vec<String>,
#[serde(rename="currentDocumentMutated")] pub f_7: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S6UnknownReply { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="outcome")] pub f_2: String,
#[serde(rename="attemptedGenerationDigest")] pub f_3: String,
#[serde(rename="effectivePreferenceVersionBefore")] pub f_4: String,
#[serde(rename="retryAction")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S6Reply { V0(S6AppliedReply), V1(S6UnknownReply) }

pub type S6Root = S6Request;

pub type S7Id = String;

pub type S7Digest = String;

pub type S7U64 = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7ResourceRef { #[serde(rename="sha256")] pub f_0: String,
#[serde(rename="byteLength")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7LossConsent { V0(()), V1(S7LossConsentV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7LossConsentV1 { #[serde(rename="sourceDigest")] pub f_0: String,
#[serde(rename="profileId")] pub f_1: String,
#[serde(rename="lossReportSha256")] pub f_2: String,
#[serde(rename="explicit")] pub f_3: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7Request { V0(S7RequestV0), V1(S7RequestV1), V2(S7RequestV2), V3(S7RequestV3) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV0 { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="sessionId")] pub f_2: String,
#[serde(rename="documentId")] pub f_3: String,
#[serde(rename="epoch")] pub f_4: String,
#[serde(rename="expectedRevision")] pub f_5: String,
#[serde(rename="operation")] pub f_6: String,
#[serde(rename="params")] pub f_7: S7RequestV0F7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV0F7 { #[serde(rename="phaseId")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV1 { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="sessionId")] pub f_2: String,
#[serde(rename="documentId")] pub f_3: String,
#[serde(rename="epoch")] pub f_4: String,
#[serde(rename="expectedRevision")] pub f_5: String,
#[serde(rename="operation")] pub f_6: String,
#[serde(rename="params")] pub f_7: S7RequestV1F7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV1F7 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="radiusA")] pub f_1: f64,
#[serde(rename="regionInvA")] pub f_2: f64,
#[serde(rename="lossConsent")] pub f_3: S7LossConsent }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV2 { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="sessionId")] pub f_2: String,
#[serde(rename="documentId")] pub f_3: String,
#[serde(rename="epoch")] pub f_4: String,
#[serde(rename="expectedRevision")] pub f_5: String,
#[serde(rename="operation")] pub f_6: String,
#[serde(rename="params")] pub f_7: S7RequestV2F7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV2F7 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="template")] pub f_1: S7ResourceRef,
#[serde(rename="companions")] pub f_2: Vec<S7RequestV2F7F2Item>,
#[serde(rename="lossConsent")] pub f_3: S7LossConsent }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV2F7F2Item { #[serde(rename="role")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV3 { #[serde(rename="schema")] pub f_0: String,
#[serde(rename="requestId")] pub f_1: String,
#[serde(rename="sessionId")] pub f_2: String,
#[serde(rename="documentId")] pub f_3: String,
#[serde(rename="epoch")] pub f_4: String,
#[serde(rename="expectedRevision")] pub f_5: String,
#[serde(rename="operation")] pub f_6: String,
#[serde(rename="params")] pub f_7: S7RequestV3F7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7RequestV3F7 { #[serde(rename="phaseId")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7MadelNumericInterpretation { #[serde(rename="producerParserDigest")] pub f_0: S7Digest,
#[serde(rename="inputDigest")] pub f_1: S7Digest,
#[serde(rename="potentialUnit")] pub f_2: String,
#[serde(rename="energyUnit")] pub f_3: String,
#[serde(rename="energyAsymmetricDecimal")] pub f_4: String,
#[serde(rename="generalMultiplicity")] pub f_5: S7U64,
#[serde(rename="energyCellDecimal")] pub f_6: String,
#[serde(rename="formulaUnitsPerCell")] pub f_7: S7MadelNumericInterpretationF7,
#[serde(rename="energyPerFormulaDecimal")] pub f_8: S7MadelNumericInterpretationF8,
#[serde(rename="sitePotentials")] pub f_9: Vec<S7MadelNumericInterpretationF9Item>,
#[serde(rename="numericAcceptanceReceipt")] pub f_10: S7MadelNumericInterpretationF10,
#[serde(rename="energyEVAsymmetricDecimal")] pub f_11: String,
#[serde(rename="energyMJmolAsymmetricDecimal")] pub f_12: String,
#[serde(rename="reportProfileDigest")] pub f_13: S7Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7MadelNumericInterpretationF7 { V0(()), V1(S7U64) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7MadelNumericInterpretationF8 { V0(()), V1(String) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7MadelNumericInterpretationF9Item { #[serde(rename="emittedRow")] pub f_0: S7U64,
#[serde(rename="sourceSiteMapDigest")] pub f_1: S7Digest,
#[serde(rename="potentialDecimal")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7MadelNumericInterpretationF10 { V0(()), V1(S7ResourceRef) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7TemplateRevision { #[serde(rename="originalTemplate")] pub f_0: S7ResourceRef,
#[serde(rename="preparedInputIns")] pub f_1: S7ResourceRef,
#[serde(rename="returnedIns")] pub f_2: S7ResourceRef,
#[serde(rename="sourcePhaseDigest")] pub f_3: S7Digest,
#[serde(rename="preparedPhaseDigest")] pub f_4: S7Digest,
#[serde(rename="sourceMap")] pub f_5: S7ResourceRef,
#[serde(rename="patchLedger")] pub f_6: S7ResourceRef,
#[serde(rename="controlProfileDigest")] pub f_7: S7Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7WorkflowResult { V0(S7WorkflowResultV0), V1(S7WorkflowResultV1), V2(S7WorkflowResultV2), V3(S7WorkflowResultV3), V4(S7WorkflowResultV4), V5(S7WorkflowResultV5) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV0 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="report")] pub f_10: S7ResourceRef,
#[serde(rename="candidatePhase")] pub f_11: S7ResourceRef,
#[serde(rename="sourceToResultMap")] pub f_12: S7ResourceRef,
#[serde(rename="validationReceipt")] pub f_13: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV1 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="report")] pub f_10: S7ResourceRef,
#[serde(rename="templateRevision")] pub f_11: S7TemplateRevision,
#[serde(rename="plotDialect")] pub f_12: String,
#[serde(rename="artifacts")] pub f_13: Vec<S7WorkflowResultV1F13Item>,
#[serde(rename="plotOutcome")] pub f_14: String,
#[serde(rename="numericAcceptanceReceipt")] pub f_15: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV1F13Item { #[serde(rename="role")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV2 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="sourceResource")] pub f_10: S7ResourceRef,
#[serde(rename="sourceTable")] pub f_11: S7ResourceRef,
#[serde(rename="activeMapping")] pub f_12: S7ResourceRef,
#[serde(rename="validationReceipt")] pub f_13: S7ResourceRef }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV3 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="report")] pub f_10: S7ResourceRef,
#[serde(rename="diagnostics")] pub f_11: S7ResourceRef,
#[serde(rename="numericInterpretation")] pub f_12: (),
#[serde(rename="convergenceState")] pub f_13: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV4 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="report")] pub f_10: S7ResourceRef,
#[serde(rename="diagnostics")] pub f_11: S7ResourceRef,
#[serde(rename="numericInterpretation")] pub f_12: S7MadelNumericInterpretation,
#[serde(rename="convergenceState")] pub f_13: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV5 { #[serde(rename="operation")] pub f_0: String,
#[serde(rename="jobId")] pub f_1: S7Id,
#[serde(rename="sessionId")] pub f_2: S7Id,
#[serde(rename="documentId")] pub f_3: S7Id,
#[serde(rename="epoch")] pub f_4: S7Id,
#[serde(rename="baseRevision")] pub f_5: S7U64,
#[serde(rename="phaseId")] pub f_6: S7Id,
#[serde(rename="inputManifestDigest")] pub f_7: S7Digest,
#[serde(rename="workflowProfileDigest")] pub f_8: S7Digest,
#[serde(rename="kind")] pub f_9: String,
#[serde(rename="report")] pub f_10: S7ResourceRef,
#[serde(rename="diagnostics")] pub f_11: S7ResourceRef,
#[serde(rename="numericInterpretation")] pub f_12: S7WorkflowResultV5F12,
#[serde(rename="convergenceState")] pub f_13: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV5F12 { #[serde(rename="producerParserDigest")] pub f_0: S7Digest,
#[serde(rename="inputDigest")] pub f_1: S7Digest,
#[serde(rename="potentialUnit")] pub f_2: String,
#[serde(rename="energyUnit")] pub f_3: String,
#[serde(rename="energyAsymmetricDecimal")] pub f_4: String,
#[serde(rename="generalMultiplicity")] pub f_5: S7U64,
#[serde(rename="energyCellDecimal")] pub f_6: String,
#[serde(rename="formulaUnitsPerCell")] pub f_7: S7WorkflowResultV5F12F7,
#[serde(rename="energyPerFormulaDecimal")] pub f_8: S7WorkflowResultV5F12F8,
#[serde(rename="sitePotentials")] pub f_9: Vec<S7WorkflowResultV5F12F9Item>,
#[serde(rename="numericAcceptanceReceipt")] pub f_10: S7ResourceRef,
#[serde(rename="energyEVAsymmetricDecimal")] pub f_11: String,
#[serde(rename="energyMJmolAsymmetricDecimal")] pub f_12: String,
#[serde(rename="reportProfileDigest")] pub f_13: S7Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7WorkflowResultV5F12F7 { V0(()), V1(S7U64) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S7WorkflowResultV5F12F8 { V0(()), V1(String) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S7WorkflowResultV5F12F9Item { #[serde(rename="emittedRow")] pub f_0: S7U64,
#[serde(rename="sourceSiteMapDigest")] pub f_1: S7Digest,
#[serde(rename="potentialDecimal")] pub f_2: String }

pub type S7Root = S7Request;

pub type S0Id = String;

pub type S0Digest = String;

pub type S0Vec3 = Vec<f64>;

pub type S0Int3 = Vec<i64>;

pub type S0Mat3 = Vec<S0Vec3>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Resource { #[serde(rename="digest")] pub f_0: S0Digest,
#[serde(rename="bytes")] pub f_1: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Species { #[serde(rename="element")] pub f_0: S0SpeciesF0,
#[serde(rename="charge")] pub f_1: S0SpeciesF1,
#[serde(rename="massNumber")] pub f_2: S0SpeciesF2,
#[serde(rename="rawToken")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SpeciesF0 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SpeciesF1 { V0(i64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SpeciesF2 { V0(i64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Cell { #[serde(rename="lengthA")] pub f_0: Vec<f64>,
#[serde(rename="anglesDeg")] pub f_1: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0Adp { V0(S0AdpV0), V1(S0AdpV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0AdpV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="uA2")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0AdpV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="basis")] pub f_1: String,
#[serde(rename="six")] pub f_2: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Parameter { #[serde(rename="object")] pub f_0: S0Id,
#[serde(rename="field")] pub f_1: String,
#[serde(rename="unit")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Covariance { #[serde(rename="parameters")] pub f_0: Vec<S0Parameter>,
#[serde(rename="matrix")] pub f_1: Vec<Vec<f64>>,
#[serde(rename="source")] pub f_2: String,
#[serde(rename="sourceDigest")] pub f_3: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Site { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="label")] pub f_1: String,
#[serde(rename="species")] pub f_2: S0Species,
#[serde(rename="fractional")] pub f_3: S0Vec3,
#[serde(rename="occupancy")] pub f_4: f64,
#[serde(rename="adp")] pub f_5: S0SiteF5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SiteF5 { V0(S0Adp), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SymmetryOp { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="matrix")] pub f_1: Vec<S0Int3>,
#[serde(rename="translation")] pub f_2: Vec<S0SymmetryOpF2Item>,
#[serde(rename="timeReversal")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SymmetryOpF2Item { #[serde(rename="numerator")] pub f_0: i64,
#[serde(rename="denominator")] pub f_1: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Structure { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="sites")] pub f_1: Vec<S0Site>,
#[serde(rename="operations")] pub f_2: Vec<S0SymmetryOp>,
#[serde(rename="covariance")] pub f_3: S0StructureF3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0StructureF3 { V0(S0Covariance), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Placement { #[serde(rename="phase")] pub f_0: S0Id,
#[serde(rename="parent")] pub f_1: S0PlacementF1,
#[serde(rename="rotation")] pub f_2: S0Mat3,
#[serde(rename="translationA")] pub f_3: S0Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0PlacementF1 { V0(S0Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Image { #[serde(rename="phase")] pub f_0: S0Id,
#[serde(rename="site")] pub f_1: S0Id,
#[serde(rename="operation")] pub f_2: S0Id,
#[serde(rename="shift")] pub f_3: S0Int3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ImagePoint { #[serde(rename="image")] pub f_0: S0Image,
#[serde(rename="cartesianA")] pub f_1: S0Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Field { #[serde(rename="resource")] pub f_0: S0Resource,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="unit")] pub f_2: String,
#[serde(rename="declaredUnit")] pub f_3: S0FieldF3,
#[serde(rename="coreShape")] pub f_4: Vec<i64>,
#[serde(rename="closure")] pub f_5: Vec<bool>,
#[serde(rename="periodic")] pub f_6: Vec<bool>,
#[serde(rename="axisOrder")] pub f_7: String,
#[serde(rename="originA")] pub f_8: S0Vec3,
#[serde(rename="dtype")] pub f_9: String,
#[serde(rename="stepBasisA")] pub f_10: S0Mat3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FieldF3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Reflection { #[serde(rename="hkl")] pub f_0: S0Int3,
#[serde(rename="real")] pub f_1: f64,
#[serde(rename="imaginary")] pub f_2: f64,
#[serde(rename="unit")] pub f_3: String,
#[serde(rename="source")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Mesh { #[serde(rename="verticesA")] pub f_0: Vec<S0Vec3>,
#[serde(rename="triangles")] pub f_1: Vec<Vec<i64>>,
#[serde(rename="normals")] pub f_2: S0MeshF2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0MeshF2 { V0(Vec<S0Vec3>), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Camera { #[serde(rename="rotation")] pub f_0: S0Mat3,
#[serde(rename="projection")] pub f_1: String,
#[serde(rename="zoom")] pub f_2: f64,
#[serde(rename="widthA")] pub f_3: S0CameraF3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0CameraF3 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Diagnostic { #[serde(rename="code")] pub f_0: String,
#[serde(rename="object")] pub f_1: S0DiagnosticF1,
#[serde(rename="field")] pub f_2: S0DiagnosticF2,
#[serde(rename="message")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0DiagnosticF1 { V0(S0Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0DiagnosticF2 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Plane { #[serde(rename="unitNormalCartesian")] pub f_0: S0Vec3,
#[serde(rename="offsetA")] pub f_1: f64,
#[serde(rename="spacingA")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Metric { #[serde(rename="name")] pub f_0: String,
#[serde(rename="value")] pub f_1: f64,
#[serde(rename="unit")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Peak { #[serde(rename="fractional")] pub f_0: S0Vec3,
#[serde(rename="value")] pub f_1: f64,
#[serde(rename="kind")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Line { #[serde(rename="distanceA")] pub f_0: Vec<f64>,
#[serde(rename="values")] pub f_1: Vec<f64>,
#[serde(rename="fieldUnit")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Slice { #[serde(rename="width")] pub f_0: i64,
#[serde(rename="height")] pub f_1: i64,
#[serde(rename="values")] pub f_2: Vec<f64>,
#[serde(rename="originA")] pub f_3: S0Vec3,
#[serde(rename="uA")] pub f_4: S0Vec3,
#[serde(rename="vA")] pub f_5: S0Vec3,
#[serde(rename="unit")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Bundle { #[serde(rename="primary")] pub f_0: S0Resource,
#[serde(rename="dependencies")] pub f_1: Vec<S0BundleF1Item>,
#[serde(rename="losses")] pub f_2: Vec<S0Diagnostic>,
#[serde(rename="sourceDigest")] pub f_3: S0Digest,
#[serde(rename="profileId")] pub f_4: String,
#[serde(rename="lossReportSha256")] pub f_5: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0BundleF1Item { #[serde(rename="logicalPath")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S0Resource }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0AtomInstance { #[serde(rename="image")] pub f_0: S0Image,
#[serde(rename="centerA")] pub f_1: S0Vec3,
#[serde(rename="radiusA")] pub f_2: f64,
#[serde(rename="colorRGBA")] pub f_3: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransformResult { #[serde(rename="structure")] pub f_0: S0Structure,
#[serde(rename="oldToNew")] pub f_1: Vec<S0TransformResultF1Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransformResultF1Item { #[serde(rename="old")] pub f_0: S0Id,
#[serde(rename="new")] pub f_1: Vec<S0Id> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Curve { #[serde(rename="x")] pub f_0: Vec<f64>,
#[serde(rename="y")] pub f_1: Vec<f64>,
#[serde(rename="xUnit")] pub f_2: String,
#[serde(rename="yUnit")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Capability { #[serde(rename="action")] pub f_0: String,
#[serde(rename="referenceVersion")] pub f_1: String,
#[serde(rename="menuId")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ExternalFiles { #[serde(rename="files")] pub f_0: Vec<S0ExternalFilesF0Item>,
#[serde(rename="runAuthorized")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ExternalFilesF0Item { #[serde(rename="logicalPath")] pub f_0: String,
#[serde(rename="resource")] pub f_1: S0Resource }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0PattersonCoefficient { #[serde(rename="hkl")] pub f_0: S0Int3,
#[serde(rename="value")] pub f_1: f64,
#[serde(rename="unit")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ResolvePlacementsInput { #[serde(rename="placements")] pub f_0: Vec<S0Placement>,
#[serde(rename="root")] pub f_1: S0Id }

pub type S0ResolvePlacementsOutput = Vec<S0ResolvedPlacement>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ExpandImagesInput { #[serde(rename="phase")] pub f_0: S0Id,
#[serde(rename="structure")] pub f_1: S0Structure,
#[serde(rename="shifts")] pub f_2: Vec<S0Int3> }

pub type S0ExpandImagesOutput = Vec<S0ImagePoint>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ChangeBasisInput { #[serde(rename="structure")] pub f_0: S0Structure,
#[serde(rename="basisP")] pub f_1: S0Mat3,
#[serde(rename="originFractional")] pub f_2: S0Vec3,
#[serde(rename="mode")] pub f_3: String,
#[serde(rename="translations")] pub f_4: Vec<S0Int3> }

pub type S0ChangeBasisOutput = S0TransformResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransportAdpInput { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="adp")] pub f_1: S0Adp }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransportAdpOutput { #[serde(rename="cartesianUA2")] pub f_0: S0Mat3,
#[serde(rename="uEquivalentA2")] pub f_1: f64,
#[serde(rename="classification")] pub f_2: String,
#[serde(rename="inertia")] pub f_3: S0TransportAdpOutputF3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransportAdpOutputF3 { #[serde(rename="positive")] pub f_0: i64,
#[serde(rename="zero")] pub f_1: i64,
#[serde(rename="negative")] pub f_2: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FindDuplicatesInput { #[serde(rename="points")] pub f_0: Vec<S0ImagePoint>,
#[serde(rename="thresholdA")] pub f_1: f64 }

pub type S0FindDuplicatesOutput = Vec<Vec<S0Id>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SelectBondPairsInput { #[serde(rename="points")] pub f_0: Vec<S0ImagePoint>,
#[serde(rename="minimumA")] pub f_1: f64,
#[serde(rename="maximumA")] pub f_2: f64,
#[serde(rename="thresholdMode")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SelectBondPairsOutputItem { #[serde(rename="a")] pub f_0: S0Image,
#[serde(rename="b")] pub f_1: S0Image,
#[serde(rename="distanceA")] pub f_2: f64 }

pub type S0SelectBondPairsOutput = Vec<S0SelectBondPairsOutputItem>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0MeasurePolyhedronInput { #[serde(rename="centerA")] pub f_0: S0Vec3,
#[serde(rename="mesh")] pub f_1: S0Mesh }

pub type S0MeasurePolyhedronOutput = Vec<S0Metric>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0TransformVectorInput { #[serde(rename="vector")] pub f_0: S0Vec3,
#[serde(rename="cartesianOrthogonal")] pub f_1: S0Mat3,
#[serde(rename="kind")] pub f_2: String,
#[serde(rename="timeReversal")] pub f_3: f64 }

pub type S0TransformVectorOutput = S0Vec3;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0MillerPlaneInput { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="hkl")] pub f_1: S0Int3,
#[serde(rename="order")] pub f_2: i64 }

pub type S0MillerPlaneOutput = S0Plane;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ClipExplicitImagesInput { #[serde(rename="points")] pub f_0: Vec<S0ImagePoint>,
#[serde(rename="cell")] pub f_1: S0Cell,
#[serde(rename="minimumFractional")] pub f_2: S0Vec3,
#[serde(rename="maximumFractional")] pub f_3: S0Vec3 }

pub type S0ClipExplicitImagesOutput = Vec<S0ImagePoint>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ComposeCameraInput { #[serde(rename="camera")] pub f_0: S0Camera,
#[serde(rename="xDeg")] pub f_1: f64,
#[serde(rename="yDeg")] pub f_2: f64,
#[serde(rename="zDeg")] pub f_3: f64 }

pub type S0ComposeCameraOutput = S0Camera;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0MeasureImagesInput { #[serde(rename="points")] pub f_0: Vec<S0MeasurementPoint>,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="covariance")] pub f_2: S0MeasureImagesInputF2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0MeasureImagesInputF2 { V0(S0CartesianPointCovariance), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0MeasureImagesOutput { #[serde(rename="value")] pub f_0: f64,
#[serde(rename="unit")] pub f_1: String,
#[serde(rename="variance")] pub f_2: S0MeasureImagesOutputF2,
#[serde(rename="sigma")] pub f_3: S0MeasureImagesOutputF3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0MeasureImagesOutputF2 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0MeasureImagesOutputF3 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ProposeTidyResultInput { #[serde(rename="source")] pub f_0: S0Structure,
#[serde(rename="candidate")] pub f_1: S0Structure,
#[serde(rename="inputDigest")] pub f_2: S0Digest,
#[serde(rename="toolOutput")] pub f_3: S0Resource }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ProposeTidyResultOutput { #[serde(rename="candidate")] pub f_0: S0Structure,
#[serde(rename="inputDigest")] pub f_1: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0PrepareMadelInputInput { #[serde(rename="structure")] pub f_0: S0Structure,
#[serde(rename="charges")] pub f_1: Vec<S0PrepareMadelInputInputF1Item>,
#[serde(rename="radiusA")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0PrepareMadelInputInputF1Item { #[serde(rename="site")] pub f_0: S0Id,
#[serde(rename="chargeElectron")] pub f_1: f64 }

pub type S0PrepareMadelInputOutput = S0ExternalFiles;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ConvolvePowderLinesInput { #[serde(rename="positionsDeg")] pub f_0: Vec<f64>,
#[serde(rename="intensities")] pub f_1: Vec<f64>,
#[serde(rename="sampleDeg")] pub f_2: Vec<f64>,
#[serde(rename="fwhmDeg")] pub f_3: f64,
#[serde(rename="mixing")] pub f_4: f64,
#[serde(rename="compatibilityFirstPointZero")] pub f_5: bool }

pub type S0ConvolvePowderLinesOutput = S0Curve;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SumStructureFactorsInput { #[serde(rename="structure")] pub f_0: S0Structure,
#[serde(rename="hkl")] pub f_1: Vec<S0Int3>,
#[serde(rename="siteAmplitudes")] pub f_2: Vec<S0SumStructureFactorsInputF2Item>,
#[serde(rename="unit")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SumStructureFactorsInputF2Item { #[serde(rename="site")] pub f_0: S0Id,
#[serde(rename="values")] pub f_1: Vec<f64> }

pub type S0SumStructureFactorsOutput = Vec<S0Reflection>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SynthesizeFourierInput { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="reflections")] pub f_1: Vec<S0Reflection>,
#[serde(rename="coreShape")] pub f_2: Vec<i64>,
#[serde(rename="mode")] pub f_3: String,
#[serde(rename="includeConjugates")] pub f_4: bool,
#[serde(rename="dcReal")] pub f_5: f64 }

pub type S0SynthesizeFourierOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ElectronDcInput { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="electronCount")] pub f_1: f64,
#[serde(rename="coreShape")] pub f_2: Vec<i64> }

pub type S0ElectronDcOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0NuclearDcInput { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="signedScatteringSumFm")] pub f_1: f64,
#[serde(rename="coreShape")] pub f_2: Vec<i64> }

pub type S0NuclearDcOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ModelPattersonCoefficientsInput { #[serde(rename="reflections")] pub f_0: Vec<S0Reflection> }

pub type S0ModelPattersonCoefficientsOutput = Vec<S0PattersonCoefficient>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0VolumePattersonInput { #[serde(rename="field")] pub f_0: S0Field }

pub type S0VolumePattersonOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0NormalizeGridLayoutInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="targetAxisOrder")] pub f_1: String,
#[serde(rename="targetClosure")] pub f_2: Vec<bool> }

pub type S0NormalizeGridLayoutOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0IntersectHalfspacesInput { #[serde(rename="normalsCartesian")] pub f_0: Vec<S0Vec3>,
#[serde(rename="offsetsA")] pub f_1: Vec<f64> }

pub type S0IntersectHalfspacesOutput = S0Mesh;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SelectIsoEdgesInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="level")] pub f_1: f64 }

pub type S0SelectIsoEdgesOutput = Vec<S0Vec3>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0BuildAtomInstancesInput { #[serde(rename="points")] pub f_0: Vec<S0ImagePoint>,
#[serde(rename="radiusA")] pub f_1: f64,
#[serde(rename="colorRGBA")] pub f_2: Vec<f64> }

pub type S0BuildAtomInstancesOutput = Vec<S0AtomInstance>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SliceTrilinearInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="originA")] pub f_1: S0Vec3,
#[serde(rename="uA")] pub f_2: S0Vec3,
#[serde(rename="vA")] pub f_3: S0Vec3,
#[serde(rename="width")] pub f_4: i64,
#[serde(rename="height")] pub f_5: i64 }

pub type S0SliceTrilinearOutput = S0Slice;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SampleLineInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="startFractional")] pub f_1: S0Vec3,
#[serde(rename="endFractional")] pub f_2: S0Vec3,
#[serde(rename="count")] pub f_3: i64 }

pub type S0SampleLineOutput = S0Line;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FindLocalExtremaInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="limit")] pub f_2: i64 }

pub type S0FindLocalExtremaOutput = Vec<S0Peak>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0QueryPolyIntegralCapabilityInput { #[serde(rename="referenceVersion")] pub f_0: String }

pub type S0QueryPolyIntegralCapabilityOutput = S0Capability;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ConvertDensityInput { #[serde(rename="field")] pub f_0: S0Field,
#[serde(rename="mode")] pub f_1: String,
#[serde(rename="lengthUnit")] pub f_2: String }

pub type S0ConvertDensityOutput = S0Field;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0PlanEncodeInput { #[serde(rename="profileId")] pub f_0: String,
#[serde(rename="source")] pub f_1: S0EncodeSource,
#[serde(rename="purpose")] pub f_2: String }

pub type S0PlanEncodeOutput = S0EncodePlan;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0Context { #[serde(rename="document")] pub f_0: S0Id,
#[serde(rename="epoch")] pub f_1: S0Id,
#[serde(rename="baseRevision")] pub f_2: String,
#[serde(rename="maxResidentBytes")] pub f_3: i64,
#[serde(rename="maxOutputBytes")] pub f_4: i64,
#[serde(rename="workUnits")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0Request { V0(S0RequestV0), V1(S0RequestV1), V2(S0RequestV2), V3(S0RequestV3), V4(S0RequestV4), V5(S0RequestV5), V6(S0RequestV6), V7(S0RequestV7), V8(S0RequestV8), V9(S0RequestV9), V10(S0RequestV10), V11(S0RequestV11), V12(S0RequestV12), V13(S0RequestV13), V14(S0RequestV14), V15(S0RequestV15), V16(S0RequestV16), V17(S0RequestV17), V18(S0RequestV18), V19(S0RequestV19), V20(S0RequestV20), V21(S0RequestV21), V22(S0RequestV22), V23(S0RequestV23), V24(S0RequestV24), V25(S0RequestV25), V26(S0RequestV26), V27(S0RequestV27), V28(S0RequestV28), V29(S0RequestV29), V30(S0RequestV30), V31(S0RequestV31) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV0 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ResolvePlacementsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV1 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ExpandImagesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV2 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ChangeBasisInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV3 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0TransportAdpInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV4 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0FindDuplicatesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV5 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SelectBondPairsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV6 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0MeasurePolyhedronInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV7 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0TransformVectorInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV8 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0MillerPlaneInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV9 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ClipExplicitImagesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV10 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ComposeCameraInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV11 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0MeasureImagesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV12 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ProposeTidyResultInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV13 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0PrepareMadelInputInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV14 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ConvolvePowderLinesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV15 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SumStructureFactorsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV16 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SynthesizeFourierInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV17 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ElectronDcInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV18 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0NuclearDcInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV19 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ModelPattersonCoefficientsInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV20 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0VolumePattersonInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV21 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0NormalizeGridLayoutInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV22 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0IntersectHalfspacesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV23 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SelectIsoEdgesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV24 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0BuildAtomInstancesInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV25 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SliceTrilinearInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV26 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0SampleLineInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV27 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0FindLocalExtremaInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV28 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0QueryPolyIntegralCapabilityInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV29 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0ConvertDensityInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV30 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0PlanEncodeInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RequestV31 { #[serde(rename="protocol")] pub f_0: String,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="context")] pub f_2: S0Context,
#[serde(rename="input")] pub f_3: S0EncodeApprovedInput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0ResolvedPlacement { #[serde(rename="phase")] pub f_0: S0Id,
#[serde(rename="rotation")] pub f_1: S0Mat3,
#[serde(rename="translationA")] pub f_2: S0Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0MeasurementPoint { #[serde(rename="pointId")] pub f_0: S0Id,
#[serde(rename="cartesianA")] pub f_1: S0Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0CartesianPointParameter { #[serde(rename="pointId")] pub f_0: S0Id,
#[serde(rename="axis")] pub f_1: String,
#[serde(rename="unit")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0CartesianPointCovariance { #[serde(rename="parameters")] pub f_0: Vec<S0CartesianPointParameter>,
#[serde(rename="matrixA2")] pub f_1: Vec<Vec<f64>>,
#[serde(rename="source")] pub f_2: String,
#[serde(rename="sourceDigest")] pub f_3: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0RawBlockForEncode { #[serde(rename="source")] pub f_0: S0Resource,
#[serde(rename="byteStart")] pub f_1: i64,
#[serde(rename="byteLength")] pub f_2: i64,
#[serde(rename="formatRef")] pub f_3: String,
#[serde(rename="dependentFields")] pub f_4: Vec<String>,
#[serde(rename="invalidated")] pub f_5: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodeIdentity { #[serde(rename="documentId")] pub f_0: S0Id,
#[serde(rename="epoch")] pub f_1: S0Id,
#[serde(rename="revision")] pub f_2: String,
#[serde(rename="stateDigest")] pub f_3: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodeInventory { #[serde(rename="structures")] pub f_0: Vec<S0EncodeInventoryF0Item>,
#[serde(rename="fields")] pub f_1: Vec<S0FieldHandle>,
#[serde(rename="placements")] pub f_2: Vec<S0Placement>,
#[serde(rename="scene")] pub f_3: S0EncodeInventoryF3,
#[serde(rename="comment")] pub f_4: String,
#[serde(rename="opaqueBlocks")] pub f_5: Vec<S0RawBlockForEncode> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodeInventoryF0Item { #[serde(rename="phaseId")] pub f_0: S0Id,
#[serde(rename="structure")] pub f_1: S0Structure }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0EncodeInventoryF3 { V0(S0Resource), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodeSource { #[serde(rename="identity")] pub f_0: S0EncodeIdentity,
#[serde(rename="inventory")] pub f_1: S0EncodeInventory }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0LossField { #[serde(rename="objectId")] pub f_0: S0LossFieldF0,
#[serde(rename="fieldPath")] pub f_1: String,
#[serde(rename="reason")] pub f_2: String,
#[serde(rename="objectKind")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0LossFieldF0 { V0(S0Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0LossReport1 { #[serde(rename="sourceDigest")] pub f_0: S0Digest,
#[serde(rename="profileId")] pub f_1: String,
#[serde(rename="droppedFields")] pub f_2: Vec<S0LossField>,
#[serde(rename="unsupportedBlocks")] pub f_3: Vec<S0RawBlockForEncode>,
#[serde(rename="requiredResources")] pub f_4: Vec<S0Digest>,
#[serde(rename="purpose")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0LossConsent1 { #[serde(rename="sourceDigest")] pub f_0: S0Digest,
#[serde(rename="profileId")] pub f_1: String,
#[serde(rename="lossReportSha256")] pub f_2: S0Digest,
#[serde(rename="explicit")] pub f_3: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodePlan { #[serde(rename="sourceDigest")] pub f_0: S0Digest,
#[serde(rename="profileId")] pub f_1: String,
#[serde(rename="lossReport")] pub f_2: S0LossReport1,
#[serde(rename="lossReportSha256")] pub f_3: S0Digest,
#[serde(rename="requiresConsent")] pub f_4: bool,
#[serde(rename="purpose")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0EncodeApprovedInput { #[serde(rename="source")] pub f_0: S0EncodeSource,
#[serde(rename="plan")] pub f_1: S0EncodePlan,
#[serde(rename="lossConsent")] pub f_2: S0EncodeApprovedInputF2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0EncodeApprovedInputF2 { V0(S0LossConsent1), V1(()) }

pub type S0EncodeApprovedOutput = S0Bundle;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FieldHandle { #[serde(rename="fieldId")] pub f_0: S0Id,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="field")] pub f_2: S0Field }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSourceSpan { #[serde(rename="resource")] pub f_0: S0Digest,
#[serde(rename="byteOffset")] pub f_1: i64,
#[serde(rename="byteLength")] pub f_2: i64,
#[serde(rename="fieldPath")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGTableIdentity { #[serde(rename="resource")] pub f_0: S0Digest,
#[serde(rename="sourceId")] pub f_1: String,
#[serde(rename="sourceVersion")] pub f_2: String,
#[serde(rename="edition")] pub f_3: String,
#[serde(rename="licenseResource")] pub f_4: S0Digest,
#[serde(rename="tableKey")] pub f_5: String,
#[serde(rename="tableContentDigest")] pub f_6: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRational { #[serde(rename="numerator")] pub f_0: i64,
#[serde(rename="denominator")] pub f_1: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSymmetryOperator3 { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="fractionalMatrix")] pub f_1: Vec<Vec<S0SGRational>>,
#[serde(rename="fractionalTranslation")] pub f_2: Vec<S0SGRational>,
#[serde(rename="timeSign")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGGroupSetting3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="symbol")] pub f_1: String,
#[serde(rename="hallSymbol")] pub f_2: S0SGGroupSetting3F2,
#[serde(rename="bnsNumber")] pub f_3: S0SGGroupSetting3F3,
#[serde(rename="ogNumber")] pub f_4: S0SGGroupSetting3F4,
#[serde(rename="settingName")] pub f_5: String,
#[serde(rename="originChoice")] pub f_6: String,
#[serde(rename="changeOfBasis")] pub f_7: S0Mat3,
#[serde(rename="originFractional")] pub f_8: S0Vec3,
#[serde(rename="table")] pub f_9: S0SGGroupSetting3F9,
#[serde(rename="operators")] pub f_10: Vec<S0SGSymmetryOperator3>,
#[serde(rename="sourceSpans")] pub f_11: Vec<S0SGSourceSpan> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGGroupSetting3F2 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGGroupSetting3F3 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGGroupSetting3F4 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGGroupSetting3F9 { V0(S0SGTableIdentity), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGAdp { V0(S0SGAdpV0), V1(S0SGAdpV1), V2(S0SGAdpV2), V3(S0SGAdpV3), V4(S0SGAdpV4) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAdpV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="uA2")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAdpV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="bA2")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAdpV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="sixA2")] pub f_1: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAdpV3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="six")] pub f_1: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAdpV4 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="sixA2")] pub f_1: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGParameter { V0(S0SGParameterV0), V1(S0SGParameterV1), V2(S0SGParameterV2), V3(S0SGParameterV3), V4(S0SGParameterV4), V5(S0SGParameterV5) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="siteId")] pub f_2: S0Id,
#[serde(rename="axis")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="pointId")] pub f_1: S0Id,
#[serde(rename="axis")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="axis")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="angle")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV4 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="siteId")] pub f_2: S0Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGParameterV5 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="siteId")] pub f_2: S0Id,
#[serde(rename="component")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGCovariance { #[serde(rename="parameters")] pub f_0: Vec<S0SGParameter>,
#[serde(rename="matrix")] pub f_1: Vec<Vec<f64>>,
#[serde(rename="provenance")] pub f_2: String,
#[serde(rename="sourceSpans")] pub f_3: Vec<S0SGSourceSpan> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSite { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="label")] pub f_1: String,
#[serde(rename="species")] pub f_2: S0Species,
#[serde(rename="fractional")] pub f_3: S0Vec3,
#[serde(rename="occupancy")] pub f_4: f64,
#[serde(rename="adp")] pub f_5: S0SGSiteF5,
#[serde(rename="sourceSpans")] pub f_6: Vec<S0SGSourceSpan> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGSiteF5 { V0(S0SGAdp), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPeriodic3 { #[serde(rename="cell")] pub f_0: S0Cell,
#[serde(rename="group")] pub f_1: S0SGGroupSetting3,
#[serde(rename="sites")] pub f_2: Vec<S0SGSite>,
#[serde(rename="covariance")] pub f_3: S0SGPeriodic3F3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPeriodic3F3 { V0(S0SGCovariance), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPhaseNode { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="label")] pub f_1: String,
#[serde(rename="parent")] pub f_2: S0SGPhaseNodeF2,
#[serde(rename="rotation")] pub f_3: S0Mat3,
#[serde(rename="translationA")] pub f_4: S0Vec3,
#[serde(rename="structure")] pub f_5: S0SGPhaseNodeF5,
#[serde(rename="fieldIds")] pub f_6: Vec<S0Id>,
#[serde(rename="shapeId")] pub f_7: S0SGPhaseNodeF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPhaseNodeF2 { V0(S0Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPhaseNodeF5 { V0(S0SGPeriodic3), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPhaseNodeF7 { V0(S0Id), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPhaseGraph { #[serde(rename="root")] pub f_0: S0Id,
#[serde(rename="phases")] pub f_1: Vec<S0SGPhaseNode> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSelectedImage { #[serde(rename="key")] pub f_0: S0Image,
#[serde(rename="fractional")] pub f_1: S0Vec3,
#[serde(rename="localCartesianA")] pub f_2: S0Vec3,
#[serde(rename="worldCartesianA")] pub f_3: S0Vec3,
#[serde(rename="pointId")] pub f_4: S0Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGVector { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="anchor")] pub f_1: S0Image,
#[serde(rename="components")] pub f_2: S0Vec3,
#[serde(rename="basis")] pub f_3: String,
#[serde(rename="kind")] pub f_4: String,
#[serde(rename="unit")] pub f_5: String,
#[serde(rename="displayScale")] pub f_6: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGDuplicateQuery { #[serde(rename="images")] pub f_0: Vec<S0SGSelectedImage>,
#[serde(rename="thresholdA")] pub f_1: f64,
#[serde(rename="grouping")] pub f_2: String,
#[serde(rename="merge")] pub f_3: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGSiteSelector { V0(S0SGSiteSelectorV0), V1(S0SGSiteSelectorV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSiteSelectorV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="siteId")] pub f_1: S0Id }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGSiteSelectorV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="element")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGBondRule { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="fromSelector")] pub f_1: S0SGSiteSelector,
#[serde(rename="toSelector")] pub f_2: S0SGSiteSelector,
#[serde(rename="minimumA")] pub f_3: f64,
#[serde(rename="maximumA")] pub f_4: f64,
#[serde(rename="thresholdMode")] pub f_5: String,
#[serde(rename="extraAtoms")] pub f_6: String,
#[serde(rename="maxDepth")] pub f_7: i64,
#[serde(rename="maxImages")] pub f_8: i64,
#[serde(rename="bondValenceTable")] pub f_9: S0SGBondRuleF9 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGBondRuleF9 { V0(S0SGTableIdentity), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGBoundary { #[serde(rename="phaseId")] pub f_0: S0Id,
#[serde(rename="minimumFractional")] pub f_1: S0Vec3,
#[serde(rename="maximumFractional")] pub f_2: S0Vec3,
#[serde(rename="endpoints")] pub f_3: String,
#[serde(rename="includeBondExtensions")] pub f_4: bool,
#[serde(rename="ruleIds")] pub f_5: Vec<S0Id> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPolyhedron { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="center")] pub f_1: S0Image,
#[serde(rename="vertices")] pub f_2: Vec<S0Image>,
#[serde(rename="mesh")] pub f_3: S0Mesh,
#[serde(rename="boundaryRule")] pub f_4: S0Id,
#[serde(rename="valenceTable")] pub f_5: S0SGPolyhedronF5 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPolyhedronF5 { V0(S0SGTableIdentity), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGPlaneSpec { V0(S0SGPlaneSpecV0), V1(S0SGPlaneSpecV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPlaneSpecV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="hkl")] pub f_2: S0Int3,
#[serde(rename="order")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPlaneSpecV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="points")] pub f_1: Vec<S0SGSelectedImage> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGPlane { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="localUnitNormal")] pub f_1: S0Vec3,
#[serde(rename="localOffsetA")] pub f_2: f64,
#[serde(rename="worldUnitNormal")] pub f_3: S0Vec3,
#[serde(rename="worldOffsetA")] pub f_4: f64,
#[serde(rename="source")] pub f_5: S0SGPlaneSpec }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGHalfspace { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="normal")] pub f_1: S0Vec3,
#[serde(rename="offsetA")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGShape { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="halfspaces")] pub f_2: Vec<S0SGHalfspace> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGHalfspaceCertificate { V0(S0SGHalfspaceCertificateV0), V1(S0SGHalfspaceCertificateV1), V2(S0SGHalfspaceCertificateV2), V3(S0SGHalfspaceCertificateV3) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGHalfspaceCertificateV0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="nonnegativeWeights")] pub f_1: Vec<f64>,
#[serde(rename="contradictionMarginA")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGHalfspaceCertificateV1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="feasiblePointA")] pub f_1: S0Vec3,
#[serde(rename="recessionDirection")] pub f_2: S0Vec3 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGHalfspaceCertificateV2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="feasiblePointA")] pub f_1: S0Vec3,
#[serde(rename="affineDimension")] pub f_2: f64,
#[serde(rename="supportingEqualities")] pub f_3: Vec<S0Id> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGHalfspaceCertificateV3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="interiorPointA")] pub f_1: S0Vec3,
#[serde(rename="mesh")] pub f_2: S0Mesh }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGLineWidth { V0(S0SGLineWidthV0), V1(S0SGLineWidthV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGLineWidthV0 { #[serde(rename="unit")] pub f_0: String,
#[serde(rename="value")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGLineWidthV1 { #[serde(rename="unit")] pub f_0: String,
#[serde(rename="value")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGMaterial { #[serde(rename="rgba")] pub f_0: Vec<f64>,
#[serde(rename="ambient")] pub f_1: Vec<f64>,
#[serde(rename="diffuse")] pub f_2: Vec<f64>,
#[serde(rename="specular")] pub f_3: Vec<f64>,
#[serde(rename="shininess")] pub f_4: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGAtomStyle { #[serde(rename="mode")] pub f_0: String,
#[serde(rename="radiusA")] pub f_1: f64,
#[serde(rename="radiusTable")] pub f_2: S0SGAtomStyleF2,
#[serde(rename="material")] pub f_3: S0SGMaterial,
#[serde(rename="probabilityPercent")] pub f_4: i64,
#[serde(rename="principalEllipse")] pub f_5: bool,
#[serde(rename="outlineWidth")] pub f_6: S0SGLineWidth,
#[serde(rename="slices")] pub f_7: i64,
#[serde(rename="stacks")] pub f_8: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGAtomStyleF2 { V0(S0SGTableIdentity), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGCamera { #[serde(rename="rotation")] pub f_0: S0Mat3,
#[serde(rename="panWorldA")] pub f_1: S0Vec3,
#[serde(rename="zoom")] pub f_2: f64,
#[serde(rename="projection")] pub f_3: String,
#[serde(rename="widthA")] pub f_4: f64,
#[serde(rename="perspectiveVerticalFovDeg")] pub f_5: f64,
#[serde(rename="nearA")] pub f_6: f64,
#[serde(rename="farA")] pub f_7: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGNativeCameraFields { #[serde(rename="sceneMatrix4")] pub f_0: Vec<Vec<f64>>,
#[serde(rename="sceneShift")] pub f_1: S0Vec3,
#[serde(rename="zoom")] pub f_2: f64,
#[serde(rename="dispf")] pub f_3: i64,
#[serde(rename="projt")] pub f_4: Vec<f64>,
#[serde(rename="sourceSpan")] pub f_5: S0SGSourceSpan }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGDisplay { #[serde(rename="showModels")] pub f_0: bool,
#[serde(rename="asymmetricOnly")] pub f_1: bool,
#[serde(rename="dotSurface")] pub f_2: bool,
#[serde(rename="style")] pub f_3: String,
#[serde(rename="hiddenImages")] pub f_4: Vec<S0Image>,
#[serde(rename="atomStyle")] pub f_5: S0SGAtomStyle }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGMeasurement { #[serde(rename="id")] pub f_0: S0Id,
#[serde(rename="kind")] pub f_1: String,
#[serde(rename="images")] pub f_2: Vec<S0Image>,
#[serde(rename="value")] pub f_3: f64,
#[serde(rename="unit")] pub f_4: String,
#[serde(rename="variance")] pub f_5: S0SGMeasurementF5,
#[serde(rename="sigma")] pub f_6: S0SGMeasurementF6,
#[serde(rename="covariance")] pub f_7: S0SGMeasurementF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGMeasurementF5 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGMeasurementF6 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGMeasurementF7 { V0(S0SGCovariance), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderContext { #[serde(rename="documentId")] pub f_0: S0Id,
#[serde(rename="epoch")] pub f_1: S0Id,
#[serde(rename="revision")] pub f_2: String,
#[serde(rename="sceneDigest")] pub f_3: S0Digest,
#[serde(rename="geometryDigest")] pub f_4: S0Digest,
#[serde(rename="widthPx")] pub f_5: i64,
#[serde(rename="heightPx")] pub f_6: i64,
#[serde(rename="dpr")] pub f_7: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderSphere { #[serde(rename="image")] pub f_0: S0Image,
#[serde(rename="centerWorldA")] pub f_1: S0Vec3,
#[serde(rename="radiusA")] pub f_2: f64,
#[serde(rename="material")] pub f_3: S0SGMaterial }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderEllipsoid { #[serde(rename="image")] pub f_0: S0Image,
#[serde(rename="centerWorldA")] pub f_1: S0Vec3,
#[serde(rename="axesWorld")] pub f_2: S0Mat3,
#[serde(rename="semiaxesA")] pub f_3: Vec<f64>,
#[serde(rename="probabilityPercent")] pub f_4: i64,
#[serde(rename="material")] pub f_5: S0SGMaterial }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderBond { #[serde(rename="ruleId")] pub f_0: S0Id,
#[serde(rename="images")] pub f_1: Vec<S0Image>,
#[serde(rename="endpointsWorldA")] pub f_2: Vec<S0Vec3>,
#[serde(rename="radiusA")] pub f_3: f64,
#[serde(rename="material")] pub f_4: S0SGMaterial }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderLabel { #[serde(rename="image")] pub f_0: S0SGRenderLabelF0,
#[serde(rename="positionWorldA")] pub f_1: S0Vec3,
#[serde(rename="text")] pub f_2: String,
#[serde(rename="font")] pub f_3: S0SGTableIdentity,
#[serde(rename="sizePx")] pub f_4: f64,
#[serde(rename="rgba")] pub f_5: Vec<f64> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0SGRenderLabelF0 { V0(S0Image), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderPacket { #[serde(rename="context")] pub f_0: S0SGRenderContext,
#[serde(rename="spheres")] pub f_1: Vec<S0SGRenderSphere>,
#[serde(rename="ellipsoids")] pub f_2: Vec<S0SGRenderEllipsoid>,
#[serde(rename="bonds")] pub f_3: Vec<S0SGRenderBond>,
#[serde(rename="meshes")] pub f_4: Vec<S0SGRenderMesh>,
#[serde(rename="labels")] pub f_5: Vec<S0SGRenderLabel>,
#[serde(rename="camera")] pub f_6: S0SGCamera,
#[serde(rename="retainedResources")] pub f_7: Vec<S0Digest> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGReviewBundle { #[serde(rename="graph")] pub f_0: S0SGPhaseGraph,
#[serde(rename="vectors")] pub f_1: Vec<S0SGVector>,
#[serde(rename="bondRules")] pub f_2: Vec<S0SGBondRule>,
#[serde(rename="boundaries")] pub f_3: Vec<S0SGBoundary>,
#[serde(rename="planes")] pub f_4: Vec<S0SGPlane>,
#[serde(rename="shapes")] pub f_5: Vec<S0SGShape>,
#[serde(rename="display")] pub f_6: S0SGDisplay,
#[serde(rename="camera")] pub f_7: S0SGCamera }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0SGRenderMesh { #[serde(rename="objectId")] pub f_0: S0Id,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="space")] pub f_2: String,
#[serde(rename="geometry")] pub f_3: S0Mesh,
#[serde(rename="material")] pub f_4: S0SGMaterial }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSContext { #[serde(rename="document")] pub f_0: S0Id,
#[serde(rename="epoch")] pub f_1: S0Id,
#[serde(rename="baseRevision")] pub f_2: String,
#[serde(rename="stateDigest")] pub f_3: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSByteSpan { #[serde(rename="resource")] pub f_0: S0Resource,
#[serde(rename="offset")] pub f_1: i64,
#[serde(rename="length")] pub f_2: i64,
#[serde(rename="spanDigest")] pub f_3: S0Digest,
#[serde(rename="sourceId")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSFieldBinding { #[serde(rename="base")] pub f_0: S0FSContext,
#[serde(rename="handle")] pub f_1: S0FieldHandle,
#[serde(rename="descriptorDigest")] pub f_2: S0Digest,
#[serde(rename="encoding")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSBudgetPlan { #[serde(rename="inputRetainedBytes")] pub f_0: i64,
#[serde(rename="inputDecodeBytes")] pub f_1: i64,
#[serde(rename="scratchBytes")] pub f_2: i64,
#[serde(rename="outputBytes")] pub f_3: i64,
#[serde(rename="transferCopyBytes")] pub f_4: i64,
#[serde(rename="historyRetainedBytes")] pub f_5: i64,
#[serde(rename="peakResidentBytes")] pub f_6: i64,
#[serde(rename="maxResidentBytes")] pub f_7: i64,
#[serde(rename="maxOutputBytes")] pub f_8: i64,
#[serde(rename="maxWorkUnits")] pub f_9: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSTableUse { #[serde(rename="identity")] pub f_0: S0SGTableIdentity,
#[serde(rename="species")] pub f_1: S0Species,
#[serde(rename="purpose")] pub f_2: String,
#[serde(rename="qualificationRecordDigest")] pub f_3: S0Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSProvenance { #[serde(rename="jobId")] pub f_0: S0Id,
#[serde(rename="base")] pub f_1: S0FSContext,
#[serde(rename="phaseId")] pub f_2: S0Id,
#[serde(rename="profile")] pub f_3: String,
#[serde(rename="algorithmVersion")] pub f_4: String,
#[serde(rename="parameterDigest")] pub f_5: S0Digest,
#[serde(rename="inputs")] pub f_6: Vec<S0FSFieldBinding>,
#[serde(rename="sourceSpans")] pub f_7: Vec<S0FSByteSpan>,
#[serde(rename="tables")] pub f_8: Vec<S0FSTableUse> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSFieldOutput { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="provenance")] pub f_1: S0FSProvenance,
#[serde(rename="budget")] pub f_2: S0FSBudgetPlan,
#[serde(rename="integralCore")] pub f_3: S0FSFieldOutputF3,
#[serde(rename="integralUnit")] pub f_4: S0FSFieldOutputF4,
#[serde(rename="ownership")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSFieldOutputF3 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSFieldOutputF4 { V0(String), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSCoefficient { #[serde(rename="hkl")] pub f_0: S0Int3,
#[serde(rename="real")] pub f_1: f64,
#[serde(rename="imaginary")] pub f_2: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSpectrumInput { #[serde(rename="base")] pub f_0: S0FSContext,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="cell")] pub f_2: S0Cell,
#[serde(rename="coreShape")] pub f_3: Vec<i64>,
#[serde(rename="coefficients")] pub f_4: Vec<S0FSCoefficient>,
#[serde(rename="dcReal")] pub f_5: f64,
#[serde(rename="amplitudeUnit")] pub f_6: String,
#[serde(rename="completion")] pub f_7: String,
#[serde(rename="outputDtype")] pub f_8: String,
#[serde(rename="maxResidentBytes")] pub f_9: i64,
#[serde(rename="maxOutputBytes")] pub f_10: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSpectrumResult { #[serde(rename="field")] pub f_0: S0FSFieldOutput,
#[serde(rename="admittedCoefficients")] pub f_1: Vec<S0FSCoefficient>,
#[serde(rename="dcReal")] pub f_2: f64,
#[serde(rename="normalization")] pub f_3: String,
#[serde(rename="amplitudeUnit")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSiteAmplitude { #[serde(rename="imagePointId")] pub f_0: S0Id,
#[serde(rename="hklValues")] pub f_1: Vec<S0FSSiteAmplitudeF1Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSiteAmplitudeF1Item { #[serde(rename="real")] pub f_0: f64,
#[serde(rename="imaginary")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSumInput { #[serde(rename="base")] pub f_0: S0FSContext,
#[serde(rename="phaseId")] pub f_1: S0Id,
#[serde(rename="structure")] pub f_2: S0SGPeriodic3,
#[serde(rename="hkl")] pub f_3: Vec<S0Int3>,
#[serde(rename="expandedImages")] pub f_4: Vec<S0SGSelectedImage>,
#[serde(rename="siteAmplitudes")] pub f_5: Vec<S0FSSiteAmplitude>,
#[serde(rename="unit")] pub f_6: String,
#[serde(rename="wavelengthA")] pub f_7: S0FSSumInputF7 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSSumInputF7 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSReflectionRow { #[serde(rename="hkl")] pub f_0: S0Int3,
#[serde(rename="dA")] pub f_1: S0FSReflectionRowF1,
#[serde(rename="real")] pub f_2: f64,
#[serde(rename="imaginary")] pub f_3: f64,
#[serde(rename="modulus")] pub f_4: f64,
#[serde(rename="phaseDeg")] pub f_5: S0FSReflectionRowF5,
#[serde(rename="multiplicity")] pub f_6: i64,
#[serde(rename="twoThetaDeg")] pub f_7: S0FSReflectionRowF7,
#[serde(rename="braggStatus")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSReflectionRowF1 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSReflectionRowF5 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSReflectionRowF7 { V0(f64), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSReflectionResult { #[serde(rename="provenance")] pub f_0: S0FSProvenance,
#[serde(rename="rows")] pub f_1: Vec<S0FSReflectionRow>,
#[serde(rename="amplitudeUnit")] pub f_2: String,
#[serde(rename="intensityNormalization")] pub f_3: String,
#[serde(rename="ordering")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSLineResult { #[serde(rename="provenance")] pub f_0: S0FSProvenance,
#[serde(rename="startFractional")] pub f_1: S0Vec3,
#[serde(rename="endFractional")] pub f_2: S0Vec3,
#[serde(rename="count")] pub f_3: i64,
#[serde(rename="distanceA")] pub f_4: Vec<f64>,
#[serde(rename="values")] pub f_5: Vec<f64>,
#[serde(rename="valueUnit")] pub f_6: String,
#[serde(rename="endpointPolicy")] pub f_7: String,
#[serde(rename="interpolation")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSSliceResult { #[serde(rename="provenance")] pub f_0: S0FSProvenance,
#[serde(rename="width")] pub f_1: i64,
#[serde(rename="height")] pub f_2: i64,
#[serde(rename="originA")] pub f_3: S0Vec3,
#[serde(rename="uStepA")] pub f_4: S0Vec3,
#[serde(rename="vStepA")] pub f_5: S0Vec3,
#[serde(rename="values")] pub f_6: Vec<f64>,
#[serde(rename="unit")] pub f_7: String,
#[serde(rename="axisOrder")] pub f_8: String,
#[serde(rename="endpointPolicy")] pub f_9: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSProjectionInput { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="width")] pub f_1: i64,
#[serde(rename="height")] pub f_2: i64,
#[serde(rename="originA")] pub f_3: S0Vec3,
#[serde(rename="uStepA")] pub f_4: S0Vec3,
#[serde(rename="vStepA")] pub f_5: S0Vec3,
#[serde(rename="normalUnitCartesian")] pub f_6: S0Vec3,
#[serde(rename="offsetSamplesA")] pub f_7: Vec<f64>,
#[serde(rename="weightsA")] pub f_8: Vec<f64>,
#[serde(rename="weightPolicy")] pub f_9: String,
#[serde(rename="profile")] pub f_10: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSProjectionResult { #[serde(rename="slice")] pub f_0: S0FSSliceResult,
#[serde(rename="sourceValueUnit")] pub f_1: String,
#[serde(rename="weightUnit")] pub f_2: String,
#[serde(rename="normalUnitCartesian")] pub f_3: S0Vec3,
#[serde(rename="offsetSamplesA")] pub f_4: Vec<f64>,
#[serde(rename="weightsA")] pub f_5: Vec<f64>,
#[serde(rename="meaning")] pub f_6: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSPeak { #[serde(rename="peakId")] pub f_0: S0Id,
#[serde(rename="coreIndex")] pub f_1: Vec<i64>,
#[serde(rename="fractional")] pub f_2: S0Vec3,
#[serde(rename="cartesianA")] pub f_3: S0Vec3,
#[serde(rename="value")] pub f_4: f64,
#[serde(rename="sign")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSBasinAllocation { #[serde(rename="peakId")] pub f_0: S0Id,
#[serde(rename="integral")] pub f_1: f64,
#[serde(rename="integralUnit")] pub f_2: String,
#[serde(rename="volumeA3")] pub f_3: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSPeakResult { #[serde(rename="provenance")] pub f_0: S0FSProvenance,
#[serde(rename="detectedCount")] pub f_1: i64,
#[serde(rename="selected")] pub f_2: Vec<S0FSPeak>,
#[serde(rename="selectionLimit")] pub f_3: i64,
#[serde(rename="limitMeaning")] pub f_4: String,
#[serde(rename="boundaryNodeCount")] pub f_5: i64,
#[serde(rename="coreNodeCount")] pub f_6: i64,
#[serde(rename="basins")] pub f_7: Vec<S0FSBasinAllocation>,
#[serde(rename="wholeSignedIntegral")] pub f_8: f64,
#[serde(rename="wholePositiveIntegral")] pub f_9: f64,
#[serde(rename="wholeNegativeIntegral")] pub f_10: f64,
#[serde(rename="integralUnit")] pub f_11: String,
#[serde(rename="policy")] pub f_12: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSNonfiniteDiagnostic { #[serde(rename="base")] pub f_0: S0FSContext,
#[serde(rename="input")] pub f_1: S0FSFieldBinding,
#[serde(rename="mode")] pub f_2: String,
#[serde(rename="coreIndices")] pub f_3: Vec<Vec<i64>>,
#[serde(rename="special")] pub f_4: String,
#[serde(rename="error")] pub f_5: String,
#[serde(rename="referenceOutputPreserved")] pub f_6: S0FSNonfiniteDiagnosticF6 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSNonfiniteDiagnosticF6 { V0(S0Resource), V1(()) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S0FSConversionResult { V0(S0FSConversionResultV0), V1(S0FSConversionResultV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSConversionResultV0 { #[serde(rename="status")] pub f_0: String,
#[serde(rename="mode")] pub f_1: String,
#[serde(rename="rawDensityInterpretation")] pub f_2: String,
#[serde(rename="output")] pub f_3: S0FSFieldOutput }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSConversionResultV1 { #[serde(rename="status")] pub f_0: String,
#[serde(rename="diagnostic")] pub f_1: S0FSNonfiniteDiagnostic }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSGridOperationInput { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="operation")] pub f_1: String,
#[serde(rename="targetAxisOrder")] pub f_2: String,
#[serde(rename="targetClosure")] pub f_3: Vec<bool>,
#[serde(rename="maxResidentBytes")] pub f_4: i64,
#[serde(rename="maxOutputBytes")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S0FSNoActionResult { #[serde(rename="referenceVersion")] pub f_0: String,
#[serde(rename="menuId")] pub f_1: f64,
#[serde(rename="handler")] pub f_2: String,
#[serde(rename="outcome")] pub f_3: String,
#[serde(rename="mutates")] pub f_4: bool,
#[serde(rename="numericResultAvailable")] pub f_5: bool }

pub type S0Root = S0FSFieldBinding;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S1Root { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="captureRole")] pub f_1: String,
#[serde(rename="owner")] pub f_2: S1RootF2,
#[serde(rename="dataResource")] pub f_3: S1RootF3,
#[serde(rename="dataEncoding")] pub f_4: String,
#[serde(rename="pointShape")] pub f_5: Vec<i64>,
#[serde(rename="axisOrder")] pub f_6: String,
#[serde(rename="coordinateRole")] pub f_7: String,
#[serde(rename="coordinatePhysicalUnit")] pub f_8: String,
#[serde(rename="scalarRole")] pub f_9: String,
#[serde(rename="scalarPhysicalUnit")] pub f_10: String,
#[serde(rename="domainTreatment")] pub f_11: String,
#[serde(rename="label")] pub f_12: String,
#[serde(rename="ruleId")] pub f_13: String,
#[serde(rename="ruleVersion")] pub f_14: String,
#[serde(rename="gridDivisions")] pub f_15: Vec<i64>,
#[serde(rename="gridToDisplayRowsF64Bits")] pub f_16: Vec<Vec<String>> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S1RootF2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S1RootF3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="resourceId")] pub f_1: String,
#[serde(rename="sha256")] pub f_2: String,
#[serde(rename="byteLength")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3Root { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="captureRole")] pub f_1: String,
#[serde(rename="owner")] pub f_2: S3RootF2,
#[serde(rename="dataResource")] pub f_3: S3RootF3,
#[serde(rename="maskResource")] pub f_4: S3RootF4,
#[serde(rename="dataEncoding")] pub f_5: String,
#[serde(rename="pointShape")] pub f_6: Vec<i64>,
#[serde(rename="axisOrder")] pub f_7: String,
#[serde(rename="originF64Bits")] pub f_8: Vec<String>,
#[serde(rename="stepColumnsF64Bits")] pub f_9: Vec<Vec<String>>,
#[serde(rename="coordinateRole")] pub f_10: String,
#[serde(rename="coordinatePhysicalUnit")] pub f_11: String,
#[serde(rename="scalarRole")] pub f_12: String,
#[serde(rename="scalarPhysicalUnit")] pub f_13: String,
#[serde(rename="domainTreatment")] pub f_14: String,
#[serde(rename="label")] pub f_15: String,
#[serde(rename="ruleId")] pub f_16: String,
#[serde(rename="ruleVersion")] pub f_17: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3RootF2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3RootF3 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="resourceId")] pub f_1: String,
#[serde(rename="sha256")] pub f_2: String,
#[serde(rename="byteLength")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S3RootF4 { V0(()), V1(S3RootF4V1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S3RootF4V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="resourceId")] pub f_1: String,
#[serde(rename="sha256")] pub f_2: String,
#[serde(rename="byteLength")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S4Root { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="target")] pub f_1: S4RootF1,
#[serde(rename="levels")] pub f_2: Vec<S4RootF2Item>,
#[serde(rename="levelMeaning")] pub f_3: String,
#[serde(rename="ruleId")] pub f_4: String,
#[serde(rename="ruleVersion")] pub f_5: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S4RootF1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S4RootF2Item { #[serde(rename="storedValueF32Bits")] pub f_0: String,
#[serde(rename="signMode")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSResampleAffineInput { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="targetCoreShape")] pub f_1: Vec<i64>,
#[serde(rename="targetOriginA")] pub f_2: S0Vec3,
#[serde(rename="targetStepBasisColumnsA")] pub f_3: S0Mat3,
#[serde(rename="targetAxisOrder")] pub f_4: String,
#[serde(rename="targetClosure")] pub f_5: Vec<bool>,
#[serde(rename="boundaryPolicy")] pub f_6: String,
#[serde(rename="dtype")] pub f_7: String,
#[serde(rename="maxResidentBytes")] pub f_8: i64,
#[serde(rename="maxOutputBytes")] pub f_9: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSFieldLinearTerm { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="coefficientDimensionless")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSLinearCombineFieldsInput { #[serde(rename="terms")] pub f_0: Vec<S9FSFieldLinearTerm>,
#[serde(rename="targetAxisOrder")] pub f_1: String,
#[serde(rename="targetClosure")] pub f_2: Vec<bool>,
#[serde(rename="dtype")] pub f_3: String,
#[serde(rename="maxResidentBytes")] pub f_4: i64,
#[serde(rename="maxOutputBytes")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSCartesianLineInput { #[serde(rename="field")] pub f_0: S0FSFieldBinding,
#[serde(rename="startA")] pub f_1: S0Vec3,
#[serde(rename="endA")] pub f_2: S0Vec3,
#[serde(rename="count")] pub f_3: i64,
#[serde(rename="boundaryPolicy")] pub f_4: String,
#[serde(rename="maxResidentBytes")] pub f_5: i64,
#[serde(rename="maxOutputBytes")] pub f_6: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSCartesianLineOutput { #[serde(rename="provenance")] pub f_0: S0FSProvenance,
#[serde(rename="startA")] pub f_1: S0Vec3,
#[serde(rename="endA")] pub f_2: S0Vec3,
#[serde(rename="count")] pub f_3: i64,
#[serde(rename="distanceA")] pub f_4: Vec<f64>,
#[serde(rename="values")] pub f_5: Vec<f64>,
#[serde(rename="valueUnit")] pub f_6: String,
#[serde(rename="endpointPolicy")] pub f_7: String,
#[serde(rename="interpolation")] pub f_8: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSCurveLinearTerm { #[serde(rename="curve")] pub f_0: S0Curve,
#[serde(rename="coefficientDimensionless")] pub f_1: f64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSLinearCombineCurvesInput { #[serde(rename="terms")] pub f_0: Vec<S9FSCurveLinearTerm>,
#[serde(rename="background")] pub f_1: S0Curve }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S9FSLinearCombineCurvesOutput { #[serde(rename="scaledTermCurves")] pub f_0: Vec<S0Curve>,
#[serde(rename="background")] pub f_1: S0Curve,
#[serde(rename="total")] pub f_2: S0Curve }

pub type S9Root = serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2Root { #[serde(rename="schemaVersion")] pub f_0: String,
#[serde(rename="captureRole")] pub f_1: String,
#[serde(rename="document")] pub f_2: S2RootF2,
#[serde(rename="orderedEntries")] pub f_3: Vec<S2RootF3Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3Item { #[serde(rename="owner")] pub f_0: S2RootF3ItemF0,
#[serde(rename="position")] pub f_1: S2RootF3ItemF1,
#[serde(rename="orientation")] pub f_2: S2RootF3ItemF2,
#[serde(rename="effectiveNativeMatrix4")] pub f_3: Vec<S2RootF3ItemF3Item>,
#[serde(rename="effectiveNativeTranslation")] pub f_4: Vec<S2RootF3ItemF4Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S2RootF3ItemF0 { V0(S2RootF3ItemF0V0), V1(S2RootF3ItemF0V1), V2(S2RootF3ItemF0V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF0V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF0V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF0V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1 { #[serde(rename="selector")] pub f_0: S2RootF3ItemF1F0,
#[serde(rename="currentAnchor")] pub f_1: Vec<S2RootF3ItemF1F1Item>,
#[serde(rename="referenceAnchor")] pub f_2: Vec<S2RootF3ItemF1F2Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0 { #[serde(rename="capturedSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S2RootF3ItemF1F0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S2RootF3ItemF1F0F1 { V0(S2RootF3ItemF1F0F1V0), V1(S2RootF3ItemF1F0F1V1), V2(S2RootF3ItemF1F0F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="target")] pub f_1: S2RootF3ItemF1F0F1V1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S2RootF3ItemF1F0F1V1F1 { V0(S2RootF3ItemF1F0F1V1F1V0), V1(S2RootF3ItemF1F0F1V1F1V1), V2(S2RootF3ItemF1F0F1V1F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V1F1V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V1F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V1F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F0F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F1Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF1F2Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2 { #[serde(rename="selector")] pub f_0: S2RootF3ItemF2F0,
#[serde(rename="storedFlagBits")] pub f_1: String,
#[serde(rename="currentFirst")] pub f_2: Vec<S2RootF3ItemF2F2Item>,
#[serde(rename="referenceFirst")] pub f_3: Vec<S2RootF3ItemF2F3Item>,
#[serde(rename="currentSecond")] pub f_4: Vec<S2RootF3ItemF2F4Item>,
#[serde(rename="referenceSecond")] pub f_5: Vec<S2RootF3ItemF2F5Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0 { #[serde(rename="capturedSelector")] pub f_0: i64,
#[serde(rename="reference")] pub f_1: S2RootF3ItemF2F0F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S2RootF3ItemF2F0F1 { V0(S2RootF3ItemF2F0F1V0), V1(S2RootF3ItemF2F0F1V1), V2(S2RootF3ItemF2F0F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V0 { #[serde(rename="kind")] pub f_0: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="target")] pub f_1: S2RootF3ItemF2F0F1V1F1 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S2RootF3ItemF2F0F1V1F1 { V0(S2RootF3ItemF2F0F1V1F1V0), V1(S2RootF3ItemF2F0F1V1F1V1), V2(S2RootF3ItemF2F0F1V1F1V2) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V1F1V0 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V1F1V1 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V1F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="objectKind")] pub f_1: String,
#[serde(rename="objectId")] pub f_2: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F0F1V2 { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="reason")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F2Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F3Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F4Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF2F5Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF3Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S2RootF3ItemF4Item { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="ieeeHexBE")] pub f_1: String }

pub type S5F32Bits = S8F32Bits;

pub type S5F64Bits = S8F64Bits;

pub type S5ScalarSource = S8ScalarSource;

pub type S5Vec3F32Bits = S8Vec3F32Bits;

pub type S5Vec3F64Bits = S8Vec3F64Bits;

pub type S5SHA256 = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeSamplerContext { #[serde(rename="source")] pub f_0: S5ScalarSource,
#[serde(rename="ownerPhaseId")] pub f_1: String,
#[serde(rename="coreDivisions")] pub f_2: Vec<i64>,
#[serde(rename="expandedScalarF32Resource")] pub f_3: S5SHA256,
#[serde(rename="sourceCartesianColumnsA")] pub f_4: Vec<S5Vec3F64Bits>,
#[serde(rename="inputChart")] pub f_5: String,
#[serde(rename="producerRuleDigest")] pub f_6: S5SHA256 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativePointSampleInput { #[serde(rename="context")] pub f_0: S5NativeSamplerContext,
#[serde(rename="fractionalF32")] pub f_1: S5Vec3F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativePointSampleOutput { #[serde(rename="valueF32Bits")] pub f_0: S5F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeLineProfileInput { #[serde(rename="context")] pub f_0: S5NativeSamplerContext,
#[serde(rename="point1FractionalF32")] pub f_1: S5Vec3F32Bits,
#[serde(rename="point2FractionalF32")] pub f_2: S5Vec3F32Bits,
#[serde(rename="nativeCountI32")] pub f_3: i64,
#[serde(rename="titleUtf8")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeLineSample { #[serde(rename="fractionalF32")] pub f_0: S5Vec3F32Bits,
#[serde(rename="distanceAF32Bits")] pub f_1: S5F32Bits,
#[serde(rename="valueF32Bits")] pub f_2: S5F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeLineComplete { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="samples")] pub f_1: Vec<S5NativeLineSample>,
#[serde(rename="textUtf8")] pub f_2: String,
#[serde(rename="encoding")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeLineEmpty { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="nativePrintedCountI32")] pub f_1: i64,
#[serde(rename="samples")] pub f_2: Vec<serde_json::Value>,
#[serde(rename="textUtf8")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeReferenceFailure { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="code")] pub f_1: String,
#[serde(rename="oldModelPreserved")] pub f_2: bool,
#[serde(rename="noCandidatePublished")] pub f_3: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S5NativeLineProfileOutput { V0(S5NativeLineComplete), V1(S5NativeLineEmpty), V2(S5NativeReferenceFailure) }

pub type S5ContourLevelMetadata6 = Vec<S5F32Bits>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5ContourSettingsLinear { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="enabled")] pub f_1: bool,
#[serde(rename="minimumF32Bits")] pub f_2: S5F32Bits,
#[serde(rename="maximumF32Bits")] pub f_3: S5F32Bits,
#[serde(rename="stepF32Bits")] pub f_4: S5F32Bits,
#[serde(rename="majorIntervalI32")] pub f_5: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5ContourSettingsGeometric { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="enabled")] pub f_1: bool,
#[serde(rename="minimumExponentI32")] pub f_2: i64,
#[serde(rename="maximumExponentI32")] pub f_3: i64,
#[serde(rename="subdivisionsI32")] pub f_4: i64,
#[serde(rename="baseF32Bits")] pub f_5: S5F32Bits,
#[serde(rename="coefficientF32Bits")] pub f_6: S5F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourLevelsInput { #[serde(rename="dataMinimumF32Bits")] pub f_0: S5F32Bits,
#[serde(rename="dataMaximumF32Bits")] pub f_1: S5F32Bits,
#[serde(rename="settings")] pub f_2: S5NativeContourLevelsInputF2 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S5NativeContourLevelsInputF2 { V0(S5ContourSettingsLinear), V1(S5ContourSettingsGeometric) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourLevelsOutput { #[serde(rename="metadata")] pub f_0: Vec<S5ContourLevelMetadata6> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourSegmentsInput { #[serde(rename="vertices")] pub f_0: Vec<S5Vec3F32Bits>,
#[serde(rename="values")] pub f_1: Vec<S5F32Bits>,
#[serde(rename="polygonIndices")] pub f_2: Vec<i64>,
#[serde(rename="levelMetadata")] pub f_3: Vec<S5ContourLevelMetadata6> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourSegment { #[serde(rename="levelOrdinal0")] pub f_0: i64,
#[serde(rename="endpoints")] pub f_1: Vec<S5Vec3F32Bits> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourSegmentsOutput { #[serde(rename="segments")] pub f_0: Vec<S5NativeContourSegment>,
#[serde(rename="unpairedFinalIntersections")] pub f_1: Vec<S5NativeContourSegmentsOutputF1Item> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativeContourSegmentsOutputF1Item { #[serde(rename="levelOrdinal0")] pub f_0: i64,
#[serde(rename="point")] pub f_1: S5Vec3F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativePolyIntegralInput { #[serde(rename="profile")] pub f_0: String,
#[serde(rename="documentId")] pub f_1: String,
#[serde(rename="epoch")] pub f_2: String,
#[serde(rename="expectedRevision")] pub f_3: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S5NativePolyIntegralOutput { #[serde(rename="kind")] pub f_0: String,
#[serde(rename="numericIntegrationPerformed")] pub f_1: bool,
#[serde(rename="modelChanged")] pub f_2: bool,
#[serde(rename="outputAppendUtf8")] pub f_3: String,
#[serde(rename="warningCode")] pub f_4: String }

pub type S5Root = serde_json::Value;

pub type S12ScalarSource = S8ScalarSource;

pub type S12F32Bits = S8F32Bits;

pub type S12F64Bits = S8F64Bits;

pub type S12Vec3F32Bits = S8Vec3F32Bits;

pub type S12Vec3F64Bits = S8Vec3F64Bits;

pub type S12Mat4ColumnsF64Bits = S8Mat4ColumnsF64Bits;

pub type S12Digest = String;

pub type S12U32 = i64;

pub type S12Mat3RowsF64Bits = Vec<S12Vec3F64Bits>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeSliceFrame { #[serde(rename="dimensionD")] pub f_0: f64,
#[serde(rename="crystallographicToOrthoRowsF64Bits")] pub f_1: S12Mat3RowsF64Bits,
#[serde(rename="orthoToCrystallographicRowsF64Bits")] pub f_2: S12Mat3RowsF64Bits,
#[serde(rename="currentC2XYZColumnsF64Bits")] pub f_3: S12Mat4ColumnsF64Bits,
#[serde(rename="sourceStateDigest")] pub f_4: S12Digest }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeOrthoCutoffPlaneGeometry { #[serde(rename="normalOrthoF64Bits")] pub f_0: S12Vec3F64Bits,
#[serde(rename="distanceAF64Bits")] pub f_1: S12F64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeOrthoCutoffPlane { #[serde(rename="sourcePlaneDigest")] pub f_0: S12Digest,
#[serde(rename="geometry")] pub f_1: S12NativeOrthoCutoffPlaneGeometry }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeFractionalBox { #[serde(rename="xMinMaxF32Bits")] pub f_0: Vec<S12F32Bits>,
#[serde(rename="yMinMaxF32Bits")] pub f_1: Vec<S12F32Bits>,
#[serde(rename="zMinMaxF32Bits")] pub f_2: Vec<S12F32Bits> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeScalarWindow { #[serde(rename="lowerF32Bits")] pub f_0: S12F32Bits,
#[serde(rename="upperF32Bits")] pub f_1: S12F32Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S12NativeContourSettings { V0(S5ContourSettingsLinear), V1(S5ContourSettingsGeometric) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeObliqueSliceInput { #[serde(rename="source")] pub f_0: S12ScalarSource,
#[serde(rename="frame")] pub f_1: S12NativeSliceFrame,
#[serde(rename="gridIntervalsXYZ")] pub f_2: Vec<i64>,
#[serde(rename="sourceNormalF64Bits")] pub f_3: S12Vec3F64Bits,
#[serde(rename="sourceStateDigest")] pub f_4: S12Digest,
#[serde(rename="kind")] pub f_5: String,
#[serde(rename="distanceAF64Bits")] pub f_6: S12F64Bits,
#[serde(rename="fractionalBox")] pub f_7: S12NativeFractionalBox,
#[serde(rename="scalarWindow")] pub f_8: S12NativeScalarWindow,
#[serde(rename="sourceDataRangeF32Bits")] pub f_9: Vec<S12F32Bits>,
#[serde(rename="contourSettings")] pub f_10: S12NativeContourSettings }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S12NativeSlicePlaneRecipe { V0(S12NativeSlicePlaneRecipeV0), V1(S12NativeSlicePlaneRecipeV1) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeSlicePlaneRecipeV0 { #[serde(rename="mode")] pub f_0: String,
#[serde(rename="sourceStyleByte")] pub f_1: i64 }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeSlicePlaneRecipeV1 { #[serde(rename="mode")] pub f_0: String,
#[serde(rename="sourceStyleByte")] pub f_1: i64,
#[serde(rename="upperDistanceAF64Bits")] pub f_2: S12F64Bits }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativePeriodicSliceInput { #[serde(rename="source")] pub f_0: S12ScalarSource,
#[serde(rename="frame")] pub f_1: S12NativeSliceFrame,
#[serde(rename="gridIntervalsXYZ")] pub f_2: Vec<i64>,
#[serde(rename="sourceNormalF64Bits")] pub f_3: S12Vec3F64Bits,
#[serde(rename="sourceStateDigest")] pub f_4: S12Digest,
#[serde(rename="centerFractionalF64Bits")] pub f_5: S12Vec3F64Bits,
#[serde(rename="span1FractionalF32Bits")] pub f_6: S12Vec3F32Bits,
#[serde(rename="span2FractionalF32Bits")] pub f_7: S12Vec3F32Bits,
#[serde(rename="lowerDistanceAF64Bits")] pub f_8: S12F64Bits,
#[serde(rename="sourceDataRangeF32Bits")] pub f_9: Vec<S12F32Bits>,
#[serde(rename="contourSettings")] pub f_10: S12NativeContourSettings,
#[serde(rename="kind")] pub f_11: String,
#[serde(rename="planeRecipe")] pub f_12: S12NativeSlicePlaneRecipe }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativePlanePolygon { #[serde(rename="verticesCurrentXYZF64Bits")] pub f_0: Vec<S12Vec3F64Bits>,
#[serde(rename="orderedClosedCycle")] pub f_1: bool }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeSliceMesh { #[serde(rename="verticesPlaneF32Bits")] pub f_0: Vec<S12Vec3F32Bits>,
#[serde(rename="valuesF32Bits")] pub f_1: Vec<S12F32Bits>,
#[serde(rename="triangles")] pub f_2: Vec<Vec<S12U32>> }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeObliqueSliceComplete { #[serde(rename="sourceStateDigest")] pub f_0: S12Digest,
#[serde(rename="sourceRuleDigest")] pub f_1: S12Digest,
#[serde(rename="planeHColumnsF64Bits")] pub f_2: S12Mat4ColumnsF64Bits,
#[serde(rename="kind")] pub f_3: String,
#[serde(rename="planeConstantF64Bits")] pub f_4: S12F64Bits,
#[serde(rename="normalOrthoUnnormalizedF64Bits")] pub f_5: S12Vec3F64Bits,
#[serde(rename="roundedIndexBoundsXYZ")] pub f_6: Vec<Vec<i64>>,
#[serde(rename="fullBoundary")] pub f_7: S12NativePlanePolygon,
#[serde(rename="meshAfterScalarWindow")] pub f_8: S12NativeSliceMesh,
#[serde(rename="contourConsumer")] pub f_9: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativeObliqueSliceEmpty { #[serde(rename="sourceStateDigest")] pub f_0: S12Digest,
#[serde(rename="sourceRuleDigest")] pub f_1: S12Digest,
#[serde(rename="planeHColumnsF64Bits")] pub f_2: S12Mat4ColumnsF64Bits,
#[serde(rename="kind")] pub f_3: String,
#[serde(rename="reason")] pub f_4: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S12NativeObliqueSliceOutput { V0(S12NativeObliqueSliceComplete), V1(S12NativeObliqueSliceEmpty) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct S12NativePeriodicSliceOutput { #[serde(rename="sourceStateDigest")] pub f_0: S12Digest,
#[serde(rename="sourceRuleDigest")] pub f_1: S12Digest,
#[serde(rename="planeHColumnsF64Bits")] pub f_2: S12Mat4ColumnsF64Bits,
#[serde(rename="kind")] pub f_3: String,
#[serde(rename="intervalsUV")] pub f_4: Vec<S12U32>,
#[serde(rename="normalIntervalsU32")] pub f_5: S12U32,
#[serde(rename="continuousLayerDeltaF64Bits")] pub f_6: S12F64Bits,
#[serde(rename="additionalLayerCountU32")] pub f_7: S12U32,
#[serde(rename="thicknessWeightF32Bits")] pub f_8: S12F32Bits,
#[serde(rename="lowerPlaneConstantF64Bits")] pub f_9: S12F64Bits,
#[serde(rename="upperPlaneConstantF64Bits")] pub f_10: S12NativePeriodicSliceOutputF10,
#[serde(rename="sampleFractionalPointsF32Bits")] pub f_11: Vec<S12Vec3F32Bits>,
#[serde(rename="sampleValuesPerLayerF32Bits")] pub f_12: Vec<Vec<S12F32Bits>>,
#[serde(rename="renderPositionsPlaneF32Bits")] pub f_13: Vec<S12Vec3F32Bits>,
#[serde(rename="completeValuesF32Bits")] pub f_14: Vec<S12F32Bits>,
#[serde(rename="contourQuads")] pub f_15: Vec<Vec<S12U32>>,
#[serde(rename="valueQuantity")] pub f_16: String,
#[serde(rename="contourConsumer")] pub f_17: String }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S12NativePeriodicSliceOutputF10 { V0(()), V1(S12F64Bits) }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum S12Root { V0(S12NativeObliqueSliceInput), V1(S12NativePeriodicSliceInput) }
