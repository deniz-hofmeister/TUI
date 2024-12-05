use ratatui::{prelude::*, widgets::*};

pub struct TypingWidget<'a> {
    text: &'a str,
    current_frame: usize,
    style: Style,
    alignment: Alignment,
    wrap: Option<Wrap>,
}

impl<'a> TypingWidget<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            current_frame: 0,
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
        let visible_text =
            tui_markdown::from_str(&self.text[..self.current_frame.min(self.text.len())]);

        Paragraph::new(visible_text)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(self.wrap.unwrap_or(Wrap { trim: true }))
            .render(area, buf);
    }
}
