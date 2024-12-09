use async_trait::async_trait;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use rand_core::OsRng;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use russh::server::Server;
use russh::server::{Auth, Handle, Msg, Session};
use russh::ChannelId;
use russh::{Channel, Pty};
use russh_keys::Algorithm;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

mod app;
mod data;
mod events;
mod theme;
mod tui;
mod widgets;

// Type alias for our SSH Terminal
type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

// TerminalHandle is used to bridge between the terminal backend and the SSH channel
struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    // The sink collects the data which is finally sent to sender.
    sink: Vec<u8>,
}

impl TerminalHandle {
    async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();

        // Spawn a task to send terminal output over SSH channel
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                let result = handle.data(channel_id, data.into()).await;
                if result.is_err() {
                    eprintln!("Failed to send data: {:?}", result);
                }
            }
        });

        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

// Implement Write trait for TerminalHandle to collect terminal output
impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let result = self.sender.send(self.sink.clone());
        if result.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                result.unwrap_err(),
            ));
        }

        self.sink.clear();
        Ok(())
    }
}

// Define an Event enum to represent input events
#[derive(Debug)]
enum Event {
    Input(KeyEvent),
    Tick,
}

// Client struct holds the terminal, app, and sender for events
struct Client {
    terminal: SshTerminal,
    app: app::App,
    event_sender: UnboundedSender<Event>,
}

#[derive(Clone)]
struct AppServer {
    clients: Arc<Mutex<HashMap<usize, UnboundedSender<Event>>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let config = russh::server::Config {
            auth_rejection_time: Duration::from_secs(3),
            keys: vec![russh_keys::PrivateKey::random(&mut OsRng, Algorithm::Ed25519).unwrap()],
            ..Default::default()
        };
        self.run_on_address(Arc::new(config), ("0.0.0.0", 2222))
            .await?;
        Ok(())
    }
}

impl Server for AppServer {
    type Handler = Self;

    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

#[async_trait]
impl russh::server::Handler for AppServer {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let terminal_handle = TerminalHandle::start(session.handle(), channel.id()).await;
        let backend = CrosstermBackend::new(terminal_handle);

        // Create the SSH terminal
        let terminal = Terminal::new(backend)?;
        let mut app = app::App::new();

        // Create event channels
        let (event_sender, mut event_receiver) = unbounded_channel();

        // Store the event sender for this client
        let client = Arc::clone(&self.clients);
        let id = self.id;
        let mut clients = self.clients.lock().await;
        clients.insert(self.id, event_sender.clone());

        // Spawn the main loop for the client
        tokio::spawn(async move {
            let mut terminal = terminal;
            let mut app = app;
            let mut tui = tui::terminal::Terminal::new().expect("Unable to create TUi app");
            // Create a periodic tick event
            let tick_rate = Duration::from_millis(25);
            let mut tick_interval = time::interval(tick_rate);

            loop {
                // Wait for either input event or tick
                tokio::select! {
                    _ = tick_interval.tick() => {
                        // Send tick event
                        if event_sender.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    Some(event) = event_receiver.recv() => {
                        match event {
                            Event::Input(key) => {
                                // Handle input event
                                if let Err(e) = app.handle_event(events::AppEvent::Key(key)) {
                                    eprintln!("Error handling event: {:?}", e);
                                    break;
                                }
                            }
                            Event::Tick => {
                                // Handle tick event if necessary
                            }
                        }
                    }
                }

                // Draw the app
                tui.draw(&app).expect("Unable to draw app");

                // Exit if the app is no longer running
                if !app.running {
                    break;
                }
            }

            // Clean up after client disconnects
            let mut c = client.lock().await;
            c.remove(&id);
        });

        Ok(true)
    }

    async fn auth_publickey(
        &mut self,
        _: &str,
        _: &russh_keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_password(
        &mut self,
        _: &str,
        _: &str,
    ) -> Result<russh::server::Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    // Handle data received from the SSH client
    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let input = String::from_utf8_lossy(data);
        let mut clients = self.clients.lock().await;
        if let Some(sender) = clients.get_mut(&self.id) {
            // Parse input into KeyEvent
            for c in input.chars() {
                let key_event = KeyCode::Char(c);
                let event = Event::Input(key_event.into());
                if sender.send(event).is_err() {
                    break;
                }
            }
        }
        Ok(())
    }

    /// Handle window size change requests
    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut clients = self.clients.lock().await;
        if let Some(sender) = clients.get_mut(&self.id) {
            // You can handle terminal resizing here if your app supports it
            // For example, you might send a resize event to the app
        }
        Ok(())
    }

    /// Handle PTY requests
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _: &str,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        session.channel_success(channel)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
}
