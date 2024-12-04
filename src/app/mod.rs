use crate::events::AppEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use std::error::Error;

pub struct App {
    pub running: bool,
    pub message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            message: String::from("Welcome"),
        }
    }

    pub fn handle_event(&mut self, event: AppEvent) -> Result<(), Box<dyn Error>> {
        if let AppEvent::Key(key) = event {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => self.running = false,
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.running = false,
                _ => {}
            };
        };

        Ok(())
    }
}
