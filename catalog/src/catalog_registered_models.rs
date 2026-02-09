use crate::types::{
    CreateRegisteredModel, ListRegisteredModelsResponse, RegisteredModelAlias, RegisteredModelInfo,
    SetRegisteredModelAlias, UpdateRegisteredModel,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/models";

pub struct CatalogRegisteredModels {
    client: Client,
}

impl CatalogRegisteredModels {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateRegisteredModel,
    ) -> Result<RegisteredModelInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, full_name: &str) -> Result<RegisteredModelInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<RegisteredModelInfo>, Error> {
        let response: ListRegisteredModelsResponse = self.client.get(PATH).await?;
        Ok(response.registered_models)
    }

    pub async fn update(
        &self,
        full_name: &str,
        request: &UpdateRegisteredModel,
    ) -> Result<RegisteredModelInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, full_name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.delete_empty(&path).await
    }

    /// Set an alias on a registered model.
    pub async fn set_alias(
        &self,
        full_name: &str,
        alias: &str,
        request: &SetRegisteredModelAlias,
    ) -> Result<RegisteredModelAlias, Error> {
        let path = format!("{}/{}/aliases/{}", PATH, full_name, alias);
        self.client.put(&path, request).await
    }

    /// Delete an alias from a registered model.
    pub async fn delete_alias(&self, full_name: &str, alias: &str) -> Result<(), Error> {
        let path = format!("{}/{}/aliases/{}", PATH, full_name, alias);
        self.client.delete_empty(&path).await
    }
}
