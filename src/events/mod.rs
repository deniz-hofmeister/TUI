use crossterm::event::KeyEvent;
use std::{sync::mpsc, time::Duration};

#[derive(Debug, Clone, Copy)]
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

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                interval.tick().await;
                if event_tx.send(AppEvent::Tick).is_err() {
                    break;
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<AppEvent, ()> {
        self.rx.recv().map_err(|_| ())
    }
}
