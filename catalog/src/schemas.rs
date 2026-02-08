use crate::types::{CreateSchema, ListSchemasResponse, SchemaInfo, UpdateSchema};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/schemas";

pub struct Schemas {
    client: Client,
}

impl Schemas {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateSchema) -> Result<SchemaInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, full_name: &str) -> Result<SchemaInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn list(&self, catalog_name: &str) -> Result<Vec<SchemaInfo>, Error> {
        let response: ListSchemasResponse = self
            .client
            .get_with_query(PATH, &[("catalog_name", catalog_name)])
            .await?;
        Ok(response.schemas)
    }

    pub async fn update(
        &self,
        full_name: &str,
        request: &UpdateSchema,
    ) -> Result<SchemaInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, full_name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.delete_empty(&path).await
    }
}
