mod application;
mod infrastructure;

use std::sync::Arc;

use application::server::{ServerBuilder, ServerBuilderError};
use infrastructure::config::Cfg;
use infrastructure::instruments::Instruments;
use tracing::info;

fn main() -> anyhow::Result<()> {
    // Initialize config
    let config = Cfg::from_env()?;

    // Initialize tracing
    let _ = Instruments::new()?;

    // Initialize memory storage

    // Initialize Roster WAN clusturing

    // Initialize Roster LAN clusturing

    // Initialize server to accept connections;
    let server = ServerBuilder::default()
        .connections_limit(Arc::new(config.max_connection.into()))
        .bind_addr(config.bind_addr)
        .build()
        .expect("Couldn't create the server");

    server.run();

    Ok(())
}
