use std::net::SocketAddr;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use std::time::Duration;

use derive_builder::Builder;
use monoio::io::AsyncReadRent;
use monoio::net::TcpListener;
use tracing::{error, info};

mod connection;
mod frame;
mod handle;
use handle::Handler;

mod cmd;

use crate::application::server::connection::Connection;

#[derive(Debug, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct Server {
    bind_addr: SocketAddr,
    connections_limit: Arc<AtomicU16>,
}

impl Server {
    pub fn run(self) {
        // Use IoUringDriver
        let mut rt = monoio::RuntimeBuilder::<monoio::LegacyDriver>::new()
            .enable_timer()
            .build()
            .expect("Cannot build runtime");

        rt.block_on(async move {
            let listener = TcpListener::bind(self.bind_addr)
                .expect("Couldn't listen to addr");

            loop {
                // We accept the TCP Connection
                let (conn, _addr) = listener
                    .accept()
                    .await
                    .expect("Unable to accept connections");

                // We map it to an `Handler` which is able to understand the Redis protocol

                let _spawned = monoio::spawn(async move {
                    info!(
                        "[Server] Accepted a new connection, will read form it"
                    );

                    let mut handler = Handler {
                        connection: Connection::new(conn, 4 * 1024),
                    };

                    if let Err(err) = handler.run().await {
                        error!(?err);
                    }

                    // monoio::time::sleep(Duration::from_secs(5)).await;
                });
            }
        })
    }
}
