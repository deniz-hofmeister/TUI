mod app;
mod data;
mod events;
mod server;
mod theme;
mod tui;
mod widgets;

use crate::server::AppServer;

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
}
