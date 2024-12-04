use catppuccin::Flavor;
use ratatui::style::{Color, Style};

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
                palette.mauve.rgb.r,
                palette.mauve.rgb.g,
                palette.mauve.rgb.b,
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
