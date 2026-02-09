use crate::types::{
    EnhancedSecurityMonitoringSetting, UpdateEnhancedSecurityMonitoringSettingRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/settings/types/shield_esm_enablement_ws_db/names/default";

pub struct EnhancedSecurityMonitoring {
    client: Client,
}

impl EnhancedSecurityMonitoring {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the enhanced security monitoring setting.
    pub async fn get(&self) -> Result<EnhancedSecurityMonitoringSetting, Error> {
        self.client.get(PATH).await
    }

    /// Update the enhanced security monitoring setting.
    pub async fn update(
        &self,
        request: &UpdateEnhancedSecurityMonitoringSettingRequest,
    ) -> Result<EnhancedSecurityMonitoringSetting, Error> {
        self.client.patch(PATH, request).await
    }
}
