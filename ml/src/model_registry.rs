use crate::types::{
    ApproveTransitionRequestRequest, ApproveTransitionRequestResponse, CreateCommentRequest,
    CreateCommentResponse, CreateModelRequest, CreateModelResponse, CreateModelVersionRequest,
    CreateModelVersionResponse, CreateRegistryWebhookRequest, CreateTransitionRequestRequest,
    CreateTransitionRequestResponse, CreateWebhookResponse, DeleteModelTagRequest,
    DeleteModelVersionTagRequest, DeleteTransitionRequestResponse, EmptyResponse,
    GetLatestVersionsRequest, GetLatestVersionsResponse, GetModelResponse,
    GetModelVersionDownloadUriResponse, GetModelVersionResponse,
    GetRegisteredModelPermissionLevelsResponse, ListModelsResponse, ListRegistryWebhooksResponse,
    ListTransitionRequestsResponse, RegisteredModelPermissions, RegisteredModelPermissionsRequest,
    RejectTransitionRequestRequest, RejectTransitionRequestResponse, RenameModelRequest,
    RenameModelResponse, SearchModelVersionsResponse, SearchModelsResponse, SetModelTagRequest,
    SetModelVersionTagRequest, TestRegistryWebhookRequest, TestRegistryWebhookResponse,
    TransitionModelVersionStageDatabricksRequest, TransitionStageResponse, UpdateCommentRequest,
    UpdateCommentResponse, UpdateModelRequest, UpdateModelResponse, UpdateModelVersionRequest,
    UpdateModelVersionResponse, UpdateRegistryWebhookRequest, UpdateWebhookResponse,
};
use databricks_core::{Client, Error};

/// The Workspace Model Registry service provides APIs for managing the full
/// lifecycle of MLflow Models in the workspace-level registry.
pub struct ModelRegistry {
    client: Client,
}

impl ModelRegistry {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    // ========================================================================
    // Model CRUD
    // ========================================================================

    /// Create a new registered model.
    pub async fn create_model(
        &self,
        request: &CreateModelRequest,
    ) -> Result<CreateModelResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/registered-models/create", request)
            .await
    }

    /// Get the details of a model (Databricks workspace variant).
    pub async fn get_model(&self, name: &str) -> Result<GetModelResponse, Error> {
        self.client
            .get_with_query(
                "/api/2.0/mlflow/databricks/registered-models/get",
                &[("name", name)],
            )
            .await
    }

    /// List all available registered models.
    pub async fn list_models(
        &self,
        max_results: Option<i64>,
        page_token: Option<&str>,
    ) -> Result<ListModelsResponse, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(m) = max_results {
            params.push(("max_results", m.to_string()));
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/api/2.0/mlflow/registered-models/list", &query)
            .await
    }

    /// Search for registered models matching a filter.
    pub async fn search_models(
        &self,
        filter: Option<&str>,
        max_results: Option<i64>,
        order_by: Option<&[&str]>,
        page_token: Option<&str>,
    ) -> Result<SearchModelsResponse, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(f) = filter {
            params.push(("filter", f.to_string()));
        }
        if let Some(m) = max_results {
            params.push(("max_results", m.to_string()));
        }
        if let Some(ob) = order_by {
            for o in ob {
                params.push(("order_by", o.to_string()));
            }
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/api/2.0/mlflow/registered-models/search", &query)
            .await
    }

    /// Delete a registered model.
    pub async fn delete_model(&self, name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "/api/2.0/mlflow/registered-models/delete?name={}",
                name
            ))
            .await?;
        Ok(())
    }

    /// Rename a registered model.
    pub async fn rename_model(
        &self,
        request: &RenameModelRequest,
    ) -> Result<RenameModelResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/registered-models/rename", request)
            .await
    }

    /// Update a registered model.
    pub async fn update_model(
        &self,
        request: &UpdateModelRequest,
    ) -> Result<UpdateModelResponse, Error> {
        self.client
            .patch("/api/2.0/mlflow/registered-models/update", request)
            .await
    }

    // ========================================================================
    // Model version CRUD
    // ========================================================================

    /// Create a model version.
    pub async fn create_model_version(
        &self,
        request: &CreateModelVersionRequest,
    ) -> Result<CreateModelVersionResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/model-versions/create", request)
            .await
    }

    /// Get a model version.
    pub async fn get_model_version(
        &self,
        name: &str,
        version: &str,
    ) -> Result<GetModelVersionResponse, Error> {
        self.client
            .get_with_query(
                "/api/2.0/mlflow/model-versions/get",
                &[("name", name), ("version", version)],
            )
            .await
    }

    /// Delete a model version.
    pub async fn delete_model_version(&self, name: &str, version: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "/api/2.0/mlflow/model-versions/delete?name={}&version={}",
                name, version
            ))
            .await?;
        Ok(())
    }

    /// Search for model versions matching a filter.
    pub async fn search_model_versions(
        &self,
        filter: Option<&str>,
        max_results: Option<i64>,
        order_by: Option<&[&str]>,
        page_token: Option<&str>,
    ) -> Result<SearchModelVersionsResponse, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(f) = filter {
            params.push(("filter", f.to_string()));
        }
        if let Some(m) = max_results {
            params.push(("max_results", m.to_string()));
        }
        if let Some(ob) = order_by {
            for o in ob {
                params.push(("order_by", o.to_string()));
            }
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/api/2.0/mlflow/model-versions/search", &query)
            .await
    }

    /// Update a model version (e.g. description).
    pub async fn update_model_version(
        &self,
        request: &UpdateModelVersionRequest,
    ) -> Result<UpdateModelVersionResponse, Error> {
        self.client
            .patch("/api/2.0/mlflow/model-versions/update", request)
            .await
    }

    /// Get the latest version of a registered model.
    pub async fn get_latest_versions(
        &self,
        request: &GetLatestVersionsRequest,
    ) -> Result<GetLatestVersionsResponse, Error> {
        self.client
            .post(
                "/api/2.0/mlflow/registered-models/get-latest-versions",
                request,
            )
            .await
    }

    /// Get a URI to download the model version artifacts.
    pub async fn get_model_version_download_uri(
        &self,
        name: &str,
        version: &str,
    ) -> Result<GetModelVersionDownloadUriResponse, Error> {
        self.client
            .get_with_query(
                "/api/2.0/mlflow/model-versions/get-download-uri",
                &[("name", name), ("version", version)],
            )
            .await
    }

    // ========================================================================
    // Model / model version tags
    // ========================================================================

    /// Set a tag on a registered model.
    pub async fn set_model_tag(&self, request: &SetModelTagRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/registered-models/set-tag", request)
            .await?;
        Ok(())
    }

    /// Delete a tag on a registered model.
    pub async fn delete_model_tag(&self, request: &DeleteModelTagRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "/api/2.0/mlflow/registered-models/delete-tag?name={}&key={}",
                request.name, request.key
            ))
            .await?;
        Ok(())
    }

    /// Set a tag on a model version.
    pub async fn set_model_version_tag(
        &self,
        request: &SetModelVersionTagRequest,
    ) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post("/api/2.0/mlflow/model-versions/set-tag", request)
            .await?;
        Ok(())
    }

    /// Delete a tag on a model version.
    pub async fn delete_model_version_tag(
        &self,
        request: &DeleteModelVersionTagRequest,
    ) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "/api/2.0/mlflow/model-versions/delete-tag?name={}&version={}&key={}",
                request.name, request.version, request.key
            ))
            .await?;
        Ok(())
    }

    // ========================================================================
    // Stage transitions
    // ========================================================================

    /// Transition a model version's stage (Databricks workspace variant).
    pub async fn transition_stage(
        &self,
        request: &TransitionModelVersionStageDatabricksRequest,
    ) -> Result<TransitionStageResponse, Error> {
        self.client
            .post(
                "/api/2.0/mlflow/databricks/model-versions/transition-stage",
                request,
            )
            .await
    }

    /// Create a model version stage transition request.
    pub async fn create_transition_request(
        &self,
        request: &CreateTransitionRequestRequest,
    ) -> Result<CreateTransitionRequestResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/transition-requests/create", request)
            .await
    }

    /// Approve a model version stage transition request.
    pub async fn approve_transition_request(
        &self,
        request: &ApproveTransitionRequestRequest,
    ) -> Result<ApproveTransitionRequestResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/transition-requests/approve", request)
            .await
    }

    /// Reject a model version stage transition request.
    pub async fn reject_transition_request(
        &self,
        request: &RejectTransitionRequestRequest,
    ) -> Result<RejectTransitionRequestResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/transition-requests/reject", request)
            .await
    }

    /// Cancel (delete) a model version stage transition request.
    pub async fn delete_transition_request(
        &self,
        name: &str,
        version: &str,
        stage: &str,
        creator: &str,
        comment: Option<&str>,
    ) -> Result<DeleteTransitionRequestResponse, Error> {
        let mut params = format!(
            "/api/2.0/mlflow/transition-requests/delete?name={}&version={}&stage={}&creator={}",
            name, version, stage, creator
        );
        if let Some(c) = comment {
            params.push_str(&format!("&comment={}", c));
        }
        self.client.delete(&params).await
    }

    /// List all open stage transition requests for a model version.
    pub async fn list_transition_requests(
        &self,
        name: &str,
        version: &str,
    ) -> Result<ListTransitionRequestsResponse, Error> {
        self.client
            .get_with_query(
                "/api/2.0/mlflow/transition-requests/list",
                &[("name", name), ("version", version)],
            )
            .await
    }

    // ========================================================================
    // Comments
    // ========================================================================

    /// Post a comment on a model version.
    pub async fn create_comment(
        &self,
        request: &CreateCommentRequest,
    ) -> Result<CreateCommentResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/comments/create", request)
            .await
    }

    /// Edit a comment on a model version.
    pub async fn update_comment(
        &self,
        request: &UpdateCommentRequest,
    ) -> Result<UpdateCommentResponse, Error> {
        self.client
            .patch("/api/2.0/mlflow/comments/update", request)
            .await
    }

    /// Delete a comment on a model version.
    pub async fn delete_comment(&self, id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!("/api/2.0/mlflow/comments/delete?id={}", id))
            .await?;
        Ok(())
    }

    // ========================================================================
    // Webhooks
    // ========================================================================

    /// Create a registry webhook.
    pub async fn create_webhook(
        &self,
        request: &CreateRegistryWebhookRequest,
    ) -> Result<CreateWebhookResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/registry-webhooks/create", request)
            .await
    }

    /// List all registry webhooks.
    pub async fn list_webhooks(
        &self,
        model_name: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<ListRegistryWebhooksResponse, Error> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(n) = model_name {
            params.push(("model_name", n.to_string()));
        }
        if let Some(t) = page_token {
            params.push(("page_token", t.to_string()));
        }
        let query: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/api/2.0/mlflow/registry-webhooks/list", &query)
            .await
    }

    /// Test a registry webhook.
    pub async fn test_registry_webhook(
        &self,
        request: &TestRegistryWebhookRequest,
    ) -> Result<TestRegistryWebhookResponse, Error> {
        self.client
            .post("/api/2.0/mlflow/registry-webhooks/test", request)
            .await
    }

    /// Update a registry webhook.
    pub async fn update_webhook(
        &self,
        request: &UpdateRegistryWebhookRequest,
    ) -> Result<UpdateWebhookResponse, Error> {
        self.client
            .patch("/api/2.0/mlflow/registry-webhooks/update", request)
            .await
    }

    /// Delete a registry webhook.
    pub async fn delete_webhook(&self, id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .delete(&format!(
                "/api/2.0/mlflow/registry-webhooks/delete?id={}",
                id
            ))
            .await?;
        Ok(())
    }

    // ========================================================================
    // Permissions
    // ========================================================================

    /// Get the permissions of a registered model.
    pub async fn get_permissions(
        &self,
        registered_model_id: &str,
    ) -> Result<RegisteredModelPermissions, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/registered-models/{}",
                registered_model_id
            ))
            .await
    }

    /// Get the permission levels that a user can have on a registered model.
    pub async fn get_permission_levels(
        &self,
        registered_model_id: &str,
    ) -> Result<GetRegisteredModelPermissionLevelsResponse, Error> {
        self.client
            .get(&format!(
                "/api/2.0/permissions/registered-models/{}/permissionLevels",
                registered_model_id
            ))
            .await
    }

    /// Set permissions on a registered model, replacing existing permissions.
    pub async fn set_permissions(
        &self,
        registered_model_id: &str,
        request: &RegisteredModelPermissionsRequest,
    ) -> Result<RegisteredModelPermissions, Error> {
        self.client
            .put(
                &format!(
                    "/api/2.0/permissions/registered-models/{}",
                    registered_model_id
                ),
                request,
            )
            .await
    }

    /// Update the permissions on a registered model.
    pub async fn update_permissions(
        &self,
        registered_model_id: &str,
        request: &RegisteredModelPermissionsRequest,
    ) -> Result<RegisteredModelPermissions, Error> {
        self.client
            .patch(
                &format!(
                    "/api/2.0/permissions/registered-models/{}",
                    registered_model_id
                ),
                request,
            )
            .await
    }
}
