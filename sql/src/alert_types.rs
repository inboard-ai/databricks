use serde::{Deserialize, Serialize};

// ============================================================================
// Alert types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<AlertCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<AlertLifecycleState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_on_ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds_to_retrigger: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<AlertState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertLifecycleState {
    Active,
    Trashed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertState {
    Ok,
    Triggered,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertOperator {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    IsNull,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_result_state: Option<AlertState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<AlertOperator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operand: Option<AlertConditionOperand>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<AlertConditionThreshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConditionOperand {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<AlertOperandColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertOperandColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConditionThreshold {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<AlertOperandValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertOperandValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

// ============================================================================
// Request / Response types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateAlertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<CreateAlertRequestAlert>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateAlertRequestAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<AlertCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_retrigger: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAlertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<UpdateAlertRequestAlert>,
    pub update_mask: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAlertRequestAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<AlertCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_retrigger: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAlertsResponse {
    #[serde(default)]
    pub results: Vec<Alert>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
