use crate::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// SDK configuration resolved from explicit values, environment, and config files.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: Option<String>,
    pub token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub profile: Option<String>,
    pub config_file: Option<PathBuf>,
    pub account_id: Option<String>,
    pub retry_timeout: Option<Duration>,
    pub http_timeout: Option<Duration>,
}

impl Config {
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Resolve configuration from all sources in priority order:
    /// explicit values > env vars > config file.
    pub fn resolve(builder: Builder) -> Result<Self, Error> {
        let profile = builder
            .profile
            .or_else(|| std::env::var("DATABRICKS_CONFIG_PROFILE").ok())
            .unwrap_or_else(|| "DEFAULT".into());

        let config_file = builder.config_file.or_else(|| {
            std::env::var("DATABRICKS_CONFIG_FILE")
                .ok()
                .map(PathBuf::from)
                .or_else(|| dirs::home_dir().map(|h| h.join(".databrickscfg")))
        });

        let file_values = config_file
            .as_ref()
            .and_then(|p| parse_cfg_file(p, &profile));

        let get = |explicit: Option<String>, env_key: &str, file_key: &str| -> Option<String> {
            explicit
                .or_else(|| std::env::var(env_key).ok())
                .or_else(|| file_values.as_ref().and_then(|m| m.get(file_key).cloned()))
        };

        Ok(Config {
            host: get(builder.host, "DATABRICKS_HOST", "host"),
            token: get(builder.token, "DATABRICKS_TOKEN", "token"),
            client_id: get(builder.client_id, "DATABRICKS_CLIENT_ID", "client_id"),
            client_secret: get(
                builder.client_secret,
                "DATABRICKS_CLIENT_SECRET",
                "client_secret",
            ),
            account_id: get(builder.account_id, "DATABRICKS_ACCOUNT_ID", "account_id"),
            profile: Some(profile),
            config_file,
            retry_timeout: builder.retry_timeout,
            http_timeout: builder.http_timeout,
        })
    }
}

#[derive(Default)]
pub struct Builder {
    host: Option<String>,
    token: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    profile: Option<String>,
    config_file: Option<PathBuf>,
    account_id: Option<String>,
    retry_timeout: Option<Duration>,
    http_timeout: Option<Duration>,
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

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    pub fn config_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.config_file = Some(path.into());
        self
    }

    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }

    pub fn retry_timeout(mut self, timeout: Duration) -> Self {
        self.retry_timeout = Some(timeout);
        self
    }

    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.http_timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<Config, Error> {
        Config::resolve(self)
    }
}

/// Parse a `.databrickscfg` INI file, returning values for the given profile.
fn parse_cfg_file(path: &PathBuf, profile: &str) -> Option<HashMap<String, String>> {
    let contents = std::fs::read_to_string(path).ok()?;
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].trim().to_string();
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            sections
                .entry(current_section.clone())
                .or_default()
                .insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    sections.remove(profile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_cfg_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmp,
            "[DEFAULT]\nhost = https://example.cloud.databricks.com\ntoken = dapi123\n\n[staging]\nhost = https://staging.cloud.databricks.com\ntoken = dapi456\n"
        )
        .unwrap();

        let values = parse_cfg_file(&tmp.path().to_path_buf(), "DEFAULT").unwrap();
        assert_eq!(
            values.get("host").unwrap(),
            "https://example.cloud.databricks.com"
        );
        assert_eq!(values.get("token").unwrap(), "dapi123");

        let values = parse_cfg_file(&tmp.path().to_path_buf(), "staging").unwrap();
        assert_eq!(
            values.get("host").unwrap(),
            "https://staging.cloud.databricks.com"
        );
        assert_eq!(values.get("token").unwrap(), "dapi456");
    }

    #[test]
    fn test_missing_profile() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "[DEFAULT]\nhost = https://example.com\n").unwrap();

        let values = parse_cfg_file(&tmp.path().to_path_buf(), "nonexistent");
        assert!(values.is_none());
    }
}
