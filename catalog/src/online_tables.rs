use crate::types::OnlineTable;
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/online-tables";

pub struct OnlineTables {
    client: Client,
}

impl OnlineTables {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &OnlineTable) -> Result<OnlineTable, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<OnlineTable, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }
}
