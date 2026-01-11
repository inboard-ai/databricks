use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use databricks::{genie, sql, Client};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use crate::component::{chat, header, input, suggestion};

// Re-export ChatEntry for ui.rs
pub use chat::ChatEntry;

pub enum Message {
    // Screen transitions
    SelectWarehouse(usize),
    SelectSpace(usize),
    BackToWarehouse,
    RequestQuit,
    ConfirmQuit,
    CancelQuit,

    // Delegated to components
    Input(input::Message),
    Chat(chat::Message),
    Suggestion(suggestion::Message),

    // Commands
    Clear,
    Help,
    Submit(String),

    // Animation
    Tick,

    // Async results
    Genie(Result<genie::Message, databricks::Error>),
    Sql { sql: String, result: Result<sql::Response, databricks::Error> },
    WarehouseStatus(Result<sql::Warehouse, databricks::Error>),
}


#[derive(Clone)]
pub struct Space {
    pub id: String,
    pub title: String,
}

#[derive(Clone)]
pub struct Warehouse {
    pub id: String,
    pub name: String,
    pub state: sql::State,
}

pub enum Screen {
    SelectWarehouse {
        warehouses: Vec<Warehouse>,
        selected: usize,
    },
    SelectSpace {
        spaces: Vec<Space>,
        selected: usize,
    },
    Chat,
    QuitConfirm,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FocusTarget {
    Input,
    Chat,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Status {
    Idle,
    Thinking,
    Running,
}

pub struct Model {
    // Current screen
    pub screen: Screen,

    // Components
    pub input: input::Model,
    pub chat: chat::Model,
    pub suggestion: suggestion::Model,
    pub header: header::Model,

    // Focus
    pub focus: FocusTarget,

    // State
    pub status: Status,
    pub quit: bool,
    pub animation_tick: u8,
    conversation_id: Option<String>,
    last_esc: Option<Instant>,
    warehouse_check_tick: u8,

    // Resources
    client: Arc<Client>,
    warehouses: Vec<Warehouse>,
    spaces: Vec<Space>,
    space_id: Option<String>,
    warehouse_id: Option<String>,
    tx: mpsc::UnboundedSender<Message>,
}


impl Model {
    pub fn new(
        client: Arc<Client>,
        warehouses: Vec<Warehouse>,
        spaces: Vec<Space>,
        tx: mpsc::UnboundedSender<Message>,
    ) -> Self {
        let screen = Screen::SelectWarehouse {
            warehouses: warehouses.clone(),
            selected: 0,
        };
        Self {
            screen,
            input: input::Model::new(),
            chat: chat::Model::new(),
            suggestion: suggestion::Model::new(),
            header: header::Model::new(),
            focus: FocusTarget::Input,
            status: Status::Idle,
            quit: false,
            animation_tick: 0,
            conversation_id: None,
            last_esc: None,
            warehouse_check_tick: 0,
            client,
            warehouses,
            spaces,
            space_id: None,
            warehouse_id: None,
            tx,
        }
    }

    /// Handle an event, return a Message if one should be processed
    pub fn handle_event(&mut self, event: Event) -> Option<Message> {
        let Event::Key(key) = event else {
            return None;
        };

        if key.kind != event::KeyEventKind::Press {
            return None;
        }

        // Ctrl+C always quits immediately (no confirmation)
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Message::ConfirmQuit);
        }

        match &mut self.screen {
            Screen::SelectWarehouse { warehouses, selected } => {
                match key.code {
                    KeyCode::Up => {
                        *selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        *selected = (*selected + 1).min(warehouses.len().saturating_sub(1));
                    }
                    KeyCode::Enter => {
                        return Some(Message::SelectWarehouse(*selected));
                    }
                    KeyCode::Esc => {
                        // Double-ESC to quit at root
                        if let Some(last) = self.last_esc {
                            if last.elapsed() < Duration::from_millis(500) {
                                return Some(Message::RequestQuit);
                            }
                        }
                        self.last_esc = Some(Instant::now());
                    }
                    _ => {}
                }
                None
            }
            Screen::SelectSpace { spaces, selected } => {
                match key.code {
                    KeyCode::Up => {
                        *selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        *selected = (*selected + 1).min(spaces.len().saturating_sub(1));
                    }
                    KeyCode::Enter => {
                        return Some(Message::SelectSpace(*selected));
                    }
                    KeyCode::Esc => {
                        // Go back to warehouse selection
                        return Some(Message::BackToWarehouse);
                    }
                    _ => {}
                }
                None
            }
            Screen::Chat => self.handle_chat_event(key),
            Screen::QuitConfirm => {
                match key.code {
                    KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                        Some(Message::ConfirmQuit)
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                        Some(Message::CancelQuit)
                    }
                    _ => None,
                }
            }
        }
    }

    fn handle_chat_event(&mut self, key: event::KeyEvent) -> Option<Message> {
        // Suggestion takes priority when expanded
        if self.suggestion.is_expanded() {
            return self.suggestion.handle_event(key).map(Message::Suggestion);
        }

        // Route based on focus state
        match self.focus {
            FocusTarget::Input => self.handle_input_focus(key),
            FocusTarget::Chat => self.chat.handle_event(key).map(Message::Chat),
        }
    }

    fn handle_input_focus(&mut self, key: event::KeyEvent) -> Option<Message> {
        // Double-ESC to quit (always works, even during query)
        if key.code == KeyCode::Esc {
            if let Some(last) = self.last_esc {
                if last.elapsed() < Duration::from_millis(500) {
                    return Some(Message::RequestQuit);
                }
            }
            self.last_esc = Some(Instant::now());
            return None;
        }

        // Tab toggles suggestions
        if key.code == KeyCode::Tab && self.suggestion.has_suggestions() {
            return Some(Message::Suggestion(suggestion::Message::Toggle));
        }

        // Up arrow focuses chat
        if key.code == KeyCode::Up && !self.chat.entries.is_empty() {
            self.chat.focus_last();
            self.focus = FocusTarget::Chat;
            return None;
        }

        // Scroll
        match key.code {
            KeyCode::PageUp => return Some(Message::Chat(chat::Message::Scroll(-10))),
            KeyCode::PageDown => return Some(Message::Chat(chat::Message::Scroll(10))),
            KeyCode::Down => return Some(Message::Chat(chat::Message::Scroll(3))),
            _ => {}
        }

        // Enter: commands always work, messages only when idle
        if key.code == KeyCode::Enter {
            let text = self.input.text.trim();
            if !text.is_empty() {
                // Commands always work (even during query)
                if let Some(msg) = self.parse_command(text) {
                    self.input.clear();
                    return Some(msg);
                }
                // Messages only when idle
                if self.status == Status::Idle {
                    return Some(Message::Input(input::Message::Submit));
                }
            }
            return None;
        }

        // Block typing while busy (except commands above)
        if self.status != Status::Idle {
            return None;
        }

        // Delegate to input component
        self.input.handle_event(key).map(Message::Input)
    }

    fn parse_command(&self, input: &str) -> Option<Message> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }

        let cmd = input[1..].split_whitespace().next()?;
        match cmd.to_lowercase().as_str() {
            "exit" | "quit" | "q" => Some(Message::RequestQuit),
            "clear" => Some(Message::Clear),
            "help" | "?" => Some(Message::Help),
            _ => None,
        }
    }

    /// Process a message, optionally return another to chain
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::SelectWarehouse(idx) => {
                if let Some(warehouse) = self.warehouses.get(idx) {
                    self.warehouse_id = Some(warehouse.id.clone());
                    // Don't set state from list - wait for fresh check
                    self.spawn_warehouse_check();
                    // Move to space selection
                    self.screen = Screen::SelectSpace {
                        spaces: self.spaces.clone(),
                        selected: 0,
                    };
                }
            }

            Message::BackToWarehouse => {
                self.warehouse_id = None;
                self.header.warehouse_state = None;
                self.screen = Screen::SelectWarehouse {
                    warehouses: self.warehouses.clone(),
                    selected: 0,
                };
            }

            Message::SelectSpace(idx) => {
                if let Screen::SelectSpace { spaces, .. } = &self.screen {
                    if let Some(space) = spaces.get(idx) {
                        self.space_id = Some(space.id.clone());
                        self.header.set_space(space.title.clone(), space.id.clone());
                        self.screen = Screen::Chat;
                        self.spawn_warehouse_check();
                    }
                }
            }

            Message::Input(input_msg) => {
                if let Some(out) = self.input.update(input_msg) {
                    return self.handle_input_output(out);
                }
            }

            Message::Chat(chat_msg) => {
                if let Some(out) = self.chat.update(chat_msg) {
                    return self.handle_chat_output(out);
                }
            }

            Message::Suggestion(sugg_msg) => {
                if let Some(out) = self.suggestion.update(sugg_msg) {
                    return self.handle_suggestion_output(out);
                }
            }

            Message::Submit(question) => {
                self.chat.push(ChatEntry::User(question.clone()));
                self.status = Status::Thinking;
                self.chat.scroll_to_bottom();
                self.spawn_genie(question);
            }

            Message::Clear => {
                self.chat.clear();
                self.conversation_id = None;
            }

            Message::Help => {
                self.chat.push(ChatEntry::Assistant(
                    "Commands:\n  /clear - Clear chat history\n  /exit  - Exit the app\n  /help  - Show this help\n\nShortcuts:\n  Ctrl+C - Exit immediately\n  ESC ESC - Exit with confirmation\n  Tab - Toggle suggestions\n  ↑↓ - Scroll or navigate".to_string()
                ));
                self.chat.scroll_to_bottom();
            }

            Message::RequestQuit => {
                self.screen = Screen::QuitConfirm;
            }

            Message::ConfirmQuit => {
                self.quit = true;
            }

            Message::CancelQuit => {
                self.screen = Screen::Chat;
            }

            Message::Tick => {
                self.animation_tick = self.animation_tick.wrapping_add(1);
                // Check warehouse status every ~30 seconds (600 ticks at 50ms each)
                self.warehouse_check_tick = self.warehouse_check_tick.wrapping_add(1);
                if self.warehouse_check_tick == 0 {
                    self.spawn_warehouse_check();
                }
            }

            Message::Genie(result) => {
                let sql_query = self.apply_genie_result(result);
                self.chat.scroll_to_bottom();

                if let Some(query) = sql_query {
                    // If warehouse is stopped, it will auto-start - show Starting
                    if self.header.warehouse_state == Some(sql::State::Stopped) {
                        self.header.set_warehouse_state(sql::State::Starting);
                    }
                    self.status = Status::Running;
                    self.spawn_sql(query);
                } else {
                    self.status = Status::Idle;
                }
            }

            Message::Sql { sql, result } => {
                self.apply_sql_result(sql, result);
                self.status = Status::Idle;
                self.chat.scroll_to_bottom();
            }

            Message::WarehouseStatus(result) => {
                if let Ok(warehouse) = result {
                    self.header.set_warehouse_state(warehouse.state);
                }
            }
        }

        None
    }

    // Output handlers for component messages

    fn handle_input_output(&mut self, out: input::OutMessage) -> Option<Message> {
        match out {
            input::OutMessage::Submit(text) => Some(Message::Submit(text)),
        }
    }

    fn handle_chat_output(&mut self, out: chat::OutMessage) -> Option<Message> {
        match out {
            chat::OutMessage::ReturnToInput => {
                self.focus = FocusTarget::Input;
                None
            }
        }
    }

    fn handle_suggestion_output(&mut self, out: suggestion::OutMessage) -> Option<Message> {
        match out {
            suggestion::OutMessage::AcceptSuggestion(question) => Some(Message::Submit(question)),
        }
    }

    // Async spawning

    fn spawn_genie(&self, question: String) {
        let Some(space_id) = self.space_id.clone() else {
            return;
        };

        let client = self.client.clone();
        let conv_id = self.conversation_id.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let api = genie::Conversations::new(&client, &space_id);
            let result = match &conv_id {
                Some(id) => {
                    api.send_wait(id, &question, Duration::from_secs(2), Duration::from_secs(120))
                        .await
                }
                None => {
                    api.start_wait(&question, Duration::from_secs(2), Duration::from_secs(120))
                        .await
                }
            };
            let _ = tx.send(Message::Genie(result));
        });
    }

    fn spawn_sql(&self, query: String) {
        let Some(warehouse_id) = self.warehouse_id.clone() else {
            return;
        };

        let client = self.client.clone();
        let tx = self.tx.clone();
        let sql = query.clone();

        tokio::spawn(async move {
            let api = sql::Statements::new(&client);
            let req = sql::Request::new(&query, &warehouse_id);
            let result = api
                .execute_wait(&req, Duration::from_secs(1), Duration::from_secs(60))
                .await;
            let _ = tx.send(Message::Sql { sql, result });
        });
    }

    fn spawn_warehouse_check(&self) {
        let Some(warehouse_id) = self.warehouse_id.clone() else {
            return;
        };

        let client = self.client.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let api = sql::Warehouses::new(&client);
            let result = api.get(&warehouse_id).await;
            let _ = tx.send(Message::WarehouseStatus(result));
        });
    }

    fn apply_genie_result(
        &mut self,
        result: Result<genie::Message, databricks::Error>,
    ) -> Option<String> {
        let mut sql_query = None;

        match result {
            Ok(msg) => {
                for att in &msg.attachments {
                    // Text attachment
                    if let Some(t) = &att.text {
                        if let Some(c) = &t.content {
                            if !c.trim().is_empty() {
                                self.chat.push(ChatEntry::Assistant(c.clone()));
                            }
                        }
                    }
                    // Query attachment - extract SQL to execute via Statements API
                    if let Some(q) = &att.query {
                        // Show description
                        if let Some(desc) = &q.description {
                            if !desc.trim().is_empty() {
                                self.chat.push(ChatEntry::Assistant(desc.clone()));
                            }
                        }
                        // Get the SQL query to execute
                        if let Some(query) = &q.query {
                            sql_query = Some(query.clone());
                        }
                    }
                    // Suggested follow-up questions
                    if let Some(s) = &att.suggested_questions {
                        self.suggestion.set_suggestions(s.questions.clone());
                    }
                }
                self.conversation_id = Some(msg.conversation_id);
            }
            Err(e) => {
                self.chat.push(ChatEntry::Error(e.to_string()));
            }
        }

        sql_query
    }

    fn apply_sql_result(&mut self, sql: String, result: Result<sql::Response, databricks::Error>) {
        match result {
            Ok(resp) => {
                // Extract column names from manifest
                let headers: Vec<String> = resp
                    .manifest
                    .as_ref()
                    .and_then(|m| m.schema.as_ref())
                    .map(|s| s.columns.iter().map(|c| c.name.clone()).collect())
                    .unwrap_or_default();

                // Extract row data - SQL Statements API uses Vec<Vec<Option<String>>>
                let rows: Vec<Vec<String>> = resp
                    .result
                    .as_ref()
                    .map(|r| {
                        r.data_array
                            .iter()
                            .map(|row| {
                                row.iter()
                                    .map(|v| v.as_deref().unwrap_or("NULL").to_string())
                                    .collect()
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                // Add table to chat with its SQL
                if !headers.is_empty() || !rows.is_empty() {
                    self.chat.push(ChatEntry::Table {
                        sql: Some(sql),
                        headers,
                        rows,
                        expanded: false,
                    });
                }
            }
            Err(e) => {
                self.chat.push(ChatEntry::Error(format!("Query failed: {e}")));
            }
        }
    }
}
