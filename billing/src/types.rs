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
