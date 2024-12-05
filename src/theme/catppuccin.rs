use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Text;

pub struct Theme {
    pub text: Style,
    pub highlight: Style,
    pub background: Color,
    pub surface: Color,
}

impl Theme {
    pub fn macchiato() -> Self {
        let palette = catppuccin::PALETTE.macchiato.colors;

        Self {
            text: Style::default().fg(Color::Rgb(
                palette.text.rgb.r,
                palette.text.rgb.g,
                palette.text.rgb.b,
            )),
            highlight: Style::default().fg(Color::Rgb(
                palette.teal.rgb.r,
                palette.teal.rgb.g,
                palette.teal.rgb.b,
            )),
            background: Color::Rgb(palette.base.rgb.r, palette.base.rgb.g, palette.base.rgb.b),
            surface: Color::Rgb(
                palette.surface0.rgb.r,
                palette.surface0.rgb.g,
                palette.surface0.rgb.b,
            ),
        }
    }
}

pub fn apply_custom_styles(text: &mut Text, theme: &Theme) {
    for line in text.lines.iter_mut() {
        if let Some(first_span) = line.spans.first() {
            if first_span.content.starts_with("# ") || first_span.content.starts_with("## ") {
                line.style = theme.highlight;
            }
        }
    }
}
