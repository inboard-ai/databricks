use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Internal empty response helper
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}

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

#[derive(Debug, Clone, Serialize)]
pub struct CreateTable {
    pub name: String,
    pub catalog_name: String,
    pub schema_name: String,
    pub table_type: TableType,
    pub data_source_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ColumnInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TableExistsResponse {
    #[serde(default)]
    pub table_exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSummary {
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub table_type: Option<TableType>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListTableSummariesResponse {
    #[serde(default)]
    pub tables: Vec<TableSummary>,
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

#[derive(Debug, Clone, Serialize)]
pub struct UpdateVolume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
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

#[derive(Debug, Clone, Serialize)]
pub struct CreateStorageCredential {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_service_principal: Option<AzureServicePrincipal>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateStorageCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_service_principal: Option<AzureServicePrincipal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsIamRole {
    pub role_arn: String,
    #[serde(default)]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureServicePrincipal {
    pub directory_id: String,
    pub application_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateStorageCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_credential_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_location_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_service_principal: Option<AzureServicePrincipal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidateStorageCredentialResponse {
    #[serde(default)]
    pub is_dir: Option<bool>,
    #[serde(default)]
    pub results: Option<Vec<ValidationResult>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidationResult {
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
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

#[derive(Debug, Clone, Serialize)]
pub struct UpdateExternalLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivePermissionsList {
    #[serde(default)]
    pub privilege_assignments: Vec<EffectivePrivilegeAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivePrivilegeAssignment {
    #[serde(default)]
    pub principal: Option<String>,
    #[serde(default)]
    pub privileges: Vec<EffectivePrivilege>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivePrivilege {
    #[serde(default)]
    pub privilege: Option<String>,
    #[serde(default)]
    pub inherited_from_name: Option<String>,
    #[serde(default)]
    pub inherited_from_type: Option<String>,
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

#[derive(Debug, Clone, Serialize)]
pub struct CreateMetastore {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_root: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMetastore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_root_credential_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetastoreAssignment {
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub workspace_id: Option<i64>,
    #[serde(default)]
    pub default_catalog_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetastoreSummaryResponse {
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
    pub cloud: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub global_metastore_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMetastoreAssignment {
    pub metastore_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_catalog_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateMetastoreAssignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metastore_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_catalog_name: Option<String>,
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

#[derive(Debug, Clone, Serialize)]
pub struct CreateFunction {
    pub name: String,
    pub catalog_name: String,
    pub schema_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routine_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routine_definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deterministic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_data_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null_call: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFunctionsResponse {
    #[serde(default)]
    pub functions: Vec<FunctionInfo>,
}

// ============================================================================
// Connection types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub connection_type: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, String>>,
    #[serde(default)]
    pub options: Option<HashMap<String, String>>,
    #[serde(default)]
    pub read_only: Option<bool>,
    #[serde(default)]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateConnection {
    pub name: String,
    pub connection_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListConnectionsResponse {
    #[serde(default)]
    pub connections: Vec<ConnectionInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Online Table types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineTable {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub spec: Option<OnlineTableSpec>,
    #[serde(default)]
    pub status: Option<OnlineTableStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineTableSpec {
    #[serde(default)]
    pub source_table_full_name: Option<String>,
    #[serde(default)]
    pub primary_key_columns: Option<Vec<String>>,
    #[serde(default)]
    pub timeseries_key: Option<String>,
    #[serde(default)]
    pub run_triggered: Option<serde_json::Value>,
    #[serde(default)]
    pub run_continuously: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineTableStatus {
    #[serde(default)]
    pub detailed_state: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

// ============================================================================
// Table Constraint types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConstraint {
    #[serde(default)]
    pub primary_key_constraint: Option<PrimaryKeyConstraint>,
    #[serde(default)]
    pub foreign_key_constraint: Option<ForeignKeyConstraint>,
    #[serde(default)]
    pub named_table_constraint: Option<NamedTableConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryKeyConstraint {
    pub name: String,
    pub child_columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyConstraint {
    pub name: String,
    pub child_columns: Vec<String>,
    pub parent_table: String,
    pub parent_columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTableConstraint {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTableConstraint {
    pub full_name_arg: String,
    pub constraint: TableConstraint,
}

// ============================================================================
// System Schema types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSchemaInfo {
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSystemSchemasResponse {
    #[serde(default)]
    pub schemas: Vec<SystemSchemaInfo>,
}

// ============================================================================
// Workspace Bindings types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceBinding {
    #[serde(default)]
    pub workspace_id: Option<i64>,
    #[serde(default)]
    pub binding_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCatalogWorkspaceBindingsResponse {
    #[serde(default)]
    pub workspace_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkspaceBindings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_workspaces: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unassign_workspaces: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCatalogWorkspaceBindingsResponse {
    #[serde(default)]
    pub workspace_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetWorkspaceBindingsResponse {
    #[serde(default)]
    pub bindings: Vec<WorkspaceBinding>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkspaceBindingsParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<WorkspaceBinding>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<WorkspaceBinding>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateWorkspaceBindingsResponse {
    #[serde(default)]
    pub bindings: Vec<WorkspaceBinding>,
}

// ============================================================================
// Artifact Allowlist types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactAllowlistInfo {
    #[serde(default)]
    pub artifact_matchers: Vec<ArtifactMatcher>,
    #[serde(default)]
    pub metastore_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMatcher {
    pub artifact: String,
    pub match_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetArtifactAllowlist {
    pub artifact_matchers: Vec<ArtifactMatcher>,
}

// ============================================================================
// UC Registered Model types (catalog-level)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModelInfo {
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
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub storage_location: Option<String>,
    #[serde(default)]
    pub aliases: Option<Vec<RegisteredModelAlias>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredModelAlias {
    #[serde(default)]
    pub alias_name: Option<String>,
    #[serde(default)]
    pub version_num: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRegisteredModel {
    pub name: String,
    pub catalog_name: String,
    pub schema_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateRegisteredModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetRegisteredModelAlias {
    pub version_num: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRegisteredModelsResponse {
    #[serde(default)]
    pub registered_models: Vec<RegisteredModelInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// UC Model Version types (catalog-level)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersionInfo {
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub catalog_name: Option<String>,
    #[serde(default)]
    pub schema_name: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateModelVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListModelVersionsResponse {
    #[serde(default)]
    pub model_versions: Vec<ModelVersionInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
