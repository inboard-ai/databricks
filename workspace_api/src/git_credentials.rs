use crate::types::{
    CreateGitCredential, GitCredential, ListGitCredentialsResponse, UpdateGitCredential,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/git-credentials";

pub struct GitCredentials {
    client: Client,
}

impl GitCredentials {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateGitCredential) -> Result<GitCredential, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, credential_id: i64) -> Result<GitCredential, Error> {
        let path = format!("{}/{}", PATH, credential_id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<GitCredential>, Error> {
        let response: ListGitCredentialsResponse = self.client.get(PATH).await?;
        Ok(response.credentials)
    }

    pub async fn update(
        &self,
        credential_id: i64,
        request: &UpdateGitCredential,
    ) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, credential_id);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, credential_id: i64) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, credential_id);
        self.client.delete_empty(&path).await
    }
}
