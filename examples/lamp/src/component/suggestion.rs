use crossterm::event::{KeyCode, KeyEvent};

#[derive(Clone)]
pub enum Message {
    Toggle,
    SelectPrev,
    SelectNext,
    Accept,
    Dismiss,
}

pub enum OutMessage {
    AcceptSuggestion(String),
}

pub struct Model {
    pub items: Vec<String>,
    pub selected: Option<usize>,
    pub expanded: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            expanded: false,
        }
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<String>) {
        self.items = suggestions;
        self.expanded = false; // Default to collapsed
        self.selected = if self.items.is_empty() { None } else { Some(0) };
    }

    pub fn handle_event(&self, key: KeyEvent) -> Option<Message> {
        if self.items.is_empty() {
            return None;
        }

        if self.expanded {
            match key.code {
                KeyCode::Up => Some(Message::SelectPrev),
                KeyCode::Down => Some(Message::SelectNext),
                KeyCode::Enter => Some(Message::Accept),
                KeyCode::Esc => Some(Message::Dismiss),
                KeyCode::Tab => Some(Message::Toggle),
                _ => Some(Message::Dismiss),
            }
        } else {
            match key.code {
                KeyCode::Tab => Some(Message::Toggle),
                _ => None,
            }
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<OutMessage> {
        match msg {
            Message::Toggle => {
                self.expanded = !self.expanded;
                if self.expanded && self.selected.is_none() && !self.items.is_empty() {
                    self.selected = Some(0);
                }
                None
            }
            Message::SelectPrev => {
                if let Some(idx) = self.selected {
                    self.selected = Some(if idx == 0 {
                        self.items.len() - 1
                    } else {
                        idx - 1
                    });
                }
                None
            }
            Message::SelectNext => {
                if let Some(idx) = self.selected {
                    self.selected = Some(if idx + 1 >= self.items.len() {
                        0
                    } else {
                        idx + 1
                    });
                }
                None
            }
            Message::Accept => {
                if let Some(idx) = self.selected {
                    if let Some(question) = self.items.get(idx).cloned() {
                        self.expanded = false;
                        return Some(OutMessage::AcceptSuggestion(question));
                    }
                }
                None
            }
            Message::Dismiss => {
                self.expanded = false;
                None
            }
        }
    }

    pub fn has_suggestions(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn is_expanded(&self) -> bool {
        self.expanded && !self.items.is_empty()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
