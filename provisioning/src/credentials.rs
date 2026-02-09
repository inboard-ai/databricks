use crate::types::{CreateCredentialRequest, Credential};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Credentials {
    client: Client,
    account_id: String,
}

impl Credentials {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/credentials", PATH, self.account_id)
    }

    pub async fn create(&self, request: &CreateCredentialRequest) -> Result<Credential, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, credentials_id: &str) -> Result<Credential, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), credentials_id))
            .await
    }

    pub async fn list(&self) -> Result<Vec<Credential>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, credentials_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), credentials_id))
            .await
    }
}
