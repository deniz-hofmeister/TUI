mod app;
mod events;
mod tui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::terminal::Terminal::new()?;
    let mut app = app::App::new();

    while app.running {
        terminal.draw(&app)?;

        if let Some(event) = terminal.next_event()? {
            app.handle_event(event)?;
        }
    }

    Ok(())
}
