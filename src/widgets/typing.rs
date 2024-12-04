use ratatui::{prelude::*, widgets::*};

pub struct TypingWidget<'a> {
    text: &'a str,
    current_frame: usize,
    style: Style,
    alignment: Alignment,
}

impl<'a> TypingWidget<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            current_frame: 0,
            style: Style::default(),
            alignment: Alignment::Left,
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
}

impl Widget for TypingWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let visible_text = &self.text[..self.current_frame.min(self.text.len())];

        Paragraph::new(Line::from(visible_text).alignment(self.alignment))
            .style(self.style)
            .render(area, buf);
    }
}
