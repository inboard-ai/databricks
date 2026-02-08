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
// Token types
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
