use crate::types::{
    CancelCommand, CommandStatusResponse, ContextStatusResponse, CreateContext, Created,
    DestroyContext, EmptyResponse, ExecuteCommand,
};
use databricks_core::{Client, Error};

pub struct CommandExecution {
    client: Client,
}

impl CommandExecution {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn cancel(&self, request: &CancelCommand) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/1.2/commands/cancel", request)
            .await?;
        Ok(())
    }

    pub async fn command_status(
        &self,
        cluster_id: &str,
        context_id: &str,
        command_id: &str,
    ) -> Result<CommandStatusResponse, Error> {
        self.client
            .get_with_query(
                "/api/1.2/commands/status",
                &[
                    ("clusterId", cluster_id),
                    ("contextId", context_id),
                    ("commandId", command_id),
                ],
            )
            .await
    }

    pub async fn context_status(
        &self,
        cluster_id: &str,
        context_id: &str,
    ) -> Result<ContextStatusResponse, Error> {
        self.client
            .get_with_query(
                "/api/1.2/contexts/status",
                &[("clusterId", cluster_id), ("contextId", context_id)],
            )
            .await
    }

    pub async fn create(&self, request: &CreateContext) -> Result<Created, Error> {
        self.client.post("/api/1.2/contexts/create", request).await
    }

    pub async fn destroy(&self, request: &DestroyContext) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/1.2/contexts/destroy", request)
            .await?;
        Ok(())
    }

    pub async fn execute(&self, request: &ExecuteCommand) -> Result<Created, Error> {
        self.client.post("/api/1.2/commands/execute", request).await
    }
}
