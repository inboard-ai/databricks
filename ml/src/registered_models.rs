use crate::types::{
    CreateRegisteredModelRequest, CreateRegisteredModelResponse, EmptyResponse,
    GetRegisteredModelResponse, ListRegisteredModelsResponse, SearchRegisteredModelsResponse,
    UpdateRegisteredModelRequest, UpdateRegisteredModelResponse,
};
use databricks_core::{Client, Error};

const REGISTRY_PATH: &str = "/api/2.0/mlflow/registered-models";

pub struct RegisteredModels {
    client: Client,
}

impl RegisteredModels {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new registered model.
    pub async fn create(
        &self,
        request: &CreateRegisteredModelRequest,
    ) -> Result<CreateRegisteredModelResponse, Error> {
        self.client
            .post(&format!("{}/create", REGISTRY_PATH), request)
            .await
    }

    /// Get details for a registered model by name.
    pub async fn get(&self, name: &str) -> Result<GetRegisteredModelResponse, Error> {
        self.client
            .get_with_query(&format!("{}/get", REGISTRY_PATH), &[("name", name)])
            .await
    }

    /// List all registered models.
    pub async fn list(&self) -> Result<ListRegisteredModelsResponse, Error> {
        self.client.get(&format!("{}/list", REGISTRY_PATH)).await
    }

    /// Update a registered model (e.g. description).
    pub async fn update(
        &self,
        request: &UpdateRegisteredModelRequest,
    ) -> Result<UpdateRegisteredModelResponse, Error> {
        self.client
            .patch(&format!("{}/update", REGISTRY_PATH), request)
            .await
    }

    /// Delete a registered model by name.
    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!("{}/delete?name={}", REGISTRY_PATH, name))
            .await?;
        Ok(())
    }

    /// Search for registered models matching a filter.
    pub async fn search(
        &self,
        filter: Option<&str>,
        max_results: Option<i64>,
        page_token: Option<&str>,
    ) -> Result<SearchRegisteredModelsResponse, Error> {
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
}
