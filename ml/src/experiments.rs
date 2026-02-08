use crate::types::{
    CreateExperiment, CreateExperimentResponse, DeleteExperimentRequest, EmptyResponse,
    GetExperimentResponse, ListExperimentsResponse, RestoreExperimentRequest,
    SearchExperimentsRequest, SearchExperimentsResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/mlflow/experiments";

pub struct Experiments {
    client: Client,
}

impl Experiments {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create an experiment with a given name.
    pub async fn create(
        &self,
        request: &CreateExperiment,
    ) -> Result<CreateExperimentResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    /// Get metadata for an experiment by ID.
    pub async fn get(&self, experiment_id: &str) -> Result<GetExperimentResponse, Error> {
        self.client
            .get_with_query(
                &format!("{}/get", PATH),
                &[("experiment_id", experiment_id)],
            )
            .await
    }

    /// Get metadata for an experiment by name.
    pub async fn get_by_name(&self, experiment_name: &str) -> Result<GetExperimentResponse, Error> {
        self.client
            .get_with_query(
                &format!("{}/get-by-name", PATH),
                &[("experiment_name", experiment_name)],
            )
            .await
    }

    /// List all experiments, with optional pagination via query params.
    pub async fn list(&self) -> Result<ListExperimentsResponse, Error> {
        self.client.get(&format!("{}/list", PATH)).await
    }

    /// Mark an experiment for deletion.
    pub async fn delete(&self, experiment_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &DeleteExperimentRequest {
                    experiment_id: experiment_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    /// Restore a deleted experiment.
    pub async fn restore(&self, experiment_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/restore", PATH),
                &RestoreExperimentRequest {
                    experiment_id: experiment_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    /// Search for experiments matching a filter.
    pub async fn search(
        &self,
        request: &SearchExperimentsRequest,
    ) -> Result<SearchExperimentsResponse, Error> {
        self.client.post(&format!("{}/search", PATH), request).await
    }
}
