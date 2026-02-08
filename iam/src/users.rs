use crate::types::{EmptyResponse, ListUsersResponse, PatchRequest, User};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/preview/scim/v2/Users";

pub struct Users {
    client: Client,
}

impl Users {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, user: &User) -> Result<User, Error> {
        self.client.post(PATH, user).await
    }

    pub async fn get(&self, id: &str) -> Result<User, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<User>, Error> {
        let response: ListUsersResponse = self.client.get(PATH).await?;
        Ok(response.resources)
    }

    pub async fn update(&self, id: &str, user: &User) -> Result<User, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.put(&path, user).await
    }

    pub async fn patch(&self, id: &str, request: &PatchRequest) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        let _: EmptyResponse = self.client.patch(&path, request).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }
}
