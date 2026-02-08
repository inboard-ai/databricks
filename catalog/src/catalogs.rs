use crate::types::{CatalogInfo, CreateCatalog, ListCatalogsResponse, UpdateCatalog};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/catalogs";

pub struct Catalogs {
    client: Client,
}

impl Catalogs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateCatalog) -> Result<CatalogInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<CatalogInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<CatalogInfo>, Error> {
        let response: ListCatalogsResponse = self.client.get(PATH).await?;
        Ok(response.catalogs)
    }

    pub async fn update(&self, name: &str, request: &UpdateCatalog) -> Result<CatalogInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }
}
