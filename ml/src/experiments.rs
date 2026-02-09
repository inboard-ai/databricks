use crate::types::{
    CreateExperiment, CreateExperimentResponse, DeleteExperimentRequest, DeleteRunsRequest,
    DeleteRunsResponse, EmptyResponse, ExperimentPermissions, ExperimentPermissionsRequest,
    GetExperimentPermissionLevelsResponse, GetExperimentResponse, GetMetricHistoryResponse,
    ListExperimentsResponse, LogInputsRequest, LogModelRequest, LogOutputsRequest,
    RestoreExperimentRequest, RestoreRunsRequest, RestoreRunsResponse, SearchExperimentsRequest,
    SearchExperimentsResponse, SetExperimentTagRequest, UpdateExperimentRequest,
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

    /// Update experiment metadata (e.g. rename).
    pub async fn update_experiment(&self, request: &UpdateExperimentRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/update", PATH), request)
            .await?;
        Ok(())
    }

    /// Set a tag on an experiment.
    pub async fn set_experiment_tag(&self, request: &SetExperimentTagRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/set-experiment-tag", PATH), request)
            .await?;
        Ok(())
    }

    /// Get a list of all values for the specified metric for a given run.
    pub async fn get_history(
        &self,
        metric_key: &str,
        run_id: &str,
        max_results: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<GetMetricHistoryResponse, Error> {
        let mut params: Vec<(&str, String)> = vec![
            ("metric_key", metric_key.to_string()),
            ("run_id", run_id.to_string()),
        ];
        if let Some(m) = max_results {
            params.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/api/2.0/mlflow/metrics/get-history", &query)
            .await
    }

    /// Log a model to an MLflow Run.
    pub async fn log_model(&self, request: &LogModelRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/log-model", request)
            .await?;
        Ok(())
    }

    /// Log inputs (datasets) to an MLflow Run.
    pub async fn log_inputs(&self, request: &LogInputsRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/log-inputs", request)
            .await?;
        Ok(())
    }

    /// Log outputs (models) from an MLflow Run.
    pub async fn log_outputs(&self, request: &LogOutputsRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/outputs", request)
            .await?;
        Ok(())
    }

    /// Bulk delete runs in an experiment created prior to or at the specified timestamp.
    pub async fn delete_runs(
        &self,
        request: &DeleteRunsRequest,
    ) -> Result<DeleteRunsResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/databricks/runs/delete-runs", request)
            .await
    }

    /// Bulk restore runs in an experiment deleted no earlier than the specified timestamp.
    pub async fn restore_runs(
        &self,
        request: &RestoreRunsRequest,
    ) -> Result<RestoreRunsResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/databricks/runs/restore-runs", request)
            .await
    }

    /// Get the permissions of an experiment.
    pub async fn get_permissions(
        &self,
        experiment_id: &str,
    ) -> Result<ExperimentPermissions, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/experiments/{}",
                experiment_id
            ))
            .await
    }

    /// Get the permission levels that a user can have on an experiment.
    pub async fn get_permission_levels(
        &self,
        experiment_id: &str,
    ) -> Result<GetExperimentPermissionLevelsResponse, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/experiments/{}/permissionLevels",
                experiment_id
            ))
            .await
    }

    /// Set permissions on an experiment, replacing existing permissions.
    pub async fn set_permissions(
        &self,
        experiment_id: &str,
        request: &ExperimentPermissionsRequest,
    ) -> Result<ExperimentPermissions, Error> {
        self.client
            .put(
                &format!("/api/2.0/permissions/experiments/{}", experiment_id),
                request,
            )
            .await
    }

    /// Update the permissions on an experiment.
    pub async fn update_permissions(
        &self,
        experiment_id: &str,
        request: &ExperimentPermissionsRequest,
    ) -> Result<ExperimentPermissions, Error> {
        self.client
            .patch(
                &format!("/api/2.0/permissions/experiments/{}", experiment_id),
                request,
            )
            .await
    }
}
