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
