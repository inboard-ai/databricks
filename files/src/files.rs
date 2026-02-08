use crate::types::{DirectoryEntry, FileStatus, ListDirectoryResponse};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/fs/files";
const DIR_PATH: &str = "/api/2.0/fs/directories";

pub struct Files {
    client: Client,
}

impl Files {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Upload a file to Unity Catalog volumes path.
    pub async fn upload(&self, path: &str, data: &[u8]) -> Result<(), Error> {
        // Files API uses PUT with raw bytes at the path
        let uri = format!("{}/{}", PATH, path.trim_start_matches('/'));
        let _ = (uri, data);
        Err(Error::Other(
            "Binary upload not yet implemented for Files API. Use Dbfs instead.".into(),
        ))
    }

    /// Download a file from Unity Catalog volumes path.
    pub async fn download(&self, path: &str) -> Result<Vec<u8>, Error> {
        let uri = format!("{}/{}", PATH, path.trim_start_matches('/'));
        self.client.get_bytes(&uri).await
    }

    /// Delete a file.
    pub async fn delete(&self, path: &str) -> Result<(), Error> {
        let uri = format!("{}/{}", PATH, path.trim_start_matches('/'));
        self.client.delete_empty(&uri).await
    }

    /// Get file status/metadata.
    pub async fn get_status(&self, path: &str) -> Result<FileStatus, Error> {
        let uri = format!("{}/{}", PATH, path.trim_start_matches('/'));
        self.client.get(&uri).await
    }

    /// List directory contents.
    pub async fn list_directory_contents(&self, path: &str) -> Result<Vec<DirectoryEntry>, Error> {
        let uri = format!("{}/{}", DIR_PATH, path.trim_start_matches('/'));
        let response: ListDirectoryResponse = self.client.get(&uri).await?;
        Ok(response.contents)
    }
}
