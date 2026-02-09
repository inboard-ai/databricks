use crate::types::{
    CancelAllRuns, CreateJob, CreateJobResponse, EmptyResponse, GetJobPermissionLevelsResponse,
    Job, JobId, JobPermissions, JobPermissionsRequest, JobSettings, ListJobsResponse, ResetJob,
    RunId, RunNow, RunNowResponse, UpdateJob,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/jobs";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/jobs";

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

    /// Cancel all runs of a job.
    pub async fn cancel_all_runs(&self, job_id: i64) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/runs/cancel-all", PATH),
                &CancelAllRuns { job_id },
            )
            .await?;
        Ok(())
    }

    /// Delete a run by run_id.
    pub async fn delete_run(&self, run_id: i64) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/runs/delete", PATH), &RunId { run_id })
            .await?;
        Ok(())
    }

    /// Overwrite all settings for a job, resetting it to the provided configuration.
    pub async fn reset(&self, job_id: i64, new_settings: &JobSettings) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/reset", PATH),
                &ResetJob {
                    job_id,
                    new_settings: new_settings.clone(),
                },
            )
            .await?;
        Ok(())
    }

    /// Get the permissions of a job.
    pub async fn get_permissions(&self, job_id: i64) -> Result<JobPermissions, Error> {
        self.client
            .get(&format!("{}/{}", PERMISSIONS_PATH, job_id))
            .await
    }

    /// Get the permission levels that a user can have on a job.
    pub async fn get_permission_levels(
        &self,
        job_id: i64,
    ) -> Result<GetJobPermissionLevelsResponse, Error> {
        self.client
            .get(&format!("{}/{}/permissionLevels", PERMISSIONS_PATH, job_id))
            .await
    }

    /// Set permissions on a job, replacing existing permissions.
    pub async fn set_permissions(
        &self,
        job_id: i64,
        request: &JobPermissionsRequest,
    ) -> Result<JobPermissions, Error> {
        self.client
            .put(&format!("{}/{}", PERMISSIONS_PATH, job_id), request)
            .await
    }

    /// Update the permissions on a job.
    pub async fn update_permissions(
        &self,
        job_id: i64,
        request: &JobPermissionsRequest,
    ) -> Result<JobPermissions, Error> {
        self.client
            .patch(&format!("{}/{}", PERMISSIONS_PATH, job_id), request)
            .await
    }
}
