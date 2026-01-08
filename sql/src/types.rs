use serde::{Deserialize, Serialize};

// ============================================================================
// Warehouse types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Warehouse {
    pub id: String,
    pub name: String,
    pub state: State,
    #[serde(default)]
    pub cluster_size: Option<String>,
    #[serde(default)]
    pub auto_stop_mins: Option<i32>,
    #[serde(default)]
    pub num_clusters: Option<i32>,
    #[serde(default)]
    pub creator_name: Option<String>,
    #[serde(default)]
    pub enable_serverless_compute: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleting,
    Deleted,
}

impl State {
    pub fn is_running(&self) -> bool {
        matches!(self, State::Running)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, State::Stopped)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListWarehousesResponse {
    #[serde(default)]
    pub warehouses: Vec<Warehouse>,
}

// ============================================================================
// Statement execution types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub statement: String,
    pub warehouse_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_wait_timeout: Option<OnWaitTimeout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<Disposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
}

impl Request {
    pub fn new(statement: impl Into<String>, warehouse_id: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
            warehouse_id: warehouse_id.into(),
            catalog: None,
            schema: None,
            wait_timeout: None,
            on_wait_timeout: None,
            row_limit: None,
            disposition: None,
            format: None,
        }
    }

    pub fn catalog(mut self, catalog: impl Into<String>) -> Self {
        self.catalog = Some(catalog.into());
        self
    }

    pub fn schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = Some(schema.into());
        self
    }

    pub fn wait_timeout(mut self, timeout: impl Into<String>) -> Self {
        self.wait_timeout = Some(timeout.into());
        self
    }

    pub fn row_limit(mut self, limit: i64) -> Self {
        self.row_limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OnWaitTimeout {
    Continue,
    Cancel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Disposition {
    Inline,
    ExternalLinks,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Format {
    JsonArray,
    ArrowStream,
    Csv,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub statement_id: String,
    pub status: Status,
    #[serde(default)]
    pub manifest: Option<Manifest>,
    #[serde(default)]
    pub result: Option<ResultData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub state: StatementState,
    #[serde(default)]
    pub error: Option<ServiceError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatementState {
    Pending,
    Running,
    Succeeded,
    Failed,
    Canceled,
    Closed,
}

impl StatementState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            StatementState::Succeeded
                | StatementState::Failed
                | StatementState::Canceled
                | StatementState::Closed
        )
    }

    pub fn is_success(&self) -> bool {
        matches!(self, StatementState::Succeeded)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceError {
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub schema: Option<Schema>,
    #[serde(default)]
    pub total_row_count: Option<i64>,
    #[serde(default)]
    pub total_chunk_count: Option<i32>,
    #[serde(default)]
    pub truncated: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    #[serde(default)]
    pub column_count: Option<i32>,
    #[serde(default)]
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(default)]
    pub type_name: Option<String>,
    #[serde(default)]
    pub type_text: Option<String>,
    #[serde(default)]
    pub position: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultData {
    #[serde(default)]
    pub data_array: Vec<Vec<Option<String>>>,
    #[serde(default)]
    pub row_count: Option<i64>,
    #[serde(default)]
    pub row_offset: Option<i64>,
    #[serde(default)]
    pub chunk_index: Option<i32>,
    #[serde(default)]
    pub next_chunk_index: Option<i32>,
    #[serde(default)]
    pub next_chunk_internal_link: Option<String>,
}

// ============================================================================
// Internal helpers
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Empty {}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EmptyResponse {}
