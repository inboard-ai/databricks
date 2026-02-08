use serde::Deserialize;

/// Structured error code mapped from Databricks API error code strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Code {
    NotFound,
    Unauthenticated,
    PermissionDenied,
    BadRequest,
    AlreadyExists,
    Conflict,
    TooManyRequests,
    Internal,
    Unavailable,
    Unknown(String),
}

impl Code {
    /// Map from Databricks API error code strings.
    pub fn parse(s: &str) -> Self {
        match s {
            "RESOURCE_DOES_NOT_EXIST" | "NOT_FOUND" | "RESOURCE_NOT_FOUND" => Code::NotFound,
            "UNAUTHENTICATED" | "INVALID_AUTHENTICATION" => Code::Unauthenticated,
            "PERMISSION_DENIED" | "FORBIDDEN" => Code::PermissionDenied,
            "BAD_REQUEST" | "INVALID_PARAMETER_VALUE" | "MALFORMED_REQUEST" => Code::BadRequest,
            "RESOURCE_ALREADY_EXISTS" => Code::AlreadyExists,
            "RESOURCE_CONFLICT" | "ABORTED" => Code::Conflict,
            "TOO_MANY_REQUESTS" | "REQUEST_LIMIT_EXCEEDED" | "RATE_LIMITED" => {
                Code::TooManyRequests
            }
            "INTERNAL_ERROR" | "TEMPORARILY_UNAVAILABLE" => Code::Internal,
            "SERVICE_UNAVAILABLE" | "DEADLINE_EXCEEDED" => Code::Unavailable,
            other => Code::Unknown(other.to_string()),
        }
    }

    /// Map from HTTP status code when no API error body is available.
    pub fn from_status(status: u16) -> Self {
        match status {
            401 => Code::Unauthenticated,
            403 => Code::PermissionDenied,
            404 => Code::NotFound,
            400 => Code::BadRequest,
            409 => Code::Conflict,
            429 => Code::TooManyRequests,
            500 => Code::Internal,
            503 => Code::Unavailable,
            _ => Code::Unknown(format!("HTTP_{}", status)),
        }
    }

    /// Whether this error code represents a retryable condition.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Code::TooManyRequests | Code::Internal | Code::Unavailable
        )
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::NotFound => write!(f, "NOT_FOUND"),
            Code::Unauthenticated => write!(f, "UNAUTHENTICATED"),
            Code::PermissionDenied => write!(f, "PERMISSION_DENIED"),
            Code::BadRequest => write!(f, "BAD_REQUEST"),
            Code::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            Code::Conflict => write!(f, "CONFLICT"),
            Code::TooManyRequests => write!(f, "TOO_MANY_REQUESTS"),
            Code::Internal => write!(f, "INTERNAL"),
            Code::Unavailable => write!(f, "UNAVAILABLE"),
            Code::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] hyper::Error),

    #[error("HTTP client error: {0}")]
    Client(#[from] hyper_util::client::legacy::Error),

    #[error("API error ({code}, HTTP {status}): {message}")]
    Api {
        code: Code,
        status: u16,
        message: String,
        retry_after_secs: Option<u64>,
    },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URI: {0}")]
    Uri(#[from] hyper::http::uri::InvalidUri),

    #[error("HTTP error: {0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            Error::Api {
                code: Code::NotFound,
                ..
            }
        )
    }

    pub fn is_unauthenticated(&self) -> bool {
        matches!(
            self,
            Error::Api {
                code: Code::Unauthenticated,
                ..
            }
        )
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Api { code, status, .. } => {
                code.is_retryable() || *status == 429 || *status == 503
            }
            Error::Client(_) => true,
            _ => false,
        }
    }

    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn retry_after_secs(&self) -> Option<u64> {
        match self {
            Error::Api {
                retry_after_secs, ..
            } => *retry_after_secs,
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error_code: String,
    pub message: String,
}

impl ApiError {
    pub fn into_error(self, status: u16, retry_after_secs: Option<u64>) -> Error {
        Error::Api {
            code: Code::parse(&self.error_code),
            status,
            message: self.message,
            retry_after_secs,
        }
    }
}
