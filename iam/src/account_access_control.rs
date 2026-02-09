use crate::types::{GetAssignableRolesForResourceResponse, RuleSetResponse, UpdateRuleSetRequest};
use databricks_core::{Client, Error};

pub struct AccountAccessControl {
    client: Client,
    account_id: String,
}

impl AccountAccessControl {
    pub fn new(client: Client, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
    }

    /// Get assignable roles for a resource.
    pub async fn get_assignable_roles_for_resource(
        &self,
        resource: &str,
    ) -> Result<GetAssignableRolesForResourceResponse, Error> {
        let path = format!(
            "/api/2.0/preview/accounts/{}/access-control/assignable-roles",
            self.account_id
        );
        self.client
            .get_with_query(&path, &[("resource", resource)])
            .await
    }

    /// Get a rule set by name and etag.
    pub async fn get_rule_set(&self, name: &str, etag: &str) -> Result<RuleSetResponse, Error> {
        let path = format!(
            "/api/2.0/preview/accounts/{}/access-control/rule-sets",
            self.account_id
        );
        self.client
            .get_with_query(&path, &[("name", name), ("etag", etag)])
            .await
    }

    /// Update a rule set.
    pub async fn update_rule_set(
        &self,
        request: &UpdateRuleSetRequest,
    ) -> Result<RuleSetResponse, Error> {
        let path = format!(
            "/api/2.0/preview/accounts/{}/access-control/rule-sets",
            self.account_id
        );
        self.client.put(&path, request).await
    }
}
