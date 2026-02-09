use crate::types::{EmptyResponse, RetrieveTokenResponse};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/public/data_sharing_activation_info";

pub struct RecipientActivation {
    client: Client,
}

impl RecipientActivation {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get activation URL info for a recipient.
    pub async fn get_activation_url_info(&self, activation_url: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .get_with_query(PATH, &[("activation_url", activation_url)])
            .await?;
        Ok(())
    }

    /// Retrieve token for a recipient activation.
    pub async fn retrieve_token(
        &self,
        activation_url: &str,
    ) -> Result<RetrieveTokenResponse, Error> {
        self.client
            .get(&format!("{}/{}/retrieve-token", PATH, activation_url))
            .await
    }
}
