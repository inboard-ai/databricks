<div align="center">

<img src="banner.png" alt="databricks" width="600" />

Unofficial Databricks SDK for Rust

</div>

## Quick Start

```rust
use databricks::workspace;

#[tokio::main]
async fn main() -> Result<(), databricks::Error> {
    // Reads DATABRICKS_HOST + DATABRICKS_TOKEN from env / ~/.databrickscfg
    let ws = workspace::Client::new()?;

    // List SQL warehouses
    for wh in ws.warehouses().list().await? {
        println!("{} ({:?})", wh.name, wh.state);
    }

    // Run a SQL query
    let req = databricks::sql::Request::new(
        "SELECT 1 + 1 AS answer",
        "your-warehouse-id",
    );
    let resp = ws.statements()
        .execute_wait(&req, Duration::from_secs(1), Duration::from_secs(30))
        .await?;

    Ok(())
}
```

## Services

| Crate | Services |
|-------|----------|
| `databricks` | `workspace::Client` / `account::Client` facades |
| `core` | HTTP client, auth, config, retry, pagination, `Wait<T>` |
| `sql` | Warehouses, Statements, Catalog |
| `genie` | Spaces, Conversations |
| `compute` | Clusters, InstancePools, ClusterPolicies, Libraries |
| `jobs` | Jobs, Runs |
| `files` | Dbfs, Files |
| `workspace_api` | Notebooks, Repos, Secrets, GitCredentials |
| `iam` | Users, Groups, ServicePrincipals, Permissions, CurrentUser |
| `catalog` | Catalogs, Schemas, Tables, Volumes, Grants |
| `serving` | ServingEndpoints |
| `pipelines` | Pipelines |
| `ml` | Experiments, Runs, RegisteredModels, ModelVersions |

## Authentication

Resolved automatically via credential chain:

1. Explicit values passed to `config::Builder`
2. Environment variables (`DATABRICKS_HOST`, `DATABRICKS_TOKEN`, etc.)
3. `~/.databrickscfg` INI file (supports `[DEFAULT]` and named profiles)

Supported methods: **PAT**, **Basic**, **OAuth M2M** (client credentials).

```rust
// Explicit config
let ws = workspace::Client::with_config(
    databricks::core::config::Builder::default()
        .host("https://my-workspace.cloud.databricks.com")
        .token("dapi...")
        .build()?,
)?;
```

## License

MIT OR Apache-2.0
