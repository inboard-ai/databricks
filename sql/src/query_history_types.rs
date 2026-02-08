use serde::{Deserialize, Serialize};

// ============================================================================
// Query History types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_as_user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_as_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_produced: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_start_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_end_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_end_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spark_ui_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<QueryStatementType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<QueryMetrics>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryStatus {
    Queued,
    Running,
    Canceled,
    Failed,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryStatementType {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Drop,
    Alter,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compilation_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_produced_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_read_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_from_cache: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_fetch_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_sent_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spill_to_disk_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_total_time_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_start_time_range: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<QueryStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warehouse_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_ms: Option<i64>,
}

// ============================================================================
// Response types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ListQueriesResponse {
    #[serde(default)]
    pub res: Vec<QueryInfo>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub has_next_page: Option<bool>,
}
