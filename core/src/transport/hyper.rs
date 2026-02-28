use super::{Error, Response};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;

type HttpClient = HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>;

/// HTTP transport backed by hyper + rustls.
pub struct HyperTransport {
    client: HttpClient,
}

impl HyperTransport {
    pub fn new() -> Self {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        let client = HyperClient::builder(TokioExecutor::new()).build(https);
        Self { client }
    }
}

impl Default for HyperTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Http for HyperTransport {
    async fn request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: Option<&[u8]>,
    ) -> Result<Response, Error> {
        let method = hyper::Method::from_bytes(method.as_bytes())
            .map_err(|e| Error::Other(e.to_string()))?;

        let mut builder = hyper::Request::builder().method(method).uri(url);

        for (name, value) in headers {
            builder = builder.header(name.as_str(), value.as_str());
        }

        let body_bytes = match body {
            Some(b) => Bytes::copy_from_slice(b),
            None => Bytes::new(),
        };
        let req = builder.body(Full::new(body_bytes))?;

        let response = self.client.request(req).await?;
        let status = response.status().as_u16();
        let body = response.into_body().collect().await?.to_bytes().to_vec();

        Ok(Response { status, body })
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        if e.is_timeout() {
            Error::Timeout(e.to_string())
        } else {
            Error::Connection(e.to_string())
        }
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::Connection(e.to_string())
    }
}

impl From<hyper::http::Error> for Error {
    fn from(e: hyper::http::Error) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(e: hyper::http::uri::InvalidUri) -> Self {
        Error::InvalidUrl(e.to_string())
    }
}
