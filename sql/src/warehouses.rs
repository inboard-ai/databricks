use crate::types::{Empty, EmptyResponse, ListWarehousesResponse, Warehouse};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/warehouses";

pub struct Warehouses {
    client: Client,
}

impl Warehouses {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Warehouse>, Error> {
        let response: ListWarehousesResponse = self.client.get(PATH).await?;
        Ok(response.warehouses)
    }

    pub async fn get(&self, id: &str) -> Result<Warehouse, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn start(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/start", PATH, id);
        let _: EmptyResponse = self.client.post(&path, &Empty {}).await?;
        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/stop", PATH, id);
        let _: EmptyResponse = self.client.post(&path, &Empty {}).await?;
        Ok(())
    }
}
