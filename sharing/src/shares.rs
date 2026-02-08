use crate::types::{CreateShare, EmptyResponse, ListSharesResponse, ShareInfo, UpdateShare};
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
}
