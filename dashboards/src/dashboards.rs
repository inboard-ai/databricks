use crate::types::{
    CreateDashboardRequest, Dashboard, ListDashboardsRequest, ListDashboardsResponse,
    ListSchedulesResponse, ListSubscriptionsResponse, MigrateDashboardRequest, PublishRequest,
    PublishedDashboard, Schedule, Subscription, UpdateDashboardRequest,
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

    /// Get the published version of a dashboard.
    pub async fn get_published(&self, dashboard_id: &str) -> Result<PublishedDashboard, Error> {
        let path = format!("{}/{}/published", PATH, dashboard_id);
        self.client.get(&path).await
    }

    /// Migrate a legacy dashboard to Lakeview.
    pub async fn migrate(&self, request: &MigrateDashboardRequest) -> Result<Dashboard, Error> {
        let path = format!("{}/migrate", PATH);
        self.client.post(&path, request).await
    }

    /// Trash a dashboard (alias for delete).
    pub async fn trash(&self, dashboard_id: &str) -> Result<(), Error> {
        self.delete(dashboard_id).await
    }

    // ========================================================================
    // Schedules
    // ========================================================================

    /// Create a schedule for a dashboard.
    pub async fn create_schedule(
        &self,
        dashboard_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule, Error> {
        let path = format!("{}/{}/schedules", PATH, dashboard_id);
        self.client.post(&path, schedule).await
    }

    /// Get a schedule by ID.
    pub async fn get_schedule(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
    ) -> Result<Schedule, Error> {
        let path = format!("{}/{}/schedules/{}", PATH, dashboard_id, schedule_id);
        self.client.get(&path).await
    }

    /// List schedules for a dashboard.
    pub async fn list_schedules(&self, dashboard_id: &str) -> Result<ListSchedulesResponse, Error> {
        let path = format!("{}/{}/schedules", PATH, dashboard_id);
        self.client.get(&path).await
    }

    /// Delete a schedule.
    pub async fn delete_schedule(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
    ) -> Result<(), Error> {
        let path = format!("{}/{}/schedules/{}", PATH, dashboard_id, schedule_id);
        self.client.delete_empty(&path).await
    }

    /// Update a schedule.
    pub async fn update_schedule(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule, Error> {
        let path = format!("{}/{}/schedules/{}", PATH, dashboard_id, schedule_id);
        self.client.put(&path, schedule).await
    }

    // ========================================================================
    // Subscriptions
    // ========================================================================

    /// Create a subscription for a schedule.
    pub async fn create_subscription(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
        subscription: &Subscription,
    ) -> Result<Subscription, Error> {
        let path = format!(
            "{}/{}/schedules/{}/subscriptions",
            PATH, dashboard_id, schedule_id
        );
        self.client.post(&path, subscription).await
    }

    /// Get a subscription by ID.
    pub async fn get_subscription(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
        subscription_id: &str,
    ) -> Result<Subscription, Error> {
        let path = format!(
            "{}/{}/schedules/{}/subscriptions/{}",
            PATH, dashboard_id, schedule_id, subscription_id
        );
        self.client.get(&path).await
    }

    /// List subscriptions for a schedule.
    pub async fn list_subscriptions(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
    ) -> Result<ListSubscriptionsResponse, Error> {
        let path = format!(
            "{}/{}/schedules/{}/subscriptions",
            PATH, dashboard_id, schedule_id
        );
        self.client.get(&path).await
    }

    /// Delete a subscription.
    pub async fn delete_subscription(
        &self,
        dashboard_id: &str,
        schedule_id: &str,
        subscription_id: &str,
    ) -> Result<(), Error> {
        let path = format!(
            "{}/{}/schedules/{}/subscriptions/{}",
            PATH, dashboard_id, schedule_id, subscription_id
        );
        self.client.delete_empty(&path).await
    }
}
