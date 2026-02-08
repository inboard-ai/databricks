use crate::types::{
    BudgetConfiguration, CreateBudgetConfigurationRequest, CreateBudgetConfigurationResponse,
    GetBudgetConfigurationResponse, ListBudgetConfigurationsResponse,
    UpdateBudgetConfigurationRequest, UpdateBudgetConfigurationResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Budgets {
    client: Client,
    account_id: String,
}

impl Budgets {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/budgets", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &CreateBudgetConfigurationRequest,
    ) -> Result<CreateBudgetConfigurationResponse, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, budget_id: &str) -> Result<BudgetConfiguration, Error> {
        let resp: GetBudgetConfigurationResponse = self
            .client
            .get(&format!("{}/{}", self.base_path(), budget_id))
            .await?;
        resp.budget
            .ok_or_else(|| Error::Other("Missing budget in response".into()))
    }

    pub async fn list(&self) -> Result<Vec<BudgetConfiguration>, Error> {
        let resp: ListBudgetConfigurationsResponse = self.client.get(&self.base_path()).await?;
        Ok(resp.budgets)
    }

    pub async fn update(
        &self,
        request: &UpdateBudgetConfigurationRequest,
    ) -> Result<UpdateBudgetConfigurationResponse, Error> {
        self.client
            .put(
                &format!("{}/{}", self.base_path(), request.budget_id),
                request,
            )
            .await
    }

    pub async fn delete(&self, budget_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), budget_id))
            .await
    }
}
