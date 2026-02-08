use crate::types::{
    CreateJob, CreateJobResponse, EmptyResponse, Job, JobId, ListJobsResponse, RunNow,
    RunNowResponse, UpdateJob,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/jobs";

pub struct Jobs {
    client: Client,
}

impl Jobs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateJob) -> Result<CreateJobResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    pub async fn get(&self, job_id: i64) -> Result<Job, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("job_id", &job_id.to_string())])
            .await
    }

    pub async fn list(&self) -> Result<Vec<Job>, Error> {
        let response: ListJobsResponse = self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.jobs)
    }

    pub async fn update(&self, request: &UpdateJob) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/update", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, job_id: i64) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/delete", PATH), &JobId { job_id })
            .await?;
        Ok(())
    }

    pub async fn run_now(&self, request: &RunNow) -> Result<RunNowResponse, Error> {
        self.client
            .post(&format!("{}/run-now", PATH), request)
            .await
    }
}
