use crate::alert_types::{Alert, CreateAlertRequest, ListAlertsResponse, UpdateAlertRequest};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/alerts";

pub struct Alerts {
    client: Client,
}

impl Alerts {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create an alert.
    pub async fn create(&self, request: &CreateAlertRequest) -> Result<Alert, Error> {
        self.client.post(PATH, request).await
    }

    /// Get an alert by ID.
    pub async fn get(&self, id: &str) -> Result<Alert, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    /// List alerts.
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<ListAlertsResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ps) = page_size {
            query.push(("page_size", ps.to_string()));
        }
        if let Some(pt) = page_token {
            query.push(("page_token", pt.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(PATH, &pairs).await
    }

    /// Update an alert.
    pub async fn update(&self, id: &str, request: &UpdateAlertRequest) -> Result<Alert, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.patch(&path, request).await
    }

    /// Trash (delete) an alert.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }
}
