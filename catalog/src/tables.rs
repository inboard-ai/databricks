use crate::types::{
    CreateTable, ListTableSummariesResponse, ListTablesResponse, TableExistsResponse, TableInfo,
    TableSummary, UpdateTable,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/tables";
const SUMMARIES_PATH: &str = "/api/2.1/unity-catalog/table-summaries";

pub struct Tables {
    client: Client,
}

impl Tables {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateTable) -> Result<TableInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, full_name: &str) -> Result<TableInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn list(
        &self,
        catalog_name: &str,
        schema_name: &str,
    ) -> Result<Vec<TableInfo>, Error> {
        let response: ListTablesResponse = self
            .client
            .get_with_query(
                PATH,
                &[("catalog_name", catalog_name), ("schema_name", schema_name)],
            )
            .await?;
        Ok(response.tables)
    }

    pub async fn exists(&self, full_name: &str) -> Result<TableExistsResponse, Error> {
        let path = format!("{}/{}/exists", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn list_summaries(&self, catalog_name: &str) -> Result<Vec<TableSummary>, Error> {
        let response: ListTableSummariesResponse = self
            .client
            .get_with_query(SUMMARIES_PATH, &[("catalog_name", catalog_name)])
            .await?;
        Ok(response.tables)
    }

    pub async fn update(&self, full_name: &str, request: &UpdateTable) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        let _: serde_json::Value = self.client.patch(&path, request).await?;
        Ok(())
    }

    pub async fn delete(&self, full_name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.delete_empty(&path).await
    }
}
