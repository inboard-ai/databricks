use serde::{Deserialize, Serialize};

// ============================================================================
// Workspace / Notebook types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectInfo {
    #[serde(default)]
    pub object_type: Option<ObjectType>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub object_id: Option<i64>,
    #[serde(default)]
    pub language: Option<Language>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub modified_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectType {
    Notebook,
    Directory,
    Library,
    File,
    Repo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    Scala,
    Python,
    Sql,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExportFormat {
    Source,
    Html,
    Jupyter,
    Dbc,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse {
    #[serde(default)]
    pub objects: Vec<ObjectInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportRequest {
    pub path: String,
    pub content: String, // base64-encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ExportFormat>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExportResponse {
    pub content: String, // base64-encoded
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DeleteRequest {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct MkdirsRequest {
    pub path: String,
}

// ============================================================================
// Repos types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub head_commit_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRepo {
    pub url: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRepo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListReposResponse {
    #[serde(default)]
    pub repos: Vec<Repo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Secrets types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct SecretScope {
    pub name: String,
    #[serde(default)]
    pub backend_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecretMetadata {
    pub key: String,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListScopesResponse {
    #[serde(default)]
    pub scopes: Vec<SecretScope>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSecretsResponse {
    #[serde(default)]
    pub secrets: Vec<SecretMetadata>,
}

// ============================================================================
// Git Credentials types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCredential {
    #[serde(default)]
    pub credential_id: Option<i64>,
    #[serde(default)]
    pub git_provider: Option<String>,
    #[serde(default)]
    pub git_username: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateGitCredential {
    pub git_provider: String,
    pub git_username: String,
    pub personal_access_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateGitCredential {
    pub git_provider: String,
    pub git_username: String,
    pub personal_access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListGitCredentialsResponse {
    #[serde(default)]
    pub credentials: Vec<GitCredential>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}

// ============================================================================
// Workspace Object Permission types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkspaceObjectPermissionLevel {
    CanEdit,
    CanManage,
    CanRead,
    CanRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceObjectPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<WorkspaceObjectAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceObjectAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<WorkspaceObjectPermission>>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceObjectPermission {
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(default)]
    pub permission_level: Option<WorkspaceObjectPermissionLevel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceObjectPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<WorkspaceObjectAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceObjectAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<WorkspaceObjectPermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkspaceObjectPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<WorkspaceObjectPermissionLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetWorkspaceObjectPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<WorkspaceObjectPermissionsDescription>,
}

// ============================================================================
// Repo Permission types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RepoPermissionLevel {
    CanEdit,
    CanManage,
    CanRead,
    CanRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPermissions {
    #[serde(default)]
    pub access_control_list: Option<Vec<RepoAccessControlResponse>>,
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoAccessControlResponse {
    #[serde(default)]
    pub all_permissions: Option<Vec<RepoPermission>>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub service_principal_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPermission {
    #[serde(default)]
    pub inherited: Option<bool>,
    #[serde(default)]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(default)]
    pub permission_level: Option<RepoPermissionLevel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepoPermissionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<RepoAccessControlRequest>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepoAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<RepoPermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoPermissionsDescription {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_level: Option<RepoPermissionLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoPermissionLevelsResponse {
    #[serde(default)]
    pub permission_levels: Vec<RepoPermissionsDescription>,
}

// ============================================================================
// Secret ACL types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclItem {
    pub principal: String,
    pub permission: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ListAclsResponse {
    #[serde(default)]
    pub items: Vec<AclItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSecretResponse {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}
