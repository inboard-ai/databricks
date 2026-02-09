use crate::types::{
    CreateUsageDashboardRequest, CreateUsageDashboardResponse, GetUsageDashboardResponse,
    UsageDashboardType,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct UsageDashboards {
    client: Client,
    account_id: String,
}

impl UsageDashboards {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/dashboard", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &CreateUsageDashboardRequest,
    ) -> Result<CreateUsageDashboardResponse, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(
        &self,
        dashboard_type: Option<UsageDashboardType>,
        workspace_id: Option<i64>,
    ) -> Result<GetUsageDashboardResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(dt) = dashboard_type {
            let dt_str = serde_json::to_value(dt)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            query.push(("dashboard_type", dt_str));
        }

        if let Some(wid) = workspace_id {
            query.push(("workspace_id", wid.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&self.base_path(), &query_refs)
            .await
    }
}
