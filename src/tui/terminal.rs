use crate::{theme::catppuccin::Theme, tui::layout::centered_rect};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use std::{error::Error, io::Stderr};

use crate::{app::App, widgets::typing::TypingWidget};

pub struct Terminal {
    terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<Stderr>>,
}

impl Terminal {
    pub fn draw(
        &mut self,
        app: &App,
    ) -> Result<(), Box<dyn Error>> {
        let theme = Theme::macchiato();
        self.terminal.draw(|f| {
            let (main_area, bottom_bar_area) = centered_rect(f.area(), 80, 80, 2);

            let mut msg = if app.current_frame < 70 {
                TypingWidget::new(&app.splash, app.scroll_position, 1)
                    .frame(app.current_frame)
                    .style(theme.text)
                    .show_caret(true)
                    .alignment(Alignment::Center)
                    .wrap(Some(Wrap { trim: true }))
            } else {
                TypingWidget::new(&app.message, app.scroll_position, 10)
                    .frame(app.current_frame.saturating_sub(70))
                    .style(theme.text)
                    .show_caret(true)
                    .alignment(Alignment::Left)
                    .wrap(Some(Wrap { trim: true }))
            };

            if msg.is_finished() {
                msg.show_caret = app.caret_visible;
            }

            f.render_widget(msg, main_area);

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
