use crate::query_types::{CreateQueryRequest, ListQueryObjectsResponse, Query, UpdateQueryRequest};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/queries";

pub struct Queries {
    client: Client,
}

impl Queries {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a query.
    pub async fn create(&self, request: &CreateQueryRequest) -> Result<Query, Error> {
        self.client.post(PATH, request).await
    }

    /// Get a query by ID.
    pub async fn get(&self, id: &str) -> Result<Query, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    /// List queries.
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<ListQueryObjectsResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ps) = page_size {
            query.push(("page_size", ps.to_string()));
        }
        if let Some(pt) = page_token {
            query.push(("page_token", pt.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(PATH, &pairs).await
    }

    /// Update a query.
    pub async fn update(&self, id: &str, request: &UpdateQueryRequest) -> Result<Query, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.patch(&path, request).await
    }

    /// Trash (delete) a query.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }
}
