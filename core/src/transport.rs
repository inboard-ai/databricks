#[cfg(feature = "hyper")]
pub mod hyper;

/// Raw HTTP transport.
///
/// Implementations handle the actual network call.
/// [`crate::Client`] handles auth headers, serialization, retries, and error
/// parsing on top.
#[async_trait::async_trait]
pub trait Http: Send + Sync {
    async fn request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&[u8]>,
    ) -> Result<Response, Error>;
}

/// Raw HTTP response from a transport.
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Transport-level error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("connection failed: {0}")]
    Connection(String),

    #[error("timeout: {0}")]
    Timeout(String),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Whether this transport error is worth retrying.
    pub fn is_retryable(&self) -> bool {
        matches!(self, Error::Connection(_) | Error::Timeout(_))
    }
}
