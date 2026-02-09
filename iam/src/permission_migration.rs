use crate::types::{MigratePermissionsRequest, MigratePermissionsResponse};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/permissionmigration";

pub struct PermissionMigration {
    client: Client,
}

impl PermissionMigration {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Migrate permissions from workspace group to account group.
    pub async fn migrate_permissions(
        &self,
        request: &MigratePermissionsRequest,
    ) -> Result<MigratePermissionsResponse, Error> {
        self.client.post(PATH, request).await
    }
}
