mod basic;
mod chain;
mod oauth;
mod pat;

pub use basic::Basic;
pub use chain::Chain;
pub use oauth::OAuthM2M;
pub use pat::Pat;

use crate::error::Error;

/// Credential provider for Databricks API authentication.
///
/// Implementations return a list of HTTP headers to attach to each request.
#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    async fn authorize(&self) -> Result<Vec<(String, String)>, Error>;
    fn auth_type(&self) -> &str;
}
