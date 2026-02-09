use crate::types::{
    BuildLogsResponse, CreateEndpoint, Endpoint, EndpointTags, ExternalFunctionRequest,
    GetServingEndpointPermissionLevelsResponse, ListEndpointsResponse, PatchServingEndpointTags,
    PutAiGatewayRequest, PutAiGatewayResponse, QueryRequest, QueryResponse, ServerLogsResponse,
    ServingEndpointPermissions, ServingEndpointPermissionsRequest, UpdateConfig,
    UpdateNotificationsRequest, UpdateNotificationsResponse,
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

    /// Get build logs for a served model.
    pub async fn build_logs(
        &self,
        name: &str,
        served_model_name: &str,
    ) -> Result<BuildLogsResponse, Error> {
        self.client
            .get(&format!(
                "{}/{}/served-models/{}/build-logs",
                PATH, name, served_model_name
            ))
            .await
    }

    /// Get server logs for a served model.
    pub async fn logs(
        &self,
        name: &str,
        served_model_name: &str,
    ) -> Result<ServerLogsResponse, Error> {
        self.client
            .get(&format!(
                "{}/{}/served-models/{}/logs",
                PATH, name, served_model_name
            ))
            .await
    }

    /// Export Prometheus metrics for a serving endpoint.
    ///
    /// Returns raw bytes (text/plain Prometheus metrics format).
    pub async fn export_metrics(&self, name: &str) -> Result<Vec<u8>, Error> {
        self.client
            .get_bytes(&format!("{}/{}/metrics", PATH, name))
            .await
    }

    /// Get the OpenAPI specification for a serving endpoint.
    ///
    /// Returns raw bytes (text/plain OpenAPI spec).
    pub async fn get_open_api(&self, name: &str) -> Result<Vec<u8>, Error> {
        self.client
            .get_bytes(&format!("{}/{}/openapi", PATH, name))
            .await
    }

    /// Patch tags on a serving endpoint (add and/or remove tags).
    pub async fn patch_tags(
        &self,
        name: &str,
        request: &PatchServingEndpointTags,
    ) -> Result<EndpointTags, Error> {
        self.client
            .patch(&format!("{}/{}/tags", PATH, name), request)
            .await
    }

    /// Put (create or replace) an AI Gateway configuration on a serving endpoint.
    pub async fn put_ai_gateway(
        &self,
        name: &str,
        request: &PutAiGatewayRequest,
    ) -> Result<PutAiGatewayResponse, Error> {
        self.client
            .put(&format!("{}/{}/ai-gateway", PATH, name), request)
            .await
    }

    /// Invoke an external function via HTTP.
    pub async fn http_request(
        &self,
        request: &ExternalFunctionRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .post("/api/2.0/external-function", request)
            .await
    }

    /// Update email notifications for a serving endpoint.
    pub async fn update_notifications(
        &self,
        name: &str,
        request: &UpdateNotificationsRequest,
    ) -> Result<UpdateNotificationsResponse, Error> {
        self.client
            .patch(&format!("{}/{}/notifications", PATH, name), request)
            .await
    }

    /// Get permissions for a serving endpoint.
    pub async fn get_permissions(
        &self,
        serving_endpoint_id: &str,
    ) -> Result<ServingEndpointPermissions, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/serving-endpoints/{}",
                serving_endpoint_id
            ))
            .await
    }

    /// Get permission levels for a serving endpoint.
    pub async fn get_permission_levels(
        &self,
        serving_endpoint_id: &str,
    ) -> Result<GetServingEndpointPermissionLevelsResponse, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/serving-endpoints/{}/permissionLevels",
                serving_endpoint_id
            ))
            .await
    }

    /// Set (replace) permissions for a serving endpoint.
    pub async fn set_permissions(
        &self,
        serving_endpoint_id: &str,
        request: &ServingEndpointPermissionsRequest,
    ) -> Result<ServingEndpointPermissions, Error> {
        self.client
            .put(
                &format!(
                    "/api/2.0/permissions/serving-endpoints/{}",
                    serving_endpoint_id
                ),
                request,
            )
            .await
    }

    /// Update (patch) permissions for a serving endpoint.
    pub async fn update_permissions(
        &self,
        serving_endpoint_id: &str,
        request: &ServingEndpointPermissionsRequest,
    ) -> Result<ServingEndpointPermissions, Error> {
        self.client
            .patch(
                &format!(
                    "/api/2.0/permissions/serving-endpoints/{}",
                    serving_endpoint_id
                ),
                request,
            )
            .await
    }
}
