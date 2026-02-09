use crate::types::{
    AclItem, EmptyResponse, GetSecretResponse, ListAclsResponse, ListScopesResponse,
    ListSecretsResponse, SecretMetadata, SecretScope,
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

    pub async fn get_secret(&self, scope: &str, key: &str) -> Result<GetSecretResponse, Error> {
        self.client
            .get_with_query(&format!("{}/get", PATH), &[("scope", scope), ("key", key)])
            .await
    }

    pub async fn get_acl(&self, scope: &str, principal: &str) -> Result<AclItem, Error> {
        self.client
            .get_with_query(
                &format!("{}/acls/get", PATH),
                &[("scope", scope), ("principal", principal)],
            )
            .await
    }

    pub async fn delete_acl(&self, scope: &str, principal: &str) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
            principal: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/acls/delete", PATH),
                &Req {
                    scope: scope.to_string(),
                    principal: principal.to_string(),
                },
            )
            .await?;
        Ok(())
    }

    pub async fn list_acls(&self, scope: &str) -> Result<Vec<AclItem>, Error> {
        let response: ListAclsResponse = self
            .client
            .get_with_query(&format!("{}/acls/list", PATH), &[("scope", scope)])
            .await?;
        Ok(response.items)
    }

    pub async fn put_acl(
        &self,
        scope: &str,
        principal: &str,
        permission: &str,
    ) -> Result<(), Error> {
        #[derive(serde::Serialize)]
        struct Req {
            scope: String,
            principal: String,
            permission: String,
        }
        let _: EmptyResponse = self
            .client
            .post(
                &format!("{}/acls/put", PATH),
                &Req {
                    scope: scope.to_string(),
                    principal: principal.to_string(),
                    permission: permission.to_string(),
                },
            )
            .await?;
        Ok(())
    }
}
