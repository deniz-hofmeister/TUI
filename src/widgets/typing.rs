use crate::theme::catppuccin::{apply_custom_styles, Theme};
use ratatui::{prelude::*, widgets::*};

pub struct TypingWidget<'a> {
    text: &'a str,
    current_frame: usize,
    scroll_position: usize,
    scroll_speed: usize,
    style: Style,
    alignment: Alignment,
    wrap: Option<Wrap>,
    show_caret: bool,
}

impl<'a> TypingWidget<'a> {
    pub fn new(
        text: &'a str,
        scroll_position: usize,
        scroll_speed: usize,
    ) -> Self {
        Self {
            text,
            current_frame: 0,
            scroll_position,
            scroll_speed,
            style: Style::default(),
            alignment: Alignment::Left,
            wrap: Some(Wrap { trim: true }),
            show_caret: true,
        }
    }

    // Add new method to control caret visibility
    pub fn show_caret(
        mut self,
        show: bool,
    ) -> Self {
        self.show_caret = show;
        self
    }

    pub fn style(
        mut self,
        style: Style,
    ) -> Self {
        self.style = style;
        self
    }

    pub fn alignment(
        mut self,
        alignment: Alignment,
    ) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn frame(
        mut self,
        frame: usize,
    ) -> Self {
        self.current_frame = frame;
        self
    }

    pub fn wrap(
        mut self,
        wrap: Option<Wrap>,
    ) -> Self {
        self.wrap = wrap;
        self
    }
}

impl Widget for TypingWidget<'_> {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        let theme = Theme::macchiato();
        let mut visible_len = (self.current_frame * self.scroll_speed).min(self.text.len());
        if self.show_caret {
            visible_len = visible_len.saturating_sub(2);
        }
        let mut visible_text = self.text[..visible_len].to_string();

        // Add caret if enabled and text is not empty
        if self.show_caret && !visible_text.is_empty() {
            visible_text.push('█');
        }

        let mut parsed_text = tui_markdown::from_str(&visible_text);
        apply_custom_styles(&mut parsed_text, &theme);

        Paragraph::new(parsed_text)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(self.wrap.unwrap_or(Wrap { trim: true }))
            .scroll((self.scroll_position as u16, 0))
            .render(area, buf);
    }
}
