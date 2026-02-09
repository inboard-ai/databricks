use databricks_core::config;
use databricks_core::error::Error;
use databricks_core::Client as CoreClient;

/// High-level client for account-level Databricks APIs.
///
/// Account-level APIs use a different base URL pattern:
/// `https://accounts.cloud.databricks.com/api/2.0/accounts/{account_id}/...`
pub struct Client {
    inner: CoreClient,
    account_id: String,
}

impl Client {
    /// Create an account client from default configuration.
    pub fn new() -> Result<Self, Error> {
        let config = config::Builder::default().build()?;
        Self::with_config(config)
    }

    /// Create an account client from an explicit configuration.
    pub fn with_config(config: config::Config) -> Result<Self, Error> {
        let account_id = config
            .account_id
            .clone()
            .ok_or_else(|| Error::Config("account_id is required for account-level APIs".into()))?;

        let inner = databricks_core::Builder::from_config(config)?;
        Ok(Self { inner, account_id })
    }

    /// The account ID this client is configured for.
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Access the underlying core client.
    pub fn core_client(&self) -> &CoreClient {
        &self.inner
    }

    // Billing services

    pub fn budgets(&self) -> databricks_billing::Budgets {
        databricks_billing::Budgets::new(self.inner.clone(), &self.account_id)
    }

    pub fn usage(&self) -> databricks_billing::Usage {
        databricks_billing::Usage::new(self.inner.clone(), &self.account_id)
    }

    // Provisioning services

    pub fn workspaces(&self) -> databricks_provisioning::Workspaces {
        databricks_provisioning::Workspaces::new(self.inner.clone(), &self.account_id)
    }

    pub fn credentials(&self) -> databricks_provisioning::Credentials {
        databricks_provisioning::Credentials::new(self.inner.clone(), &self.account_id)
    }

    pub fn encryption_keys(&self) -> databricks_provisioning::EncryptionKeys {
        databricks_provisioning::EncryptionKeys::new(self.inner.clone(), &self.account_id)
    }

    pub fn networks(&self) -> databricks_provisioning::Networks {
        databricks_provisioning::Networks::new(self.inner.clone(), &self.account_id)
    }

    pub fn private_access(&self) -> databricks_provisioning::PrivateAccess {
        databricks_provisioning::PrivateAccess::new(self.inner.clone(), &self.account_id)
    }

    pub fn storage(&self) -> databricks_provisioning::Storage {
        databricks_provisioning::Storage::new(self.inner.clone(), &self.account_id)
    }

    pub fn vpc_endpoints(&self) -> databricks_provisioning::VpcEndpoints {
        databricks_provisioning::VpcEndpoints::new(self.inner.clone(), &self.account_id)
    }

    // Settings services (account-level)

    pub fn network_connectivity(&self) -> databricks_settings::NetworkConnectivity {
        databricks_settings::NetworkConnectivity::new(self.inner.clone(), &self.account_id)
    }

    pub fn account_ip_access_lists(&self) -> databricks_settings::AccountIpAccessLists {
        databricks_settings::AccountIpAccessLists::new(self.inner.clone(), &self.account_id)
    }

    // IAM services (account-level)

    pub fn workspace_assignment(&self) -> databricks_iam::WorkspaceAssignment {
        databricks_iam::WorkspaceAssignment::new(self.inner.clone(), &self.account_id)
    }

    pub fn account_access_control(&self) -> databricks_iam::AccountAccessControl {
        databricks_iam::AccountAccessControl::new(self.inner.clone(), &self.account_id)
    }
}
