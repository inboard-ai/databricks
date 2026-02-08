use crate::types::{
    CreateEndpoint, Endpoint, ListEndpointsResponse, QueryRequest, QueryResponse, UpdateConfig,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/serving-endpoints";

pub struct ServingEndpoints {
    client: Client,
}

impl ServingEndpoints {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new serving endpoint.
    pub async fn create(&self, request: &CreateEndpoint) -> Result<Endpoint, Error> {
        self.client.post(PATH, request).await
    }

    /// Get details for a single serving endpoint by name.
    pub async fn get(&self, name: &str) -> Result<Endpoint, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    /// List all serving endpoints.
    pub async fn list(&self) -> Result<Vec<crate::types::EndpointSummary>, Error> {
        let response: ListEndpointsResponse = self.client.get(PATH).await?;
        Ok(response.endpoints)
    }

    /// Update the config (served entities, traffic config) of a serving endpoint.
    pub async fn update_config(
        &self,
        name: &str,
        request: &UpdateConfig,
    ) -> Result<Endpoint, Error> {
        self.client
            .put(&format!("{}/{}/config", PATH, name), request)
            .await
    }

    /// Delete a serving endpoint by name.
    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, name))
            .await
    }

    /// Query a serving endpoint (inference).
    pub async fn query(&self, name: &str, request: &QueryRequest) -> Result<QueryResponse, Error> {
        self.client
            .post(&format!("{}/{}/invocations", PATH, name), request)
            .await
    }
}
