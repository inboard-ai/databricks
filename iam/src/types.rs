use serde::{Deserialize, Serialize};

// ============================================================================
// SCIM User types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "userName")]
    pub user_name: Option<String>,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub emails: Option<Vec<Email>>,
    #[serde(default)]
    pub groups: Option<Vec<GroupRef>>,
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub entitlements: Option<Vec<Entitlement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub value: String,
    #[serde(default, rename = "type")]
    pub email_type: Option<String>,
    #[serde(default)]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRef {
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUsersResponse {
    #[serde(default, rename = "Resources")]
    pub resources: Vec<User>,
    #[serde(default, rename = "totalResults")]
    pub total_results: Option<i64>,
    #[serde(default, rename = "startIndex")]
    pub start_index: Option<i64>,
    #[serde(default, rename = "itemsPerPage")]
    pub items_per_page: Option<i64>,
}

// ============================================================================
// SCIM Group types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub members: Option<Vec<Member>>,
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub entitlements: Option<Vec<Entitlement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub value: String,
    #[serde(default)]
    pub display: Option<String>,
    #[serde(default, rename = "type")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListGroupsResponse {
    #[serde(default, rename = "Resources")]
    pub resources: Vec<Group>,
    #[serde(default, rename = "totalResults")]
    pub total_results: Option<i64>,
}

// ============================================================================
// Service Principal types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePrincipal {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, rename = "applicationId")]
    pub application_id: Option<String>,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub entitlements: Option<Vec<Entitlement>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListServicePrincipalsResponse {
    #[serde(default, rename = "Resources")]
    pub resources: Vec<ServicePrincipal>,
    #[serde(default, rename = "totalResults")]
    pub total_results: Option<i64>,
}

// ============================================================================
// Permissions types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPermissions {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
    #[serde(default)]
    pub access_control_list: Option<Vec<AccessControlEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlEntry {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub all_permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub permission_level: String,
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetPermissions {
    pub access_control_list: Vec<AccessControlRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePermissions {
    pub access_control_list: Vec<AccessControlRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionLevels {
    #[serde(default)]
    pub permission_levels: Vec<PermissionsDescription>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionsDescription {
    #[serde(default)]
    pub permission_level: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

// ============================================================================
// Current User types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentUser {
    #[serde(default, rename = "userName")]
    pub user_name: Option<String>,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

// ============================================================================
// SCIM Patch operations
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct PatchRequest {
    #[serde(rename = "Operations")]
    pub operations: Vec<PatchOperation>,
    #[serde(default)]
    pub schemas: Vec<String>,
}

impl PatchRequest {
    pub fn new(operations: Vec<PatchOperation>) -> Self {
        Self {
            operations,
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchOperation {
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}

// ============================================================================
// Password Permission types (service-specific, matching Go SDK)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PasswordPermissionLevel {
    CanUse,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasswordAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    pub permission_level: PasswordPermissionLevel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordPermission {
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(default)]
    pub permission_level: Option<PasswordPermissionLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<PasswordPermission>>,
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
pub struct PasswordPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<PasswordAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasswordPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<PasswordAccessControlRequest>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<PasswordPermissionLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPasswordPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<PasswordPermissionsDescription>,
}

// ============================================================================
// Permission Migration types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct MigratePermissionsRequest {
    pub from_workspace_group_name: String,
    pub to_account_group_name: String,
    pub workspace_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MigratePermissionsResponse {
    #[serde(default)]
    pub permissions_migrated: Option<i32>,
}

// ============================================================================
// Workspace Assignment types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkspacePermission {
    Admin,
    User,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionOutput {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<WorkspacePermission>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkspacePermissions {
    #[serde(default)]
    pub permissions: Vec<PermissionOutput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrincipalOutput {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub principal_id: Option<i64>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionAssignment {
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub permissions: Option<Vec<WorkspacePermission>>,
    #[serde(default)]
    pub principal: Option<PrincipalOutput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionAssignments {
    #[serde(default)]
    pub permission_assignments: Vec<PermissionAssignment>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkspaceAssignments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<WorkspacePermission>>,
}

// ============================================================================
// Account Access Control types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRule {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleSetResponse {
    pub etag: String,
    pub name: String,
    #[serde(default)]
    pub grant_rules: Vec<GrantRule>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuleSetUpdateRequest {
    pub etag: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_rules: Option<Vec<GrantRule>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRuleSetRequest {
    pub name: String,
    pub rule_set: RuleSetUpdateRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignableRole {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAssignableRolesForResourceResponse {
    #[serde(default)]
    pub roles: Vec<AssignableRole>,
}
