use crate::types::{ObjectPermissions, SetPermissions};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/permissions";

pub struct Permissions {
    client: Client,
}

impl Permissions {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get permissions for an object.
    /// `object_type` examples: "clusters", "jobs", "sql/warehouses", "notebooks"
    pub async fn get(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("{}/{}/{}", PATH, object_type, object_id);
        self.client.get(&path).await
    }

    /// Set (replace) permissions for an object.
    pub async fn set(
        &self,
        object_type: &str,
        object_id: &str,
        request: &SetPermissions,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("{}/{}/{}", PATH, object_type, object_id);
        self.client.put(&path, request).await
    }

    /// Update (patch) permissions for an object.
    pub async fn update(
        &self,
        object_type: &str,
        object_id: &str,
        request: &SetPermissions,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("{}/{}/{}", PATH, object_type, object_id);
        self.client.patch(&path, request).await
    }
}
