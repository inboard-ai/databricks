use serde::{Deserialize, Serialize};

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppState {
    Crashed,
    Deploying,
    Running,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComputeState {
    Active,
    Deleting,
    Error,
    Starting,
    Stopped,
    Stopping,
    Updating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeploymentState {
    Cancelled,
    Failed,
    InProgress,
    Succeeded,
}

// ============================================================================
// App types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub updater: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub app_status: Option<ApplicationStatus>,
    #[serde(default)]
    pub compute_status: Option<ComputeStatus>,
    #[serde(default)]
    pub active_deployment: Option<Deployment>,
    #[serde(default)]
    pub pending_deployment: Option<Deployment>,
    #[serde(default)]
    pub resources: Option<Vec<AppResource>>,
    #[serde(default)]
    pub default_source_code_path: Option<String>,
    #[serde(default)]
    pub service_principal_id: Option<i64>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub budget_policy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationStatus {
    #[serde(default)]
    pub state: Option<AppState>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeStatus {
    #[serde(default)]
    pub state: Option<ComputeState>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResource {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub job: Option<serde_json::Value>,
    #[serde(default)]
    pub secret: Option<serde_json::Value>,
    #[serde(default)]
    pub serving_endpoint: Option<serde_json::Value>,
    #[serde(default)]
    pub sql_warehouse: Option<serde_json::Value>,
}

// ============================================================================
// Deployment types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(default)]
    pub deployment_id: Option<String>,
    #[serde(default)]
    pub source_code_path: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub status: Option<DeploymentStatus>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub deployment_artifacts: Option<DeploymentArtifacts>,
    #[serde(default)]
    pub env_vars: Option<Vec<EnvVar>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStatus {
    #[serde(default)]
    pub state: Option<DeploymentState>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentArtifacts {
    #[serde(default)]
    pub source_code_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_from: Option<String>,
}

// ============================================================================
// Request / Response types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateApp {
    pub app: App,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_compute: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateApp {
    pub app: App,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAppsResponse {
    #[serde(default)]
    pub apps: Vec<App>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// App update types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
    pub update_mask: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUpdate {
    #[serde(default)]
    pub budget_policy_id: Option<String>,
    #[serde(default)]
    pub compute_size: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub resources: Option<Vec<AppResource>>,
    #[serde(default)]
    pub status: Option<serde_json::Value>,
    #[serde(default)]
    pub user_api_scopes: Option<Vec<String>>,
}

// ============================================================================
// Deployment request/response types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateDeploymentRequest {
    pub app_deployment: Deployment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDeploymentsResponse {
    #[serde(default)]
    pub app_deployments: Vec<Deployment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct AppPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<AppAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<AppPermission>>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<AppAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<AppPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

// ============================================================================
// Custom template types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTemplate {
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub git_provider: String,
    pub git_repo: String,
    #[serde(default)]
    pub manifest: Option<serde_json::Value>,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCustomTemplateRequest {
    pub template: CustomTemplate,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCustomTemplateRequest {
    pub template: CustomTemplate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListCustomTemplatesResponse {
    #[serde(default)]
    pub templates: Vec<CustomTemplate>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
