use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PipelineState {
    Deleted,
    Deploying,
    Failed,
    Idle,
    Recovering,
    Resetting,
    Running,
    Starting,
    Stopping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpdateState {
    Canceled,
    Completed,
    Created,
    Failed,
    Initializing,
    Queued,
    Resetting,
    Running,
    SettingUpTables,
    Stopping,
    WaitingForResources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventLevel {
    Error,
    Info,
    Metrics,
    Warn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Health {
    Healthy,
    Unhealthy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpdateCause {
    ApiCall,
    InfrastructureMaintenance,
    JobTask,
    RetryOnFailure,
    SchemaChange,
    ServiceUpgrade,
    UserAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutoscaleMode {
    Enhanced,
    Legacy,
}

// ============================================================================
// Pipeline cluster types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAutoscale {
    pub min_workers: i32,
    pub max_workers: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<AutoscaleMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineCluster {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub node_type_id: Option<String>,
    #[serde(default)]
    pub driver_node_type_id: Option<String>,
    #[serde(default)]
    pub num_workers: Option<i32>,
    #[serde(default)]
    pub autoscale: Option<ClusterAutoscale>,
    #[serde(default)]
    pub spark_conf: Option<HashMap<String, String>>,
    #[serde(default)]
    pub spark_env_vars: Option<HashMap<String, String>>,
    #[serde(default)]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(default)]
    pub instance_pool_id: Option<String>,
    #[serde(default)]
    pub driver_instance_pool_id: Option<String>,
    #[serde(default)]
    pub policy_id: Option<String>,
    #[serde(default)]
    pub ssh_public_keys: Option<Vec<String>>,
}

// ============================================================================
// Pipeline library types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookLibrary {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLibrary {
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineLibrary {
    #[serde(default)]
    pub notebook: Option<NotebookLibrary>,
    #[serde(default)]
    pub file: Option<FileLibrary>,
    #[serde(default)]
    pub jar: Option<String>,
    #[serde(default)]
    pub whl: Option<String>,
}

// ============================================================================
// Pipeline spec & configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filters {
    #[serde(default)]
    pub include: Option<Vec<String>>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notifications {
    #[serde(default)]
    pub alerts: Option<Vec<String>>,
    #[serde(default)]
    pub email_recipients: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSpec {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catalog: Option<String>,
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub storage: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default)]
    pub continuous: Option<bool>,
    #[serde(default)]
    pub development: Option<bool>,
    #[serde(default)]
    pub photon: Option<bool>,
    #[serde(default)]
    pub serverless: Option<bool>,
    #[serde(default)]
    pub clusters: Option<Vec<PipelineCluster>>,
    #[serde(default)]
    pub libraries: Option<Vec<PipelineLibrary>>,
    #[serde(default)]
    pub filters: Option<Filters>,
    #[serde(default)]
    pub notifications: Option<Vec<Notifications>>,
    #[serde(default)]
    pub configuration: Option<HashMap<String, String>>,
    #[serde(default)]
    pub root_path: Option<String>,
    #[serde(default)]
    pub budget_policy_id: Option<String>,
}

// ============================================================================
// Pipeline (get response)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub state: Option<PipelineState>,
    #[serde(default)]
    pub health: Option<Health>,
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub creator_user_name: Option<String>,
    #[serde(default)]
    pub run_as_user_name: Option<String>,
    #[serde(default)]
    pub cause: Option<String>,
    #[serde(default)]
    pub last_modified: Option<i64>,
    #[serde(default)]
    pub spec: Option<PipelineSpec>,
    #[serde(default)]
    pub latest_updates: Option<Vec<UpdateStateInfo>>,
}

// ============================================================================
// Pipeline list item (returned by list pipelines)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStateInfo {
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub state: Option<PipelineState>,
    #[serde(default)]
    pub health: Option<Health>,
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub creator_user_name: Option<String>,
    #[serde(default)]
    pub run_as_user_name: Option<String>,
    #[serde(default)]
    pub latest_updates: Option<Vec<UpdateStateInfo>>,
}

// ============================================================================
// Update types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStateInfo {
    #[serde(default)]
    pub update_id: Option<String>,
    #[serde(default)]
    pub state: Option<UpdateState>,
    #[serde(default)]
    pub creation_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    #[serde(default)]
    pub update_id: Option<String>,
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub state: Option<UpdateState>,
    #[serde(default)]
    pub cause: Option<UpdateCause>,
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub full_refresh: Option<bool>,
    #[serde(default)]
    pub full_refresh_selection: Option<Vec<String>>,
    #[serde(default)]
    pub refresh_selection: Option<Vec<String>>,
    #[serde(default)]
    pub validate_only: Option<bool>,
    #[serde(default)]
    pub config: Option<PipelineSpec>,
}

// ============================================================================
// Event types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub pipeline_name: Option<String>,
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub update_id: Option<String>,
    #[serde(default)]
    pub flow_id: Option<String>,
    #[serde(default)]
    pub flow_name: Option<String>,
    #[serde(default)]
    pub dataset_name: Option<String>,
    #[serde(default)]
    pub batch_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(default)]
    pub fatal: Option<bool>,
    #[serde(default)]
    pub exceptions: Option<Vec<SerializedException>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedException {
    #[serde(default)]
    pub class_name: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineEvent {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub level: Option<EventLevel>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub origin: Option<Origin>,
    #[serde(default)]
    pub error: Option<ErrorDetail>,
}

// ============================================================================
// Request types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreatePipeline {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<PipelineCluster>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<PipelineLibrary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notifications>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_duplicate_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_policy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditPipeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub development: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<PipelineCluster>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<PipelineLibrary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notifications>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_duplicate_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_last_modified: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StartUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refresh: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_refresh_selection: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_selection: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<UpdateCause>,
}

// ============================================================================
// Response types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePipelineResponse {
    #[serde(default)]
    pub pipeline_id: Option<String>,
    #[serde(default)]
    pub effective_settings: Option<PipelineSpec>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartUpdateResponse {
    #[serde(default)]
    pub update_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUpdateResponse {
    #[serde(default)]
    pub update: Option<Update>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPipelinesResponse {
    #[serde(default)]
    pub statuses: Vec<PipelineStateInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUpdatesResponse {
    #[serde(default)]
    pub updates: Vec<Update>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPipelineEventsResponse {
    #[serde(default)]
    pub events: Vec<PipelineEvent>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub(crate) struct PipelineId {
    pub pipeline_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
