use crate::query_history_types::ListQueriesResponse;
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/history/queries";

pub struct QueryHistory {
    client: Client,
}

impl QueryHistory {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List query history.
    pub async fn list(
        &self,
        max_results: Option<i32>,
        page_token: Option<&str>,
        include_metrics: Option<bool>,
    ) -> Result<ListQueriesResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(mr) = max_results {
            query.push(("max_results", mr.to_string()));
        }
        if let Some(pt) = page_token {
            query.push(("page_token", pt.to_string()));
        }
        if let Some(im) = include_metrics {
            query.push(("include_metrics", im.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(PATH, &pairs).await
    }
}
