use crate::types::WorkspaceConfMap;
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/workspace-conf";

pub struct WorkspaceConf {
    client: Client,
}

impl WorkspaceConf {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get workspace configuration for the specified keys.
    pub async fn get(&self, keys: &str) -> Result<WorkspaceConfMap, Error> {
        self.client.get_with_query(PATH, &[("keys", keys)]).await
    }

    /// Set workspace configuration values.
    pub async fn set(&self, config: &WorkspaceConfMap) -> Result<(), Error> {
        let _: WorkspaceConfMap = self.client.patch(PATH, config).await?;
        Ok(())
    }
}
