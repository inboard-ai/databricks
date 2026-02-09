use crate::types::{CreateTableConstraint, TableConstraint};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/constraints";

pub struct TableConstraints {
    client: Client,
}

impl TableConstraints {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateTableConstraint) -> Result<TableConstraint, Error> {
        self.client.post(PATH, request).await
    }

    /// Delete a table constraint.
    /// `full_name` is the full name of the table, `constraint_name` is the constraint to delete,
    /// `cascade` indicates whether to cascade the delete.
    pub async fn delete(
        &self,
        full_name: &str,
        constraint_name: &str,
        cascade: bool,
    ) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, full_name);
        let cascade_str = if cascade { "true" } else { "false" };
        // The delete endpoint uses query parameters for constraint_name and cascade
        let full_path = format!(
            "{}?constraint_name={}&cascade={}",
            path, constraint_name, cascade_str
        );
        self.client.delete_empty(&full_path).await
    }
}
