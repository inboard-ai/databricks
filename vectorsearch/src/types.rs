use serde::{Deserialize, Serialize};
use serde_json;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndexType {
    DeltaSync,
    DirectAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndpointState {
    Offline,
    Online,
    Provisioning,
    RedState,
    YellowState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndpointType {
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PipelineType {
    Continuous,
    Triggered,
}

// ============================================================================
// Endpoint types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub endpoint_type: Option<EndpointType>,
    #[serde(default)]
    pub endpoint_status: Option<EndpointStatus>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_user: Option<String>,
    #[serde(default)]
    pub num_indexes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointStatus {
    #[serde(default)]
    pub state: Option<EndpointState>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEndpoint {
    pub name: String,
    pub endpoint_type: EndpointType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_policy_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListEndpointsResponse {
    #[serde(default)]
    pub endpoints: Vec<Endpoint>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Index types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub endpoint_name: Option<String>,
    #[serde(default)]
    pub index_type: Option<IndexType>,
    #[serde(default)]
    pub primary_key: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub delta_sync_index_spec: Option<DeltaSyncIndexSpec>,
    #[serde(default)]
    pub direct_access_index_spec: Option<DirectAccessIndexSpec>,
    #[serde(default)]
    pub status: Option<IndexStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    #[serde(default)]
    pub ready: Option<bool>,
    #[serde(default)]
    pub indexed_row_count: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub index_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSyncIndexSpec {
    #[serde(default)]
    pub source_table: Option<String>,
    #[serde(default)]
    pub pipeline_type: Option<PipelineType>,
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub embedding_source_columns: Option<Vec<EmbeddingSourceColumn>>,
    #[serde(default)]
    pub embedding_vector_columns: Option<Vec<EmbeddingVectorColumn>>,
    #[serde(default)]
    pub embedding_writeback_table: Option<String>,
    #[serde(default)]
    pub columns_to_sync: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectAccessIndexSpec {
    #[serde(default)]
    pub embedding_source_columns: Option<Vec<EmbeddingSourceColumn>>,
    #[serde(default)]
    pub embedding_vector_columns: Option<Vec<EmbeddingVectorColumn>>,
    #[serde(default)]
    pub schema_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingSourceColumn {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub embedding_model_endpoint_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingVectorColumn {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub embedding_dimension: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateIndex {
    pub name: String,
    pub endpoint_name: String,
    pub index_type: IndexType,
    pub primary_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_sync_index_spec: Option<DeltaSyncIndexSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_access_index_spec: Option<DirectAccessIndexSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniIndex {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub endpoint_name: Option<String>,
    #[serde(default)]
    pub index_type: Option<IndexType>,
    #[serde(default)]
    pub primary_key: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListIndexesResponse {
    #[serde(default)]
    pub vector_indexes: Vec<MiniIndex>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Endpoint metric types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct RetrieveMetricsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity_in_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RetrieveMetricsResponse {
    #[serde(default)]
    pub metric_values: Vec<MetricValues>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    #[serde(default)]
    pub labels: Option<Vec<MetricLabel>>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub percentile: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricLabel {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValues {
    #[serde(default)]
    pub metric: Option<Metric>,
    #[serde(default)]
    pub values: Option<Vec<MetricValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub value: Option<f64>,
}

// ============================================================================
// Endpoint budget policy types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBudgetPolicyRequest {
    pub budget_policy_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBudgetPolicyResponse {
    #[serde(default)]
    pub effective_budget_policy_id: Option<String>,
}

// ============================================================================
// Endpoint custom tags types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTag {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCustomTagsRequest {
    pub custom_tags: Vec<CustomTag>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomTagsResponse {
    #[serde(default)]
    pub custom_tags: Option<Vec<CustomTag>>,
    #[serde(default)]
    pub name: Option<String>,
}

// ============================================================================
// Index query types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct QueryIndexRequest {
    pub columns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_to_rerank: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_vector: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reranker: Option<RerankerConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankerConfig {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub parameters: Option<RerankerParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RerankerParameters {
    #[serde(default)]
    pub columns_to_rerank: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryIndexResponse {
    #[serde(default)]
    pub manifest: Option<ResultManifest>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub result: Option<ResultData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultManifest {
    #[serde(default)]
    pub column_count: Option<i32>,
    #[serde(default)]
    pub columns: Option<Vec<ColumnInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub type_text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultData {
    #[serde(default)]
    pub data_array: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub row_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryNextPageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    pub page_token: String,
}

// ============================================================================
// Index scan types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct ScanIndexRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_primary_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScanIndexResponse {
    #[serde(default)]
    pub data: Option<Vec<ScanDataEntry>>,
    #[serde(default)]
    pub last_primary_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScanDataEntry {
    #[serde(default)]
    pub fields: Option<Vec<MapStringValueEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapStringValueEntry {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
}

// ============================================================================
// Index upsert types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct UpsertDataRequest {
    pub inputs_json: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertDataResponse {
    #[serde(default)]
    pub result: Option<UpsertDataResult>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertDataResult {
    #[serde(default)]
    pub failed_primary_keys: Option<Vec<String>>,
    #[serde(default)]
    pub success_row_count: Option<i64>,
}

// ============================================================================
// Index delete data types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct DeleteDataRequest {
    pub primary_keys: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDataResponse {
    #[serde(default)]
    pub result: Option<DeleteDataResult>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDataResult {
    #[serde(default)]
    pub failed_primary_keys: Option<Vec<String>>,
    #[serde(default)]
    pub success_row_count: Option<i64>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
