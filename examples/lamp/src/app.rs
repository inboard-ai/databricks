use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use databricks::{genie, sql, Client};
use std::sync::Arc;
use std::time::{Duration, Instant};
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

    // Focus navigation
    FocusChat,           // Enter chat focus mode (from input)
    FocusInput,          // Return to input focus
    FocusUp,             // Move focus up in chat
    FocusDown,           // Move focus down in chat
    ToggleExpand,        // Toggle SQL expansion on focused table

    // Commands
    Clear,
    Help,
    RequestQuit,  // Show confirmation
    ConfirmQuit,  // Actually quit
    CancelQuit,   // Go back to chat

    // Animation
    Tick,

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
    QuitConfirm,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Focus {
    Input,
    Chat(usize),
}

#[derive(Clone)]
pub enum ChatEntry {
    User(String),
    Assistant(String),
    Table {
        sql: Option<String>,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        expanded: bool,
    },
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
    pub focus: Focus,

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
    pub animation_tick: u8,
    conversation_id: Option<String>,
    last_esc: Option<Instant>,
    pending_sql: Option<String>,

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
            focus: Focus::Input,
            input: String::new(),
            cursor: 0,
            suggestions: Vec::new(),
            suggestion_idx: None,
            show_suggestions: false,
            status: Status::Idle,
            quit: false,
            animation_tick: 0,
            conversation_id: None,
            last_esc: None,
            pending_sql: None,
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

        // Ctrl+C always quits immediately (no confirmation)
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Message::ConfirmQuit);
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
                    KeyCode::Esc => {
                        return Some(Message::RequestQuit);
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
        // Route based on focus state
        match self.focus {
            Focus::Input => self.handle_input_focus(key),
            Focus::Chat(_) => self.handle_chat_focus(key),
        }
    }

    fn handle_input_focus(&mut self, key: event::KeyEvent) -> Option<Message> {
        // When suggestions popup is showing, it takes priority for navigation
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
                    // Collapse but keep suggestions available
                    self.collapse_suggestions();
                }
            }
        }

        // Double-ESC to quit when idle
        if key.code == KeyCode::Esc && self.status == Status::Idle && !self.show_suggestions {
            if let Some(last) = self.last_esc {
                if last.elapsed() < Duration::from_millis(500) {
                    return Some(Message::RequestQuit);
                }
            }
            self.last_esc = Some(Instant::now());
            return None;
        }

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
            KeyCode::Up => {
                // Enter chat focus mode (suggestions only via Tab)
                if !self.chat.is_empty() {
                    return Some(Message::FocusChat);
                }
            }
            KeyCode::Down => return Some(Message::Scroll(3)),
            KeyCode::PageUp => return Some(Message::Scroll(-10)),
            KeyCode::PageDown => return Some(Message::Scroll(10)),
            KeyCode::Tab if !self.suggestions.is_empty() => {
                return Some(Message::ToggleSuggestions)
            }
            KeyCode::Enter if self.status == Status::Idle => {
                let text = self.input.trim();
                if !text.is_empty() {
                    if let Some(msg) = self.parse_command(text) {
                        self.input.clear();
                        self.cursor = 0;
                        return Some(msg);
                    }
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

    fn handle_chat_focus(&mut self, key: event::KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Up => return Some(Message::FocusUp),
            KeyCode::Down => return Some(Message::FocusDown),
            KeyCode::Char(' ') => return Some(Message::ToggleExpand),
            KeyCode::Esc => return Some(Message::FocusInput),
            KeyCode::Enter => return Some(Message::FocusInput),
            // Any other character returns to input and types it
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.focus = Focus::Input;
                self.input.insert(self.cursor, c);
                self.cursor += 1;
            }
            _ => {}
        }
        None
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
                    self.clear_suggestions();
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
                self.collapse_suggestions();
            }

            Message::FocusChat => {
                // Enter chat focus mode on last entry
                if !self.chat.is_empty() {
                    self.focus = Focus::Chat(self.chat.len() - 1);
                }
            }

            Message::FocusInput => {
                self.focus = Focus::Input;
            }

            Message::FocusUp => {
                if let Focus::Chat(idx) = self.focus {
                    if idx > 0 {
                        self.focus = Focus::Chat(idx - 1);
                    }
                }
            }

            Message::FocusDown => {
                if let Focus::Chat(idx) = self.focus {
                    if idx + 1 < self.chat.len() {
                        self.focus = Focus::Chat(idx + 1);
                    } else {
                        // At bottom, return to input
                        self.focus = Focus::Input;
                    }
                }
            }

            Message::ToggleExpand => {
                if let Focus::Chat(idx) = self.focus {
                    if let Some(ChatEntry::Table { expanded, .. }) = self.chat.get_mut(idx) {
                        *expanded = !*expanded;
                    }
                }
            }

            Message::Clear => {
                self.chat.clear();
                self.scroll = 0;
                self.conversation_id = None;
            }

            Message::Help => {
                self.chat.push(ChatEntry::Assistant(
                    "Commands:\n  /clear - Clear chat history\n  /exit  - Exit the app\n  /help  - Show this help\n\nShortcuts:\n  Ctrl+C - Exit immediately\n  ESC ESC - Exit with confirmation\n  Tab - Toggle suggestions\n  ↑↓ - Scroll or navigate suggestions".to_string()
                ));
                self.scroll_to_bottom();
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

    fn collapse_suggestions(&mut self) {
        self.show_suggestions = false;
        self.suggestion_idx = None;
    }

    fn clear_suggestions(&mut self) {
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
                    // Text attachment (may contain additional explanatory text)
                    if let Some(t) = &att.text {
                        if let Some(c) = &t.content {
                            if !c.trim().is_empty() {
                                self.chat.push(ChatEntry::Assistant(c.clone()));
                            }
                        }
                    }
                    // Query attachment
                    if let Some(q) = &att.query {
                        // Title or description may contain the assistant's explanation
                        if let Some(title) = &q.title {
                            if !title.trim().is_empty() {
                                self.chat.push(ChatEntry::Assistant(title.clone()));
                            }
                        }
                        if let Some(desc) = &q.description {
                            if !desc.trim().is_empty() {
                                self.chat.push(ChatEntry::Assistant(desc.clone()));
                            }
                        }
                        if let Some(query) = &q.query {
                            // Store SQL to attach to table result, don't push as separate entry
                            self.pending_sql = Some(query.clone());
                            sql = Some(query.clone());
                        }
                    }
                    // Suggested follow-up questions
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
        let sql = self.pending_sql.take();

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
                    self.chat.push(ChatEntry::Table {
                        sql,
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
