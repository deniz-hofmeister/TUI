mod app;
mod data;
mod events;
mod theme;
mod tui;
mod widgets;

use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::terminal::Terminal::new()?;
    let mut app = app::App::new();
    let events = events::EventHandler::new(Duration::from_millis(25));

    while app.running {
        terminal.draw(&app)?;

        if let Ok(event) = events.next() {
            app.handle_event(event)?;
        }
    }

    Ok(())
}
