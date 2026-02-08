use crate::types::{
    CreateRecipient, EmptyResponse, ListRecipientsResponse, RecipientInfo, UpdateRecipient,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/recipients";

pub struct Recipients {
    client: Client,
}

impl Recipients {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateRecipient) -> Result<RecipientInfo, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, name: &str) -> Result<RecipientInfo, Error> {
        self.client.get(&format!("{}/{}", PATH, name)).await
    }

    pub async fn list(&self) -> Result<Vec<RecipientInfo>, Error> {
        let response: ListRecipientsResponse = self.client.get(PATH).await?;
        Ok(response.recipients)
    }

    pub async fn update(
        &self,
        name: &str,
        request: &UpdateRecipient,
    ) -> Result<RecipientInfo, Error> {
        self.client
            .patch(&format!("{}/{}", PATH, name), request)
            .await
    }

    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self.client.delete(&format!("{}/{}", PATH, name)).await?;
        Ok(())
    }
}
