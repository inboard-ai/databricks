use crate::types::{ListTablesResponse, TableInfo};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/tables";

pub struct Tables {
    client: Client,
}

impl Tables {
    pub fn new(client: Client) -> Self {
        Self { client }
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

    pub async fn delete(&self, full_name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.delete_empty(&path).await
    }
}
