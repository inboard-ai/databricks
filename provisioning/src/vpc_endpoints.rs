use crate::types::{CreateVpcEndpointRequest, VpcEndpoint};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct VpcEndpoints {
    client: Client,
    account_id: String,
}

impl VpcEndpoints {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/vpc-endpoints", PATH, self.account_id)
    }

    pub async fn create(&self, request: &CreateVpcEndpointRequest) -> Result<VpcEndpoint, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, vpc_endpoint_id: &str) -> Result<VpcEndpoint, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), vpc_endpoint_id))
            .await
    }

    pub async fn list(&self) -> Result<Vec<VpcEndpoint>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn delete(&self, vpc_endpoint_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), vpc_endpoint_id))
            .await
    }
}
