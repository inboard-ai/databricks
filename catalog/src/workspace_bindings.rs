use crate::types::{
    GetCatalogWorkspaceBindingsResponse, GetWorkspaceBindingsResponse,
    UpdateCatalogWorkspaceBindingsResponse, UpdateWorkspaceBindings,
    UpdateWorkspaceBindingsParameters, UpdateWorkspaceBindingsResponse, WorkspaceBinding,
};
use databricks_core::{Client, Error};

pub struct WorkspaceBindings {
    client: Client,
}

impl WorkspaceBindings {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get workspace bindings of a catalog (legacy endpoint).
    pub async fn get(&self, name: &str) -> Result<GetCatalogWorkspaceBindingsResponse, Error> {
        let path = format!(
            "/api/2.1/unity-catalog/workspace-bindings/catalogs/{}",
            name
        );
        self.client.get(&path).await
    }

    /// Get workspace bindings of a securable (new endpoint).
    pub async fn get_bindings(
        &self,
        securable_type: &str,
        securable_name: &str,
    ) -> Result<Vec<WorkspaceBinding>, Error> {
        let path = format!(
            "/api/2.1/unity-catalog/bindings/{}/{}",
            securable_type, securable_name
        );
        let response: GetWorkspaceBindingsResponse = self.client.get(&path).await?;
        Ok(response.bindings)
    }

    /// Update workspace bindings of a catalog (legacy endpoint).
    pub async fn update(
        &self,
        name: &str,
        request: &UpdateWorkspaceBindings,
    ) -> Result<UpdateCatalogWorkspaceBindingsResponse, Error> {
        let path = format!(
            "/api/2.1/unity-catalog/workspace-bindings/catalogs/{}",
            name
        );
        self.client.patch(&path, request).await
    }

    /// Update workspace bindings of a securable (new endpoint).
    pub async fn update_bindings(
        &self,
        securable_type: &str,
        securable_name: &str,
        request: &UpdateWorkspaceBindingsParameters,
    ) -> Result<UpdateWorkspaceBindingsResponse, Error> {
        let path = format!(
            "/api/2.1/unity-catalog/bindings/{}/{}",
            securable_type, securable_name
        );
        self.client.patch(&path, request).await
    }
}
