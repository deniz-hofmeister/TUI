use crate::theme::catppuccin::Theme;
use crate::tui::layout::centered_rect;
use ratatui::prelude::*;
use ratatui::widgets::Wrap;
use ratatui::widgets::{Block, Paragraph};
use russh::server::*;
use russh::ChannelId;
use std::{error::Error, io::Stderr};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::app::App;
use crate::widgets::typing::TypingWidget;

pub struct Terminal {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl Terminal {
    pub fn draw(f: &mut ratatui::Frame, app: &App) -> Result<(), Box<dyn Error>> {
        let theme = Theme::macchiato();
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

        Ok(())
    }
    pub async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let result = handle.data(channel_id, data.into()).await;
                if result.is_err() {
                    eprintln!("Failed to send data: {:?}", result);
                }
            }
        });

        Self {
            sender,
            sink: Vec::new(),
        }
    }
}
impl std::io::Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let result = self.sender.send(self.sink.clone());
        if let Err(err) = result {
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, err));
        }

        self.sink.clear();
        Ok(())
    }
}
