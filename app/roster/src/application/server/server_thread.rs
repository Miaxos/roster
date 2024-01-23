use std::rc::Rc;
use std::thread::JoinHandle;

use futures::future::Join;
use monoio::net::{ListenerConfig, TcpListener, TcpStream};

use super::ServerConfig;
use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::handle::Handler;
use crate::domain::dialer::{Dialer, RootDialer};
use crate::domain::storage::{Storage, StorageSegment};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        type Driver = monoio::IoUringDriver;
    } else {
        type Driver = monoio::LegacyDriver;
    }
}

/// This structure is used only in a single thread.
pub struct ServerMonoThreadedHandle {
    config: ServerConfig,
    /// Communication layer which will know if we need to send the message
    /// through another thread or through another TCP connection.
    dial: Dialer,
    /// The cpu where the thread should be binded here.
    cpu: usize,
    /// The [StorageSegment] for this thread.
    storage: StorageSegment,
}

pub struct ServerMonoThreadedInitialized {
    handle: JoinHandle<()>,
}

impl ServerMonoThreadedHandle {
    /// Start a new server thread on the cpu_core
    pub fn new(
        config: ServerConfig,
        dialer: &RootDialer,
        cpu: usize,
        storage: &Storage,
    ) -> Self {
        let (slot, storage_segment) = storage.part(cpu as u16);
        let dialer = dialer.part(cpu as u16).unwrap();

        Self {
            config,
            dial: dialer,
            cpu,
            storage: storage_segment,
        }
    }

    pub fn initialize(self) -> JoinHandle<()> {
        let handle = std::thread::spawn(move || {
            monoio::utils::bind_to_cpu_set(Some(self.cpu)).unwrap();

            let mut rt = monoio::RuntimeBuilder::<Driver>::new()
                .with_entries(1024)
                .enable_timer()
                .build()
                .expect("Cannot build runtime");

            rt.block_on(async move {
                let listener = TcpListener::bind_with_config(
                    self.config.bind_addr,
                    &ListenerConfig::new().backlog(16192),
                )?;

                let shard = Rc::new(self.dial.shard);
                // We initialize the listener on the TCP for this thread.
                loop {
                    // TODO(@miaxos): Check cancellation
                    let storage = self.storage.clone();
                    let shard = shard.clone();

                    // We accept the TCP Connection
                    let (conn, _addr) = listener
                        .accept()
                        .await
                        .expect("Unable to accept connections");

                    conn.set_nodelay(true).unwrap();
                    let ctx = Context::new(storage);

                    // We map it to an `Handler` which is able to understand
                    // the Redis protocol
                    let _spawned = monoio::spawn(async move {
                        let (connection, r) =
                            WriteConnection::new(conn, 4 * 1024);

                        let handler = Handler {
                            connection,
                            connection_r: r,
                            shard,
                        };

                        if let Err(err) = handler.run(ctx).await {
                            dbg!(err.backtrace());
                            dbg!(&err);
                            // error!(?err);
                            panic!("blbl");
                        }
                        // handler.connection.stop().await.unwrap();
                    });
                }

                #[allow(unreachable_code)]
                Ok::<(), anyhow::Error>(())
            });
        });

        handle
    }
}
