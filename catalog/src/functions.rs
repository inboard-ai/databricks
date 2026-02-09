use crate::types::{CreateFunction, FunctionInfo, ListFunctionsResponse, UpdateFunction};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/functions";

pub struct Functions {
    client: Client,
}

impl Functions {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateFunction) -> Result<FunctionInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<FunctionInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn list(
        &self,
        catalog_name: &str,
        schema_name: &str,
    ) -> Result<Vec<FunctionInfo>, Error> {
        let response: ListFunctionsResponse = self
            .client
            .get_with_query(
                PATH,
                &[("catalog_name", catalog_name), ("schema_name", schema_name)],
            )
            .await?;
        Ok(response.functions)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateFunction,
    ) -> Result<FunctionInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }
}
