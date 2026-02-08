use crate::types::{
    CreateDashboardRequest, Dashboard, ListDashboardsRequest, ListDashboardsResponse,
    PublishRequest, PublishedDashboard, UpdateDashboardRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/lakeview/dashboards";

pub struct Dashboards {
    client: Client,
}

impl Dashboards {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a draft dashboard.
    pub async fn create(&self, request: &CreateDashboardRequest) -> Result<Dashboard, Error> {
        self.client.post(PATH, request).await
    }

    /// Get a draft dashboard by ID.
    pub async fn get(&self, dashboard_id: &str) -> Result<Dashboard, Error> {
        let path = format!("{}/{}", PATH, dashboard_id);
        self.client.get(&path).await
    }

    /// List dashboards.
    pub async fn list(
        &self,
        request: &ListDashboardsRequest,
    ) -> Result<ListDashboardsResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ps) = request.page_size {
            query.push(("page_size", ps.to_string()));
        }
        if let Some(ref pt) = request.page_token {
            query.push(("page_token", pt.clone()));
        }
        if let Some(st) = request.show_trashed {
            query.push(("show_trashed", st.to_string()));
        }
        if let Some(ref v) = request.view {
            let v_str = serde_json::to_value(v)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default();
            query.push(("view", v_str));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(PATH, &pairs).await
    }

    /// Update a draft dashboard.
    pub async fn update(
        &self,
        dashboard_id: &str,
        request: &UpdateDashboardRequest,
    ) -> Result<Dashboard, Error> {
        let path = format!("{}/{}", PATH, dashboard_id);
        self.client.patch(path.as_str(), request).await
    }

    /// Trash a dashboard (soft delete).
    pub async fn delete(&self, dashboard_id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, dashboard_id);
        self.client.delete_empty(&path).await
    }

    /// Publish the current draft dashboard.
    pub async fn publish(
        &self,
        dashboard_id: &str,
        request: &PublishRequest,
    ) -> Result<PublishedDashboard, Error> {
        let path = format!("{}/{}/published", PATH, dashboard_id);
        self.client.post(&path, request).await
    }

    /// Unpublish the dashboard.
    pub async fn unpublish(&self, dashboard_id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/published", PATH, dashboard_id);
        self.client.delete_empty(&path).await
    }
}
