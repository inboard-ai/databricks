use crate::types::{
    CreateNetworkConnectivityConfigRequest, CreatePrivateEndpointRuleRequest,
    ListNetworkConnectivityConfigurationsResponse, ListPrivateEndpointRulesResponse,
    NccPrivateEndpointRule, NetworkConnectivityConfiguration,
};
use databricks_core::{Client, Error};

pub struct NetworkConnectivity {
    client: Client,
    account_id: String,
}

impl NetworkConnectivity {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!(
            "/api/2.0/accounts/{}/network-connectivity-configs",
            self.account_id
        )
    }

    /// Create a network connectivity configuration.
    pub async fn create_network_connectivity_configuration(
        &self,
        request: &CreateNetworkConnectivityConfigRequest,
    ) -> Result<NetworkConnectivityConfiguration, Error> {
        self.client.post(&self.base_path(), request).await
    }

    /// Get a network connectivity configuration by ID.
    pub async fn get_network_connectivity_configuration(
        &self,
        network_connectivity_config_id: &str,
    ) -> Result<NetworkConnectivityConfiguration, Error> {
        self.client
            .get(&format!(
                "{}/{}",
                self.base_path(),
                network_connectivity_config_id
            ))
            .await
    }

    /// List all network connectivity configurations.
    pub async fn list_network_connectivity_configurations(
        &self,
    ) -> Result<ListNetworkConnectivityConfigurationsResponse, Error> {
        self.client.get(&self.base_path()).await
    }

    /// Delete a network connectivity configuration by ID.
    pub async fn delete_network_connectivity_configuration(
        &self,
        network_connectivity_config_id: &str,
    ) -> Result<(), Error> {
        self.client
            .delete_empty(&format!(
                "{}/{}",
                self.base_path(),
                network_connectivity_config_id
            ))
            .await
    }

    /// Create a private endpoint rule for a network connectivity configuration.
    pub async fn create_private_endpoint_rule(
        &self,
        network_connectivity_config_id: &str,
        request: &CreatePrivateEndpointRuleRequest,
    ) -> Result<NccPrivateEndpointRule, Error> {
        self.client
            .post(
                &format!(
                    "{}/{}/private-endpoint-rules",
                    self.base_path(),
                    network_connectivity_config_id
                ),
                request,
            )
            .await
    }

    /// Get a private endpoint rule.
    pub async fn get_private_endpoint_rule(
        &self,
        network_connectivity_config_id: &str,
        private_endpoint_rule_id: &str,
    ) -> Result<NccPrivateEndpointRule, Error> {
        self.client
            .get(&format!(
                "{}/{}/private-endpoint-rules/{}",
                self.base_path(),
                network_connectivity_config_id,
                private_endpoint_rule_id
            ))
            .await
    }

    /// List private endpoint rules for a network connectivity configuration.
    pub async fn list_private_endpoint_rules(
        &self,
        network_connectivity_config_id: &str,
    ) -> Result<ListPrivateEndpointRulesResponse, Error> {
        self.client
            .get(&format!(
                "{}/{}/private-endpoint-rules",
                self.base_path(),
                network_connectivity_config_id
            ))
            .await
    }

    /// Delete a private endpoint rule.
    pub async fn delete_private_endpoint_rule(
        &self,
        network_connectivity_config_id: &str,
        private_endpoint_rule_id: &str,
    ) -> Result<NccPrivateEndpointRule, Error> {
        self.client
            .delete(&format!(
                "{}/{}/private-endpoint-rules/{}",
                self.base_path(),
                network_connectivity_config_id,
                private_endpoint_rule_id
            ))
            .await
    }
}
