use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Workspace types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkspaceState {
    NotProvisioned,
    Provisioning,
    Running,
    Failed,
    Cancelling,
    Banned,
}

impl WorkspaceState {
    pub fn is_running(&self) -> bool {
        matches!(self, WorkspaceState::Running)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            WorkspaceState::Running | WorkspaceState::Failed | WorkspaceState::Banned
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudResourceContainer {
    #[serde(default)]
    pub gcp: Option<GcpCloudResourceContainer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpCloudResourceContainer {
    #[serde(default)]
    pub project_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub workspace_id: Option<i64>,
    #[serde(default)]
    pub workspace_name: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub deployment_name: Option<String>,
    #[serde(default)]
    pub workspace_status: Option<WorkspaceState>,
    #[serde(default)]
    pub workspace_status_message: Option<String>,
    #[serde(default)]
    pub aws_region: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub cloud_resource_container: Option<CloudResourceContainer>,
    #[serde(default)]
    pub credentials_id: Option<String>,
    #[serde(default)]
    pub storage_configuration_id: Option<String>,
    #[serde(default)]
    pub network_id: Option<String>,
    #[serde(default)]
    pub managed_services_customer_managed_key_id: Option<String>,
    #[serde(default)]
    pub storage_customer_managed_key_id: Option<String>,
    #[serde(default)]
    pub private_access_settings_id: Option<String>,
    #[serde(default)]
    pub pricing_tier: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub network_connectivity_config_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateWorkspaceRequest {
    pub workspace_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_container: Option<CloudResourceContainer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_services_customer_managed_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_customer_managed_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_access_settings_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connectivity_config_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkspaceRequest {
    #[serde(skip)]
    pub workspace_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_services_customer_managed_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_customer_managed_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_access_settings_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connectivity_config_id: Option<String>,
}
