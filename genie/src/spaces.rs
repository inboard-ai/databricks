use crate::types::{ListSpacesResponse, Space};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/genie/spaces";

pub struct Spaces {
    client: Client,
}

impl Spaces {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Space>, Error> {
        let response: ListSpacesResponse = self.client.get(PATH).await?;
        Ok(response.spaces)
    }

    pub async fn get(&self, space_id: &str) -> Result<Space, Error> {
        let path = format!("{}/{}", PATH, space_id);
        self.client.get(&path).await
    }
}
