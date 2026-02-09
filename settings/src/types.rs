use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// IP Access List types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListFilter {
    Allow,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAccessList {
    #[serde(default)]
    pub list_id: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub list_type: Option<ListFilter>,
    #[serde(default)]
    pub ip_addresses: Vec<String>,
    #[serde(default)]
    pub address_count: Option<i32>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateIpAccessListRequest {
    pub label: String,
    pub list_type: ListFilter,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateIpAccessListResponse {
    #[serde(default)]
    pub ip_access_list: Option<IpAccessList>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateIpAccessListRequest {
    #[serde(skip)]
    pub ip_access_list_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_type: Option<ListFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReplaceIpAccessListRequest {
    #[serde(skip)]
    pub ip_access_list_id: String,
    pub label: String,
    pub list_type: ListFilter,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIpAccessListResponse {
    #[serde(default)]
    pub ip_access_list: Option<IpAccessList>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListIpAccessListsResponse {
    #[serde(default)]
    pub ip_access_lists: Vec<IpAccessList>,
}

// ============================================================================
// Token types (personal tokens)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(default)]
    pub token_id: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub expiry_time: Option<i64>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub created_by_id: Option<i64>,
    #[serde(default)]
    pub created_by_username: Option<String>,
    #[serde(default)]
    pub last_used_day: Option<i64>,
    #[serde(default)]
    pub owner_id: Option<i64>,
    #[serde(default)]
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_seconds: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTokenResponse {
    #[serde(default)]
    pub token_info: Option<TokenInfo>,
    #[serde(default)]
    pub token_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTokensResponse {
    #[serde(default)]
    pub token_infos: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTokenResponse {
    #[serde(default)]
    pub token_info: Option<TokenInfo>,
}

// ============================================================================
// Token Management types (admin)
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateOboTokenRequest {
    pub application_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_seconds: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOboTokenResponse {
    #[serde(default)]
    pub token_info: Option<TokenInfo>,
    #[serde(default)]
    pub token_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPermissions {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub access_control_list: Option<Vec<TokenAccessControlResponse>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenAccessControlResponse {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<TokenPermission>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPermission {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTokenPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<TokenPermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenPermissionsRequest {
    pub access_control_list: Vec<TokenAccessControlRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
}

// ============================================================================
// Notification Destinations types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DestinationType {
    Email,
    Slack,
    MicrosoftTeams,
    Pagerduty,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDestinationConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microsoft_teams: Option<MicrosoftTeamsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagerduty: Option<PagerdutyConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_webhook: Option<GenericWebhookConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    #[serde(default)]
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token_set: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftTeamsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_secret_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_url_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id_set: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagerdutyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_key_set: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericWebhookConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_set: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_set: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDestination {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub destination_type: Option<DestinationType>,
    #[serde(default)]
    pub config: Option<NotificationDestinationConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateNotificationDestinationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<NotificationDestinationConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateNotificationDestinationRequest {
    #[serde(skip)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<NotificationDestinationConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListNotificationDestinationsResponse {
    #[serde(default)]
    pub results: Vec<NotificationDestination>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Network Connectivity types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateNetworkConnectivityConfigRequest {
    pub name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnectivityConfiguration {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub network_connectivity_config_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub updated_time: Option<i64>,
    #[serde(default)]
    pub egress_config: Option<NccEgressConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccEgressConfig {
    #[serde(default)]
    pub default_rules: Option<NccEgressDefaultRules>,
    #[serde(default)]
    pub target_rules: Option<NccEgressTargetRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccEgressDefaultRules {
    #[serde(default)]
    pub aws_stable_ip_rule: Option<NccAwsStableIpRule>,
    #[serde(default)]
    pub azure_service_endpoint_rule: Option<NccAzureServiceEndpointRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccAwsStableIpRule {
    #[serde(default)]
    pub cidr_blocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccAzureServiceEndpointRule {
    #[serde(default)]
    pub subnets: Vec<String>,
    #[serde(default)]
    pub target_region: Option<String>,
    #[serde(default)]
    pub target_services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccEgressTargetRules {
    #[serde(default)]
    pub azure_private_endpoint_rules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NccPrivateEndpointRule {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub network_connectivity_config_id: Option<String>,
    #[serde(default)]
    pub rule_id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub endpoint_name: Option<String>,
    #[serde(default)]
    pub connection_state: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub updated_time: Option<i64>,
    #[serde(default)]
    pub deactivated: Option<bool>,
    #[serde(default)]
    pub deactivated_at: Option<i64>,
    #[serde(default)]
    pub domain_names: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_service: Option<String>,
    #[serde(default)]
    pub resource_names: Option<Vec<String>>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePrivateEndpointRuleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListNetworkConnectivityConfigurationsResponse {
    #[serde(default)]
    pub items: Vec<NetworkConnectivityConfiguration>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPrivateEndpointRulesResponse {
    #[serde(default)]
    pub items: Vec<NccPrivateEndpointRule>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Settings API types (etag-based settings)
// ============================================================================

/// Generic setting response/request wrapper for etag-based settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringMessage {
    #[serde(default)]
    pub value: Option<String>,
}

// -- DefaultNamespace --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultNamespaceSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub namespace: StringMessage,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDefaultNamespaceSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: DefaultNamespaceSetting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDefaultNamespaceSettingResponse {
    #[serde(default)]
    pub etag: Option<String>,
}

// -- RestrictWorkspaceAdmins --

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RestrictWorkspaceAdminsStatus {
    AllowAll,
    RestrictTokensAndJobRunAs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictWorkspaceAdminsMessage {
    pub status: RestrictWorkspaceAdminsStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictWorkspaceAdminsSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub restrict_workspace_admins: RestrictWorkspaceAdminsMessage,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRestrictWorkspaceAdminsSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: RestrictWorkspaceAdminsSetting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRestrictWorkspaceAdminsSettingResponse {
    #[serde(default)]
    pub etag: Option<String>,
}

// -- AutomaticClusterUpdate --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAutoRestartMessage {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub can_toggle: Option<bool>,
    #[serde(default)]
    pub restart_even_if_no_updates_available: Option<bool>,
    #[serde(default)]
    pub enablement_details: Option<serde_json::Value>,
    #[serde(default)]
    pub maintenance_window: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticClusterUpdateSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub automatic_cluster_update_workspace: ClusterAutoRestartMessage,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAutomaticClusterUpdateSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: AutomaticClusterUpdateSetting,
}

// -- ComplianceSecurityProfile --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSecurityProfile {
    #[serde(default)]
    pub is_enabled: Option<bool>,
    #[serde(default)]
    pub compliance_standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSecurityProfileSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub compliance_security_profile_workspace: ComplianceSecurityProfile,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateComplianceSecurityProfileSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: ComplianceSecurityProfileSetting,
}

// -- EnhancedSecurityMonitoring --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSecurityMonitoring {
    #[serde(default)]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSecurityMonitoringSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub enhanced_security_monitoring_workspace: EnhancedSecurityMonitoring,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateEnhancedSecurityMonitoringSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: EnhancedSecurityMonitoringSetting,
}

// -- PersonalCompute --

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonalComputeMessageEnum {
    On,
    Delegate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalComputeMessage {
    pub value: PersonalComputeMessageEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalComputeSetting {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub setting_name: Option<String>,
    pub personal_compute: PersonalComputeMessage,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePersonalComputeSettingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_missing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    pub setting: PersonalComputeSetting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletePersonalComputeSettingResponse {
    #[serde(default)]
    pub etag: Option<String>,
}

// ============================================================================
// Workspace Conf types
// ============================================================================

/// Workspace configuration is a map of string keys to string values.
pub type WorkspaceConfMap = HashMap<String, String>;

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RevokeTokenId {
    pub token_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
