use crate::types::{CreateStorageConfigurationRequest, StorageConfiguration};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Storage {
    client: Client,
    account_id: String,
}

impl Storage {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/storage-configurations", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &CreateStorageConfigurationRequest,
    ) -> Result<StorageConfiguration, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(
        &self,
        storage_configuration_id: &str,
    ) -> Result<StorageConfiguration, Error> {
        self.client
            .get(&format!(
                "{}/{}",
                self.base_path(),
                storage_configuration_id
            ))
            .await
    }

    pub async fn list(&self) -> Result<Vec<StorageConfiguration>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, storage_configuration_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!(
                "{}/{}",
                self.base_path(),
                storage_configuration_id
            ))
            .await
    }
}
