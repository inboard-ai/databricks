use crossterm::event::{KeyCode, KeyEvent};

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
pub enum Focus {
    None,
    Entry(usize),
}

#[derive(Clone)]
pub enum Message {
    Scroll(i16),
    FocusUp,
    FocusDown,
    ToggleExpand,
    ExitFocus,
}

pub enum OutMessage {
    ReturnToInput,
}

pub struct Model {
    pub entries: Vec<ChatEntry>,
    pub scroll: u16,
    pub max_scroll: u16,
    pub focus: Focus,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            scroll: 0,
            max_scroll: 0,
            focus: Focus::None,
        }
    }

    pub fn handle_event(&self, key: KeyEvent) -> Option<Message> {
        match self.focus {
            Focus::None => None,
            Focus::Entry(_) => match key.code {
                KeyCode::Up => Some(Message::FocusUp),
                KeyCode::Down => Some(Message::FocusDown),
                KeyCode::Char(' ') => Some(Message::ToggleExpand),
                KeyCode::Esc | KeyCode::Enter => Some(Message::ExitFocus),
                KeyCode::PageUp => Some(Message::Scroll(-10)),
                KeyCode::PageDown => Some(Message::Scroll(10)),
                _ => None,
            },
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<OutMessage> {
        match msg {
            Message::Scroll(delta) => {
                let new_scroll = self.scroll as i32 + delta as i32;
                self.scroll = new_scroll.clamp(0, self.max_scroll as i32) as u16;
                None
            }
            Message::FocusUp => {
                if let Focus::Entry(idx) = self.focus {
                    if idx > 0 {
                        self.focus = Focus::Entry(idx - 1);
                    }
                }
                None
            }
            Message::FocusDown => {
                if let Focus::Entry(idx) = self.focus {
                    if idx + 1 < self.entries.len() {
                        self.focus = Focus::Entry(idx + 1);
                    } else {
                        self.focus = Focus::None;
                        return Some(OutMessage::ReturnToInput);
                    }
                }
                None
            }
            Message::ToggleExpand => {
                if let Focus::Entry(idx) = self.focus {
                    if let Some(ChatEntry::Table { expanded, .. }) = self.entries.get_mut(idx) {
                        *expanded = !*expanded;
                    }
                }
                None
            }
            Message::ExitFocus => {
                self.focus = Focus::None;
                Some(OutMessage::ReturnToInput)
            }
        }
    }

    pub fn push(&mut self, entry: ChatEntry) {
        self.entries.push(entry);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll = 0;
        self.focus = Focus::None;
    }

    pub fn focus_last(&mut self) {
        if !self.entries.is_empty() {
            self.focus = Focus::Entry(self.entries.len() - 1);
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll = self.max_scroll;
    }

}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
