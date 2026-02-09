use crate::types::{
    CreateProvider, EmptyResponse, ListProviderSharesResponse, ListProvidersResponse, ProviderInfo,
    UpdateProvider,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/providers";

pub struct Providers {
    client: Client,
}

impl Providers {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateProvider) -> Result<ProviderInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<ProviderInfo, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    pub async fn list(&self) -> Result<Vec<ProviderInfo>, Error> {
        let response: ListProvidersResponse = self.client.get(PATH).await?;
        Ok(response.providers)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateProvider,
    ) -> Result<ProviderInfo, Error> {
        self.client
            .patch(&format!("{}/{}", PATH, name), request)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self.client.delete(&format!("{}/{}", PATH, name)).await?;
        Ok(())
    }

    /// List shares owned by the specified provider.
    pub async fn list_shares(&self, name: &str) -> Result<ListProviderSharesResponse, Error> {
        self.client.get(&format!("{}/{}/shares", PATH, name)).await
    }
}
