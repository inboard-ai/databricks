use serde::{Deserialize, Serialize};

// ============================================================================
// Query types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_auto_limit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modifier_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<QueryLifecycleState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_as_mode: Option<RunAsMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryLifecycleState {
    Active,
    Trashed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunAsMode {
    Owner,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<TextValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<NumericValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<EnumValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

// ============================================================================
// Request / Response types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct CreateQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<CreateQueryRequestQuery>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateQueryRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_auto_limit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_mode: Option<RunAsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<UpdateQueryRequestQuery>,
    pub update_mask: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateQueryRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_auto_limit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<QueryParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_mode: Option<RunAsMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListQueryObjectsResponse {
    #[serde(default)]
    pub results: Vec<Query>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}
