use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] hyper::Error),

    #[error("HTTP client error: {0}")]
    Client(#[from] hyper_util::client::legacy::Error),

    #[error("API error: {code} - {message}")]
    Api { code: String, message: String },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URI: {0}")]
    Uri(#[from] hyper::http::uri::InvalidUri),

    #[error("HTTP error: {0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error_code: String,
    pub message: String,
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::Api {
            code: e.error_code,
            message: e.message,
        }
    }
}
