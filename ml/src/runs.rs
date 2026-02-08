use crate::types::{
    CreateRunRequest, CreateRunResponse, DeleteRunRequest, DeleteTagRequest, EmptyResponse,
    GetRunResponse, ListArtifactsResponse, LogBatchRequest, LogMetricRequest, LogParamRequest,
    RestoreRunRequest, SearchRunsRequest, SearchRunsResponse, SetTagRequest, UpdateRunRequest,
    UpdateRunResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/mlflow/runs";

pub struct Runs {
    client: Client,
}

impl Runs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new run within an experiment.
    pub async fn create(&self, request: &CreateRunRequest) -> Result<CreateRunResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    /// Get metadata, metrics, params, and tags for a run.
    pub async fn get(&self, run_id: &str) -> Result<GetRunResponse, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("run_id", run_id)])
            .await
    }

    /// Update run metadata (status, end time, name).
    pub async fn update(&self, request: &UpdateRunRequest) -> Result<UpdateRunResponse, Error> {
        self.client.post(&format!("{}/update", PATH), request).await
    }

    /// Search for runs matching a filter expression.
    pub async fn search(&self, request: &SearchRunsRequest) -> Result<SearchRunsResponse, Error> {
        self.client.post(&format!("{}/search", PATH), request).await
    }

    /// Mark a run for deletion.
    pub async fn delete(&self, run_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &DeleteRunRequest {
                    run_id: run_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    /// Restore a deleted run.
    pub async fn restore(&self, run_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/restore", PATH),
                &RestoreRunRequest {
                    run_id: run_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    /// Log a metric for a run.
    pub async fn log_metric(&self, request: &LogMetricRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/log-metric", request)
            .await?;
        Ok(())
    }

    /// Log a param for a run.
    pub async fn log_param(&self, request: &LogParamRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/log-parameter", request)
            .await?;
        Ok(())
    }

    /// Log a batch of metrics, params, and tags for a run.
    pub async fn log_batch(&self, request: &LogBatchRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/log-batch", request)
            .await?;
        Ok(())
    }

    /// Set a tag on a run.
    pub async fn set_tag(&self, request: &SetTagRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/set-tag", request)
            .await?;
        Ok(())
    }

    /// Delete a tag on a run.
    pub async fn delete_tag(&self, request: &DeleteTagRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/runs/delete-tag", request)
            .await?;
        Ok(())
    }

    /// List artifacts for a run.
    pub async fn list_artifacts(
        &self,
        run_id: &str,
        path: Option<&str>,
    ) -> Result<ListArtifactsResponse, Error> {
        let mut params = vec![("run_id", run_id)];
        if let Some(p) = path {
            params.push(("path", p));
        }
        self.client
            .get_with_query("/api/2.0/mlflow/artifacts/list", &params)
            .await
    }
}
