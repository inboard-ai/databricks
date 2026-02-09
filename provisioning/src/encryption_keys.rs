use crate::types::{CreateCustomerManagedKeyRequest, CustomerManagedKey};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct EncryptionKeys {
    client: Client,
    account_id: String,
}

impl EncryptionKeys {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/customer-managed-keys", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &CreateCustomerManagedKeyRequest,
    ) -> Result<CustomerManagedKey, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, customer_managed_key_id: &str) -> Result<CustomerManagedKey, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), customer_managed_key_id))
            .await
    }

    pub async fn list(&self) -> Result<Vec<CustomerManagedKey>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, customer_managed_key_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), customer_managed_key_id))
            .await
    }
}
