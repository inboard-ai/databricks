use databricks::{sql, Client};
use std::collections::HashMap;
use std::fs;
use std::time::Duration;

fn load_config() -> HashMap<String, String> {
    let home = std::env::var("HOME").expect("HOME not set");
    let path = format!("{}/projects/databrickscfg", home);
    let contents = fs::read_to_string(&path).expect("Failed to read databrickscfg");

    contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let mut parts = line.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts.next()?.to_string();
            Some((key, value))
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    let host = config.get("DATABRICKS_HOST").expect("DATABRICKS_HOST");
    let token = config.get("DATABRICKS_API_KEY").expect("DATABRICKS_API_KEY");
    let warehouse_id = config.get("DATABRICKS_WAREHOUSE_ID").expect("DATABRICKS_WAREHOUSE_ID");

    let client = Client::builder().host(host).token(token).build()?;

    // Check warehouse state
    let warehouses = sql::Warehouses::new(&client);
    let wh = warehouses.get(warehouse_id).await?;
    println!("Warehouse: {} ({:?})", wh.name, wh.state);

    if !wh.state.is_running() {
        println!("Starting warehouse...");
        warehouses.start(warehouse_id).await?;

        // Poll until running
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let wh = warehouses.get(warehouse_id).await?;
            println!("  State: {:?}", wh.state);
            if wh.state.is_running() {
                break;
            }
        }
    }

    // Execute a simple query
    let statements = sql::Statements::new(&client);

    let request = sql::Request::new("SELECT 1 as one, 2 as two, 'hello' as greeting", warehouse_id);

    println!("\nExecuting query...");
    let response = statements
        .execute_wait(&request, Duration::from_secs(1), Duration::from_secs(60))
        .await?;

    println!("Statement ID: {}", response.statement_id);
    println!("State: {:?}", response.status.state);

    if let Some(manifest) = &response.manifest {
        if let Some(schema) = &manifest.schema {
            println!("\nColumns:");
            for col in &schema.columns {
                println!(
                    "  {} ({})",
                    col.name,
                    col.type_text.as_deref().unwrap_or("?")
                );
            }
        }
    }

    if let Some(result) = &response.result {
        println!("\nRows:");
        for row in &result.data_array {
            let values: Vec<_> = row
                .iter()
                .map(|v| v.as_deref().unwrap_or("NULL"))
                .collect();
            println!("  {:?}", values);
        }
    }

    Ok(())
}
