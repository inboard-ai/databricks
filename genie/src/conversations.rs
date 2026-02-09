use crate::types::{
    CreateMessageRequest, ExecuteQueryResponse, ListConversationsResponse, ListMessagesResponse,
    Message, SendMessageFeedbackRequest, StartConversationRequest, StartConversationResponse,
};
use databricks_core::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

pub struct Conversations {
    client: Client,
    space_id: String,
}

impl Conversations {
    pub fn new(client: Client, space_id: impl Into<String>) -> Self {
        Self {
            client,
            space_id: space_id.into(),
        }
    }

    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    /// Start a new conversation with an initial message
    pub async fn start(
        &self,
        content: impl Into<String>,
    ) -> Result<StartConversationResponse, Error> {
        let path = format!("/api/2.0/genie/spaces/{}/start-conversation", self.space_id);
        let request = StartConversationRequest {
            content: content.into(),
        };
        self.client.post(&path, &request).await
    }

    /// Start a conversation and wait for the response to complete
    pub async fn start_wait(
        &self,
        content: impl Into<String>,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Message, Error> {
        let response = self.start(content).await?;
        self.wait_message(
            &response.conversation.id,
            &response.message_id,
            poll_interval,
            timeout,
        )
        .await
    }

    /// Send a message to an existing conversation
    pub async fn send(
        &self,
        conversation_id: &str,
        content: impl Into<String>,
    ) -> Result<Message, Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages",
            self.space_id, conversation_id
        );
        let request = CreateMessageRequest {
            content: content.into(),
        };
        self.client.post(&path, &request).await
    }

    /// Send a message and wait for the response to complete
    pub async fn send_wait(
        &self,
        conversation_id: &str,
        content: impl Into<String>,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Message, Error> {
        let message = self.send(conversation_id, content).await?;
        self.wait_message(conversation_id, &message.message_id, poll_interval, timeout)
            .await
    }

    /// Get message status
    pub async fn get_message(
        &self,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<Message, Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages/{}",
            self.space_id, conversation_id, message_id
        );
        self.client.get(&path).await
    }

    /// Poll until message reaches a terminal state
    pub async fn wait_message(
        &self,
        conversation_id: &str,
        message_id: &str,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Message, Error> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(Error::Timeout("Genie response timed out".into()));
            }

            sleep(poll_interval).await;

            let message = self.get_message(conversation_id, message_id).await?;

            if let Some(status) = &message.status {
                if status.is_terminal() {
                    return if status.is_success() {
                        Ok(message)
                    } else {
                        let error_msg = message
                            .error
                            .and_then(|e| e.message)
                            .unwrap_or_else(|| format!("Genie failed with status: {:?}", status));
                        Err(Error::Other(error_msg))
                    };
                }
            }
        }
    }

    /// Execute a query attachment and get results
    pub async fn execute_attachment_query(
        &self,
        conversation_id: &str,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<ExecuteQueryResponse, Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages/{}/attachments/{}/execute-query",
            self.space_id, conversation_id, message_id, attachment_id
        );
        self.client.post_empty(&path).await
    }

    /// List all messages in a conversation
    pub async fn list_messages(&self, conversation_id: &str) -> Result<Vec<Message>, Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages",
            self.space_id, conversation_id
        );
        let response: ListMessagesResponse = self.client.get(&path).await?;
        Ok(response.messages)
    }

    /// Delete a conversation.
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<(), Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}",
            self.space_id, conversation_id
        );
        self.client.delete_empty(&path).await
    }

    /// Delete a message.
    pub async fn delete_message(
        &self,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<(), Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages/{}",
            self.space_id, conversation_id, message_id
        );
        self.client.delete_empty(&path).await
    }

    /// List conversations in the space.
    pub async fn list_conversations(&self) -> Result<ListConversationsResponse, Error> {
        let path = format!("/api/2.0/genie/spaces/{}/conversations", self.space_id);
        self.client.get(&path).await
    }

    /// Send feedback for a message.
    pub async fn send_message_feedback(
        &self,
        conversation_id: &str,
        message_id: &str,
        request: &SendMessageFeedbackRequest,
    ) -> Result<(), Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages/{}/feedback",
            self.space_id, conversation_id, message_id
        );
        self.client.post(&path, request).await
    }

    /// Get query results for a message attachment.
    pub async fn get_message_attachment_query_result(
        &self,
        conversation_id: &str,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<ExecuteQueryResponse, Error> {
        let path = format!(
            "/api/2.0/genie/spaces/{}/conversations/{}/messages/{}/attachments/{}/query-result",
            self.space_id, conversation_id, message_id, attachment_id
        );
        self.client.get(&path).await
    }
}
