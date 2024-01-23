//! The whole redis server implementation is here.
use std::net::SocketAddr;
use std::sync::atomic::AtomicU16;
use std::sync::Arc;
use std::thread::JoinHandle;

use derive_builder::Builder;

mod connection;
mod context;
pub mod frame;
pub(crate) mod handle;

mod cmd;
mod server_thread;

use self::server_thread::ServerMonoThreadedHandle;
use crate::application::server::handle::ConnectionMsg;
use crate::domain::dialer::{RootDialer, Slot};
use crate::domain::storage::Storage;
use crate::infrastructure::hash::HASH_SLOT_MAX;

#[derive(Debug, Builder, Clone)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ServerConfig {
    bind_addr: SocketAddr,
    #[allow(dead_code)]
    connections_limit: Arc<AtomicU16>,
}

impl ServerConfig {
    /// Initialize a Server by starting the thread for each core and assigning
    /// the storage segment & hash slots based on the configuration.
    pub fn initialize(self) -> ServerHandle {
        let mut threads = Vec::new();
        let cpus: usize = std::thread::available_parallelism().unwrap().into();

        // The mesh used to pass a whole connection if needed.
        let mesh =
            sharded_thread::mesh::MeshBuilder::<ConnectionMsg>::new(cpus)
                .unwrap();

        let config_slot = Slot::from(0..HASH_SLOT_MAX);
        let storage = Storage::new(1, config_slot);
        let main_dialer = RootDialer::new(mesh, &storage);

        for cpu in 0..cpus {
            // TODO(@miaxos): There are some links between those two, mb we
            // should modelise it again.
            let config = self.clone();
            let handle = ServerMonoThreadedHandle::new(
                config,
                &main_dialer,
                cpu,
                &storage,
            );

            threads.push(handle.initialize());
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
