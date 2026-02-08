use serde::{Deserialize, Serialize};

// ============================================================================
// Common / shared types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

// ============================================================================
// Experiment types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    #[serde(default)]
    pub experiment_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub artifact_location: Option<String>,
    #[serde(default)]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub last_update_time: Option<i64>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExperiment {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExperimentResponse {
    #[serde(default)]
    pub experiment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteExperimentRequest {
    pub experiment_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoreExperimentRequest {
    pub experiment_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetExperimentResponse {
    #[serde(default)]
    pub experiment: Option<Experiment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListExperimentsResponse {
    #[serde(default)]
    pub experiments: Vec<Experiment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchExperimentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchExperimentsResponse {
    #[serde(default)]
    pub experiments: Vec<Experiment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Run types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunStatus {
    Running,
    Scheduled,
    Finished,
    Failed,
    Killed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunInfo {
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub run_name: Option<String>,
    #[serde(default)]
    pub experiment_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub status: Option<RunStatus>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub artifact_uri: Option<String>,
    #[serde(default)]
    pub lifecycle_stage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub step: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunData {
    #[serde(default)]
    pub metrics: Option<Vec<Metric>>,
    #[serde(default)]
    pub params: Option<Vec<Param>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    #[serde(default)]
    pub info: Option<RunInfo>,
    #[serde(default)]
    pub data: Option<RunData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRunResponse {
    #[serde(default)]
    pub run: Option<Run>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRunResponse {
    #[serde(default)]
    pub run: Option<Run>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRunResponse {
    #[serde(default)]
    pub run_info: Option<RunInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchRunsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_view_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchRunsResponse {
    #[serde(default)]
    pub runs: Vec<Run>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteRunRequest {
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoreRunRequest {
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogMetricRequest {
    pub key: String,
    pub value: f64,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogParamRequest {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogBatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Param>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetTagRequest {
    pub run_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteTagRequest {
    pub run_id: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub is_dir: Option<bool>,
    #[serde(default)]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArtifactsResponse {
    #[serde(default)]
    pub files: Vec<Artifact>,
    #[serde(default)]
    pub root_uri: Option<String>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Registered model types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModel {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub latest_versions: Option<Vec<ModelVersion>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRegisteredModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRegisteredModelResponse {
    #[serde(default)]
    pub registered_model: Option<RegisteredModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRegisteredModelResponse {
    #[serde(default)]
    pub registered_model_databricks: Option<RegisteredModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRegisteredModelsResponse {
    #[serde(default)]
    pub registered_models: Vec<RegisteredModel>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRegisteredModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRegisteredModelResponse {
    #[serde(default)]
    pub registered_model: Option<RegisteredModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchRegisteredModelsResponse {
    #[serde(default)]
    pub registered_models: Vec<RegisteredModel>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Model version types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelVersionStatus {
    PendingRegistration,
    FailedRegistration,
    Ready,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub current_stage: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub run_link: Option<String>,
    #[serde(default)]
    pub status: Option<ModelVersionStatus>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateModelVersionRequest {
    pub name: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateModelVersionResponse {
    #[serde(default)]
    pub model_version: Option<ModelVersion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetModelVersionResponse {
    #[serde(default)]
    pub model_version: Option<ModelVersion>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateModelVersionRequest {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateModelVersionResponse {
    #[serde(default)]
    pub model_version: Option<ModelVersion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchModelVersionsResponse {
    #[serde(default)]
    pub model_versions: Vec<ModelVersion>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EmptyResponse {}
