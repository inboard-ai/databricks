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
    let token = config
        .get("DATABRICKS_API_KEY")
        .expect("DATABRICKS_API_KEY");
    let warehouse_id = config
        .get("DATABRICKS_WAREHOUSE_ID")
        .expect("DATABRICKS_WAREHOUSE_ID");

    let client = Client::builder().host(host).token(token).build()?;

    // Check warehouse state
    let warehouses = sql::Warehouses::new(client.clone());
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

    let statements = sql::Statements::new(client.clone());
    let catalog = sql::Catalog::new(statements, warehouse_id);

    // List catalogs
    println!("\nCatalogs:");
    for name in catalog.list_catalogs().await? {
        println!("  {}", name);
    }

    // List schemas in samples
    println!("\nSchemas in 'samples':");
    for name in catalog.list_schemas("samples").await? {
        println!("  {}", name);
    }

    // List tables in samples.bakehouse
    println!("\nTables in 'samples.bakehouse':");
    for table in catalog.list_tables("samples", "bakehouse").await? {
        println!("  {}", table.name);
    }

    // Describe a table
    println!("\nColumns in 'sales_transactions':");
    for col in catalog
        .describe_table("samples", "bakehouse", "sales_transactions")
        .await?
    {
        println!(
            "  {} ({}){}",
            col.name,
            col.data_type,
            if col.nullable { "" } else { " NOT NULL" }
        );
    }

    // Run a real query
    let statements = sql::Statements::new(client);
    println!("\nTop 5 products by revenue:");
    let request = sql::Request::new(
        "SELECT product, SUM(totalPrice) as revenue \
         FROM samples.bakehouse.sales_transactions \
         GROUP BY product \
         ORDER BY revenue DESC \
         LIMIT 5",
        warehouse_id,
    );
    let response = statements
        .execute_wait(&request, Duration::from_secs(1), Duration::from_secs(60))
        .await?;

    if let Some(result) = &response.result {
        for row in &result.data_array {
            let product = row.first().and_then(|v| v.as_deref()).unwrap_or("?");
            let revenue = row.get(1).and_then(|v| v.as_deref()).unwrap_or("?");
            println!("  {} - ${}", product, revenue);
        }
    }

    Ok(())
}
