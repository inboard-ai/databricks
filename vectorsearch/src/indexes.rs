use crate::types::{
    CreateIndex, DeleteDataRequest, DeleteDataResponse, EmptyResponse, Index, ListIndexesResponse,
    MiniIndex, QueryIndexRequest, QueryIndexResponse, QueryNextPageRequest, ScanIndexRequest,
    ScanIndexResponse, UpsertDataRequest, UpsertDataResponse,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/vector-search/indexes";

pub struct Indexes {
    client: Client,
}

impl Indexes {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateIndex) -> Result<Index, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, index_name: &str) -> Result<Index, Error> {
        self.client.get(&format!("{}/{}", PATH, index_name)).await
    }

    pub async fn list(&self, endpoint_name: &str) -> Result<Vec<MiniIndex>, Error> {
        let response: ListIndexesResponse = self
            .client
            .get_with_query(PATH, &[("endpoint_name", endpoint_name)])
            .await?;
        Ok(response.vector_indexes)
    }

    pub async fn delete(&self, index_name: &str) -> Result<(), Error> {
        self.client
            .delete_empty(&format!("{}/{}", PATH, index_name))
            .await
    }

    pub async fn sync(&self, index_name: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post_empty(&format!("{}/{}/sync", PATH, index_name))
            .await?;
        Ok(())
    }

    pub async fn query(
        &self,
        index_name: &str,
        request: &QueryIndexRequest,
    ) -> Result<QueryIndexResponse, Error> {
        self.client
            .post(&format!("{}/{}/query", PATH, index_name), request)
            .await
    }

    pub async fn query_next_page(
        &self,
        index_name: &str,
        request: &QueryNextPageRequest,
    ) -> Result<QueryIndexResponse, Error> {
        self.client
            .post(&format!("{}/{}/query-next-page", PATH, index_name), request)
            .await
    }

    pub async fn scan(
        &self,
        index_name: &str,
        request: &ScanIndexRequest,
    ) -> Result<ScanIndexResponse, Error> {
        self.client
            .post(&format!("{}/{}/scan", PATH, index_name), request)
            .await
    }

    pub async fn upsert_data(
        &self,
        index_name: &str,
        request: &UpsertDataRequest,
    ) -> Result<UpsertDataResponse, Error> {
        self.client
            .post(&format!("{}/{}/upsert-data", PATH, index_name), request)
            .await
    }

    pub async fn delete_data(
        &self,
        index_name: &str,
        request: &DeleteDataRequest,
    ) -> Result<DeleteDataResponse, Error> {
        self.client
            .post(&format!("{}/{}/delete-data", PATH, index_name), request)
            .await
    }
}
