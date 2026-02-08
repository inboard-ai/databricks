use crate::auth;
use crate::error::{ApiError, Error};
use crate::retry;
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
    credentials: Box<dyn auth::Provider>,
    retry_policy: retry::Policy,
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

    pub fn host(&self) -> &str {
        &self.0.host
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request(Method::GET, &path, Option::<&()>::None))
            .await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = Bytes::from(serde_json::to_vec(body)?);
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw(Method::POST, &path, body_bytes.clone()))
            .await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request(Method::POST, &path, Option::<&()>::None))
            .await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = Bytes::from(serde_json::to_vec(body)?);
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw(Method::PUT, &path, body_bytes.clone()))
            .await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = Bytes::from(serde_json::to_vec(body)?);
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw(Method::PATCH, &path, body_bytes.clone()))
            .await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request(Method::DELETE, &path, Option::<&()>::None))
            .await
    }

    pub async fn delete_empty(&self, path: &str) -> Result<(), Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.do_delete_empty(&path))
            .await
    }

    async fn do_delete_empty(&self, path: &str) -> Result<(), Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let auth_headers = self.0.credentials.authorize().await?;

        let mut builder = Request::builder()
            .method(Method::DELETE)
            .uri(&uri)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        for (name, value) in &auth_headers {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let req = builder.body(Full::new(Bytes::new()))?;
        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            return Err(parse_error_response(status.as_u16(), &body));
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

        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{}?{}", path, query_string)
        };

        self.0
            .retry_policy
            .execute(|| self.do_get_raw(&full_path))
            .await
    }

    async fn do_get_raw<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let auth_headers = self.0.credentials.authorize().await?;

        let mut builder = Request::builder()
            .method(Method::GET)
            .uri(&uri)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        for (name, value) in &auth_headers {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let req = builder.body(Full::new(Bytes::new()))?;
        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            return Err(parse_error_response(status.as_u16(), &body));
        }

        Ok(serde_json::from_slice(&body)?)
    }

    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.do_get_bytes(&path))
            .await
    }

    async fn do_get_bytes(&self, path: &str) -> Result<Vec<u8>, Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let auth_headers = self.0.credentials.authorize().await?;

        let mut builder = Request::builder().method(Method::GET).uri(&uri);

        for (name, value) in &auth_headers {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let req = builder.body(Full::new(Bytes::new()))?;
        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            return Err(parse_error_response(status.as_u16(), &body));
        }

        Ok(body.to_vec())
    }

    async fn request<B: Serialize, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, Error> {
        let body_bytes = match body {
            Some(b) => Bytes::from(serde_json::to_vec(b)?),
            None => Bytes::new(),
        };

        self.request_raw(method, path, body_bytes).await
    }

    async fn request_raw<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body_bytes: Bytes,
    ) -> Result<T, Error> {
        let uri = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let auth_headers = self.0.credentials.authorize().await?;

        let mut builder = Request::builder()
            .method(method)
            .uri(&uri)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        for (name, value) in &auth_headers {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let req = builder.body(Full::new(body_bytes))?;
        let response = self.0.http.request(req).await?;
        let status = response.status();
        let body = response.into_body().collect().await?.to_bytes();

        if !status.is_success() {
            return Err(parse_error_response(status.as_u16(), &body));
        }

        Ok(serde_json::from_slice(&body)?)
    }
}

fn parse_error_response(status: u16, body: &[u8]) -> Error {
    let retry_after_secs = if status == 429 { Some(1) } else { None };

    if let Ok(api_error) = serde_json::from_slice::<ApiError>(body) {
        return api_error.into_error(status, retry_after_secs);
    }

    Error::Api {
        code: crate::error::Code::from_status(status),
        status,
        message: String::from_utf8_lossy(body).into_owned(),
        retry_after_secs,
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
    credentials: Option<Box<dyn auth::Provider>>,
    retry_policy: Option<retry::Policy>,
}

impl Builder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Set a PAT token directly. This wraps the token in `auth::Pat`.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set an explicit credential provider.
    pub fn credentials(mut self, provider: impl auth::Provider + 'static) -> Self {
        self.credentials = Some(Box::new(provider));
        self
    }

    pub fn retry_policy(mut self, policy: retry::Policy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Build from a resolved `config::Config`, using the credential chain.
    pub fn from_config(config: crate::config::Config) -> Result<Client, Error> {
        let host = config
            .host
            .clone()
            .ok_or_else(|| Error::Config("host is required".into()))?;

        let credentials = auth::Chain::from_config(&config)?;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        let http = HyperClient::builder(TokioExecutor::new()).build(https);

        Ok(Client(Arc::new(Inner {
            http,
            host,
            credentials: Box::new(credentials),
            retry_policy: retry::Policy::default(),
        })))
    }

    pub fn build(self) -> Result<Client, Error> {
        let host = self
            .host
            .ok_or_else(|| Error::Config("host is required".into()))?;

        let credentials: Box<dyn auth::Provider> = if let Some(creds) = self.credentials {
            creds
        } else if let Some(token) = self.token {
            Box::new(auth::Pat::new(token))
        } else {
            return Err(Error::Config("credentials or token is required".into()));
        };

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();

        let http = HyperClient::builder(TokioExecutor::new()).build(https);

        Ok(Client(Arc::new(Inner {
            http,
            host,
            credentials,
            retry_policy: self.retry_policy.unwrap_or_default(),
        })))
    }
}
