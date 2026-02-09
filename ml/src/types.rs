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

#[derive(Debug, Clone, Serialize)]
pub struct UpdateExperimentRequest {
    pub experiment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetExperimentTagRequest {
    pub experiment_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogModelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_json: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogInputsRequest {
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetInput>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogOutputsRequest {
    pub run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<ModelOutput>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelOutput {
    pub model_id: String,
    pub step: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tags: Option<Vec<InputTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dataset: Option<Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTag {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteRunsRequest {
    pub experiment_id: String,
    pub max_timestamp_millis: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runs: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRunsResponse {
    #[serde(default)]
    pub runs_deleted: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoreRunsRequest {
    pub experiment_id: String,
    pub min_timestamp_millis: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_runs: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestoreRunsResponse {
    #[serde(default)]
    pub runs_restored: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMetricHistoryResponse {
    #[serde(default)]
    pub metrics: Vec<Metric>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Experiment permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<ExperimentAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<ExperimentPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetExperimentPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<ExperimentPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExperimentPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExperimentPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<ExperimentAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExperimentAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
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
// Model Registry types (workspace legacy)
// ============================================================================

/// Model as returned by the workspace model registry (distinct from RegisteredModel
/// used by the UC-based registered-models endpoints).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
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
    pub tags: Option<Vec<ModelTag>>,
    #[serde(default)]
    pub latest_versions: Option<Vec<ModelVersion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTag {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

/// Databricks-enriched model returned by the get-model Databricks endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ModelDatabricks {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<ModelTag>>,
    #[serde(default)]
    pub latest_versions: Option<Vec<ModelVersion>>,
}

// -- Create / Rename / Update / Delete model --

#[derive(Debug, Clone, Serialize)]
pub struct CreateModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ModelTag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateModelResponse {
    #[serde(default)]
    pub registered_model: Option<Model>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetModelResponse {
    #[serde(default)]
    pub registered_model_databricks: Option<ModelDatabricks>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListModelsResponse {
    #[serde(default)]
    pub registered_models: Vec<Model>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchModelsResponse {
    #[serde(default)]
    pub registered_models: Vec<Model>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenameModelResponse {
    #[serde(default)]
    pub registered_model: Option<Model>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateModelRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateModelResponse {
    #[serde(default)]
    pub registered_model: Option<Model>,
}

// -- Model tags --

#[derive(Debug, Clone, Serialize)]
pub struct SetModelTagRequest {
    pub name: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteModelTagRequest {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetModelVersionTagRequest {
    pub name: String,
    pub version: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteModelVersionTagRequest {
    pub name: String,
    pub version: String,
    pub key: String,
}

// -- Get latest versions / download URI --

#[derive(Debug, Clone, Serialize)]
pub struct GetLatestVersionsRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetLatestVersionsResponse {
    #[serde(default)]
    pub model_versions: Vec<ModelVersion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetModelVersionDownloadUriResponse {
    #[serde(default)]
    pub artifact_uri: Option<String>,
}

// -- Transition stages --

#[derive(Debug, Clone, Serialize)]
pub struct TransitionModelVersionStageDatabricksRequest {
    pub name: String,
    pub version: String,
    pub stage: String,
    pub archive_existing_versions: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelVersionDatabricks {
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
    pub status: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<ModelTag>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransitionStageResponse {
    #[serde(default)]
    pub model_version_databricks: Option<ModelVersionDatabricks>,
}

// -- Transition requests --

#[derive(Debug, Clone, Serialize)]
pub struct CreateTransitionRequestRequest {
    pub name: String,
    pub version: String,
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransitionRequest {
    #[serde(default)]
    pub available_actions: Option<Vec<String>>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub to_stage: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTransitionRequestResponse {
    #[serde(default)]
    pub request: Option<TransitionRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApproveTransitionRequestRequest {
    pub name: String,
    pub version: String,
    pub stage: String,
    pub archive_existing_versions: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    #[serde(default)]
    pub activity_type: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub from_stage: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub system_comment: Option<String>,
    #[serde(default)]
    pub to_stage: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApproveTransitionRequestResponse {
    #[serde(default)]
    pub activity: Option<Activity>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RejectTransitionRequestRequest {
    pub name: String,
    pub version: String,
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RejectTransitionRequestResponse {
    #[serde(default)]
    pub activity: Option<Activity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTransitionRequestResponse {
    #[serde(default)]
    pub activity: Option<Activity>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTransitionRequestsResponse {
    #[serde(default)]
    pub requests: Vec<Activity>,
}

// -- Comments --

#[derive(Debug, Clone, Serialize)]
pub struct CreateCommentRequest {
    pub name: String,
    pub version: String,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommentObject {
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub available_actions: Option<Vec<String>>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentResponse {
    #[serde(default)]
    pub comment: Option<CommentObject>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCommentRequest {
    pub id: String,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommentResponse {
    #[serde(default)]
    pub comment: Option<CommentObject>,
}

// -- Webhooks --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpUrlSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_ssl_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub workspace_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRegistryWebhookRequest {
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_spec: Option<HttpUrlSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_spec: Option<JobSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryWebhook {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub events: Option<Vec<String>>,
    #[serde(default)]
    pub http_url_spec: Option<HttpUrlSpec>,
    #[serde(default)]
    pub job_spec: Option<JobSpec>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWebhookResponse {
    #[serde(default)]
    pub webhook: Option<RegistryWebhook>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRegistryWebhooksResponse {
    #[serde(default)]
    pub webhooks: Vec<RegistryWebhook>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestRegistryWebhookRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestRegistryWebhookResponse {
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRegistryWebhookRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_spec: Option<HttpUrlSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_spec: Option<JobSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateWebhookResponse {
    #[serde(default)]
    pub webhook: Option<RegistryWebhook>,
}

// ============================================================================
// Registered Model permission types (for ModelRegistry service)
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct RegisteredModelPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<RegisteredModelAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisteredModelAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<RegisteredModelPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisteredModelPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRegisteredModelPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<RegisteredModelPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisteredModelPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisteredModelPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<RegisteredModelAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisteredModelAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EmptyResponse {}
