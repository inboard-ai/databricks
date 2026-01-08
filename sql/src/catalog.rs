use crate::types::{Request, Response};
use crate::Statements;
use databricks_core::Error;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_secs(1);
const TIMEOUT: Duration = Duration::from_secs(30);

pub struct Catalog<'a> {
    statements: Statements<'a>,
    warehouse_id: String,
}

impl<'a> Catalog<'a> {
    pub fn new(statements: Statements<'a>, warehouse_id: impl Into<String>) -> Self {
        Self {
            statements,
            warehouse_id: warehouse_id.into(),
        }
    }

    async fn query(&self, sql: &str) -> Result<Response, Error> {
        let request = Request::new(sql, &self.warehouse_id);
        self.statements
            .execute_wait(&request, POLL_INTERVAL, TIMEOUT)
            .await
    }

    fn extract_first_column(response: &Response) -> Vec<String> {
        response
            .result
            .as_ref()
            .map(|r| {
                r.data_array
                    .iter()
                    .filter_map(|row| row.first().and_then(|v| v.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn list_catalogs(&self) -> Result<Vec<String>, Error> {
        let response = self.query("SHOW CATALOGS").await?;
        Ok(Self::extract_first_column(&response))
    }

    pub async fn list_schemas(&self, catalog: &str) -> Result<Vec<String>, Error> {
        let response = self.query(&format!("SHOW SCHEMAS IN {}", catalog)).await?;
        Ok(Self::extract_first_column(&response))
    }

    pub async fn list_tables(&self, catalog: &str, schema: &str) -> Result<Vec<Table>, Error> {
        let response = self
            .query(&format!("SHOW TABLES IN {}.{}", catalog, schema))
            .await?;

        let tables = response
            .result
            .as_ref()
            .map(|r| {
                r.data_array
                    .iter()
                    .filter_map(|row| {
                        let schema = row.get(0).and_then(|v| v.clone())?;
                        let name = row.get(1).and_then(|v| v.clone())?;
                        Some(Table { schema, name })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(tables)
    }

    pub async fn describe_table(
        &self,
        catalog: &str,
        schema: &str,
        table: &str,
    ) -> Result<Vec<Column>, Error> {
        let response = self
            .query(&format!("DESCRIBE {}.{}.{}", catalog, schema, table))
            .await?;

        let columns = response
            .result
            .as_ref()
            .map(|r| {
                r.data_array
                    .iter()
                    .filter_map(|row| {
                        let name = row.get(0).and_then(|v| v.clone())?;
                        let data_type = row.get(1).and_then(|v| v.clone())?;
                        let nullable = row
                            .get(2)
                            .and_then(|v| v.as_deref())
                            .map(|s| s == "NULL")
                            .unwrap_or(true);
                        Some(Column {
                            name,
                            data_type,
                            nullable,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(columns)
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub schema: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}
