use crate::types::{
    CreateOboTokenRequest, CreateOboTokenResponse, GetTokenPermissionLevelsResponse,
    GetTokenResponse, ListTokensResponse, TokenInfo, TokenPermissions, TokenPermissionsRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/token-management";
const PERMISSIONS_PATH: &str = "/api/2.0/permissions/authorization/tokens";

pub struct TokenManagement {
    client: Client,
}

impl TokenManagement {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a token on behalf of a service principal.
    pub async fn create_obo_token(
        &self,
        request: &CreateOboTokenRequest,
    ) -> Result<CreateOboTokenResponse, Error> {
        self.client
            .post(&format!("{}/on-behalf-of/tokens", PATH), request)
            .await
    }

    /// Get information about a token by its ID.
    pub async fn get(&self, token_id: &str) -> Result<TokenInfo, Error> {
        let resp: GetTokenResponse = self
            .client
            .get(&format!("{}/tokens/{}", PATH, token_id))
            .await?;
        resp.token_info
            .ok_or_else(|| Error::Other("Missing token_info in response".into()))
    }

    /// List all tokens associated with the workspace or user.
    pub async fn list(&self) -> Result<Vec<TokenInfo>, Error> {
        let resp: ListTokensResponse = self.client.get(&format!("{}/tokens", PATH)).await?;
        Ok(resp.token_infos)
    }

    /// Delete a token by its ID.
    pub async fn delete(&self, token_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/tokens/{}", PATH, token_id))
            .await
    }

    /// Get the permissions of all tokens.
    pub async fn get_permissions(&self) -> Result<TokenPermissions, Error> {
        self.client.get(PERMISSIONS_PATH).await
    }

    /// Get permission levels that a user can have on tokens.
    pub async fn get_permission_levels(&self) -> Result<GetTokenPermissionLevelsResponse, Error> {
        self.client
            .get(&format!("{}/permissionLevels", PERMISSIONS_PATH))
            .await
    }

    /// Set permissions on all tokens, replacing existing permissions.
    pub async fn set_permissions(
        &self,
        request: &TokenPermissionsRequest,
    ) -> Result<TokenPermissions, Error> {
        self.client.put(PERMISSIONS_PATH, request).await
    }

    /// Update permissions on all tokens.
    pub async fn update_permissions(
        &self,
        request: &TokenPermissionsRequest,
    ) -> Result<TokenPermissions, Error> {
        self.client.patch(PERMISSIONS_PATH, request).await
    }
}
