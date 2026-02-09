use crate::types::{
    CreateInstancePool, CreateInstancePoolResponse, EditInstancePool, EmptyResponse, InstancePool,
    ListInstancePoolsResponse, PoolId,
};
use databricks_core::{Client, Error};
use databricks_iam::{ObjectPermissions, PermissionLevels, SetPermissions, UpdatePermissions};

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

    // Permissions

    pub async fn get_permissions(
        &self,
        instance_pool_id: &str,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("/api/2.0/permissions/instance-pools/{}", instance_pool_id);
        self.client.get(&path).await
    }

    pub async fn get_permission_levels(
        &self,
        instance_pool_id: &str,
    ) -> Result<PermissionLevels, Error> {
        let path = format!(
            "/api/2.0/permissions/instance-pools/{}/permissionLevels",
            instance_pool_id
        );
        self.client.get(&path).await
    }

    pub async fn set_permissions(
        &self,
        instance_pool_id: &str,
        request: &SetPermissions,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("/api/2.0/permissions/instance-pools/{}", instance_pool_id);
        self.client.put(&path, request).await
    }

    pub async fn update_permissions(
        &self,
        instance_pool_id: &str,
        request: &UpdatePermissions,
    ) -> Result<ObjectPermissions, Error> {
        let path = format!("/api/2.0/permissions/instance-pools/{}", instance_pool_id);
        self.client.patch(&path, request).await
    }
}
