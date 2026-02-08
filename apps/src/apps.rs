use crate::types::{App, CreateApp, ListAppsResponse, UpdateApp};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/apps";

pub struct Apps {
    client: Client,
}

impl Apps {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateApp) -> Result<App, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<App, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    pub async fn list(&self) -> Result<Vec<App>, Error> {
        let response: ListAppsResponse = self.client.get(PATH).await?;
        Ok(response.apps)
    }

    pub async fn update(&self, name: &str, request: &UpdateApp) -> Result<App, Error> {
        self.client
            .patch(&format!("{}/{}", PATH, name), request)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<App, Error> {
        self.client.delete(&format!("{}/{}", PATH, name)).await
    }

    pub async fn start(&self, name: &str) -> Result<App, Error> {
        let _empty: serde_json::Value = serde_json::Value::Object(serde_json::Map::new());
        self.client
            .post(&format!("{}/{}/start", PATH, name), &_empty)
            .await
    }

    pub async fn stop(&self, name: &str) -> Result<App, Error> {
        let _empty: serde_json::Value = serde_json::Value::Object(serde_json::Map::new());
        self.client
            .post(&format!("{}/{}/stop", PATH, name), &_empty)
            .await
    }
}
