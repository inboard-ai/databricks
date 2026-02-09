use crate::types::{CreateVolume, ListVolumesResponse, UpdateVolume, VolumeInfo};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/volumes";

pub struct Volumes {
    client: Client,
}

impl Volumes {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateVolume) -> Result<VolumeInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, full_name: &str) -> Result<VolumeInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn read(&self, full_name: &str) -> Result<VolumeInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.get(&path).await
    }

    pub async fn list(
        &self,
        catalog_name: &str,
        schema_name: &str,
    ) -> Result<Vec<VolumeInfo>, Error> {
        let response: ListVolumesResponse = self
            .client
            .get_with_query(
                PATH,
                &[("catalog_name", catalog_name), ("schema_name", schema_name)],
            )
            .await?;
        Ok(response.volumes)
    }

    pub async fn update(
        &self,
        full_name: &str,
        request: &UpdateVolume,
    ) -> Result<VolumeInfo, Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.patch(&path, request).await
    }

    pub async fn delete(&self, full_name: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        self.client.delete_empty(&path).await
    }
}
