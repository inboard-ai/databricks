use crate::types::{PermissionsList, UpdatePermissions};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/permissions";

pub struct Grants {
    client: Client,
}

impl Grants {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get grants on a securable object.
    /// `securable_type` examples: "catalog", "schema", "table", "volume"
    pub async fn get(
        &self,
        securable_type: &str,
        full_name: &str,
    ) -> Result<PermissionsList, Error> {
        let path = format!("{}/{}/{}", PATH, securable_type, full_name);
        self.client.get(&path).await
    }

    /// Update grants on a securable object.
    pub async fn update(
        &self,
        securable_type: &str,
        full_name: &str,
        request: &UpdatePermissions,
    ) -> Result<PermissionsList, Error> {
        let path = format!("{}/{}/{}", PATH, securable_type, full_name);
        self.client.patch(&path, request).await
    }
}
