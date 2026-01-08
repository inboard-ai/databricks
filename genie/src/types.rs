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
    pub id: String,
    pub space_id: String,
    pub conversation_id: String,
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextAttachment {
    #[serde(default)]
    pub content: Option<String>,
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

