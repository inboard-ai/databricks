use crate::types::{
    DefaultNamespaceSetting, DeleteDefaultNamespaceSettingResponse,
    UpdateDefaultNamespaceSettingRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/default_namespace_ws/names/default";

pub struct DefaultNamespace {
    client: Client,
}

impl DefaultNamespace {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the default namespace setting.
    pub async fn get(&self) -> Result<DefaultNamespaceSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the default namespace setting.
    pub async fn update(
        &self,
        request: &UpdateDefaultNamespaceSettingRequest,
    ) -> Result<DefaultNamespaceSetting, Error> {
        self.client.patch(PATH, request).await
    }

    /// Delete the default namespace setting, reverting to the default.
    pub async fn delete(&self) -> Result<DeleteDefaultNamespaceSettingResponse, Error> {
        self.client.delete(PATH).await
    }
}
