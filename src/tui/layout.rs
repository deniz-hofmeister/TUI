use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
};

pub fn centered_rect(
    r: Rect,
    percent_x: u16,
    percent_y: u16,
    bottom_bar_height: u16,
) -> (Rect, Rect) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(bottom_bar_height),
            Constraint::Percentage(percent_y),
            Constraint::Min(bottom_bar_height),
        ])
        .split(r);

    let bottom_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(bottom_bar_height)])
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
