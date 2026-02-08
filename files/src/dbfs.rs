use crate::types::{
    DbfsListResponse, DbfsMkdirs, DbfsMove, DbfsPut, DbfsReadResponse, EmptyResponse, FileInfo,
};
use base64::Engine;
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/dbfs";

pub struct Dbfs {
    client: Client,
}

impl Dbfs {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Upload contents to a DBFS path. Data should be raw bytes (will be base64-encoded).
    pub async fn put(&self, path: &str, data: &[u8], overwrite: bool) -> Result<(), Error> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        let request = DbfsPut {
            path: path.to_string(),
            contents: encoded,
            overwrite: Some(overwrite),
        };
        let _: EmptyResponse = self.client.post(&format!("{}/put", PATH), &request).await?;
        Ok(())
    }

    /// Read contents from a DBFS path. Returns raw bytes.
    pub async fn get(&self, path: &str) -> Result<Vec<u8>, Error> {
        let response: DbfsReadResponse = self
            .client
            .get_with_query(&format!("{}/read", PATH), &[("path", path)])
            .await?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&response.data)
            .map_err(|e| Error::Other(format!("Failed to decode DBFS response: {}", e)))?;
        Ok(bytes)
    }

    pub async fn delete(&self, path: &str, recursive: bool) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct DeleteRequest {
            path: String,
            recursive: bool,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &DeleteRequest {
                    path: path.to_string(),
                    recursive,
                },
            )
            .await?;
        Ok(())
    }

    pub async fn list(&self, path: &str) -> Result<Vec<FileInfo>, Error> {
        let response: DbfsListResponse = self
            .client
            .get_with_query(&format!("{}/list", PATH), &[("path", path)])
            .await?;
        Ok(response.files)
    }

    pub async fn mkdirs(&self, path: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/mkdirs", PATH),
                &DbfsMkdirs {
                    path: path.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn move_(&self, source: &str, destination: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/move", PATH),
                &DbfsMove {
                    source_path: source.to_string(),
                    destination_path: destination.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn get_status(&self, path: &str) -> Result<FileInfo, Error> {
        self.client
            .get_with_query(&format!("{}/get-status", PATH), &[("path", path)])
            .await
    }
}
