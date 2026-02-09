use crate::types::{
    GetLogDeliveryConfigurationResponse, LogDeliveryConfigStatus, LogDeliveryConfiguration,
    PatchLogDeliveryStatusRequest, WrappedCreateLogDeliveryConfiguration,
    WrappedLogDeliveryConfiguration, WrappedLogDeliveryConfigurations,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct LogDelivery {
    client: Client,
    account_id: String,
}

impl LogDelivery {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/log-delivery", PATH, self.account_id)
    }

    pub async fn create(
        &self,
        request: &WrappedCreateLogDeliveryConfiguration,
    ) -> Result<WrappedLogDeliveryConfiguration, Error> {
        self.client.post(&self.base_path(), request).await
    }

    pub async fn get(&self, config_id: &str) -> Result<LogDeliveryConfiguration, Error> {
        let resp: GetLogDeliveryConfigurationResponse = self
            .client
            .get(&format!("{}/{}", self.base_path(), config_id))
            .await?;
        resp.log_delivery_configuration
            .ok_or_else(|| Error::Other("Missing log_delivery_configuration in response".into()))
    }

    pub async fn list(&self) -> Result<Vec<LogDeliveryConfiguration>, Error> {
        let resp: WrappedLogDeliveryConfigurations = self.client.get(&self.base_path()).await?;
        Ok(resp.log_delivery_configurations)
    }

    pub async fn patch_status(
        &self,
        config_id: &str,
        status: LogDeliveryConfigStatus,
    ) -> Result<(), Error> {
        let request = PatchLogDeliveryStatusRequest { status };
        let _: serde_json::Value = self
            .client
            .patch(&format!("{}/{}", self.base_path(), config_id), &request)
            .await?;
        Ok(())
    }
}
