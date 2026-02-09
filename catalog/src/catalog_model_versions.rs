use crate::types::{ListModelVersionsResponse, ModelVersionInfo, UpdateModelVersion};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/models";

pub struct CatalogModelVersions {
    client: Client,
}

impl CatalogModelVersions {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, full_name: &str, version: i64) -> Result<ModelVersionInfo, Error> {
        let path = format!("{}/{}/versions/{}", PATH, full_name, version);
        self.client.get(&path).await
    }

    /// Get a model version by alias.
    pub async fn get_by_alias(
        &self,
        full_name: &str,
        alias: &str,
    ) -> Result<ModelVersionInfo, Error> {
        let path = format!("{}/{}/aliases/{}", PATH, full_name, alias);
        self.client.get(&path).await
    }

    pub async fn list(&self, full_name: &str) -> Result<Vec<ModelVersionInfo>, Error> {
        let path = format!("{}/{}/versions", PATH, full_name);
        let response: ListModelVersionsResponse = self.client.get(&path).await?;
        Ok(response.model_versions)
    }

    pub async fn update(
        &self,
        full_name: &str,
        version: i64,
        request: &UpdateModelVersion,
    ) -> Result<ModelVersionInfo, Error> {
        let path = format!("{}/{}/versions/{}", PATH, full_name, version);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, full_name: &str, version: i64) -> Result<(), Error> {
        let path = format!("{}/{}/versions/{}", PATH, full_name, version);
        self.client.delete_empty(&path).await
    }
}
