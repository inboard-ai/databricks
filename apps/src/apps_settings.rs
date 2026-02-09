use crate::types::{
    CreateCustomTemplateRequest, CustomTemplate, ListCustomTemplatesResponse,
    UpdateCustomTemplateRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/apps-settings/templates";

pub struct AppsSettings {
    client: Client,
}

impl AppsSettings {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_custom_template(
        &self,
        request: &CreateCustomTemplateRequest,
    ) -> Result<CustomTemplate, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get_custom_template(&self, name: &str) -> Result<CustomTemplate, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    pub async fn list_custom_templates(&self) -> Result<ListCustomTemplatesResponse, Error> {
        self.client.get(PATH).await
    }

    pub async fn update_custom_template(
        &self,
        name: &str,
        request: &UpdateCustomTemplateRequest,
    ) -> Result<CustomTemplate, Error> {
        self.client
            .put(&format!("{}/{}", PATH, name), request)
            .await
    }

    pub async fn delete_custom_template(&self, name: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, name))
            .await
    }
}
