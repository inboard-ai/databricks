use crate::types::{
    CreateStorageCredential, ListStorageCredentialsResponse, StorageCredentialInfo,
    UpdateStorageCredential, ValidateStorageCredential, ValidateStorageCredentialResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/storage-credentials";

pub struct StorageCredentials {
    client: Client,
}

impl StorageCredentials {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateStorageCredential,
    ) -> Result<StorageCredentialInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<StorageCredentialInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<StorageCredentialInfo>, Error> {
        let response: ListStorageCredentialsResponse = self.client.get(PATH).await?;
        Ok(response.storage_credentials)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateStorageCredential,
    ) -> Result<StorageCredentialInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }

    pub async fn validate(
        &self,
        request: &ValidateStorageCredential,
    ) -> Result<ValidateStorageCredentialResponse, Error> {
        self.client
            .post(
                "/api/2.1/unity-catalog/validate-storage-credentials",
                request,
            )
            .await
    }
}
