use crate::types::{
    DeleteRestrictWorkspaceAdminsSettingResponse, RestrictWorkspaceAdminsSetting,
    UpdateRestrictWorkspaceAdminsSettingRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/restrict_workspace_admins/names/default";

pub struct RestrictWorkspaceAdmins {
    client: Client,
}

impl RestrictWorkspaceAdmins {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the restrict workspace admins setting.
    pub async fn get(&self) -> Result<RestrictWorkspaceAdminsSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the restrict workspace admins setting.
    pub async fn update(
        &self,
        request: &UpdateRestrictWorkspaceAdminsSettingRequest,
    ) -> Result<RestrictWorkspaceAdminsSetting, Error> {
        self.client.patch(PATH, request).await
    }

    /// Delete the restrict workspace admins setting, reverting to the default.
    pub async fn delete(&self) -> Result<DeleteRestrictWorkspaceAdminsSettingResponse, Error> {
        self.client.delete(PATH).await
    }
}
