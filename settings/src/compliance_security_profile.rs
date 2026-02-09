use crate::types::{
    ComplianceSecurityProfileSetting, UpdateComplianceSecurityProfileSettingRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/shield_csp_enablement_ws_db/names/default";

pub struct ComplianceSecurityProfile {
    client: Client,
}

impl ComplianceSecurityProfile {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the compliance security profile setting.
    pub async fn get(&self) -> Result<ComplianceSecurityProfileSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the compliance security profile setting.
    pub async fn update(
        &self,
        request: &UpdateComplianceSecurityProfileSettingRequest,
    ) -> Result<ComplianceSecurityProfileSetting, Error> {
        self.client.patch(PATH, request).await
    }
}
