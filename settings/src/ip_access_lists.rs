use crate::types::{
    CreateIpAccessListRequest, CreateIpAccessListResponse, EmptyResponse, GetIpAccessListResponse,
    IpAccessList, ListIpAccessListsResponse, ReplaceIpAccessListRequest, UpdateIpAccessListRequest,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/ip-access-lists";

pub struct IpAccessLists {
    client: Client,
}

impl IpAccessLists {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &CreateIpAccessListRequest,
    ) -> Result<CreateIpAccessListResponse, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, ip_access_list_id: &str) -> Result<IpAccessList, Error> {
        let resp: GetIpAccessListResponse = self
            .client
            .get(&format!("{}/{}", PATH, ip_access_list_id))
            .await?;
        resp.ip_access_list
            .ok_or_else(|| Error::Other("Missing ip_access_list in response".into()))
    }

    pub async fn list(&self) -> Result<Vec<IpAccessList>, Error> {
        let resp: ListIpAccessListsResponse = self.client.get(PATH).await?;
        Ok(resp.ip_access_lists)
    }

    pub async fn update(&self, request: &UpdateIpAccessListRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .patch(&format!("{}/{}", PATH, request.ip_access_list_id), request)
            .await?;
        Ok(())
    }

    pub async fn replace(&self, request: &ReplaceIpAccessListRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .put(&format!("{}/{}", PATH, request.ip_access_list_id), request)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, ip_access_list_id: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, ip_access_list_id))
            .await
    }
}
