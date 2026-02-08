use crate::types::{
    CreateModelVersionRequest, CreateModelVersionResponse, EmptyResponse, GetModelVersionResponse,
    SearchModelVersionsResponse, UpdateModelVersionRequest, UpdateModelVersionResponse,
};
use databricks_core::{Client, Error};

const REGISTRY_PATH: &str = "/api/2.0/mlflow/model-versions";

pub struct ModelVersions {
    client: Client,
}

impl ModelVersions {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new model version.
    pub async fn create(
        &self,
        request: &CreateModelVersionRequest,
    ) -> Result<CreateModelVersionResponse, Error> {
        self.client
            .post(&format!("{}/create", REGISTRY_PATH), request)
            .await
    }

    /// Get a specific model version.
    pub async fn get(&self, name: &str, version: &str) -> Result<GetModelVersionResponse, Error> {
        self.client
            .get_with_query(
                &format!("{}/get", REGISTRY_PATH),
                &[("name", name), ("version", version)],
            )
            .await
    }

    /// Delete a specific model version.
    pub async fn delete(&self, name: &str, version: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "{}/delete?name={}&version={}",
                REGISTRY_PATH, name, version
            ))
            .await?;
        Ok(())
    }

    /// Search for model versions matching a filter.
    pub async fn search(
        &self,
        filter: Option<&str>,
        max_results: Option<i64>,
        page_token: Option<&str>,
    ) -> Result<SearchModelVersionsResponse, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(f) = filter {
            params.push(("filter", f.to_string()));
        }
        if let Some(m) = max_results {
            params.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("{}/search", REGISTRY_PATH), &query)
            .await
    }

    /// Update a model version (e.g. description).
    pub async fn update(
        &self,
        request: &UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResponse, Error> {
        self.client
            .patch(&format!("{}/update", REGISTRY_PATH), request)
            .await
    }
}
