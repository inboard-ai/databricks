use crate::types::{CreateSpaceRequest, ListSpacesResponse, Space, UpdateSpaceRequest};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/genie/spaces";

pub struct Spaces {
    client: Client,
}

impl Spaces {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List all Genie spaces.
    pub async fn list(&self) -> Result<Vec<Space>, Error> {
        let response: ListSpacesResponse = self.client.get(PATH).await?;
        Ok(response.spaces)
    }

    /// Get a Genie space by ID.
    pub async fn get(&self, space_id: &str) -> Result<Space, Error> {
        let path = format!("{}/{}", PATH, space_id);
        self.client.get(&path).await
    }

    /// Create a new Genie space.
    pub async fn create(&self, request: &CreateSpaceRequest) -> Result<Space, Error> {
        self.client.post(PATH, request).await
    }

    /// Update a Genie space.
    pub async fn update(
        &self,
        space_id: &str,
        request: &UpdateSpaceRequest,
    ) -> Result<Space, Error> {
        let path = format!("{}/{}", PATH, space_id);
        self.client.patch(&path, request).await
    }

    /// Trash (delete) a Genie space.
    pub async fn trash(&self, space_id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, space_id);
        self.client.delete_empty(&path).await
    }
}
