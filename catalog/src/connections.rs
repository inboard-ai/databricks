use crate::types::{ConnectionInfo, CreateConnection, ListConnectionsResponse, UpdateConnection};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/connections";

pub struct Connections {
    client: Client,
}

impl Connections {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateConnection) -> Result<ConnectionInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<ConnectionInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<ConnectionInfo>, Error> {
        let response: ListConnectionsResponse = self.client.get(PATH).await?;
        Ok(response.connections)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateConnection,
    ) -> Result<ConnectionInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }
}
