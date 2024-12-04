use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::Paragraph};
use std::{error::Error, io::Stderr};

use crate::app::App;

pub struct Terminal {
    terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<Stderr>>,
}

impl Terminal {
    pub fn draw(&mut self, app: &App) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = super::layout::centered_rect(frame.area(), 50, 10);
            frame.render_widget(
                Paragraph::new(app.message.as_str()).alignment(Alignment::Center),
                area,
            );
        })?;
        Ok(())
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(std::io::stderr(), EnterAlternateScreen)?;
        let terminal =
            ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stderr()))?;

        Ok(Self { terminal })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stderr(), LeaveAlternateScreen);
    }
}
