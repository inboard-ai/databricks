use crate::types::{
    ClusterDetails, ClusterId, ClusterState, CreateCluster, CreateClusterResponse, EditCluster,
    EmptyResponse, ListClustersResponse,
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
