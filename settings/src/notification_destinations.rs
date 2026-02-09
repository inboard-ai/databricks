use crate::types::{
    CreateNotificationDestinationRequest, ListNotificationDestinationsResponse,
    NotificationDestination, UpdateNotificationDestinationRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/notification-destinations";

pub struct NotificationDestinations {
    client: Client,
}

impl NotificationDestinations {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a notification destination.
    pub async fn create(
        &self,
        request: &CreateNotificationDestinationRequest,
    ) -> Result<NotificationDestination, Error> {
        self.client.post(PATH, request).await
    }

    /// Get a notification destination by ID.
    pub async fn get(&self, id: &str) -> Result<NotificationDestination, Error> {
        self.client.get(&format!("{}/{}", PATH, id)).await
    }

    /// List notification destinations.
    pub async fn list(&self) -> Result<ListNotificationDestinationsResponse, Error> {
        self.client.get(PATH).await
    }

    /// Update a notification destination.
    pub async fn update(
        &self,
        request: &UpdateNotificationDestinationRequest,
    ) -> Result<NotificationDestination, Error> {
        self.client
            .patch(&format!("{}/{}", PATH, request.id), request)
            .await
    }

    /// Delete a notification destination by ID.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client.delete_empty(&format!("{}/{}", PATH, id)).await
    }
}
