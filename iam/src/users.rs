use crate::types::{
    EmptyResponse, GetPasswordPermissionLevelsResponse, ListUsersResponse, PasswordPermissions,
    PasswordPermissionsRequest, PatchRequest, User,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/preview/scim/v2/Users";
const PASSWORD_PERMISSIONS_PATH: &str = "/api/2.0/permissions/authorization/passwords";

pub struct Users {
    client: Client,
}

impl Users {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, user: &User) -> Result<User, Error> {
        self.client.post(PATH, user).await
    }

    pub async fn get(&self, id: &str) -> Result<User, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.get(&path).await
    }

    pub async fn list(&self) -> Result<Vec<User>, Error> {
        let response: ListUsersResponse = self.client.get(PATH).await?;
        Ok(response.resources)
    }

    pub async fn update(&self, id: &str, user: &User) -> Result<User, Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.put(&path, user).await
    }

    pub async fn patch(&self, id: &str, request: &PatchRequest) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        let _: EmptyResponse = self.client.patch(&path, request).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let path = format!("{}/{}", PATH, id);
        self.client.delete_empty(&path).await
    }

    /// Get password permissions.
    pub async fn get_permissions(&self) -> Result<PasswordPermissions, Error> {
        self.client.get(PASSWORD_PERMISSIONS_PATH).await
    }

    /// Get password permission levels.
    pub async fn get_permission_levels(&self) -> Result<GetPasswordPermissionLevelsResponse, Error> {
        let path = format!("{}/permissionLevels", PASSWORD_PERMISSIONS_PATH);
        self.client.get(&path).await
    }

    /// Set password permissions (full replacement).
    pub async fn set_permissions(
        &self,
        request: &PasswordPermissionsRequest,
    ) -> Result<PasswordPermissions, Error> {
        self.client.put(PASSWORD_PERMISSIONS_PATH, request).await
    }

    /// Update password permissions (partial update).
    pub async fn update_permissions(
        &self,
        request: &PasswordPermissionsRequest,
    ) -> Result<PasswordPermissions, Error> {
        self.client.patch(PASSWORD_PERMISSIONS_PATH, request).await
    }
}
