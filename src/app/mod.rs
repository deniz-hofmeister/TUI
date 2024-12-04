use crate::events::AppEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use std::error::Error;

pub struct App {
    pub running: bool,
    pub message: String,
    current_frame: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            message: "".into(),
            current_frame: 0,
        }
    }

    pub fn handle_event(&mut self, event: AppEvent) -> Result<(), Box<dyn Error>> {
        match event {
            AppEvent::Tick => {
                self.message = self.current_frame.to_string();
                self.current_frame += 1;
            }
            AppEvent::Key(key) => {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => self.running = false,
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.running = false,
                    _ => {}
                };
            }
        }

        Ok(())
    }
}
