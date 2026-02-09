use crate::types::{
    PermissionAssignment, PermissionAssignments, UpdateWorkspaceAssignments, WorkspacePermissions,
};
use databricks_core::{Client, Error};

pub struct WorkspaceAssignment {
    client: Client,
    account_id: String,
}

impl WorkspaceAssignment {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    /// Delete a workspace permission assignment for a principal.
    pub async fn delete(&self, workspace_id: i64, principal_id: i64) -> Result<(), Error> {
        let path = format!(
            "/api/2.0/accounts/{}/workspaces/{}/permissionassignments/principals/{}",
            self.account_id, workspace_id, principal_id
        );
        self.client.delete_empty(&path).await
    }

    /// Get permission assignments for a workspace.
    pub async fn get(&self, workspace_id: i64) -> Result<WorkspacePermissions, Error> {
        let path = format!(
            "/api/2.0/accounts/{}/workspaces/{}/permissionassignments/permissions",
            self.account_id, workspace_id
        );
        self.client.get(&path).await
    }

    /// List all permission assignments for a workspace.
    pub async fn list(&self, workspace_id: i64) -> Result<PermissionAssignments, Error> {
        let path = format!(
            "/api/2.0/accounts/{}/workspaces/{}/permissionassignments",
            self.account_id, workspace_id
        );
        self.client.get(&path).await
    }

    /// Create or update permission assignments for a principal on a workspace.
    pub async fn update(
        &self,
        workspace_id: i64,
        principal_id: i64,
        request: &UpdateWorkspaceAssignments,
    ) -> Result<PermissionAssignment, Error> {
        let path = format!(
            "/api/2.0/accounts/{}/workspaces/{}/permissionassignments/principals/{}",
            self.account_id, workspace_id, principal_id
        );
        self.client.put(&path, request).await
    }
}
