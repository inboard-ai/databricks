use crate::types::{
    CreateTokenRequest, CreateTokenResponse, EmptyResponse, ListTokensResponse, RevokeTokenId,
    TokenInfo,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/token-management";
const PERSONAL_PATH: &str = "/api/2.0/token";

pub struct Tokens {
    client: Client,
}

impl Tokens {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a personal access token for the current user.
    pub async fn create(&self, request: &CreateTokenRequest) -> Result<CreateTokenResponse, Error> {
        self.client
            .post(&format!("{}/create", PERSONAL_PATH), request)
            .await
    }

    /// List all tokens managed by the token management API.
    pub async fn list(&self) -> Result<Vec<TokenInfo>, Error> {
        let resp: ListTokensResponse = self.client.get(&format!("{}/tokens", PATH)).await?;
        Ok(resp.token_infos)
    }

    /// Delete (revoke) a token by ID.
    pub async fn delete(&self, token_id: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PERSONAL_PATH),
                &RevokeTokenId {
                    token_id: token_id.to_string(),
                },
            )
            .await?;
        Ok(())
    }
}
