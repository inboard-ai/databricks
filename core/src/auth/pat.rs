use crate::error::Error;

/// Personal Access Token authentication.
pub struct Pat {
    token: String,
}

impl Pat {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

#[async_trait::async_trait]
impl super::Provider for Pat {
    async fn authorize(&self) -> Result<Vec<(String, String)>, Error> {
        Ok(vec![(
            "Authorization".to_string(),
            format!("Bearer {}", self.token),
        )])
    }

    fn auth_type(&self) -> &str {
        "pat"
    }
}
