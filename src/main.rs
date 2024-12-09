use crate::app::App;
use crate::theme::catppuccin::Theme;
use crate::tui::layout::centered_rect;
use crate::widgets::typing::TypingWidget;
use async_trait::async_trait;
use rand_core::OsRng;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Wrap;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{Terminal, TerminalOptions, Viewport};
use russh::keys::ssh_key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use russh_keys::Algorithm;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::terminal::Terminal as TuiTerminal;

mod app;
mod data;
mod events;
mod theme;
mod tui;
mod widgets;
type TerminalAppDB = HashMap<usize, (Terminal<CrosstermBackend<TuiTerminal>>, App)>;

#[derive(Clone)]
struct AppServer {
    clients: Arc<Mutex<TerminalAppDB>>,
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
        let clients = self.clients.clone();
        let theme = Theme::macchiato();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                for (_, (terminal, app)) in clients.lock().await.iter_mut() {
                    terminal
                        .draw(|f| {
                            let (main_area, bottom_bar_area) = centered_rect(f.area(), 80, 80, 3);

                            let typing = TypingWidget::new(&app.message, app.scroll_position)
                                .frame(app.current_frame)
                                .style(theme.text)
                                .alignment(Alignment::Left)
                                .wrap(Some(Wrap { trim: true }));

                            f.render_widget(typing, main_area);

                            let key_hints = Paragraph::new(Line::from(vec![
                                Span::styled("q / Ctrl+c", theme.highlight),
                                Span::raw(" to quit, "),
                                Span::styled("Up/Down or k/j", theme.highlight),
                                Span::raw(" to scroll"),
                            ]))
                            .alignment(Alignment::Center)
                            .block(Block::default());

                            f.render_widget(key_hints, bottom_bar_area);
                        })
                        .unwrap();
                }
            }
        });

        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
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
impl Handler for AppServer {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let terminal_handle = TuiTerminal::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);

        // the correct viewport area will be set when the client request a pty
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };

        let terminal = Terminal::with_options(backend, options)?;
        let app = App::new();

        let mut clients = self.clients.lock().await;
        clients.insert(self.id, (terminal, app));

        Ok(true)
    }

    async fn auth_publickey(&mut self, _: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        match data {
            // Pressing 'q' closes the connection.
            b"q" => {
                self.clients.lock().await.remove(&self.id);
                session.close(channel)?;
            }
            // Pressing 'c' resets the counter for the app.
            // Only the client with the id sees the counter reset.
            b"c" => {
                let mut clients = self.clients.lock().await;
                let (_, app) = clients.get_mut(&self.id).unwrap();
            }
            _ => {}
        }

        Ok(())
    }

    /// The client's window size has changed.
    async fn window_change_request(
        &mut self,
        _: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let (terminal, _) = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        Ok(())
    }

    /// The client requests a pseudo-terminal with the given
    /// specifications.
    ///
    /// **Note:** Success or failure should be communicated to the client by calling
    /// `session.channel_success(channel)` or `session.channel_failure(channel)` respectively.
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
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let (terminal, _) = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        session.channel_success(channel)?;

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
        });
    }
}

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
}
