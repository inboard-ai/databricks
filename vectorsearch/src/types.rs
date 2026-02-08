use serde::{Deserialize, Serialize};

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
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
