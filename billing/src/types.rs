use serde::{Deserialize, Serialize};

// ============================================================================
// Budget types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfiguration {
    #[serde(default)]
    pub budget_configuration_id: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub alert_configurations: Vec<AlertConfiguration>,
    #[serde(default)]
    pub filter: Option<BudgetConfigurationFilter>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfiguration {
    #[serde(default)]
    pub alert_configuration_id: Option<String>,
    #[serde(default)]
    pub quantity_threshold: Option<String>,
    #[serde(default)]
    pub quantity_type: Option<String>,
    #[serde(default)]
    pub time_period: Option<String>,
    #[serde(default)]
    pub trigger_type: Option<String>,
    #[serde(default)]
    pub action_configurations: Vec<ActionConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfiguration {
    #[serde(default)]
    pub action_configuration_id: Option<String>,
    #[serde(default)]
    pub action_type: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfigurationFilter {
    #[serde(default)]
    pub tags: Vec<BudgetConfigurationFilterTagClause>,
    #[serde(default)]
    pub workspace_id: Option<BudgetConfigurationFilterWorkspaceIdClause>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfigurationFilterTagClause {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<BudgetConfigurationFilterClause>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfigurationFilterClause {
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfigurationFilterWorkspaceIdClause {
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub values: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetConfigurationRequest {
    pub budget: CreateBudgetConfigurationBudget,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetConfigurationBudget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_configurations: Option<Vec<CreateBudgetAlertConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<BudgetConfigurationFilter>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetAlertConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configurations: Option<Vec<CreateBudgetActionConfiguration>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetActionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBudgetConfigurationResponse {
    #[serde(default)]
    pub budget: Option<BudgetConfiguration>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBudgetConfigurationRequest {
    #[serde(skip)]
    pub budget_id: String,
    pub budget: UpdateBudgetConfigurationBudget,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBudgetConfigurationBudget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_configuration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_configurations: Option<Vec<AlertConfiguration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<BudgetConfigurationFilter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBudgetConfigurationResponse {
    #[serde(default)]
    pub budget: Option<BudgetConfiguration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBudgetConfigurationResponse {
    #[serde(default)]
    pub budget: Option<BudgetConfiguration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListBudgetConfigurationsResponse {
    #[serde(default)]
    pub budgets: Vec<BudgetConfiguration>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Usage types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    #[serde(default)]
    pub workspace_id: Option<i64>,
    #[serde(default)]
    pub sku_name: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub usage_start_time: Option<String>,
    #[serde(default)]
    pub usage_end_time: Option<String>,
    #[serde(default)]
    pub usage_date: Option<String>,
    #[serde(default)]
    pub usage_unit: Option<String>,
    #[serde(default)]
    pub usage_quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadUsageRequest {
    pub start_month: String,
    pub end_month: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_data: Option<bool>,
}

// ============================================================================
// Budget Policy types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetPolicy {
    #[serde(default)]
    pub binding_workspace_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub custom_tags: Option<Vec<BudgetPolicyTag>>,
    #[serde(default)]
    pub policy_id: Option<String>,
    #[serde(default)]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetPolicyTag {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetPolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<BudgetPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBudgetPolicyRequest {
    pub policy: BudgetPolicy,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListBudgetPoliciesResponse {
    #[serde(default)]
    pub policies: Vec<BudgetPolicy>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub previous_page_token: Option<String>,
}

// ============================================================================
// Log Delivery types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogType {
    AuditLogs,
    BillableUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OutputFormat {
    Csv,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogDeliveryConfigStatus {
    Disabled,
    Enabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryStatus {
    Created,
    Succeeded,
    UserFailure,
    SystemFailure,
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogDeliveryConfiguration {
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub config_id: Option<String>,
    #[serde(default)]
    pub config_name: Option<String>,
    #[serde(default)]
    pub creation_time: Option<i64>,
    #[serde(default)]
    pub credentials_id: Option<String>,
    #[serde(default)]
    pub delivery_path_prefix: Option<String>,
    #[serde(default)]
    pub delivery_start_time: Option<String>,
    #[serde(default)]
    pub log_delivery_status: Option<LogDeliveryStatus>,
    #[serde(default)]
    pub log_type: Option<LogType>,
    #[serde(default)]
    pub output_format: Option<OutputFormat>,
    #[serde(default)]
    pub status: Option<LogDeliveryConfigStatus>,
    #[serde(default)]
    pub storage_configuration_id: Option<String>,
    #[serde(default)]
    pub update_time: Option<i64>,
    #[serde(default)]
    pub workspace_ids_filter: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogDeliveryStatus {
    #[serde(default)]
    pub last_attempt_time: Option<String>,
    #[serde(default)]
    pub last_successful_attempt_time: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub status: Option<DeliveryStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateLogDeliveryConfigurationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
    pub credentials_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_path_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_start_time: Option<String>,
    pub log_type: LogType,
    pub output_format: OutputFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LogDeliveryConfigStatus>,
    pub storage_configuration_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_ids_filter: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WrappedCreateLogDeliveryConfiguration {
    pub log_delivery_configuration: CreateLogDeliveryConfigurationParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WrappedLogDeliveryConfiguration {
    #[serde(default)]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WrappedLogDeliveryConfigurations {
    #[serde(default)]
    pub log_delivery_configurations: Vec<LogDeliveryConfiguration>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GetLogDeliveryConfigurationResponse {
    #[serde(default)]
    pub log_delivery_configuration: Option<LogDeliveryConfiguration>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct PatchLogDeliveryStatusRequest {
    pub status: LogDeliveryConfigStatus,
}

// ============================================================================
// Usage Dashboard types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsageDashboardType {
    UsageDashboardTypeGlobal,
    UsageDashboardTypeWorkspace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UsageDashboardMajorVersion {
    UsageDashboardMajorVersion1,
    UsageDashboardMajorVersion2,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateUsageDashboardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_type: Option<UsageDashboardType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<UsageDashboardMajorVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUsageDashboardResponse {
    #[serde(default)]
    pub dashboard_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUsageDashboardResponse {
    #[serde(default)]
    pub dashboard_id: Option<String>,
    #[serde(default)]
    pub dashboard_url: Option<String>,
}
