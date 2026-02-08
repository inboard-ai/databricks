use crate::types::{CreateWorkspaceRequest, UpdateWorkspaceRequest, Workspace};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Workspaces {
    client: Client,
    account_id: String,
}

impl Workspaces {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/workspaces", PATH, self.account_id)
    }

    pub async fn create(&self, request: &CreateWorkspaceRequest) -> Result<Workspace, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, workspace_id: i64) -> Result<Workspace, Error> {
        self.client
            .get(&format!("{}/{}", self.base_path(), workspace_id))
            .await
    }

    pub async fn list(&self) -> Result<Vec<Workspace>, Error> {
        self.client.get(&self.base_path()).await
    }

    pub async fn update(&self, request: &UpdateWorkspaceRequest) -> Result<Workspace, Error> {
        self.client
            .patch(
                &format!("{}/{}", self.base_path(), request.workspace_id),
                request,
            )
            .await
    }

    pub async fn delete(&self, workspace_id: i64) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), workspace_id))
            .await
    }
}
