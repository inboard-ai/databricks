use crate::types::{CreateMessageRequest, Message, StartConversationRequest, StartConversationResponse};
use databricks_core::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

pub struct Conversations<'a> {
    client: &'a Client,
    space_id: String,
}

impl<'a> Conversations<'a> {
    pub fn new(client: &'a Client, space_id: impl Into<String>) -> Self {
        Self {
            client,
            space_id: space_id.into(),
        }
    }

    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    /// Start a new conversation with an initial message
    pub async fn start(&self, content: impl Into<String>) -> Result<StartConversationResponse, Error> {
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
        self.wait_message(conversation_id, &message.id, poll_interval, timeout)
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
                return Err(Error::Other("Genie response timed out".into()));
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
}
