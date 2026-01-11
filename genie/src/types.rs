use serde::{Deserialize, Serialize};

// ============================================================================
// Space types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Space {
    pub space_id: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub warehouse_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSpacesResponse {
    #[serde(default)]
    pub spaces: Vec<Space>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Conversation types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub space_id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub created_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartConversationResponse {
    pub conversation: Conversation,
    pub message: Message,
    pub message_id: String,
}

// ============================================================================
// Message types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    /// Primary message identifier
    pub message_id: String,
    /// Deprecated legacy identifier - use message_id instead
    #[serde(default, rename = "id")]
    _id: Option<String>,
    pub space_id: String,
    pub conversation_id: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub status: Option<Status>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub error: Option<MessageError>,
    #[serde(default)]
    pub created_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Submitted,
    FetchingMetadata,
    FilteringContext,
    AskingAi,
    PendingWarehouse,
    ExecutingQuery,
    Completed,
    Failed,
    QueryResultExpired,
    Cancelled,
}

impl Status {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Status::Completed | Status::Failed | Status::Cancelled | Status::QueryResultExpired
        )
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Status::Completed)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageError {
    #[serde(default)]
    pub message: Option<String>,
}

// ============================================================================
// Attachment types
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    #[serde(default)]
    pub attachment_id: Option<String>,
    #[serde(default)]
    pub query: Option<QueryAttachment>,
    #[serde(default)]
    pub text: Option<TextAttachment>,
    #[serde(default)]
    pub suggested_questions: Option<SuggestedQuestions>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryAttachment {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub statement_id: Option<String>,
    #[serde(default)]
    pub query_result_metadata: Option<QueryResultMetadata>,
    #[serde(default)]
    pub parameters: Vec<QueryAttachmentParameter>,
    #[serde(default)]
    pub last_updated_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResultMetadata {
    #[serde(default)]
    pub is_truncated: Option<bool>,
    #[serde(default)]
    pub row_count: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryAttachmentParameter {
    #[serde(default)]
    pub keyword: Option<String>,
    #[serde(default)]
    pub sql_type: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextAttachment {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub purpose: Option<TextAttachmentPurpose>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAttachmentPurpose {
    FollowUpQuestion,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestedQuestions {
    #[serde(default)]
    pub questions: Vec<String>,
}

// ============================================================================
// Request types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct StartConversationRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateMessageRequest {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListMessagesResponse {
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

// ============================================================================
// Query execution types
// ============================================================================

/// Response from executing a query attachment
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteQueryResponse {
    /// The SQL statement response (same as sql::Response)
    #[serde(default)]
    pub statement_response: Option<StatementResponse>,
}

/// Simplified statement response for Genie query results
#[derive(Debug, Clone, Deserialize)]
pub struct StatementResponse {
    #[serde(default)]
    pub status: Option<StatementStatus>,
    #[serde(default)]
    pub manifest: Option<ResultManifest>,
    #[serde(default)]
    pub result: Option<ResultData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatementStatus {
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultManifest {
    #[serde(default)]
    pub schema: Option<ResultSchema>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultSchema {
    #[serde(default)]
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultData {
    #[serde(default)]
    pub data_array: Vec<ResultRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultRow {
    #[serde(default)]
    pub values: Vec<ResultValue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResultValue {
    #[serde(default)]
    pub string_value: Option<String>,
}

