use crate::data::CV;
use crate::events::AppEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use std::error::Error;

pub struct App {
    pub running: bool,
    pub(crate) message: String,
    pub(crate) current_frame: usize,
    pub(crate) scroll_position: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            message: CV.into(),
            current_frame: 0,
            scroll_position: 0,
        }
    }

    pub fn handle_event(&mut self, event: AppEvent) -> Result<(), Box<dyn Error>> {
        match event {
            AppEvent::Tick => {
                self.current_frame += 10;
            }
            AppEvent::Key(key) => {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::NONE) => self.running = false,
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.running = false,
                    (KeyCode::Up | KeyCode::Char('k'), KeyModifiers::NONE) => {
                        if self.scroll_position > 0 {
                            self.scroll_position -= 1;
                        }
                    }
                    (KeyCode::Down | KeyCode::Char('j'), KeyModifiers::NONE) => {
                        let total_lines = self.message.lines().count();
                        let max_scroll = total_lines.saturating_sub(16);
                        if self.scroll_position < max_scroll {
                            self.scroll_position += 1;
                        }
                    }
                    _ => {}
                };
            }
        }

        Ok(())
    }
}
