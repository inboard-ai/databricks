use crate::types::{CreatePrivateAccessSettingsRequest, PrivateAccessSettings};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct PrivateAccess {
    client: Client,
    account_id: String,
}

impl PrivateAccess {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/private-access-settings", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &CreatePrivateAccessSettingsRequest,
    ) -> Result<PrivateAccessSettings, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(
        &self,
        private_access_settings_id: &str,
    ) -> Result<PrivateAccessSettings, Error> {
        self.client
            .get(&format!(
                "{}/{}",
                self.base_path(),
                private_access_settings_id
            ))
            .await
    }

    pub async fn list(&self) -> Result<Vec<PrivateAccessSettings>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, private_access_settings_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!(
                "{}/{}",
                self.base_path(),
                private_access_settings_id
            ))
            .await
    }

    pub async fn replace(
        &self,
        private_access_settings_id: &str,
        settings: &PrivateAccessSettings,
    ) -> Result<PrivateAccessSettings, Error> {
        self.client
            .put(
                &format!("{}/{}", self.base_path(), private_access_settings_id),
                settings,
            )
            .await
    }
}
