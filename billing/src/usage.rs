use crate::types::DownloadUsageRequest;
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/accounts";

pub struct Usage {
    client: Client,
    account_id: String,
}

impl Usage {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("{}/{}/usage", PATH, self.account_id)
    }

    /// Download billable usage as CSV bytes.
    pub async fn download(&self, request: &DownloadUsageRequest) -> Result<Vec<u8>, Error> {
        let mut query: Vec<(&str, &str)> = vec![
            ("start_month", &request.start_month),
            ("end_month", &request.end_month),
        ];
        let personal_str;
        if let Some(personal_data) = request.personal_data {
            personal_str = personal_data.to_string();
            query.push(("personal_data", &personal_str));
        }

        let query_string: String = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        self.client
            .get_bytes(&format!("{}/download?{}", self.base_path(), query_string))
            .await
    }
}
