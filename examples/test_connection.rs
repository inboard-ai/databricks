use databricks::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct ListWarehousesResponse {
    warehouses: Vec<Warehouse>,
}

#[derive(Debug, Deserialize)]
struct Warehouse {
    id: String,
    name: String,
    state: String,
}

#[derive(Serialize)]
struct Empty {}

#[derive(Debug, Deserialize)]
struct EmptyResponse {}

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
    let host = config
        .get("DATABRICKS_HOST")
        .expect("DATABRICKS_HOST not found in config");
    let token = config
        .get("DATABRICKS_API_KEY")
        .expect("DATABRICKS_API_KEY not found in config");

    println!("Connecting to Databricks at {}", host);

    let client = Client::builder().host(host).token(token).build()?;

    println!("Listing SQL warehouses...\n");

    let response: ListWarehousesResponse = client.get("/api/2.0/sql/warehouses").await?;

    for wh in &response.warehouses {
        println!("  {} - {} ({})", wh.id, wh.name, wh.state);

        if wh.state == "STOPPED" {
            println!("    -> Starting warehouse {}...", wh.id);
            let _: EmptyResponse = client
                .post(
                    &format!("/api/2.0/sql/warehouses/{}/start", wh.id),
                    &Empty {},
                )
                .await?;
            println!("    -> Start request sent!");
        }
    }

    println!("\nFound {} warehouse(s)", response.warehouses.len());

    Ok(())
}
