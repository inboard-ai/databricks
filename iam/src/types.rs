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
