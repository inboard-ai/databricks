use crate::types::{CreateRepo, ListReposResponse, Repo, UpdateRepo};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/repos";

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
}
