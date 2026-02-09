use crate::types::{
    ChangeClusterOwner, ClusterDetails, ClusterId, ClusterPermissions, ClusterPermissionsRequest,
    ClusterState, CreateCluster, CreateClusterResponse, EditCluster, EmptyResponse,
    GetClusterPermissionLevelsResponse, GetEvents, GetEventsResponse, GetSparkVersionsResponse,
    ListAvailableZonesResponse, ListClustersResponse, ListNodeTypesResponse, ResizeCluster,
    UpdateCluster,
};
use databricks_core::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

const PATH: &str = "/api/2.0/clusters";

pub struct Clusters {
    client: Client,
}

impl Clusters {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateCluster) -> Result<CreateClusterResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    pub async fn get(&self, cluster_id: &str) -> Result<ClusterDetails, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("cluster_id", cluster_id)])
            .await
    }

    pub async fn list(&self) -> Result<Vec<ClusterDetails>, Error> {
        let response: ListClustersResponse = self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.clusters)
    }

    pub async fn edit(&self, request: &EditCluster) -> Result<(), Error> {
        let _: EmptyResponse = self.client.post(&format!("{}/edit", PATH), request).await?;
        Ok(())
    }

    pub async fn delete(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn start(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/start", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn stop(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn restart(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/restart", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn change_owner(&self, request: &ChangeClusterOwner) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/change-owner", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn events(&self, request: &GetEvents) -> Result<GetEventsResponse, Error> {
        self.client.post(&format!("{}/events", PATH), request).await
    }

    pub async fn list_node_types(&self) -> Result<ListNodeTypesResponse, Error> {
        self.client.get(&format!("{}/list-node-types", PATH)).await
    }

    pub async fn list_zones(&self) -> Result<ListAvailableZonesResponse, Error> {
        self.client.get(&format!("{}/list-zones", PATH)).await
    }

    pub async fn permanent_delete(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/permanent-delete", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn pin(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/pin", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn unpin(&self, cluster_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/unpin", PATH),
                &ClusterId {
                    cluster_id: cluster_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn resize(&self, request: &ResizeCluster) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/resize", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn spark_versions(&self) -> Result<GetSparkVersionsResponse, Error> {
        self.client.get(&format!("{}/spark-versions", PATH)).await
    }

    pub async fn update(&self, request: &UpdateCluster) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/update", PATH), request)
            .await?;
        Ok(())
    }

    // Permissions

    pub async fn get_permissions(&self, cluster_id: &str) -> Result<ClusterPermissions, Error> {
        let path = format!("/api/2.0/permissions/clusters/{}", cluster_id);
        self.client.get(&path).await
    }

    pub async fn get_permission_levels(
        &self,
        cluster_id: &str,
    ) -> Result<GetClusterPermissionLevelsResponse, Error> {
        let path = format!(
            "/api/2.0/permissions/clusters/{}/permissionLevels",
            cluster_id
        );
        self.client.get(&path).await
    }

    pub async fn set_permissions(
        &self,
        cluster_id: &str,
        request: &ClusterPermissionsRequest,
    ) -> Result<ClusterPermissions, Error> {
        let path = format!("/api/2.0/permissions/clusters/{}", cluster_id);
        self.client.put(&path, request).await
    }

    pub async fn update_permissions(
        &self,
        cluster_id: &str,
        request: &ClusterPermissionsRequest,
    ) -> Result<ClusterPermissions, Error> {
        let path = format!("/api/2.0/permissions/clusters/{}", cluster_id);
        self.client.patch(&path, request).await
    }

    /// Create and wait until the cluster is running.
    pub async fn create_and_wait(
        &self,
        request: &CreateCluster,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<ClusterDetails, Error> {
        let response = self.create(request).await?;
        self.wait_for_state(
            &response.cluster_id,
            ClusterState::Running,
            poll_interval,
            timeout,
        )
        .await
    }

    /// Start and wait until the cluster is running.
    pub async fn start_and_wait(
        &self,
        cluster_id: &str,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<ClusterDetails, Error> {
        self.start(cluster_id).await?;
        self.wait_for_state(cluster_id, ClusterState::Running, poll_interval, timeout)
            .await
    }

    async fn wait_for_state(
        &self,
        cluster_id: &str,
        target: ClusterState,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<ClusterDetails, Error> {
        let start = std::time::Instant::now();
        loop {
            let details = self.get(cluster_id).await?;
            if let Some(state) = details.state {
                if state == target {
                    return Ok(details);
                }
                if state == ClusterState::Error {
                    return Err(Error::Other(format!(
                        "Cluster entered ERROR state: {}",
                        details.state_message.unwrap_or_default()
                    )));
                }
            }
            if start.elapsed() > timeout {
                return Err(Error::Timeout("Cluster operation timed out".into()));
            }
            sleep(poll_interval).await;
        }
    }
}
