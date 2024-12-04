use crate::events::AppEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use std::error::Error;

pub struct App {
    pub running: bool,
    pub(crate) message: String,
    pub(crate) current_frame: usize,
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
                self.message = format!("This is frame number: {}", self.current_frame);
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
