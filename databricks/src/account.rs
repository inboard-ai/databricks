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
}
