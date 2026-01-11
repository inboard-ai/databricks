use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone)]
pub enum Message {
    Insert(char),
    Backspace,
    Delete,
    Left,
    Right,
    Home,
    End,
    Submit,
}

pub enum OutMessage {
    Submit(String),
}

pub struct Model {
    pub text: String,
    pub cursor: usize,
    pub focused: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            focused: true,
        }
    }

    pub fn handle_event(&self, key: KeyEvent) -> Option<Message> {
        if !self.focused {
            return None;
        }

        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Message::Insert(c))
            }
            KeyCode::Backspace => Some(Message::Backspace),
            KeyCode::Delete => Some(Message::Delete),
            KeyCode::Left => Some(Message::Left),
            KeyCode::Right => Some(Message::Right),
            KeyCode::Home => Some(Message::Home),
            KeyCode::End => Some(Message::End),
            KeyCode::Enter => {
                if !self.text.trim().is_empty() {
                    Some(Message::Submit)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<OutMessage> {
        match msg {
            Message::Insert(c) => {
                self.text.insert(self.cursor, c);
                self.cursor += 1;
                None
            }
            Message::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                }
                None
            }
            Message::Delete => {
                if self.cursor < self.text.len() {
                    self.text.remove(self.cursor);
                }
                None
            }
            Message::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                None
            }
            Message::Right => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                }
                None
            }
            Message::Home => {
                self.cursor = 0;
                None
            }
            Message::End => {
                self.cursor = self.text.len();
                None
            }
            Message::Submit => {
                let text = std::mem::take(&mut self.text);
                self.cursor = 0;
                Some(OutMessage::Submit(text))
            }
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
