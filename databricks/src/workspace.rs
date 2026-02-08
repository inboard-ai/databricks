use databricks_core::config;
use databricks_core::error::Error;
use databricks_core::Client as CoreClient;

/// High-level client for workspace-level Databricks APIs.
///
/// Provides typed accessors for each service area.
pub struct Client {
    inner: CoreClient,
}

impl Client {
    /// Create a workspace client from default configuration
    /// (environment variables, `~/.databrickscfg`, etc.).
    pub fn new() -> Result<Self, Error> {
        let config = config::Builder::default().build()?;
        Self::with_config(config)
    }

    /// Create a workspace client from an explicit configuration.
    pub fn with_config(config: config::Config) -> Result<Self, Error> {
        let inner = databricks_core::Builder::from_config(config)?;
        Ok(Self { inner })
    }

    /// Access the underlying core client (for advanced/custom usage).
    pub fn core_client(&self) -> &CoreClient {
        &self.inner
    }

    // SQL services

    pub fn statements(&self) -> databricks_sql::Statements {
        databricks_sql::Statements::new(self.inner.clone())
    }

    pub fn warehouses(&self) -> databricks_sql::Warehouses {
        databricks_sql::Warehouses::new(self.inner.clone())
    }

    pub fn alerts(&self) -> databricks_sql::Alerts {
        databricks_sql::Alerts::new(self.inner.clone())
    }

    pub fn queries(&self) -> databricks_sql::Queries {
        databricks_sql::Queries::new(self.inner.clone())
    }

    pub fn query_history(&self) -> databricks_sql::QueryHistory {
        databricks_sql::QueryHistory::new(self.inner.clone())
    }

    // Genie services

    pub fn genie_spaces(&self) -> databricks_genie::Spaces {
        databricks_genie::Spaces::new(self.inner.clone())
    }

    pub fn genie_conversations(
        &self,
        space_id: impl Into<String>,
    ) -> databricks_genie::Conversations {
        databricks_genie::Conversations::new(self.inner.clone(), space_id)
    }

    // Compute services

    pub fn clusters(&self) -> databricks_compute::Clusters {
        databricks_compute::Clusters::new(self.inner.clone())
    }

    pub fn instance_pools(&self) -> databricks_compute::InstancePools {
        databricks_compute::InstancePools::new(self.inner.clone())
    }

    pub fn cluster_policies(&self) -> databricks_compute::ClusterPolicies {
        databricks_compute::ClusterPolicies::new(self.inner.clone())
    }

    pub fn libraries(&self) -> databricks_compute::Libraries {
        databricks_compute::Libraries::new(self.inner.clone())
    }

    // Jobs services

    pub fn jobs(&self) -> databricks_jobs::Jobs {
        databricks_jobs::Jobs::new(self.inner.clone())
    }

    pub fn runs(&self) -> databricks_jobs::Runs {
        databricks_jobs::Runs::new(self.inner.clone())
    }

    // Files services

    pub fn dbfs(&self) -> databricks_files::Dbfs {
        databricks_files::Dbfs::new(self.inner.clone())
    }

    pub fn files(&self) -> databricks_files::Files {
        databricks_files::Files::new(self.inner.clone())
    }

    // Workspace services

    pub fn notebooks(&self) -> databricks_workspace_api::Notebooks {
        databricks_workspace_api::Notebooks::new(self.inner.clone())
    }

    pub fn repos(&self) -> databricks_workspace_api::Repos {
        databricks_workspace_api::Repos::new(self.inner.clone())
    }

    pub fn secrets(&self) -> databricks_workspace_api::Secrets {
        databricks_workspace_api::Secrets::new(self.inner.clone())
    }

    pub fn git_credentials(&self) -> databricks_workspace_api::GitCredentials {
        databricks_workspace_api::GitCredentials::new(self.inner.clone())
    }

    // IAM services

    pub fn users(&self) -> databricks_iam::Users {
        databricks_iam::Users::new(self.inner.clone())
    }

    pub fn groups(&self) -> databricks_iam::Groups {
        databricks_iam::Groups::new(self.inner.clone())
    }

    pub fn service_principals(&self) -> databricks_iam::ServicePrincipals {
        databricks_iam::ServicePrincipals::new(self.inner.clone())
    }

    pub fn permissions(&self) -> databricks_iam::Permissions {
        databricks_iam::Permissions::new(self.inner.clone())
    }

    pub fn current_user(&self) -> databricks_iam::Me {
        databricks_iam::Me::new(self.inner.clone())
    }

    // Catalog services

    pub fn catalogs(&self) -> databricks_catalog::Catalogs {
        databricks_catalog::Catalogs::new(self.inner.clone())
    }

    pub fn schemas(&self) -> databricks_catalog::Schemas {
        databricks_catalog::Schemas::new(self.inner.clone())
    }

    pub fn tables(&self) -> databricks_catalog::Tables {
        databricks_catalog::Tables::new(self.inner.clone())
    }

    pub fn volumes(&self) -> databricks_catalog::Volumes {
        databricks_catalog::Volumes::new(self.inner.clone())
    }

    pub fn grants(&self) -> databricks_catalog::Grants {
        databricks_catalog::Grants::new(self.inner.clone())
    }

    // Serving services

    pub fn serving_endpoints(&self) -> databricks_serving::ServingEndpoints {
        databricks_serving::ServingEndpoints::new(self.inner.clone())
    }

    // Pipelines services

    pub fn pipelines(&self) -> databricks_pipelines::Pipelines {
        databricks_pipelines::Pipelines::new(self.inner.clone())
    }

    // ML services

    pub fn experiments(&self) -> databricks_ml::Experiments {
        databricks_ml::Experiments::new(self.inner.clone())
    }

    pub fn ml_runs(&self) -> databricks_ml::Runs {
        databricks_ml::Runs::new(self.inner.clone())
    }

    pub fn registered_models(&self) -> databricks_ml::RegisteredModels {
        databricks_ml::RegisteredModels::new(self.inner.clone())
    }

    pub fn model_versions(&self) -> databricks_ml::ModelVersions {
        databricks_ml::ModelVersions::new(self.inner.clone())
    }

    // Dashboard services

    pub fn dashboards(&self) -> databricks_dashboards::Dashboards {
        databricks_dashboards::Dashboards::new(self.inner.clone())
    }
}
