use crate::types::{EmptyResponse, ListServicePrincipalsResponse, PatchRequest, ServicePrincipal};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/preview/scim/v2/ServicePrincipals";

pub struct ServicePrincipals {
    client: Client,
}

impl ServicePrincipals {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, sp: &ServicePrincipal) -> Result<ServicePrincipal, Error> {
        self.client.post(PATH, sp).await
    }

    pub async fn get(&self, id: &str) -> Result<ServicePrincipal, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<ServicePrincipal>, Error> {
        let response: ListServicePrincipalsResponse = self.client.get(PATH).await?;
        Ok(response.resources)
    }

    pub async fn update(&self, id: &str, sp: &ServicePrincipal) -> Result<ServicePrincipal, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.put(&path, sp).await
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
