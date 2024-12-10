use crate::{
    data::{CV, SPLASH},
    events::AppEvent,
};
use crossterm::event::{KeyCode, KeyModifiers};
use std::{
    error::Error,
    time::{Duration, Instant},
};

pub struct App {
    pub running: bool,
    pub(crate) splash: String,
    pub(crate) message: String,
    pub(crate) current_frame: usize,
    pub(crate) scroll_position: usize,
    pub(crate) last_tick: Instant,
    pub(crate) caret_visible: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            message: CV.into(),
            splash: SPLASH.into(),
            current_frame: 0,
            scroll_position: 0,
            caret_visible: true,
            last_tick: Instant::now(),
        }
    }

    pub fn handle_event(
        &mut self,
        event: AppEvent,
    ) -> Result<(), Box<dyn Error>> {
        match event {
            AppEvent::Tick => {
                if self.last_tick.elapsed() >= Duration::from_millis(350) {
                    self.caret_visible = !self.caret_visible;
                    self.last_tick = Instant::now();
                }
                self.current_frame += 1;
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
