mod app;
mod events;
mod tui;

use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::terminal::Terminal::new()?;
    let mut app = app::App::new();
    let events = events::EventHandler::new(Duration::from_millis(100));

    while app.running {
        terminal.draw(&app)?;

        if let Ok(event) = events.next() {
            app.handle_event(event)?;
        }
    }

    Ok(())
}
