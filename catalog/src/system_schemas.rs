use crate::types::{EmptyResponse, ListSystemSchemasResponse, SystemSchemaInfo};
use databricks_core::{Client, Error};

pub struct SystemSchemas {
    client: Client,
}

impl SystemSchemas {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Disable a system schema for a metastore.
    pub async fn disable(&self, metastore_id: &str, schema_name: &str) -> Result<(), Error> {
        let path = format!(
            "/api/2.1/unity-catalog/metastores/{}/systemschemas/{}",
            metastore_id, schema_name
        );
        self.client.delete_empty(&path).await
    }

    /// Enable a system schema for a metastore.
    pub async fn enable(&self, metastore_id: &str, schema_name: &str) -> Result<(), Error> {
        let path = format!(
            "/api/2.1/unity-catalog/metastores/{}/systemschemas/{}",
            metastore_id, schema_name
        );
        let empty = serde_json::json!({});
        let _: EmptyResponse = self.client.put(&path, &empty).await?;
        Ok(())
    }

    /// List system schemas for a metastore.
    pub async fn list(&self, metastore_id: &str) -> Result<Vec<SystemSchemaInfo>, Error> {
        let path = format!(
            "/api/2.1/unity-catalog/metastores/{}/systemschemas",
            metastore_id
        );
        let response: ListSystemSchemasResponse = self.client.get(&path).await?;
        Ok(response.schemas)
    }
}
