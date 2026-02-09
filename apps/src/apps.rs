use crate::types::{
    App, AppPermissionLevelsResponse, AppPermissions, AppPermissionsRequest, AppUpdate, CreateApp,
    CreateDeploymentRequest, CreateUpdateRequest, Deployment, ListAppsResponse,
    ListDeploymentsResponse, UpdateApp,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/apps";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/apps";

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

    pub async fn create_update(
        &self,
        app_name: &str,
        request: &CreateUpdateRequest,
    ) -> Result<AppUpdate, Error> {
        self.client
            .post(&format!("{}/{}/update", PATH, app_name), request)
            .await
    }

    pub async fn deploy(
        &self,
        app_name: &str,
        request: &CreateDeploymentRequest,
    ) -> Result<Deployment, Error> {
        self.client
            .post(&format!("{}/{}/deployments", PATH, app_name), request)
            .await
    }

    pub async fn get_deployment(
        &self,
        app_name: &str,
        deployment_id: &str,
    ) -> Result<Deployment, Error> {
        self.client
            .get(&format!(
                "{}/{}/deployments/{}",
                PATH, app_name, deployment_id
            ))
            .await
    }

    pub async fn get_update(&self, app_name: &str, update_id: &str) -> Result<AppUpdate, Error> {
        self.client
            .get_with_query(
                &format!("{}/{}/update", PATH, app_name),
                &[("update_id", update_id)],
            )
            .await
    }

    pub async fn list_deployments(&self, app_name: &str) -> Result<ListDeploymentsResponse, Error> {
        self.client
            .get(&format!("{}/{}/deployments", PATH, app_name))
            .await
    }

    pub async fn get_permissions(&self, app_name: &str) -> Result<AppPermissions, Error> {
        self.client
            .get(&format!("{}/{}", PERMISSIONS_PATH, app_name))
            .await
    }

    pub async fn get_permission_levels(
        &self,
        app_name: &str,
    ) -> Result<AppPermissionLevelsResponse, Error> {
        self.client
            .get(&format!(
                "{}/{}/permissionLevels",
                PERMISSIONS_PATH, app_name
            ))
            .await
    }

    pub async fn set_permissions(
        &self,
        app_name: &str,
        request: &AppPermissionsRequest,
    ) -> Result<AppPermissions, Error> {
        self.client
            .put(&format!("{}/{}", PERMISSIONS_PATH, app_name), request)
            .await
    }

    pub async fn update_permissions(
        &self,
        app_name: &str,
        request: &AppPermissionsRequest,
    ) -> Result<AppPermissions, Error> {
        self.client
            .patch(&format!("{}/{}", PERMISSIONS_PATH, app_name), request)
            .await
    }
}
