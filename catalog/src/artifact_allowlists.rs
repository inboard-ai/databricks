use crate::types::{ArtifactAllowlistInfo, SetArtifactAllowlist};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.1/unity-catalog/artifact-allowlists";

pub struct ArtifactAllowlists {
    client: Client,
}

impl ArtifactAllowlists {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get the artifact allowlist for a given artifact type.
    pub async fn get(&self, artifact_type: &str) -> Result<ArtifactAllowlistInfo, Error> {
        let path = format!("{}/{}", PATH, artifact_type);
        self.client.get(&path).await
    }

    /// Set the artifact allowlist for a given artifact type.
    pub async fn update(
        &self,
        artifact_type: &str,
        request: &SetArtifactAllowlist,
    ) -> Result<ArtifactAllowlistInfo, Error> {
        let path = format!("{}/{}", PATH, artifact_type);
        self.client.put(&path, request).await
    }
}
