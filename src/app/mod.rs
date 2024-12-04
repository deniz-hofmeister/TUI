use crossterm::event::{Event, KeyCode, KeyEvent};
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

    pub fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn Error>> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => self.running = false,
                // Add other event handling here
                _ => {}
            }
        }
        Ok(())
    }
}
