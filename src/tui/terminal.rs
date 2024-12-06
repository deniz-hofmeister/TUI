use crate::theme::catppuccin::Theme;
use crate::tui::layout::centered_rect;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::Wrap;
use ratatui::widgets::{Block, Paragraph};
use std::{error::Error, io::Stderr};

use crate::app::App;
use crate::widgets::typing::TypingWidget;

pub struct Terminal {
    terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<Stderr>>,
}

impl Terminal {
    pub fn draw(&mut self, app: &App) -> Result<(), Box<dyn Error>> {
        let theme = Theme::macchiato();
        self.terminal.draw(|f| {
            let (main_area, bottom_bar_area) = centered_rect(f.area(), 80, 80, 3);

            let typing = TypingWidget::new(&app.message, app.scroll_position)
                .frame(app.current_frame)
                .style(theme.text)
                .alignment(Alignment::Left)
                .wrap(Some(Wrap { trim: true }));

            f.render_widget(typing, main_area);

            let key_hints = Paragraph::new(Line::from(vec![
                Span::styled("q / Ctrl+c", theme.highlight),
                Span::raw(" to quit, "),
                Span::styled("Up/Down or k/j", theme.highlight),
                Span::raw(" to scroll"),
            ]))
            .alignment(Alignment::Center)
            .block(Block::default());

            f.render_widget(key_hints, bottom_bar_area);
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
