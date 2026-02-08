use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Catalog types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, String>>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub catalog_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCatalog {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCatalog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListCatalogsResponse {
    #[serde(default)]
    pub catalogs: Vec<CatalogInfo>,
}

// ============================================================================
// Schema types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catalog_name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, String>>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSchema {
    pub name: String,
    pub catalog_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSchemasResponse {
    #[serde(default)]
    pub schemas: Vec<SchemaInfo>,
}

// ============================================================================
// Table types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catalog_name: Option<String>,
    #[serde(default)]
    pub schema_name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub table_type: Option<TableType>,
    #[serde(default)]
    pub data_source_format: Option<String>,
    #[serde(default)]
    pub columns: Option<Vec<ColumnInfo>>,
    #[serde(default)]
    pub storage_location: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TableType {
    Managed,
    External,
    View,
    MaterializedView,
    StreamingTable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub type_name: Option<String>,
    #[serde(default)]
    pub type_text: Option<String>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub nullable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTablesResponse {
    #[serde(default)]
    pub tables: Vec<TableInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Volume types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catalog_name: Option<String>,
    #[serde(default)]
    pub schema_name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub volume_type: Option<VolumeType>,
    #[serde(default)]
    pub storage_location: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VolumeType {
    Managed,
    External,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateVolume {
    pub name: String,
    pub catalog_name: String,
    pub schema_name: String,
    pub volume_type: VolumeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListVolumesResponse {
    #[serde(default)]
    pub volumes: Vec<VolumeInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Storage Credential types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCredentialInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListStorageCredentialsResponse {
    #[serde(default)]
    pub storage_credentials: Vec<StorageCredentialInfo>,
}

// ============================================================================
// External Location types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLocationInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub credential_name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExternalLocation {
    pub name: String,
    pub url: String,
    pub credential_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListExternalLocationsResponse {
    #[serde(default)]
    pub external_locations: Vec<ExternalLocationInfo>,
}

// ============================================================================
// Grants types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsList {
    #[serde(default)]
    pub privilege_assignments: Vec<PrivilegeAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeAssignment {
    pub principal: String,
    #[serde(default)]
    pub privileges: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatePermissions {
    pub changes: Vec<PermissionsChange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionsChange {
    pub principal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

// ============================================================================
// Metastore types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetastoreInfo {
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub default_data_access_config_id: Option<String>,
    #[serde(default)]
    pub storage_root: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListMetastoresResponse {
    #[serde(default)]
    pub metastores: Vec<MetastoreInfo>,
}

// ============================================================================
// Function types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub catalog_name: Option<String>,
    #[serde(default)]
    pub schema_name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFunctionsResponse {
    #[serde(default)]
    pub functions: Vec<FunctionInfo>,
}
