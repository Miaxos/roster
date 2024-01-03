use std::net::SocketAddr;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use std::thread::JoinHandle;

use derive_builder::Builder;
use monoio::net::TcpListener;
use tracing::{error, info};

mod connection;
mod context;
mod frame;
mod handle;
use handle::Handler;

mod cmd;

use crate::application::server::connection::Connection;

#[derive(Debug, Builder, Clone)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ServerConfig {
    bind_addr: SocketAddr,
    connections_limit: Arc<AtomicU16>,
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        type Driver = monoio::IoUringDriver;
    } else {
        type Driver = monoio::LegacyDriver;
    }
}

impl ServerConfig {
    pub fn initialize(self) -> ServerHandle {
        let mut threads = Vec::new();
        let cpus: usize = std::thread::available_parallelism().unwrap().into();

        for cpu in 1..cpus {
            let config = self.clone();
            let handle = std::thread::spawn(move || {
                info!("[Server] Spawned");
                monoio::utils::bind_to_cpu_set(Some(cpu)).unwrap();

                let mut rt = monoio::RuntimeBuilder::<Driver>::new()
                    .with_entries(32768)
                    .enable_timer()
                    .build()
                    .expect("Cannot build runtime");
                rt.block_on(async move {
                    let listener = TcpListener::bind(config.bind_addr)
                        .expect("Couldn't listen to addr");

                    loop {
                        // We accept the TCP Connection
                        let (conn, _addr) = listener
                            .accept()
                            .await
                            .expect("Unable to accept connections");

                        // We map it to an `Handler` which is able to understand
                        // the Redis protocol

                        let _spawned = monoio::spawn(async move {
                            info!(
                                "[Server] Accepted a new connection, will \
                                 read form it"
                            );

                            let mut handler = Handler {
                                connection: Connection::new(conn, 4 * 1024),
                            };

                            if let Err(err) = handler.run().await {
                                error!(?err);
                            }

                            info!("[Server] Connection terminated");
                        });
                    }
                });
            });
            threads.push(handle);
        }

        ServerHandle { threads }
    }
}

pub struct ServerHandle {
    threads: Vec<JoinHandle<()>>,
}

impl ServerHandle {
    pub fn join(self) {
        for t in self.threads {
            let _ = t.join();
        }
    }
}
