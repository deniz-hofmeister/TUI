use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;

pub fn centered_rect(
    r: Rect,
    percent_x: u16,
    percent_y: u16,
    bottom_bar_height: u16,
) -> (Rect, Rect) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(percent_y),
            Constraint::Min(3),
        ])
        .split(r);

    let bottom_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(bottom_bar_height)])
        .split(vertical_layout[2]);

    let main_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical_layout[1])[1];

    (main_area, bottom_layout[1])
}
