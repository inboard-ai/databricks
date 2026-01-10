mod app;
mod ui;

use std::collections::HashMap;
use std::fs;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use app::{Message, Model, Space};
use databricks::Client;

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
    let warehouse_id = config
        .get("DATABRICKS_WAREHOUSE_ID")
        .expect("DATABRICKS_WAREHOUSE_ID")
        .clone();

    let client = Arc::new(Client::builder().host(host).token(token).build()?);

    // Fetch spaces before entering TUI
    let spaces_api = databricks::genie::Spaces::new(&client);
    let api_spaces = spaces_api.list().await?;

    let spaces: Vec<Space> = api_spaces
        .into_iter()
        .map(|s| Space {
            id: s.space_id,
            title: s.title,
        })
        .collect();

    if spaces.is_empty() {
        eprintln!("No Genie spaces available.");
        return Ok(());
    }

    // Init terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Channel for async messages
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Model
    let mut model = Model::new(client, spaces, warehouse_id, tx);

    // Main loop
    while !model.quit {
        terminal.draw(|f| {
            model.max_scroll = ui::view(f, &model);
        })?;

        if event::poll(Duration::from_millis(50))? {
            let evt = event::read()?;
            let mut msg = model.handle_event(evt);

            while let Some(m) = msg {
                msg = model.update(m);
            }
        }

        // Tick for animation while thinking/running
        if model.status != app::Status::Idle {
            model.update(Message::Tick);
        }

        while let Ok(m) = rx.try_recv() {
            let mut msg = Some(m);
            while let Some(m) = msg {
                msg = model.update(m);
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;

    Ok(())
}
