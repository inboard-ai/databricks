use databricks::{genie, sql, Client};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Duration;

fn load_config() -> HashMap<String, String> {
    let home = env::var("HOME").expect("HOME not set");
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

    // List available Genie spaces
    let spaces_api = genie::Spaces::new(client.clone());
    let spaces = spaces_api.list().await?;

    println!("Available Genie spaces:\n");
    for space in &spaces {
        println!(
            "  {} - {}",
            space.space_id,
            space.title.as_deref().unwrap_or("(untitled)")
        );
        if let Some(desc) = &space.description {
            println!("    {}", desc);
        }
    }

    // Get space ID from CLI arg or use first available
    let space_id = env::args()
        .nth(1)
        .or_else(|| spaces.first().map(|s| s.space_id.clone()));

    let Some(space_id) = space_id else {
        println!("\nNo Genie spaces available");
        return Ok(());
    };

    println!("\n--- Starting conversation with space {} ---\n", space_id);

    let conversations = genie::Conversations::new(client.clone(), &space_id);

    // Get question from CLI arg or use default
    let question = env::args()
        .nth(2)
        .unwrap_or_else(|| "What are the top 5 products by total revenue?".to_string());
    println!("> {}\n", question);

    let message = conversations
        .start_wait(question, Duration::from_secs(2), Duration::from_secs(120))
        .await?;

    println!("Status: {:?}", message.status);

    for attachment in &message.attachments {
        // Text response (if populated by the API)
        if let Some(text) = &attachment.text {
            if let Some(content) = &text.content {
                println!("\nResponse:\n{}", content);
            }
        }

        // SQL query - execute it!
        if let Some(query_attachment) = &attachment.query {
            // Show query description
            if let Some(desc) = &query_attachment.description {
                println!("\n{}", desc);
            }

            if let Some(query) = &query_attachment.query {
                println!("\nGenerated SQL:\n{}", query);

                // Execute the query
                println!("\nExecuting...\n");
                let statements = sql::Statements::new(client.clone());
                let request = sql::Request::new(query, warehouse_id);
                let response = statements
                    .execute_wait(&request, Duration::from_secs(1), Duration::from_secs(60))
                    .await?;

                // Print column headers
                if let Some(manifest) = &response.manifest {
                    if let Some(schema) = &manifest.schema {
                        let headers: Vec<_> =
                            schema.columns.iter().map(|c| c.name.as_str()).collect();
                        println!("{}", headers.join(" | "));
                        println!("{}", "-".repeat(headers.join(" | ").len()));
                    }
                }

                // Print rows
                if let Some(result) = &response.result {
                    for row in &result.data_array {
                        let values: Vec<_> =
                            row.iter().map(|v| v.as_deref().unwrap_or("NULL")).collect();
                        println!("{}", values.join(" | "));
                    }
                }
            }
        }

        // Suggested follow-up questions
        if let Some(suggestions) = &attachment.suggested_questions {
            if !suggestions.questions.is_empty() {
                println!("\nSuggested follow-up questions:");
                for q in &suggestions.questions {
                    println!("  - {}", q);
                }
            }
        }
    }

    Ok(())
}
