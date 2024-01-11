//! The whole redis server implementation is here.
use std::net::SocketAddr;
use std::os::fd::{FromRawFd, RawFd};
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use std::thread::JoinHandle;

use crc::{Crc, CRC_16_XMODEM};
use derive_builder::Builder;
use futures::StreamExt;
use monoio::net::{ListenerConfig, TcpListener, TcpStream};

mod connection;
mod context;
pub mod frame;
mod handle;
use handle::Handler;
use monoio::time::Instant;

mod cmd;

use crate::application::server::connection::WriteConnection;
use crate::application::server::context::Context;
use crate::application::server::handle::ConnectionMsg;
use crate::domain;

#[derive(Debug, Builder, Clone)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ServerConfig {
    bind_addr: SocketAddr,
    #[allow(dead_code)]
    connections_limit: Arc<AtomicU16>,
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        type Driver = monoio::IoUringDriver;
    } else {
        type Driver = monoio::LegacyDriver;
    }
}

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);

pub const fn crc_hash(bytes: &[u8]) -> u16 {
    CRC.checksum(bytes) % 16384
}

impl ServerConfig {
    pub fn initialize(self) -> ServerHandle {
        let mut threads = Vec::new();
        let cpus: usize = std::thread::available_parallelism().unwrap().into();

        type Msg = ConnectionMsg;
        let mesh = sharded_thread::mesh::MeshBuilder::<Msg>::new().unwrap();

        let mut slots = Vec::new();
        for cpu in 0..cpus {
            let part_size: u16 = 16384 / cpus as u16;
            let remainder: u16 = 16384 % cpus as u16;

            let start = cpu as u16 * part_size as u16;
            let end = if cpu == cpus - 1 {
                (cpu as u16 + 1) * part_size + remainder as u16
            } else {
                (cpu as u16 + 1) * part_size as u16
            };

            let slot = start..end;
            slots.push(slot);
        }

        for cpu in 0..cpus {
            let config = self.clone();
            let shard = mesh.join_with(cpu).unwrap();

            let slot = slots.get(cpu).unwrap().clone();
            let slots = slots.clone();

            let handle = std::thread::spawn(move || {
                dbg!(cpu);
                monoio::utils::bind_to_cpu_set(Some(cpu)).unwrap();

                let mut rt = monoio::RuntimeBuilder::<Driver>::new()
                    .with_entries(1024)
                    .enable_timer()
                    .build()
                    .expect("Cannot build runtime");

                rt.block_on(async move {
                    // Initialize domain
                    let storage = domain::storage::Storage::new(slots, slot);
                    let shard = Rc::new(shard);

                    let listener = TcpListener::bind_with_config(
                        config.bind_addr,
                        &ListenerConfig::new().backlog(16192),
                    )
                    .expect("Couldn't listen to addr");

                    // Start the async task which is able to receive connection
                    // from other thread
                    let storage_inter_thread = storage.clone();
                    let shard_inter_thread = shard.clone();
                    monoio::spawn(async move {
                        let storage = storage_inter_thread;
                        let shard = shard_inter_thread;

                        let mut receiver = shard.receiver().unwrap();

                        loop {
                            let shard = shard.clone();
                            let ctx = Context::new(storage.clone());

                            // Pre-allocate next buffer;

                            if let Some(ConnectionMsg {
                                fd,
                                current_command,
                                rest_frame,
                            }) = receiver.next().await
                            {
                                let _spawned = monoio::spawn(async move {
                                    // TODO: We miss things in the buffer right
                                    // now &
                                    // pipelining
                                    // Already accepted tcp stream, we don't
                                    // need to
                                    // accept it again.
                                    let tcp = unsafe {
                                        std::net::TcpStream::from_raw_fd(fd)
                                    };
                                    let conn =
                                        TcpStream::from_std(tcp).unwrap();
                                    conn.set_nodelay(true).unwrap();

                                    let (connection, r) =
                                        WriteConnection::new(conn, 4 * 1024);

                                    let handler = Handler {
                                        connection,
                                        connection_r: r,
                                        shard,
                                    };

                                    if let Err(err) = handler
                                        .continue_run(ctx, current_command)
                                        .await
                                    {
                                        dbg!(err.backtrace());
                                        dbg!(&err);
                                        // error!(?err);
                                        panic!("blbl");
                                    }
                                    // handler.connection.stop().await.unwrap();
                                });
                            } else {
                                break;
                            }
                        }
                    });

                    loop {
                        let storage = storage.clone();
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
                                shard: shard.clone(),
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
