use crate::config;
use crate::error::Error;

/// Default credential chain that tries authentication strategies in order.
///
/// Resolution order:
/// 1. Explicit credentials (if provided)
/// 2. PAT from environment (`DATABRICKS_TOKEN`)
/// 3. OAuth M2M from environment (`DATABRICKS_CLIENT_ID` + `DATABRICKS_CLIENT_SECRET`)
/// 4. PAT from config file
/// 5. OAuth M2M from config file
pub struct Chain {
    provider: Box<dyn super::Provider>,
}

impl Chain {
    /// Build the credential chain from a resolved configuration.
    pub fn from_config(config: &config::Config) -> Result<Self, Error> {
        // Try PAT (token)
        if let Some(token) = &config.token {
            return Ok(Self {
                provider: Box::new(super::Pat::new(token)),
            });
        }

        // Try OAuth M2M (client credentials) — requires hyper for token exchange
        #[cfg(feature = "hyper")]
        if let (Some(client_id), Some(client_secret)) = (&config.client_id, &config.client_secret) {
            let host = config
                .host
                .as_deref()
                .ok_or_else(|| Error::Config("host is required for OAuth M2M".into()))?;
            let token_endpoint = format!("{}/oidc/v1/token", host.trim_end_matches('/'));
            return Ok(Self {
                provider: Box::new(super::OAuthM2M::new(
                    client_id,
                    client_secret,
                    token_endpoint,
                )),
            });
        }

        Err(Error::Config(
            "no authentication credentials found: set DATABRICKS_TOKEN, \
             or DATABRICKS_CLIENT_ID + DATABRICKS_CLIENT_SECRET, \
             or configure them in ~/.databrickscfg"
                .into(),
        ))
    }
}

#[async_trait::async_trait]
impl super::Provider for Chain {
    async fn authorize(&self) -> Result<Vec<(String, String)>, Error> {
        self.provider.authorize().await
    }

    fn auth_type(&self) -> &str {
        self.provider.auth_type()
    }
}
