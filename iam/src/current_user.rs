use crate::types::CurrentUser;
use databricks_core::{Client, Error};

pub struct Me {
    client: Client,
}

impl Me {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn me(&self) -> Result<CurrentUser, Error> {
        self.client.get("/api/2.0/preview/scim/v2/Me").await
    }
}
