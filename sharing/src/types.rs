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
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
