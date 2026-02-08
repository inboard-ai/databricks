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
