use crate::types::{
    CreateInstancePool, CreateInstancePoolResponse, EditInstancePool, EmptyResponse, InstancePool,
    ListInstancePoolsResponse, PoolId,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/instance-pools";

pub struct InstancePools {
    client: Client,
}

impl InstancePools {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateInstancePool,
    ) -> Result<CreateInstancePoolResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    pub async fn get(&self, instance_pool_id: &str) -> Result<InstancePool, Error> {
        self.client
            .get_with_query(
                &format!("{}/get", PATH),
                &[("instance_pool_id", instance_pool_id)],
            )
            .await
    }

    pub async fn list(&self) -> Result<Vec<InstancePool>, Error> {
        let response: ListInstancePoolsResponse =
            self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.instance_pools)
    }

    pub async fn edit(&self, request: &EditInstancePool) -> Result<(), Error> {
        let _: EmptyResponse = self.client.post(&format!("{}/edit", PATH), request).await?;
        Ok(())
    }

    pub async fn delete(&self, instance_pool_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &PoolId {
                    instance_pool_id: instance_pool_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }
}
