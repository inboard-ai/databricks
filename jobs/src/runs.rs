use crate::types::{
    EmptyResponse, ListRunsResponse, RepairRun, RepairRunResponse, Run, RunId, RunOutput,
    SubmitRun, SubmitRunResponse,
};
use databricks_core::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

const PATH: &str = "/api/2.1/jobs/runs";

pub struct Runs {
    client: Client,
}

impl Runs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, run_id: i64) -> Result<Run, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("run_id", &run_id.to_string())])
            .await
    }

    pub async fn list(&self) -> Result<Vec<Run>, Error> {
        let response: ListRunsResponse = self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.runs)
    }

    pub async fn list_by_job(&self, job_id: i64) -> Result<Vec<Run>, Error> {
        let response: ListRunsResponse = self
            .client
            .get_with_query(
                &format!("{}/list", PATH),
                &[("job_id", &job_id.to_string())],
            )
            .await?;
        Ok(response.runs)
    }

    pub async fn submit(&self, request: &SubmitRun) -> Result<SubmitRunResponse, Error> {
        self.client.post(&format!("{}/submit", PATH), request).await
    }

    pub async fn cancel(&self, run_id: i64) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/cancel", PATH), &RunId { run_id })
            .await?;
        Ok(())
    }

    pub async fn repair(&self, request: &RepairRun) -> Result<RepairRunResponse, Error> {
        self.client.post(&format!("{}/repair", PATH), request).await
    }

    pub async fn get_output(&self, run_id: i64) -> Result<RunOutput, Error> {
        self.client
            .get_with_query(
                &format!("{}/get-output", PATH),
                &[("run_id", &run_id.to_string())],
            )
            .await
    }

    pub async fn export(&self, run_id: i64) -> Result<Vec<u8>, Error> {
        let path = format!("{}/export?run_id={}", PATH, run_id);
        self.client.get_bytes(&path).await
    }

    /// Submit a run and wait for it to complete.
    pub async fn submit_and_wait(
        &self,
        request: &SubmitRun,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Run, Error> {
        let response = self.submit(request).await?;
        self.wait(response.run_id, poll_interval, timeout).await
    }

    /// Wait for a run to reach a terminal state.
    pub async fn wait(
        &self,
        run_id: i64,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Run, Error> {
        let start = std::time::Instant::now();
        loop {
            let run = self.get(run_id).await?;
            if let Some(state) = &run.state {
                if let Some(lifecycle) = state.life_cycle_state {
                    if lifecycle.is_terminal() {
                        return Ok(run);
                    }
                }
            }
            if start.elapsed() > timeout {
                return Err(Error::Timeout("Run timed out".into()));
            }
            sleep(poll_interval).await;
        }
    }
}
