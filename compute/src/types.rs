use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Cluster types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDetails {
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub cluster_name: Option<String>,
    #[serde(default)]
    pub spark_version: Option<String>,
    #[serde(default)]
    pub node_type_id: Option<String>,
    #[serde(default)]
    pub driver_node_type_id: Option<String>,
    #[serde(default)]
    pub num_workers: Option<i32>,
    #[serde(default)]
    pub autoscale: Option<AutoScale>,
    #[serde(default)]
    pub state: Option<ClusterState>,
    #[serde(default)]
    pub state_message: Option<String>,
    #[serde(default)]
    pub autotermination_minutes: Option<i32>,
    #[serde(default)]
    pub creator_user_name: Option<String>,
    #[serde(default)]
    pub instance_pool_id: Option<String>,
    #[serde(default)]
    pub policy_id: Option<String>,
    #[serde(default)]
    pub spark_conf: Option<HashMap<String, String>>,
    #[serde(default)]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(default)]
    pub cluster_source: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub terminated_time: Option<i64>,
    #[serde(default)]
    pub termination_reason: Option<TerminationReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClusterState {
    Pending,
    Running,
    Restarting,
    Resizing,
    Terminating,
    Terminated,
    Error,
    Unknown,
}

impl ClusterState {
    pub fn is_running(&self) -> bool {
        matches!(self, ClusterState::Running)
    }

    pub fn is_terminated(&self) -> bool {
        matches!(self, ClusterState::Terminated)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ClusterState::Terminated | ClusterState::Error | ClusterState::Unknown
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScale {
    pub min_workers: i32,
    pub max_workers: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationReason {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCluster {
    pub cluster_name: String,
    pub spark_version: String,
    pub node_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_node_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<AutoScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autotermination_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateClusterResponse {
    pub cluster_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditCluster {
    pub cluster_id: String,
    pub cluster_name: String,
    pub spark_version: String,
    pub node_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_node_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<AutoScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autotermination_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListClustersResponse {
    #[serde(default)]
    pub clusters: Vec<ClusterDetails>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangeClusterOwner {
    pub cluster_id: String,
    pub owner_username: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetEvents {
    pub cluster_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetEventsResponse {
    #[serde(default)]
    pub events: Vec<ClusterEvent>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub next_page: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterEvent {
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default, rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResizeCluster {
    pub cluster_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<AutoScale>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCluster {
    pub cluster_id: String,
    pub update_mask: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<UpdateClusterResource>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateClusterResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_node_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<AutoScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autotermination_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_conf: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListNodeTypesResponse {
    #[serde(default)]
    pub node_types: Vec<NodeType>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeType {
    #[serde(default)]
    pub node_type_id: Option<String>,
    #[serde(default)]
    pub memory_mb: Option<i64>,
    #[serde(default)]
    pub num_cores: Option<f64>,
    #[serde(default)]
    pub num_gpus: Option<i32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub instance_type_id: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub is_deprecated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAvailableZonesResponse {
    #[serde(default)]
    pub default_zone: Option<String>,
    #[serde(default)]
    pub zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSparkVersionsResponse {
    #[serde(default)]
    pub versions: Vec<SparkVersion>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SparkVersion {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

// ============================================================================
// Instance Pool types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePool {
    #[serde(default)]
    pub instance_pool_id: Option<String>,
    #[serde(default)]
    pub instance_pool_name: Option<String>,
    #[serde(default)]
    pub node_type_id: Option<String>,
    #[serde(default)]
    pub min_idle_instances: Option<i32>,
    #[serde(default)]
    pub max_capacity: Option<i32>,
    #[serde(default)]
    pub idle_instance_autotermination_minutes: Option<i32>,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateInstancePool {
    pub instance_pool_name: String,
    pub node_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_instances: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_instance_autotermination_minutes: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstancePoolResponse {
    pub instance_pool_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditInstancePool {
    pub instance_pool_id: String,
    pub instance_pool_name: String,
    pub node_type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_instances: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListInstancePoolsResponse {
    #[serde(default)]
    pub instance_pools: Vec<InstancePool>,
}

// ============================================================================
// Cluster Policy types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    #[serde(default)]
    pub policy_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub definition: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub creator_user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePolicy {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePolicyResponse {
    pub policy_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditPolicy {
    pub policy_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPoliciesResponse {
    #[serde(default)]
    pub policies: Vec<Policy>,
}

// ============================================================================
// Library types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pypi: Option<PythonPyPiLibrary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maven: Option<MavenLibrary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonPyPiLibrary {
    pub package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenLibrary {
    pub coordinates: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallLibraries {
    pub cluster_id: String,
    pub libraries: Vec<Library>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UninstallLibraries {
    pub cluster_id: String,
    pub libraries: Vec<Library>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LibraryFullStatus {
    #[serde(default)]
    pub library: Option<Library>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub messages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterLibraryStatuses {
    #[serde(default)]
    pub cluster_id: Option<String>,
    #[serde(default)]
    pub library_statuses: Vec<LibraryFullStatus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllClusterLibraryStatuses {
    #[serde(default)]
    pub statuses: Vec<ClusterLibraryStatuses>,
}

// ============================================================================
// Command Execution types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CancelCommand {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "commandId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateContext {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DestroyContext {
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    #[serde(rename = "contextId")]
    pub context_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteCommand {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Created {
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandStatusResponse {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub results: Option<Results>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Results {
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[serde(default, rename = "resultType")]
    pub result_type: Option<String>,
    #[serde(default)]
    pub cause: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContextStatusResponse {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

// ============================================================================
// Global Init Scripts types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct GlobalInitScriptCreateRequest {
    pub name: String,
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalInitScriptCreateResponse {
    #[serde(default)]
    pub script_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GlobalInitScriptUpdateRequest {
    pub name: String,
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalInitScriptDetails {
    #[serde(default)]
    pub script_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalInitScriptDetailsWithContent {
    #[serde(default)]
    pub script_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListGlobalInitScriptsResponse {
    #[serde(default)]
    pub scripts: Vec<GlobalInitScriptDetails>,
}

// ============================================================================
// Instance Profiles types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceProfileInfo {
    pub instance_profile_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_meta_instance_profile: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddInstanceProfile {
    pub instance_profile_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_meta_instance_profile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_validation: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveInstanceProfile {
    pub instance_profile_arn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceProfilesResponse {
    #[serde(default)]
    pub instance_profiles: Vec<InstanceProfileInfo>,
}

// ============================================================================
// Policy Families types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyFamily {
    #[serde(default)]
    pub policy_family_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPolicyFamiliesResponse {
    #[serde(default)]
    pub policy_families: Vec<PolicyFamily>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Cluster permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPermissions {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub access_control_list: Option<Vec<ClusterAccessControlResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<ClusterPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<ClusterPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClusterPermissionsRequest {
    pub access_control_list: Vec<ClusterAccessControlRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClusterAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
}

// ============================================================================
// Cluster Policy permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPolicyPermissions {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub access_control_list: Option<Vec<ClusterPolicyAccessControlResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPolicyAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<ClusterPolicyPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPolicyPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterPolicyPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<ClusterPolicyPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClusterPolicyPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClusterPolicyPermissionsRequest {
    pub access_control_list: Vec<ClusterPolicyAccessControlRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClusterPolicyAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
}

// ============================================================================
// Instance Pool permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct InstancePoolPermissions {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub access_control_list: Option<Vec<InstancePoolAccessControlResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstancePoolAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<InstancePoolPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstancePoolPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstancePoolPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<InstancePoolPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstancePoolPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstancePoolPermissionsRequest {
    pub access_control_list: Vec<InstancePoolAccessControlRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstancePoolAccessControlRequest {
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

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ClusterId {
    pub cluster_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct PoolId {
    pub instance_pool_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct PolicyId {
    pub policy_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
