use crate::types::{
    CreateRepo, GetRepoPermissionLevelsResponse, ListReposResponse, Repo, RepoPermissions,
    RepoPermissionsRequest, UpdateRepo,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/repos";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/repos";

pub struct Repos {
    client: Client,
}

impl Repos {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateRepo) -> Result<Repo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, repo_id: i64) -> Result<Repo, Error> {
        let path = format!("{}/{}", PATH, repo_id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<Repo>, Error> {
        let response: ListReposResponse = self.client.get(PATH).await?;
        Ok(response.repos)
    }

    pub async fn update(&self, repo_id: i64, request: &UpdateRepo) -> Result<Repo, Error> {
        let path = format!("{}/{}", PATH, repo_id);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, repo_id: i64) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, repo_id);
        self.client.delete_empty(&path).await
    }

    pub async fn get_permissions(&self, repo_id: i64) -> Result<RepoPermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, repo_id);
        self.client.get(&path).await
    }

    pub async fn get_permission_levels(
        &self,
        repo_id: i64,
    ) -> Result<GetRepoPermissionLevelsResponse, Error> {
        let path = format!("{}/{}/permissionLevels", PERMISSIONS_PATH, repo_id);
        self.client.get(&path).await
    }

    pub async fn set_permissions(
        &self,
        repo_id: i64,
        request: &RepoPermissionsRequest,
    ) -> Result<RepoPermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, repo_id);
        self.client.put(&path, request).await
    }

    pub async fn update_permissions(
        &self,
        repo_id: i64,
        request: &RepoPermissionsRequest,
    ) -> Result<RepoPermissions, Error> {
        let path = format!("{}/{}", PERMISSIONS_PATH, repo_id);
        self.client.patch(&path, request).await
    }
}
