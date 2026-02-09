use crate::types::{CreateVisualizationRequest, UpdateVisualizationRequest, Visualization};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/sql/visualizations";

pub struct QueryVisualizations {
    client: Client,
}

impl QueryVisualizations {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a visualization on a query.
    pub async fn create(
        &self,
        request: &CreateVisualizationRequest,
    ) -> Result<Visualization, Error> {
        self.client.post(PATH, request).await
    }

    /// Delete a visualization by ID.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }

    /// Update a visualization by ID.
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateVisualizationRequest,
    ) -> Result<Visualization, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.patch(&path, request).await
    }
}
