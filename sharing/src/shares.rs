use crate::types::{
    CreateShare, EmptyResponse, GetSharePermissionsResponse, ListSharesResponse, ShareInfo,
    UpdateShare, UpdateSharePermissions,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/shares";

pub struct Shares {
    client: Client,
}

impl Shares {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateShare) -> Result<ShareInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<ShareInfo, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    pub async fn list(&self) -> Result<Vec<ShareInfo>, Error> {
        let response: ListSharesResponse = self.client.get(PATH).await?;
        Ok(response.shares)
    }

    pub async fn update(&self, name: &str, request: &UpdateShare) -> Result<ShareInfo, Error> {
        self.client
            .patch(&format!("{}/{}", PATH, name), request)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self.client.delete(&format!("{}/{}", PATH, name)).await?;
        Ok(())
    }

    /// Get the permissions for a share.
    pub async fn share_permissions(
        &self,
        name: &str,
    ) -> Result<GetSharePermissionsResponse, Error> {
        self.client
            .get(&format!("{}/{}/permissions", PATH, name))
            .await
    }

    /// Update the permissions on a share.
    pub async fn update_permissions(
        &self,
        name: &str,
        request: &UpdateSharePermissions,
    ) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .patch(&format!("{}/{}/permissions", PATH, name), request)
            .await?;
        Ok(())
    }
}
