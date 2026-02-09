use crate::types::{AutomaticClusterUpdateSetting, UpdateAutomaticClusterUpdateSettingRequest};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/automatic_cluster_update/names/default";

pub struct AutomaticClusterUpdate {
    client: Client,
}

impl AutomaticClusterUpdate {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the automatic cluster update setting.
    pub async fn get(&self) -> Result<AutomaticClusterUpdateSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the automatic cluster update setting.
    pub async fn update(
        &self,
        request: &UpdateAutomaticClusterUpdateSettingRequest,
    ) -> Result<AutomaticClusterUpdateSetting, Error> {
        self.client.patch(PATH, request).await
    }
}
