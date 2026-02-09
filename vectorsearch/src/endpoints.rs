use crate::types::{
    CreateEndpoint, Endpoint, ListEndpointsResponse, RetrieveMetricsRequest,
    RetrieveMetricsResponse, UpdateBudgetPolicyRequest, UpdateBudgetPolicyResponse,
    UpdateCustomTagsRequest, UpdateCustomTagsResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/vector-search/endpoints";

pub struct Endpoints {
    client: Client,
}

impl Endpoints {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateEndpoint) -> Result<Endpoint, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, endpoint_name: &str) -> Result<Endpoint, Error> {
        self.client
            .get(&format!("{}/{}", PATH, endpoint_name))
            .await
    }

    pub async fn list(&self) -> Result<Vec<Endpoint>, Error> {
        let response: ListEndpointsResponse = self.client.get(PATH).await?;
        Ok(response.endpoints)
    }

    pub async fn delete(&self, endpoint_name: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, endpoint_name))
            .await
    }

    pub async fn retrieve_metrics(
        &self,
        endpoint_name: &str,
        request: &RetrieveMetricsRequest,
    ) -> Result<RetrieveMetricsResponse, Error> {
        self.client
            .post(&format!("{}/{}/metrics", PATH, endpoint_name), request)
            .await
    }

    pub async fn update_budget_policy(
        &self,
        endpoint_name: &str,
        request: &UpdateBudgetPolicyRequest,
    ) -> Result<UpdateBudgetPolicyResponse, Error> {
        self.client
            .put(
                &format!("{}/{}/budget-policy", PATH, endpoint_name),
                request,
            )
            .await
    }

    pub async fn update_custom_tags(
        &self,
        endpoint_name: &str,
        request: &UpdateCustomTagsRequest,
    ) -> Result<UpdateCustomTagsResponse, Error> {
        self.client
            .put(&format!("{}/{}/tags", PATH, endpoint_name), request)
            .await
    }
}
