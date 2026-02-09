use crate::types::{
    CreateExternalLocation, ExternalLocationInfo, ListExternalLocationsResponse,
    UpdateExternalLocation, ValidateStorageCredential, ValidateStorageCredentialResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/external-locations";

pub struct ExternalLocations {
    client: Client,
}

impl ExternalLocations {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateExternalLocation,
    ) -> Result<ExternalLocationInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<ExternalLocationInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<ExternalLocationInfo>, Error> {
        let response: ListExternalLocationsResponse = self.client.get(PATH).await?;
        Ok(response.external_locations)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateExternalLocation,
    ) -> Result<ExternalLocationInfo, Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, name);
        self.client.delete_empty(&path).await
    }

    /// Validate an external location's storage credential.
    pub async fn validate(
        &self,
        request: &ValidateStorageCredential,
    ) -> Result<ValidateStorageCredentialResponse, Error> {
        self.client
            .post(
                "/api/2.1/unity-catalog/validate-storage-credentials",
                request,
            )
            .await
    }
}
