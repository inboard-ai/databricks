use crate::types::{
    CreateIpAccessListRequest, CreateIpAccessListResponse, EmptyResponse, GetIpAccessListResponse,
    IpAccessList, ListIpAccessListsResponse, ReplaceIpAccessListRequest, UpdateIpAccessListRequest,
};
use databricks_core::{Client, Error};

pub struct AccountIpAccessLists {
    client: Client,
    account_id: String,
}

impl AccountIpAccessLists {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    fn base_path(&self) -> String {
        format!("/api/2.0/accounts/{}/ip-access-lists", self.account_id)
    }

    /// Create an IP access list for the account.
    pub async fn create(
        &self,
        request: &CreateIpAccessListRequest,
    ) -> Result<CreateIpAccessListResponse, Error> {
        self.client.post(&self.base_path(), request).await
    }

    /// Get an IP access list by ID.
    pub async fn get(&self, ip_access_list_id: &str) -> Result<IpAccessList, Error> {
        let resp: GetIpAccessListResponse = self
            .client
            .get(&format!("{}/{}", self.base_path(), ip_access_list_id))
            .await?;
        resp.ip_access_list
            .ok_or_else(|| Error::Other("Missing ip_access_list in response".into()))
    }

    /// List all IP access lists for the account.
    pub async fn list(&self) -> Result<Vec<IpAccessList>, Error> {
        let resp: ListIpAccessListsResponse = self.client.get(&self.base_path()).await?;
        Ok(resp.ip_access_lists)
    }

    /// Replace an IP access list.
    pub async fn replace(&self, request: &ReplaceIpAccessListRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .put(
                &format!("{}/{}", self.base_path(), request.ip_access_list_id),
                request,
            )
            .await?;
        Ok(())
    }

    /// Update an IP access list.
    pub async fn update(&self, request: &UpdateIpAccessListRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .patch(
                &format!("{}/{}", self.base_path(), request.ip_access_list_id),
                request,
            )
            .await?;
        Ok(())
    }

    /// Delete an IP access list by ID.
    pub async fn delete(&self, ip_access_list_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", self.base_path(), ip_access_list_id))
            .await
    }
}
