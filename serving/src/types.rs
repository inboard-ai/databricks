use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Enums
// ============================================================================

/// Whether the endpoint is ready to receive traffic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReadyState {
    Ready,
    NotReady,
}

/// The state of an endpoint's config update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfigUpdateState {
    InProgress,
    NotUpdating,
    UpdateFailed,
    UpdateCanceled,
}

/// Deployment state for a served model / entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeploymentState {
    DeploymentCreating,
    DeploymentReady,
    DeploymentFailed,
    DeploymentRecovering,
    DeploymentAborted,
}

/// The role of a chat message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

/// Workload type for a served entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkloadType {
    Cpu,
    GpuSmall,
    GpuMedium,
    GpuLarge,
    MultigpuMedium,
}

// ============================================================================
// Core state types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointState {
    #[serde(default)]
    pub ready: Option<ReadyState>,
    #[serde(default)]
    pub config_update: Option<ConfigUpdateState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServedEntityState {
    #[serde(default)]
    pub deployment: Option<DeploymentState>,
    #[serde(default)]
    pub deployment_state_message: Option<String>,
}

// ============================================================================
// Tag
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// ============================================================================
// Traffic configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficConfig {
    #[serde(default)]
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub served_model_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub served_entity_name: Option<String>,
    #[serde(default)]
    pub traffic_percentage: i32,
}

// ============================================================================
// Served entity / model types (input for create/update)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServedEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<WorkloadType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_to_zero_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_provisioned_throughput: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_provisioned_throughput: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_vars: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServedModel {
    pub model_name: String,
    pub model_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<WorkloadType>,
    #[serde(default)]
    pub scale_to_zero_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_vars: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_profile_arn: Option<String>,
}

// ============================================================================
// Served entity / model types (output from API)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServedEntityOutput {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub entity_name: Option<String>,
    #[serde(default)]
    pub entity_version: Option<String>,
    #[serde(default)]
    pub workload_size: Option<String>,
    #[serde(default)]
    pub workload_type: Option<WorkloadType>,
    #[serde(default)]
    pub scale_to_zero_enabled: Option<bool>,
    #[serde(default)]
    pub state: Option<ServedEntityState>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub environment_vars: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServedModelOutput {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub model_version: Option<String>,
    #[serde(default)]
    pub workload_size: Option<String>,
    #[serde(default)]
    pub workload_type: Option<WorkloadType>,
    #[serde(default)]
    pub scale_to_zero_enabled: Option<bool>,
    #[serde(default)]
    pub state: Option<ServedEntityState>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
}

// ============================================================================
// Endpoint config types
// ============================================================================

/// Config input used when creating or updating an endpoint's configuration.
#[derive(Debug, Clone, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_entities: Option<Vec<ServedEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_models: Option<Vec<ServedModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_config: Option<TrafficConfig>,
}

/// Config output returned by the API for a live endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigOutput {
    #[serde(default)]
    pub config_version: Option<i64>,
    #[serde(default)]
    pub served_entities: Vec<ServedEntityOutput>,
    #[serde(default)]
    pub served_models: Vec<ServedModelOutput>,
    #[serde(default)]
    pub traffic_config: Option<TrafficConfig>,
}

/// Summary config returned in list responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSummary {
    #[serde(default)]
    pub served_entities: Vec<ServedEntitySpec>,
    #[serde(default)]
    pub served_models: Vec<ServedModelSpec>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServedEntitySpec {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub entity_name: Option<String>,
    #[serde(default)]
    pub entity_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServedModelSpec {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub model_version: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PendingConfig {
    #[serde(default)]
    pub config_version: Option<i64>,
    #[serde(default)]
    pub served_entities: Vec<ServedEntityOutput>,
    #[serde(default)]
    pub served_models: Vec<ServedModelOutput>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub traffic_config: Option<TrafficConfig>,
}

// ============================================================================
// Endpoint types (responses)
// ============================================================================

/// Full endpoint detail returned by get / create / update_config.
#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub state: Option<EndpointState>,
    #[serde(default)]
    pub config: Option<ConfigOutput>,
    #[serde(default)]
    pub pending_config: Option<PendingConfig>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub endpoint_url: Option<String>,
    #[serde(default)]
    pub task: Option<String>,
    #[serde(default)]
    pub route_optimized: Option<bool>,
}

/// Summary endpoint returned by list.
#[derive(Debug, Clone, Deserialize)]
pub struct EndpointSummary {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub creation_timestamp: Option<i64>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
    #[serde(default)]
    pub state: Option<EndpointState>,
    #[serde(default)]
    pub config: Option<ConfigSummary>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub task: Option<String>,
}

// ============================================================================
// Request / response types
// ============================================================================

/// Request body for creating a serving endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct CreateEndpoint {
    pub name: String,
    pub config: Config,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_optimized: Option<bool>,
}

/// Request body for updating an endpoint's served entities / traffic config.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_entities: Option<Vec<ServedEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served_models: Option<Vec<ServedModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_config: Option<TrafficConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListEndpointsResponse {
    #[serde(default)]
    pub endpoints: Vec<EndpointSummary>,
}

// ============================================================================
// Query (inference) types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ChatRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Request body for querying a serving endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct QueryRequest {
    /// Pandas dataframe records orientation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataframe_records: Option<Vec<serde_json::Value>>,

    /// Tensor-based input in columnar format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,

    /// Tensor-based input in row format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<serde_json::Value>>,

    /// Input for embeddings endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,

    /// Prompt for completions endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<serde_json::Value>,

    /// Messages for chat endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,

    /// Max tokens for chat/completions endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// Temperature for chat/completions endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Number of candidate responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,

    /// Stop sequences for chat/completions endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Extra parameters for external/foundation model endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<HashMap<String, String>>,
}

/// Usage information returned by external/foundation model endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub prompt_tokens: Option<i32>,
    #[serde(default)]
    pub completion_tokens: Option<i32>,
    #[serde(default)]
    pub total_tokens: Option<i32>,
}

/// A choice element in a chat/completions response.
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    #[serde(default)]
    pub index: Option<i32>,
    #[serde(default)]
    pub message: Option<ChatMessage>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default, rename = "finishReason")]
    pub finish_reason: Option<String>,
}

/// Response from querying a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct QueryResponse {
    /// Predictions from custom model endpoints.
    #[serde(default)]
    pub predictions: Vec<serde_json::Value>,

    /// Choices from chat/completions endpoints.
    #[serde(default)]
    pub choices: Vec<Choice>,

    /// ID of the response.
    #[serde(default)]
    pub id: Option<String>,

    /// Model name from the response.
    #[serde(default)]
    pub model: Option<String>,

    /// Object type (e.g. "chat.completion", "text_completion", "list").
    #[serde(default)]
    pub object: Option<String>,

    /// Timestamp when the response was created.
    #[serde(default)]
    pub created: Option<i64>,

    /// Token usage information.
    #[serde(default)]
    pub usage: Option<Usage>,
}

// ============================================================================
// Build logs / Server logs
// ============================================================================

/// Response from fetching build logs for a served model.
#[derive(Debug, Clone, Deserialize)]
pub struct BuildLogsResponse {
    #[serde(default)]
    pub logs: String,
}

/// Response from fetching server logs for a served model.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerLogsResponse {
    #[serde(default)]
    pub logs: String,
}

// ============================================================================
// Endpoint tags (for patch)
// ============================================================================

/// A tag on a serving endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointTag {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Request body for patching tags on a serving endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct PatchServingEndpointTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<EndpointTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_tags: Option<Vec<String>>,
}

/// Response from patching tags on a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct EndpointTags {
    #[serde(default)]
    pub tags: Vec<EndpointTag>,
}

// ============================================================================
// AI Gateway
// ============================================================================

/// Configuration for the AI Gateway on a serving endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGatewayConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference_table_config: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_tracking_config: Option<serde_json::Value>,
}

/// Request body for putting an AI Gateway configuration.
pub type PutAiGatewayRequest = AiGatewayConfig;

/// Response from putting an AI Gateway configuration.
pub type PutAiGatewayResponse = AiGatewayConfig;

// ============================================================================
// External function (http_request)
// ============================================================================

/// HTTP method for an external function request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// Request body for invoking an external function via HTTP.
#[derive(Debug, Clone, Serialize)]
pub struct ExternalFunctionRequest {
    pub connection_name: String,
    pub method: HttpMethod,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
}

// ============================================================================
// Notifications
// ============================================================================

/// Email notification configuration for a serving endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotifications {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_start: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<String>>,
}

/// Request body for updating notifications on a serving endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateNotificationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_notifications: Option<EmailNotifications>,
}

/// Response from updating notifications on a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNotificationsResponse {
    #[serde(default)]
    pub email_notifications: Option<EmailNotifications>,
    #[serde(default)]
    pub name: Option<String>,
}

// ============================================================================
// Permission types (service-specific, matching Go SDK)
// ============================================================================

/// Permission level for a serving endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServingEndpointPermissionLevel {
    CanManage,
    CanQuery,
    CanView,
}

/// An access control entry in a permissions request.
#[derive(Debug, Clone, Serialize)]
pub struct ServingEndpointAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: ServingEndpointPermissionLevel,
}

/// A permission entry in a permissions response.
#[derive(Debug, Clone, Deserialize)]
pub struct ServingEndpointPermission {
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(default)]
    pub permission_level: Option<ServingEndpointPermissionLevel>,
}

/// An access control entry in a permissions response.
#[derive(Debug, Clone, Deserialize)]
pub struct ServingEndpointAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<ServingEndpointPermission>>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

/// Permissions for a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ServingEndpointPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<ServingEndpointAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

/// Request body for setting or updating permissions on a serving endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ServingEndpointPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<ServingEndpointAccessControlRequest>>,
}

/// Description of a permission level for a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ServingEndpointPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<ServingEndpointPermissionLevel>,
}

/// Response from getting permission levels for a serving endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GetServingEndpointPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<ServingEndpointPermissionsDescription>,
}
