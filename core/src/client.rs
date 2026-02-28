use crate::auth;
use crate::error::{ApiError, Error};
use crate::retry;
use crate::transport;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

struct Inner {
    transport: Box<dyn transport::Http>,
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
            .execute(|| self.request("GET", &path, Option::<&()>::None))
            .await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = serde_json::to_vec(body)?;
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw("POST", &path, Some(&body_bytes)))
            .await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request("POST", &path, Option::<&()>::None))
            .await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = serde_json::to_vec(body)?;
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw("PUT", &path, Some(&body_bytes)))
            .await
    }

    pub async fn patch<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let body_bytes = serde_json::to_vec(body)?;
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request_raw("PATCH", &path, Some(&body_bytes)))
            .await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.request("DELETE", &path, Option::<&()>::None))
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
        let url = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let mut headers = self.0.credentials.authorize().await?;
        headers.push(("Content-Type".into(), "application/json".into()));
        headers.push(("Accept".into(), "application/json".into()));

        let resp = self
            .0
            .transport
            .request("DELETE", &url, &headers, None)
            .await?;

        if resp.status < 200 || resp.status >= 300 {
            return Err(parse_error_response(resp.status, &resp.body));
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
        let url = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let mut headers = self.0.credentials.authorize().await?;
        headers.push(("Content-Type".into(), "application/json".into()));
        headers.push(("Accept".into(), "application/json".into()));

        let resp = self
            .0
            .transport
            .request("GET", &url, &headers, None)
            .await?;

        if resp.status < 200 || resp.status >= 300 {
            return Err(parse_error_response(resp.status, &resp.body));
        }

        Ok(serde_json::from_slice(&resp.body)?)
    }

    pub async fn get_bytes(&self, path: &str) -> Result<Vec<u8>, Error> {
        let path = path.to_string();
        self.0
            .retry_policy
            .execute(|| self.do_get_bytes(&path))
            .await
    }

    async fn do_get_bytes(&self, path: &str) -> Result<Vec<u8>, Error> {
        let url = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let headers = self.0.credentials.authorize().await?;

        let resp = self
            .0
            .transport
            .request("GET", &url, &headers, None)
            .await?;

        if resp.status < 200 || resp.status >= 300 {
            return Err(parse_error_response(resp.status, &resp.body));
        }

        Ok(resp.body)
    }

    async fn request<B: Serialize, T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, Error> {
        let body_bytes = match body {
            Some(b) => Some(serde_json::to_vec(b)?),
            None => None,
        };

        self.request_raw(method, path, body_bytes.as_deref()).await
    }

    async fn request_raw<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.0.host.trim_end_matches('/'), path);
        let mut headers = self.0.credentials.authorize().await?;
        headers.push(("Content-Type".into(), "application/json".into()));
        headers.push(("Accept".into(), "application/json".into()));

        let resp = self
            .0
            .transport
            .request(method, &url, &headers, body)
            .await?;

        if resp.status < 200 || resp.status >= 300 {
            return Err(parse_error_response(resp.status, &resp.body));
        }

        Ok(serde_json::from_slice(&resp.body)?)
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
    transport: Option<Box<dyn transport::Http>>,
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

    /// Set a custom HTTP transport.
    ///
    /// If not set, defaults to [`transport::hyper::HyperTransport`] when the
    /// `hyper` feature is enabled.
    pub fn transport(mut self, transport: impl transport::Http + 'static) -> Self {
        self.transport = Some(Box::new(transport));
        self
    }

    /// Build from a resolved `config::Config`, using the credential chain.
    pub fn from_config(config: crate::config::Config) -> Result<Client, Error> {
        let host = config
            .host
            .clone()
            .ok_or_else(|| Error::Config("host is required".into()))?;

        let credentials = auth::Chain::from_config(&config)?;

        Builder::default()
            .host(host)
            .credentials(credentials)
            .build()
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

        let transport: Box<dyn transport::Http> = match self.transport {
            Some(t) => t,
            None => Self::default_transport()?,
        };

        Ok(Client(Arc::new(Inner {
            transport,
            host,
            credentials,
            retry_policy: self.retry_policy.unwrap_or_default(),
        })))
    }

    #[cfg(feature = "hyper")]
    fn default_transport() -> Result<Box<dyn transport::Http>, Error> {
        Ok(Box::new(transport::hyper::HyperTransport::new()))
    }

    #[cfg(not(feature = "hyper"))]
    fn default_transport() -> Result<Box<dyn transport::Http>, Error> {
        Err(Error::Config(
            "no transport provided; enable the \"hyper\" feature or call .transport()".into(),
        ))
    }
}
