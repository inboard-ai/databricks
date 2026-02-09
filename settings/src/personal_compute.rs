use crate::types::{
    DeletePersonalComputeSettingResponse, PersonalComputeSetting,
    UpdatePersonalComputeSettingRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/dcp_acct_enable/names/default";

pub struct PersonalCompute {
    client: Client,
}

impl PersonalCompute {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the personal compute setting.
    pub async fn get(&self) -> Result<PersonalComputeSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the personal compute setting.
    pub async fn update(
        &self,
        request: &UpdatePersonalComputeSettingRequest,
    ) -> Result<PersonalComputeSetting, Error> {
        self.client.patch(PATH, request).await
    }

    /// Delete the personal compute setting, reverting to the default.
    pub async fn delete(&self) -> Result<DeletePersonalComputeSettingResponse, Error> {
        self.client.delete(PATH).await
    }
}
