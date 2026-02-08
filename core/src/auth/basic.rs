use crate::error::Error;
use base64::Engine;

/// HTTP Basic authentication.
pub struct Basic {
    username: String,
    password: String,
}

impl Basic {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[async_trait::async_trait]
impl super::Provider for Basic {
    async fn authorize(&self) -> Result<Vec<(String, String)>, Error> {
        let encoded = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", self.username, self.password));
        Ok(vec![(
            "Authorization".to_string(),
            format!("Basic {}", encoded),
        )])
    }

    fn auth_type(&self) -> &str {
        "basic"
    }
}
