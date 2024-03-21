#![feature(cell_update)]
#![allow(clippy::print_literal)]
#![feature(negative_impls)]
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use application::server::ServerConfigBuilder;
use infrastructure::config::Cfg;

// use crate::domain::cluster::Cluster;
// use infrastructure::instruments::Instruments;

#[cfg(debug_assertions)]
pub const VERSION: &str =
    concat!("(dev) ", env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"),);

#[cfg(not(debug_assertions))]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> anyhow::Result<()> {
    // Initialize config
    let config = Cfg::from_env()?;

    // Initialize tracing
    // TODO: tracing slow af, should change to minitrace-rust
    // let _ = Instruments::new()?;

    // Initialize memory storage

    // Initialize Roster WAN clusturing (federation)

    // Initialize Roster LAN clusturing
    // If no cluster, it means we run in `standalone` mode.

    // Initialize server with Redis Protocol to accept connections;
    let server = ServerConfigBuilder::default()
        .connections_limit(Arc::new(config.max_connection.into()))
        .bind_addr(config.bind_addr)
        .build()
        .expect("Couldn't create the config")
        .initialize();

    // From: https://ascii.co.uk/art/rooster
    println!(
        r###"
                      ~-.
    ,,,;            ~-.~-.~-            Roster {version}
   (.../           ~-.~-.~-.~-.~-.
   }} o~`,         ~-.~-.~-.~-.~-.~-.    Running in {mode} mode
   (/    \      ~-.~-.~-.~-.~-.~-.~-.   Port: {port}
    ;    \    ~-.~-.~-.~-.~-.~-.~-.     Addr: {addr}
   ;     {{_.~-.~-.~-.~-.~-.~-.~         PID: {pid}
  ;:  .-~`    ~-.~-.~-.~-.~-.
 ;.: :'    ._   ~-.~-.~-.~-.~-             https://github.com/miaxos/roster/
  ;::`-.    '-._  ~-.~-.~-.~-
   ;::. `-.    '-,~-.~-.~-.
    ';::::.`''-.-'
      ';::;;:,:'
         '||"
         / |
       ~` ~"'
        "###,
        version = VERSION,
        mode = "standalone",
        port = server.bind.port(),
        addr = server.bind.ip(),
        pid = std::process::id()
    );

    server.join();

    Ok(())
}
