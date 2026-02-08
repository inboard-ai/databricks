use crate::types::{
    EmptyResponse, ListScopesResponse, ListSecretsResponse, SecretMetadata, SecretScope,
};
use databricks_core::{Client, Error};

const PATH: &str = "/api/2.0/secrets";

pub struct Secrets {
    client: Client,
}

impl Secrets {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_scope(&self, scope: &str) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/scopes/create", PATH),
                &Req {
                    scope: scope.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn delete_scope(&self, scope: &str) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/scopes/delete", PATH),
                &Req {
                    scope: scope.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn list_scopes(&self) -> Result<Vec<SecretScope>, Error> {
        let response: ListScopesResponse =
            self.client.get(&format!("{}/scopes/list", PATH)).await?;
        Ok(response.scopes)
    }

    pub async fn put_secret(
        &self,
        scope: &str,
        key: &str,
        string_value: &str,
    ) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
            key: String,
            string_value: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/put", PATH),
                &Req {
                    scope: scope.to_string(),
                    key: key.to_string(),
                    string_value: string_value.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn delete_secret(&self, scope: &str, key: &str) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
            key: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/delete", PATH),
                &Req {
                    scope: scope.to_string(),
                    key: key.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn list_secrets(&self, scope: &str) -> Result<Vec<SecretMetadata>, Error> {
        let response: ListSecretsResponse = self
            .client
            .get_with_query(&format!("{}/list", PATH), &[("scope", scope)])
            .await?;
        Ok(response.secrets)
    }
}
