use serde::{Deserialize, Serialize};

// ============================================================================
// Enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthenticationType {
    Databricks,
    OauthClientCredentials,
    OidcFederation,
    Token,
}

// ============================================================================
// Share types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
    #[serde(default)]
    pub storage_location: Option<String>,
    #[serde(default)]
    pub storage_root: Option<String>,
    #[serde(default)]
    pub objects: Option<Vec<SharedDataObject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedDataObject {
    pub name: String,
    #[serde(default)]
    pub data_object_type: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub shared_as: Option<String>,
    #[serde(default)]
    pub added_at: Option<i64>,
    #[serde(default)]
    pub added_by: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub cdf_enabled: Option<bool>,
    #[serde(default)]
    pub start_version: Option<i64>,
    #[serde(default)]
    pub history_data_sharing_status: Option<String>,
    #[serde(default)]
    pub partitions: Option<Vec<Partition>>,
    #[serde(default)]
    pub string_shared_as: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    #[serde(default)]
    pub values: Option<Vec<PartitionValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionValue {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub op: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub recipient_property_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedDataObjectUpdate {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub data_object: Option<SharedDataObject>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateShare {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_root: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateShare {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<SharedDataObjectUpdate>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSharesResponse {
    #[serde(default)]
    pub shares: Vec<ShareInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Provider types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub authentication_type: Option<AuthenticationType>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub data_provider_global_metastore_id: Option<String>,
    #[serde(default)]
    pub recipient_profile_str: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProvider {
    pub name: String,
    pub authentication_type: AuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_profile_str: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_profile_str: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListProvidersResponse {
    #[serde(default)]
    pub providers: Vec<ProviderInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Recipient types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub authentication_type: Option<AuthenticationType>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub data_recipient_global_metastore_id: Option<String>,
    #[serde(default)]
    pub sharing_code: Option<String>,
    #[serde(default)]
    pub activated: Option<bool>,
    #[serde(default)]
    pub activation_url: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<i64>,
    #[serde(default)]
    pub tokens: Option<Vec<RecipientTokenInfo>>,
    #[serde(default)]
    pub ip_access_list: Option<IpAccessList>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientTokenInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub activation_url: Option<String>,
    #[serde(default)]
    pub expiration_time: Option<i64>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAccessList {
    #[serde(default)]
    pub allowed_ip_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRecipient {
    pub name: String,
    pub authentication_type: AuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_recipient_global_metastore_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_list: Option<IpAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_list: Option<IpAccessList>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRecipientsResponse {
    #[serde(default)]
    pub recipients: Vec<RecipientInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Provider share types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ListProviderSharesResponse {
    #[serde(default)]
    pub shares: Vec<ProviderShare>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderShare {
    #[serde(default)]
    pub name: Option<String>,
}

// ============================================================================
// Recipient share permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecipientSharePermissionsResponse {
    #[serde(default)]
    pub permissions_out: Vec<ShareToPrivilegeAssignment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShareToPrivilegeAssignment {
    #[serde(default)]
    pub share_name: Option<String>,
    #[serde(default)]
    pub privilege_assignments: Option<Vec<PrivilegeAssignment>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrivilegeAssignment {
    #[serde(default)]
    pub principal: Option<String>,
    #[serde(default)]
    pub privileges: Option<Vec<String>>,
}

// ============================================================================
// Share permission types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct GetSharePermissionsResponse {
    #[serde(default)]
    pub privilege_assignments: Vec<PrivilegeAssignment>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSharePermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<PermissionsChange>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionsChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

// ============================================================================
// Recipient activation types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct RetrieveTokenResponse {
    #[serde(default, rename = "bearerToken")]
    pub bearer_token: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default, rename = "expirationTime")]
    pub expiration_time: Option<String>,
    #[serde(default, rename = "shareCredentialsVersion")]
    pub share_credentials_version: Option<i32>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RotateRecipientTokenRequest {
    pub existing_token_expire_in_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
