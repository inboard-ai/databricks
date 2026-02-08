use crate::types::{EmptyResponse, Group, ListGroupsResponse, PatchRequest};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/preview/scim/v2/Groups";

pub struct Groups {
    client: Client,
}

impl Groups {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, group: &Group) -> Result<Group, Error> {
        self.client.post(PATH, group).await
    }

    pub async fn get(&self, id: &str) -> Result<Group, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<Group>, Error> {
        let response: ListGroupsResponse = self.client.get(PATH).await?;
        Ok(response.resources)
    }

    pub async fn update(&self, id: &str, group: &Group) -> Result<Group, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.put(&path, group).await
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
