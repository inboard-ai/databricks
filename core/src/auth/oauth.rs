use crate::error::Error;
use base64::Engine;
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// OAuth Machine-to-Machine (Client Credentials) authentication.
pub struct OAuthM2M {
    client_id: String,
    client_secret: String,
    token_endpoint: String,
    cache: RwLock<Option<CachedToken>>,
}

struct CachedToken {
    access_token: String,
    expires_at: Instant,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default = "default_expires_in")]
    expires_in: u64,
}

fn default_expires_in() -> u64 {
    3600
}

impl OAuthM2M {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        token_endpoint: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token_endpoint: token_endpoint.into(),
            cache: RwLock::new(None),
        }
    }

    async fn fetch_token(&self) -> Result<CachedToken, Error> {
        use http_body_util::{BodyExt, Full};
        use hyper::body::Bytes;
        use hyper::Request;
        use hyper_util::client::legacy::Client;
        use hyper_util::rt::TokioExecutor;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(TokioExecutor::new()).build(https);

        let credentials = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", self.client_id, self.client_secret));

        let body = "grant_type=client_credentials";

        let req = Request::builder()
            .method(hyper::Method::POST)
            .uri(&self.token_endpoint)
            .header("Authorization", format!("Basic {}", credentials))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Full::new(Bytes::from(body)))
            .map_err(|e| Error::Other(format!("Failed to build OAuth request: {}", e)))?;

        let response = client
            .request(req)
            .await
            .map_err(|e| Error::Other(format!("OAuth token request failed: {}", e)))?;

        let status = response.status();
        let body_bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| Error::Other(format!("Failed to read OAuth response: {}", e)))?
            .to_bytes();

        if !status.is_success() {
            return Err(Error::Other(format!(
                "OAuth token request failed with HTTP {}: {}",
                status,
                String::from_utf8_lossy(&body_bytes)
            )));
        }

        let token_response: TokenResponse = serde_json::from_slice(&body_bytes)?;

        // Refresh 30 seconds before expiry
        let expires_at =
            Instant::now() + Duration::from_secs(token_response.expires_in.saturating_sub(30));

        Ok(CachedToken {
            access_token: token_response.access_token,
            expires_at,
        })
    }
}

#[async_trait::async_trait]
impl super::Provider for OAuthM2M {
    async fn authorize(&self) -> Result<Vec<(String, String)>, Error> {
        // Check cache with read lock
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if Instant::now() < cached.expires_at {
                    return Ok(vec![(
                        "Authorization".to_string(),
                        format!("Bearer {}", cached.access_token),
                    )]);
                }
            }
        }

        // Refresh token with write lock
        let mut cache = self.cache.write().await;
        // Double-check after acquiring write lock
        if let Some(cached) = cache.as_ref() {
            if Instant::now() < cached.expires_at {
                return Ok(vec![(
                    "Authorization".to_string(),
                    format!("Bearer {}", cached.access_token),
                )]);
            }
        }

        let token = self.fetch_token().await?;
        let header_value = format!("Bearer {}", token.access_token);
        *cache = Some(token);

        Ok(vec![("Authorization".to_string(), header_value)])
    }

    fn auth_type(&self) -> &str {
        "oauth-m2m"
    }
}
