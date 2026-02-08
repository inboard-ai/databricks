use crate::types::{
    AllClusterLibraryStatuses, ClusterLibraryStatuses, EmptyResponse, InstallLibraries,
    UninstallLibraries,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/libraries";

pub struct Libraries {
    client: Client,
}

impl Libraries {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn install(&self, request: &InstallLibraries) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/install", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn uninstall(&self, request: &UninstallLibraries) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/uninstall", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn cluster_status(&self, cluster_id: &str) -> Result<ClusterLibraryStatuses, Error> {
        self.client
            .get_with_query(
                &format!("{}/cluster-status", PATH),
                &[("cluster_id", cluster_id)],
            )
            .await
    }

    pub async fn all_cluster_statuses(&self) -> Result<AllClusterLibraryStatuses, Error> {
        self.client
            .get(&format!("{}/all-cluster-statuses", PATH))
            .await
    }
}
