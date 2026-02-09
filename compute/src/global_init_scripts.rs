use crate::types::{
    EmptyResponse, GlobalInitScriptCreateRequest, GlobalInitScriptCreateResponse,
    GlobalInitScriptDetailsWithContent, GlobalInitScriptUpdateRequest,
    ListGlobalInitScriptsResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/global-init-scripts";

pub struct GlobalInitScripts {
    client: Client,
}

impl GlobalInitScripts {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &GlobalInitScriptCreateRequest,
    ) -> Result<GlobalInitScriptCreateResponse, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn delete(&self, script_id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, script_id);
        self.client.delete_empty(&path).await
    }

    pub async fn get(&self, script_id: &str) -> Result<GlobalInitScriptDetailsWithContent, Error> {
        let path = format!("{}/{}", PATH, script_id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<ListGlobalInitScriptsResponse, Error> {
        self.client.get(PATH).await
    }

    pub async fn update(
        &self,
        script_id: &str,
        request: &GlobalInitScriptUpdateRequest,
    ) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, script_id);
        let _: EmptyResponse = self.client.patch(&path, request).await?;
        Ok(())
    }
}
