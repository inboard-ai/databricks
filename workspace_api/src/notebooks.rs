use crate::types::{
    DeleteRequest, EmptyResponse, ExportFormat, ExportResponse, ImportRequest, ListResponse,
    MkdirsRequest, ObjectInfo,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/workspace";

pub struct Notebooks {
    client: Client,
}

impl Notebooks {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self, path: &str) -> Result<Vec<ObjectInfo>, Error> {
        let response: ListResponse = self
            .client
            .get_with_query(&format!("{}/list", PATH), &[("path", path)])
            .await?;
        Ok(response.objects)
    }

    pub async fn get_status(&self, path: &str) -> Result<ObjectInfo, Error> {
        self.client
            .get_with_query(&format!("{}/get-status", PATH), &[("path", path)])
            .await
    }

    pub async fn import(&self, request: &ImportRequest) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(&format!("{}/import", PATH), request)
            .await?;
        Ok(())
    }

    pub async fn export(&self, path: &str, format: ExportFormat) -> Result<String, Error> {
        let format_str = serde_json::to_string(&format)
            .unwrap()
            .trim_matches('"')
            .to_string();
        let response: ExportResponse = self
            .client
            .get_with_query(
                &format!("{}/export", PATH),
                &[("path", path), ("format", &format_str)],
            )
            .await?;
        Ok(response.content)
    }

    pub async fn delete(&self, path: &str, recursive: bool) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &DeleteRequest {
                    path: path.to_string(),
                    recursive: Some(recursive),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn mkdirs(&self, path: &str) -> Result<(), Error> {
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/mkdirs", PATH),
                &MkdirsRequest {
                    path: path.to_string(),
                },
            )
            .await?;
        Ok(())
    }
}
