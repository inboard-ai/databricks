use crate::types::{
    CreateMetastore, CreateMetastoreAssignment, EmptyResponse, GetMetastoreSummaryResponse,
    ListMetastoresResponse, MetastoreAssignment, MetastoreInfo, UpdateMetastore,
    UpdateMetastoreAssignment,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/metastores";

pub struct Metastores {
    client: Client,
}

impl Metastores {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Assign a metastore to a workspace.
    pub async fn assign(
        &self,
        workspace_id: i64,
        request: &CreateMetastoreAssignment,
    ) -> Result<(), Error> {
        let path = format!(
            "/api/2.1/unity-catalog/workspaces/{}/metastore",
            workspace_id
        );
        let _: EmptyResponse = self.client.put(&path, request).await?;
        Ok(())
    }

    pub async fn create(&self, request: &CreateMetastore) -> Result<MetastoreInfo, Error> {
        self.client.post(PATH, request).await
    }

    /// Get the metastore assignment for the current workspace.
    pub async fn current(&self) -> Result<MetastoreAssignment, Error> {
        self.client
            .get("/api/2.1/unity-catalog/current-metastore-assignment")
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }

    pub async fn get(&self, id: &str) -> Result<MetastoreInfo, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<MetastoreInfo>, Error> {
        let response: ListMetastoresResponse = self.client.get(PATH).await?;
        Ok(response.metastores)
    }

    /// Get a summary of the current metastore.
    pub async fn summary(&self) -> Result<GetMetastoreSummaryResponse, Error> {
        self.client
            .get("/api/2.1/unity-catalog/metastore_summary")
            .await
    }

    /// Unassign a metastore from a workspace.
    pub async fn unassign(&self, workspace_id: i64) -> Result<(), Error> {
        let path = format!(
            "/api/2.1/unity-catalog/workspaces/{}/metastore",
            workspace_id
        );
        self.client.delete_empty(&path).await
    }

    pub async fn update(
        &self,
        id: &str,
        request: &UpdateMetastore,
    ) -> Result<MetastoreInfo, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.patch(&path, request).await
    }

    /// Update the metastore assignment for a workspace.
    pub async fn update_assignment(
        &self,
        workspace_id: i64,
        request: &UpdateMetastoreAssignment,
    ) -> Result<(), Error> {
        let path = format!(
            "/api/2.1/unity-catalog/workspaces/{}/metastore",
            workspace_id
        );
        let _: EmptyResponse = self.client.patch(&path, request).await?;
        Ok(())
    }
}
