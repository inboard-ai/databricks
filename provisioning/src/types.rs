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

// ============================================================================
// Credential types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StsRole {
    #[serde(default)]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsCredentials {
    #[serde(default)]
    pub sts_role: Option<StsRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub credentials_id: Option<String>,
    #[serde(default)]
    pub credentials_name: Option<String>,
    #[serde(default)]
    pub aws_credentials: Option<AwsCredentials>,
    #[serde(default)]
    pub creation_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCredentialStsRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCredentialAwsCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_role: Option<CreateCredentialStsRole>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCredentialRequest {
    pub credentials_name: String,
    pub aws_credentials: CreateCredentialAwsCredentials,
}

// ============================================================================
// Encryption key / CustomerManagedKey types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKeyInfo {
    #[serde(default)]
    pub key_alias: Option<String>,
    #[serde(default)]
    pub key_arn: Option<String>,
    #[serde(default)]
    pub key_region: Option<String>,
    #[serde(default)]
    pub reuse_key_for_cluster_volumes: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyAccessConfiguration {
    #[serde(default)]
    pub credential_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureKeyInfo {
    #[serde(default)]
    pub disk_encryption_set_id: Option<String>,
    #[serde(default)]
    pub key_access_configuration: Option<KeyAccessConfiguration>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub key_vault_uri: Option<String>,
    #[serde(default)]
    pub tenant_id: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpServiceAccount {
    #[serde(default)]
    pub service_account_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpKeyInfo {
    #[serde(default)]
    pub gcp_service_account: Option<GcpServiceAccount>,
    #[serde(default)]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerManagedKey {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub customer_managed_key_id: Option<String>,
    #[serde(default)]
    pub aws_key_info: Option<AwsKeyInfo>,
    #[serde(default)]
    pub azure_key_info: Option<AzureKeyInfo>,
    #[serde(default)]
    pub gcp_key_info: Option<GcpKeyInfo>,
    #[serde(default)]
    pub use_cases: Option<Vec<String>>,
    #[serde(default)]
    pub creation_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateAwsKeyInfo {
    pub key_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_key_for_cluster_volumes: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateGcpKeyInfo {
    pub kms_key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_service_account: Option<GcpServiceAccount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCustomerManagedKeyRequest {
    pub use_cases: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_key_info: Option<CreateAwsKeyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_key_info: Option<CreateGcpKeyInfo>,
}

// ============================================================================
// Network types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkVpcEndpoints {
    #[serde(default)]
    pub dataplane_relay: Option<Vec<String>>,
    #[serde(default)]
    pub rest_api: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpNetworkInfo {
    #[serde(default)]
    pub network_project_id: Option<String>,
    #[serde(default)]
    pub pod_ip_range_name: Option<String>,
    #[serde(default)]
    pub service_ip_range_name: Option<String>,
    #[serde(default)]
    pub subnet_id: Option<String>,
    #[serde(default)]
    pub subnet_region: Option<String>,
    #[serde(default)]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHealth {
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkWarning {
    #[serde(default)]
    pub warning_message: Option<String>,
    #[serde(default)]
    pub warning_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub network_id: Option<String>,
    #[serde(default)]
    pub network_name: Option<String>,
    #[serde(default)]
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub vpc_endpoints: Option<NetworkVpcEndpoints>,
    #[serde(default)]
    pub gcp_network_info: Option<GcpNetworkInfo>,
    #[serde(default)]
    pub vpc_status: Option<String>,
    #[serde(default)]
    pub error_messages: Option<Vec<NetworkHealth>>,
    #[serde(default)]
    pub warning_messages: Option<Vec<NetworkWarning>>,
    #[serde(default)]
    pub workspace_id: Option<i64>,
    #[serde(default)]
    pub creation_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNetworkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<NetworkVpcEndpoints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_network_info: Option<GcpNetworkInfo>,
}

// ============================================================================
// PrivateAccessSettings types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateAccessSettings {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub private_access_settings_id: Option<String>,
    #[serde(default)]
    pub private_access_settings_name: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub private_access_level: Option<String>,
    #[serde(default)]
    pub public_access_enabled: Option<bool>,
    #[serde(default)]
    pub allowed_vpc_endpoint_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePrivateAccessSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_access_settings_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_vpc_endpoint_ids: Option<Vec<String>>,
}

// ============================================================================
// StorageConfiguration types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootBucketInfo {
    #[serde(default)]
    pub bucket_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfiguration {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub storage_configuration_id: Option<String>,
    #[serde(default)]
    pub storage_configuration_name: Option<String>,
    #[serde(default)]
    pub root_bucket_info: Option<RootBucketInfo>,
    #[serde(default)]
    pub role_arn: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateStorageConfigurationRequest {
    pub storage_configuration_name: String,
    pub root_bucket_info: RootBucketInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

// ============================================================================
// VpcEndpoint types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpVpcEndpointInfo {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub psc_endpoint_name: Option<String>,
    #[serde(default)]
    pub endpoint_region: Option<String>,
    #[serde(default)]
    pub psc_connection_id: Option<String>,
    #[serde(default)]
    pub service_attachment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpoint {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub vpc_endpoint_id: Option<String>,
    #[serde(default)]
    pub vpc_endpoint_name: Option<String>,
    #[serde(default)]
    pub aws_vpc_endpoint_id: Option<String>,
    #[serde(default)]
    pub aws_account_id: Option<String>,
    #[serde(default)]
    pub aws_endpoint_service_id: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub use_case: Option<String>,
    #[serde(default)]
    pub gcp_vpc_endpoint_info: Option<GcpVpcEndpointInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateVpcEndpointRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_vpc_endpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcp_vpc_endpoint_info: Option<GcpVpcEndpointInfo>,
}
