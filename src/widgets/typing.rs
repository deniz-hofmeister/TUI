use crate::theme::catppuccin::apply_custom_styles;
use crate::theme::catppuccin::Theme;
use ratatui::{prelude::*, widgets::*};

pub struct TypingWidget<'a> {
    text: &'a str,
    current_frame: usize,
    scroll_position: usize,
    style: Style,
    alignment: Alignment,
    wrap: Option<Wrap>,
}

impl<'a> TypingWidget<'a> {
    pub fn new(text: &'a str, scroll_position: usize) -> Self {
        Self {
            text,
            current_frame: 0,
            scroll_position,
            style: Style::default(),
            alignment: Alignment::Left,
            wrap: Some(Wrap { trim: true }),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn frame(mut self, frame: usize) -> Self {
        self.current_frame = frame;
        self
    }

    pub fn wrap(mut self, wrap: Option<Wrap>) -> Self {
        self.wrap = wrap;
        self
    }
}

impl Widget for TypingWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::macchiato();
        let mut visible_text =
            tui_markdown::from_str(&self.text[..self.current_frame.min(self.text.len())]);
        apply_custom_styles(&mut visible_text, &theme);
        Paragraph::new(visible_text)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(self.wrap.unwrap_or(Wrap { trim: true }))
            .scroll((self.scroll_position as u16, 0))
            .render(area, buf);
    }
}
