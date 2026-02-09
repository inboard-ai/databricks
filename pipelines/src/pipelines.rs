use crate::types::{
    ClonePipelineRequest, ClonePipelineResponse, CreatePipeline, CreatePipelineResponse,
    EditPipeline, EmptyResponse, GetPipelinePermissionLevelsResponse, GetUpdateResponse,
    ListPipelineEventsResponse, ListPipelinesResponse, ListUpdatesResponse, Pipeline, PipelineId,
    PipelinePermissions, PipelinePermissionsRequest, StartUpdate, StartUpdateResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/pipelines";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/pipelines";

pub struct Pipelines {
    client: Client,
}

impl Pipelines {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Creates a new data processing pipeline. Returns the ID of the new pipeline.
    pub async fn create(&self, request: &CreatePipeline) -> Result<CreatePipelineResponse, Error> {
        self.client.post(PATH, request).await
    }

    /// Gets a pipeline by ID.
    pub async fn get(&self, pipeline_id: &str) -> Result<Pipeline, Error> {
        self.client.get(&format!("{}/{}", PATH, pipeline_id)).await
    }

    /// Lists pipelines defined in the workspace.
    pub async fn list(
        &self,
        filter: Option<&str>,
        max_results: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<ListPipelinesResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(f) = filter {
            query.push(("filter", f.to_string()));
        }
        if let Some(m) = max_results {
            query.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            query.push(("page_token", t.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(PATH, &pairs).await
    }

    /// Updates a pipeline with the supplied configuration.
    pub async fn update(&self, pipeline_id: &str, request: &EditPipeline) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .put(&format!("{}/{}", PATH, pipeline_id), request)
            .await?;
        Ok(())
    }

    /// Deletes a pipeline.
    pub async fn delete(&self, pipeline_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, pipeline_id))
            .await
    }

    /// Starts a new update for the pipeline.
    pub async fn start(
        &self,
        pipeline_id: &str,
        request: &StartUpdate,
    ) -> Result<StartUpdateResponse, Error> {
        self.client
            .post(&format!("{}/{}/updates", PATH, pipeline_id), request)
            .await
    }

    /// Stops the pipeline by canceling the active update.
    pub async fn stop(&self, pipeline_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/{}/stop", PATH, pipeline_id),
                &PipelineId {
                    pipeline_id: pipeline_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    /// Gets an update from an active pipeline.
    pub async fn get_update(
        &self,
        pipeline_id: &str,
        update_id: &str,
    ) -> Result<GetUpdateResponse, Error> {
        self.client
            .get(&format!("{}/{}/updates/{}", PATH, pipeline_id, update_id))
            .await
    }

    /// Lists updates for an active pipeline.
    pub async fn list_updates(
        &self,
        pipeline_id: &str,
        max_results: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<ListUpdatesResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(m) = max_results {
            query.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            query.push(("page_token", t.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("{}/{}/updates", PATH, pipeline_id), &pairs)
            .await
    }

    /// Retrieves events for a pipeline.
    pub async fn list_events(
        &self,
        pipeline_id: &str,
        max_results: Option<i32>,
        page_token: Option<&str>,
        filter: Option<&str>,
    ) -> Result<ListPipelineEventsResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(m) = max_results {
            query.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            query.push(("page_token", t.to_string()));
        }
        if let Some(f) = filter {
            query.push(("filter", f.to_string()));
        }
        let pairs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("{}/{}/events", PATH, pipeline_id), &pairs)
            .await
    }

    /// Clones a pipeline.
    pub async fn clone(
        &self,
        pipeline_id: &str,
        request: &ClonePipelineRequest,
    ) -> Result<ClonePipelineResponse, Error> {
        self.client
            .post(&format!("{}/{}/clone", PATH, pipeline_id), request)
            .await
    }

    /// Get the permissions of a pipeline.
    pub async fn get_permissions(&self, pipeline_id: &str) -> Result<PipelinePermissions, Error> {
        self.client
            .get(&format!("{}/{}", PERMISSIONS_PATH, pipeline_id))
            .await
    }

    /// Get the permission levels that a user can have on a pipeline.
    pub async fn get_permission_levels(
        &self,
        pipeline_id: &str,
    ) -> Result<GetPipelinePermissionLevelsResponse, Error> {
        self.client
            .get(&format!(
                "{}/{}/permissionLevels",
                PERMISSIONS_PATH, pipeline_id
            ))
            .await
    }

    /// Set permissions on a pipeline, replacing existing permissions.
    pub async fn set_permissions(
        &self,
        pipeline_id: &str,
        request: &PipelinePermissionsRequest,
    ) -> Result<PipelinePermissions, Error> {
        self.client
            .put(&format!("{}/{}", PERMISSIONS_PATH, pipeline_id), request)
            .await
    }

    /// Update the permissions on a pipeline.
    pub async fn update_permissions(
        &self,
        pipeline_id: &str,
        request: &PipelinePermissionsRequest,
    ) -> Result<PipelinePermissions, Error> {
        self.client
            .patch(&format!("{}/{}", PERMISSIONS_PATH, pipeline_id), request)
            .await
    }
}
