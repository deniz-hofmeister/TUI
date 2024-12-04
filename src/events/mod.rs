use crossterm::event::{self, Event, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

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

        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                // Poll for events
                if event::poll(timeout).unwrap() {
                    if let Ok(Event::Key(key)) = event::read() {
                        let _ = event_tx.send(AppEvent::Key(key));
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    let _ = event_tx.send(AppEvent::Tick);
                    last_tick = Instant::now();
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<AppEvent, Box<dyn std::error::Error>> {
        Ok(self.rx.recv()?)
    }
}
