use crossterm::event::{self, Event, KeyEvent};
use std::{sync::mpsc, thread, time::Duration};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<AppEvent>,
    _tx: mpsc::Sender<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        // Spawn event polling thread
        thread::spawn(move || -> ! {
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let Ok(Event::Key(key)) = event::read() {
                        let _ = event_tx.send(AppEvent::Key(key));
                    }
                }
                let _ = event_tx.send(AppEvent::Tick);
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<AppEvent, Box<dyn std::error::Error>> {
        Ok(self.rx.recv()?)
    }
}
