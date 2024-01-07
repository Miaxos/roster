//! The whole redis server implementation is here.
use std::net::SocketAddr;
use std::os::fd::AsRawFd;
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use derive_builder::Builder;
use monoio::net::{ListenerConfig, TcpListener};
use thread_priority::{set_current_thread_priority, ThreadPriorityValue};
use tracing::{error, info};

mod connection;
mod context;
pub mod frame;
mod handle;
use handle::Handler;

mod cmd;

use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::domain;

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

        for cpu in 0..cpus {
            let config = self.clone();
            let handle = std::thread::spawn(move || {
                // info!("[Server] Spawned");
                dbg!(cpu);
                monoio::utils::bind_to_cpu_set(Some(cpu)).unwrap();

                let mut rt = monoio::RuntimeBuilder::<Driver>::new()
                    .with_entries(1024)
                    .enable_timer()
                    .build()
                    .expect("Cannot build runtime");

                rt.block_on(async move {
                    // Initialize domain
                    let storage = domain::storage::Storage::new();

                    let listener = TcpListener::bind_with_config(
                        config.bind_addr,
                        &ListenerConfig::new(),
                    )
                    .expect("Couldn't listen to addr");

                    loop {
                        let storage = storage.clone();

                        // listener.cancelable_accept(c)?
                        // We accept the TCP Connection
                        let (conn, _addr) = listener
                            .accept()
                            .await
                            .expect("Unable to accept connections");

                        conn.set_nodelay(true).unwrap();
                        /*
                        conn.set_tcp_keepalive(
                            Some(Duration::from_secs(1)),
                            None,
                            None,
                        )
                        .unwrap();
                        */

                        // We map it to an `Handler` which is able to understand
                        // the Redis protocol

                        let _spawned = monoio::spawn(async move {
                            let (connection, r) =
                                WriteConnection::new(conn, 4 * 1024);

                            let mut handler = Handler {
                                connection,
                                connection_r: r,
                            };

                            let ctx = Context::new(storage);

                            if let Err(err) = handler.run(ctx).await {
                                dbg!(err.backtrace());
                                dbg!(&err);
                                // error!(?err);
                                panic!("blbl");
                            }
                            // handler.connection.stop().await.unwrap();
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
