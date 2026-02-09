use crate::types::{CreateNetworkRequest, Network};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Networks {
    client: Client,
    account_id: String,
}

impl Networks {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/networks", PATH, self.account_id)
    }

    pub async fn create(&self, request: &CreateNetworkRequest) -> Result<Network, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, network_id: &str) -> Result<Network, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), network_id))
            .await
    }

    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, network_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), network_id))
            .await
    }
}
