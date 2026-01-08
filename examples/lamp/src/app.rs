use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use databricks::{genie, sql, Client};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub enum Message {
    // Space selection
    SelectSpace(usize),

    // Chat actions
    Submit(String),
    AcceptSuggestion(usize),
    Scroll(i16),
    ToggleSuggestions,
    DismissSuggestions,
    Quit,

    // Async results
    Genie(Result<genie::Message, databricks::Error>),
    Sql(Result<sql::Response, databricks::Error>),
}

#[derive(Clone)]
pub struct Space {
    pub id: String,
    pub title: String,
}

pub enum Screen {
    SelectSpace {
        spaces: Vec<Space>,
        selected: usize,
    },
    Chat,
}

#[derive(Clone)]
pub enum ChatEntry {
    User(String),
    Assistant(String),
    Sql(String),
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Error(String),
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

    // Chat
    pub chat: Vec<ChatEntry>,
    pub scroll: u16,
    pub max_scroll: u16,

    // Input
    pub input: String,
    pub cursor: usize,

    // Suggestions
    pub suggestions: Vec<String>,
    pub suggestion_idx: Option<usize>,
    pub show_suggestions: bool,

    // State
    pub status: Status,
    pub quit: bool,
    conversation_id: Option<String>,

    // Resources
    client: Arc<Client>,
    space_id: Option<String>,
    warehouse_id: String,
    tx: mpsc::UnboundedSender<Message>,
}

impl Model {
    pub fn new(
        client: Arc<Client>,
        spaces: Vec<Space>,
        warehouse_id: String,
        tx: mpsc::UnboundedSender<Message>,
    ) -> Self {
        Self {
            screen: Screen::SelectSpace { spaces, selected: 0 },
            chat: Vec::new(),
            scroll: 0,
            max_scroll: 0,
            input: String::new(),
            cursor: 0,
            suggestions: Vec::new(),
            suggestion_idx: None,
            show_suggestions: false,
            status: Status::Idle,
            quit: false,
            conversation_id: None,
            client,
            space_id: None,
            warehouse_id,
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

        // Ctrl+C always quits
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Message::Quit);
        }

        match &mut self.screen {
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
                    _ => {}
                }
                None
            }
            Screen::Chat => self.handle_chat_event(key),
        }
    }

    fn handle_chat_event(&mut self, key: event::KeyEvent) -> Option<Message> {
        // When showing suggestions, arrow keys navigate them
        if self.show_suggestions {
            match key.code {
                KeyCode::Up => {
                    self.prev_suggestion();
                    return None;
                }
                KeyCode::Down => {
                    self.next_suggestion();
                    return None;
                }
                KeyCode::Enter => {
                    if let Some(idx) = self.suggestion_idx {
                        return Some(Message::AcceptSuggestion(idx));
                    }
                }
                KeyCode::Esc => return Some(Message::DismissSuggestions),
                KeyCode::Tab => return Some(Message::ToggleSuggestions),
                _ => {
                    self.hide_suggestions();
                }
            }
        }

        // Input handling
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.input.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.input.remove(self.cursor);
            }
            KeyCode::Delete if self.cursor < self.input.len() => {
                self.input.remove(self.cursor);
            }
            KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
            KeyCode::Right if self.cursor < self.input.len() => self.cursor += 1,
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = self.input.len(),
            KeyCode::Up => return Some(Message::Scroll(-3)),
            KeyCode::Down => return Some(Message::Scroll(3)),
            KeyCode::PageUp => return Some(Message::Scroll(-10)),
            KeyCode::PageDown => return Some(Message::Scroll(10)),
            KeyCode::Tab if !self.suggestions.is_empty() => {
                return Some(Message::ToggleSuggestions)
            }
            KeyCode::Enter if self.status == Status::Idle => {
                let text = self.input.trim();
                if !text.is_empty() {
                    let question = self.input.clone();
                    self.input.clear();
                    self.cursor = 0;
                    return Some(Message::Submit(question));
                }
            }
            _ => {}
        }

        None
    }

    /// Process a message, optionally return another to chain
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::SelectSpace(idx) => {
                if let Screen::SelectSpace { spaces, .. } = &self.screen {
                    if let Some(space) = spaces.get(idx) {
                        self.space_id = Some(space.id.clone());
                        self.screen = Screen::Chat;
                    }
                }
            }

            Message::Submit(question) => {
                self.chat.push(ChatEntry::User(question.clone()));
                self.status = Status::Thinking;
                self.scroll_to_bottom();
                self.spawn_genie(question);
            }

            Message::AcceptSuggestion(idx) => {
                if let Some(question) = self.suggestions.get(idx).cloned() {
                    self.hide_suggestions();
                    return Some(Message::Submit(question));
                }
            }

            Message::Scroll(delta) => {
                if delta < 0 {
                    self.scroll = self.scroll.saturating_sub((-delta) as u16);
                } else {
                    self.scroll = (self.scroll + delta as u16).min(self.max_scroll);
                }
            }

            Message::ToggleSuggestions => {
                self.show_suggestions = !self.show_suggestions;
                if self.show_suggestions && self.suggestion_idx.is_none() {
                    self.suggestion_idx = Some(0);
                }
            }

            Message::DismissSuggestions => {
                self.hide_suggestions();
            }

            Message::Quit => {
                self.quit = true;
            }

            Message::Genie(result) => {
                let sql = self.apply_genie_result(result);
                self.scroll_to_bottom();

                if let Some(query) = sql {
                    self.status = Status::Running;
                    self.spawn_sql(query);
                } else {
                    self.status = Status::Idle;
                }
            }

            Message::Sql(result) => {
                self.apply_sql_result(result);
                self.status = Status::Idle;
                self.scroll_to_bottom();
            }
        }

        None
    }

    // Helpers

    fn prev_suggestion(&mut self) {
        let len = self.suggestions.len();
        if len == 0 {
            return;
        }
        self.suggestion_idx = Some(match self.suggestion_idx {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        });
    }

    fn next_suggestion(&mut self) {
        let len = self.suggestions.len();
        if len == 0 {
            return;
        }
        self.suggestion_idx = Some(match self.suggestion_idx {
            None => 0,
            Some(i) if i + 1 >= len => 0,
            Some(i) => i + 1,
        });
    }

    fn hide_suggestions(&mut self) {
        self.suggestions.clear();
        self.suggestion_idx = None;
        self.show_suggestions = false;
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll = self.max_scroll;
    }

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
        let client = self.client.clone();
        let warehouse_id = self.warehouse_id.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let api = sql::Statements::new(&client);
            let req = sql::Request::new(&query, &warehouse_id);
            let result = api
                .execute_wait(&req, Duration::from_secs(1), Duration::from_secs(60))
                .await;
            let _ = tx.send(Message::Sql(result));
        });
    }

    fn apply_genie_result(
        &mut self,
        result: Result<genie::Message, databricks::Error>,
    ) -> Option<String> {
        let mut sql = None;

        match result {
            Ok(msg) => {
                for att in &msg.attachments {
                    if let Some(t) = &att.text {
                        if let Some(c) = &t.content {
                            self.chat.push(ChatEntry::Assistant(c.clone()));
                        }
                    }
                    if let Some(q) = &att.query {
                        if let Some(query) = &q.query {
                            self.chat.push(ChatEntry::Sql(query.clone()));
                            sql = Some(query.clone());
                        }
                    }
                    if let Some(s) = &att.suggested_questions {
                        self.suggestions = s.questions.clone();
                        self.show_suggestions = !self.suggestions.is_empty();
                        self.suggestion_idx = if self.show_suggestions { Some(0) } else { None };
                    }
                }
                self.conversation_id = Some(msg.conversation_id);
            }
            Err(e) => {
                self.chat.push(ChatEntry::Error(e.to_string()));
            }
        }

        sql
    }

    fn apply_sql_result(&mut self, result: Result<sql::Response, databricks::Error>) {
        match result {
            Ok(resp) => {
                let headers: Vec<String> = resp
                    .manifest
                    .as_ref()
                    .and_then(|m| m.schema.as_ref())
                    .map(|s| s.columns.iter().map(|c| c.name.clone()).collect())
                    .unwrap_or_default();

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

                if !headers.is_empty() || !rows.is_empty() {
                    self.chat.push(ChatEntry::Table { headers, rows });
                }
            }
            Err(e) => {
                self.chat.push(ChatEntry::Error(format!("Query failed: {e}")));
            }
        }
    }
}
