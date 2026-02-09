use crate::types::{
    BudgetPolicy, CreateBudgetPolicyRequest, ListBudgetPoliciesResponse, UpdateBudgetPolicyRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/accounts";

pub struct BudgetPolicyService {
    client: Client,
    account_id: String,
}

impl BudgetPolicyService {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/budget-policies", PATH, self.account_id)
    }

    pub async fn create(&self, request: &CreateBudgetPolicyRequest) -> Result<BudgetPolicy, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, policy_id: &str) -> Result<BudgetPolicy, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), policy_id))
            .await
    }

    pub async fn list(&self) -> Result<ListBudgetPoliciesResponse, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn update(
        &self,
        policy_id: &str,
        request: &UpdateBudgetPolicyRequest,
    ) -> Result<BudgetPolicy, Error> {
        self.client
            .put(&format!("{}/{}", self.base_path(), policy_id), request)
            .await
    }

    pub async fn delete(&self, policy_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), policy_id))
            .await
    }
}
