use crate::types::{
    ClusterPolicyPermissions, ClusterPolicyPermissionsRequest, CreatePolicy, CreatePolicyResponse,
    EditPolicy, EmptyResponse, GetClusterPolicyPermissionLevelsResponse, ListPoliciesResponse,
    Policy, PolicyId,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/policies/clusters";

pub struct ClusterPolicies {
    client: Client,
}

impl ClusterPolicies {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreatePolicy) -> Result<CreatePolicyResponse, Error> {
        self.client.post(&format!("{}/create", PATH), request).await
    }

    pub async fn get(&self, policy_id: &str) -> Result<Policy, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("policy_id", policy_id)])
            .await
    }

    pub async fn list(&self) -> Result<Vec<Policy>, Error> {
        let response: ListPoliciesResponse = self.client.get(&format!("{}/list", PATH)).await?;
        Ok(response.policies)
    }

    pub async fn edit(&self, request: &EditPolicy) -> Result<(), Error> {
        let _: EmptyResponse = self.client.post(&format!("{}/edit", PATH), request).await?;
        Ok(())
    }

    pub async fn delete(&self, policy_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &PolicyId {
                    policy_id: policy_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    // Permissions

    pub async fn get_permissions(
        &self,
        cluster_policy_id: &str,
    ) -> Result<ClusterPolicyPermissions, Error> {
        let path = format!(
            "/api/2.0/permissions/cluster-policies/{}",
            cluster_policy_id
        );
        self.client.get(&path).await
    }

    pub async fn get_permission_levels(
        &self,
        cluster_policy_id: &str,
    ) -> Result<GetClusterPolicyPermissionLevelsResponse, Error> {
        let path = format!(
            "/api/2.0/permissions/cluster-policies/{}/permissionLevels",
            cluster_policy_id
        );
        self.client.get(&path).await
    }

    pub async fn set_permissions(
        &self,
        cluster_policy_id: &str,
        request: &ClusterPolicyPermissionsRequest,
    ) -> Result<ClusterPolicyPermissions, Error> {
        let path = format!(
            "/api/2.0/permissions/cluster-policies/{}",
            cluster_policy_id
        );
        self.client.put(&path, request).await
    }

    pub async fn update_permissions(
        &self,
        cluster_policy_id: &str,
        request: &ClusterPolicyPermissionsRequest,
    ) -> Result<ClusterPolicyPermissions, Error> {
        let path = format!(
            "/api/2.0/permissions/cluster-policies/{}",
            cluster_policy_id
        );
        self.client.patch(&path, request).await
    }
}
