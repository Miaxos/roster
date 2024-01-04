#![feature(negative_impls)]
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use application::server::ServerConfigBuilder;
use infrastructure::config::Cfg;
use infrastructure::instruments::Instruments;

fn main() -> anyhow::Result<()> {
    // Initialize config
    let config = Cfg::from_env()?;

    // Initialize tracing
    // TODO: tracing slow af, should change to minitrace-rust
    // let _ = Instruments::new()?;

    // Initialize memory storage

    // Initialize Roster WAN clusturing

    // Initialize Roster LAN clusturing

    // Initialize server with Redis Protocol to accept connections;
    let server = ServerConfigBuilder::default()
        .connections_limit(Arc::new(config.max_connection.into()))
        .bind_addr(config.bind_addr)
        .build()
        .expect("Couldn't create the config")
        .initialize();

    server.join();

    Ok(())
}
