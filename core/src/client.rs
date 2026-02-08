use crate::error::{ApiError, Error};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Method, Request};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

type HttpClient =
    HyperClient<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, Full<Bytes>>;

struct Inner {
    http: HttpClient,
    host: String,
    token: String,
}

/// Shared Databricks HTTP client, cheap to clone.
///
/// Wraps `Arc<Inner>` so callers never deal with `Arc` directly.
#[derive(Clone)]
pub struct Client(Arc<Inner>);

impl Client {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request(Method::GET, path, Option::<&()>::None).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request(Method::POST, path, Option::<&()>::None).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request(Method::DELETE, path, Option::<&()>::None)
            .await
    }

    pub async fn delete_empty(&self, path: &str) -> Result<(), Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);

        let req = Request::builder()
            .method(Method::DELETE)
            .uri(&uri)
            .header("Authorization", format!("Bearer {}", self.0.token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(Full::new(Bytes::new()))?;

        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            if let Ok(api_error) = serde_json::from_slice::<ApiError>(&body) {
                return Err(api_error.into());
            }
            return Err(Error::Other(format!(
                "HTTP {}: {}",
                status,
                String::from_utf8_lossy(&body)
            )));
        }

        Ok(())
    }

    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, Error> {
        let query_string: String = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let uri = if query_string.is_empty() {
            format!("{}{}", self.0.host.trim_end_matches('/'), path)
        } else {
            format!(
                "{}{}?{}",
                self.0.host.trim_end_matches('/'),
                path,
                query_string
            )
        };

        let req = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .header("Authorization", format!("Bearer {}", self.0.token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(Full::new(Bytes::new()))?;

        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            if let Ok(api_error) = serde_json::from_slice::<ApiError>(&body) {
                return Err(api_error.into());
            }
            return Err(Error::Other(format!(
                "HTTP {}: {}",
                status,
                String::from_utf8_lossy(&body)
            )));
        }

        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>, Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);

        let req = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .header("Authorization", format!("Bearer {}", self.0.token))
            .body(Full::new(Bytes::new()))?;

        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            if let Ok(api_error) = serde_json::from_slice::<ApiError>(&body) {
                return Err(api_error.into());
            }
            return Err(Error::Other(format!(
                "HTTP {}: {}",
                status,
                String::from_utf8_lossy(&body)
            )));
        }

        Ok(body.to_vec())
    }

    async fn request<B: Serialize, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);

        let body_bytes = match body {
            Some(b) => Bytes::from(serde_json::to_vec(b)?),
            None => Bytes::new(),
        };

        let req = Request::builder()
            .method(method)
            .uri(&uri)
            .header("Authorization", format!("Bearer {}", self.0.token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(Full::new(body_bytes))?;

        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            if let Ok(api_error) = serde_json::from_slice::<ApiError>(&body) {
                return Err(api_error.into());
            }
            return Err(Error::Other(format!(
                "HTTP {}: {}",
                status,
                String::from_utf8_lossy(&body)
            )));
        }

        Ok(serde_json::from_slice(&body)?)
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("host", &self.0.host)
            .finish()
    }
}

#[derive(Default)]
pub struct Builder {
    host: Option<String>,
    token: Option<String>,
}

impl Builder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let host = self
            .host
            .ok_or_else(|| Error::Other("host is required".into()))?;
        let token = self
            .token
            .ok_or_else(|| Error::Other("token is required".into()))?;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        let http = HyperClient::builder(TokioExecutor::new()).build(https);

        Ok(Client(Arc::new(Inner { http, host, token })))
    }
}
