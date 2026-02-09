use crate::types::{ListPolicyFamiliesResponse, PolicyFamily};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/policy-families";

pub struct PolicyFamilies {
    client: Client,
}

impl PolicyFamilies {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, policy_family_id: &str) -> Result<PolicyFamily, Error> {
        let path = format!("{}/{}", PATH, policy_family_id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<ListPolicyFamiliesResponse, Error> {
        self.client.get(PATH).await
    }
}
