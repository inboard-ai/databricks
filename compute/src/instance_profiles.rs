use crate::types::{
    AddInstanceProfile, EmptyResponse, InstanceProfileInfo, ListInstanceProfilesResponse,
    RemoveInstanceProfile,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/instance-profiles";

pub struct InstanceProfiles {
    client: Client,
}

impl InstanceProfiles {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, request: &AddInstanceProfile) -> Result<(), Error> {
        let _: EmptyResponse = self.client.post(&format!("{}/add", PATH), request).await?;
        Ok(())
    }

    pub async fn edit(&self, request: &InstanceProfileInfo) -> Result<(), Error> {
        let _: EmptyResponse = self.client.post(&format!("{}/edit", PATH), request).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<InstanceProfileInfo>, Error> {
        let response: ListInstanceProfilesResponse =
            self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.instance_profiles)
    }

    pub async fn remove(&self, request: &RemoveInstanceProfile) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/remove", PATH), request)
            .await?;
        Ok(())
    }
}
