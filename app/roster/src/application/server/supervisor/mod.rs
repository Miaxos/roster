use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::{Arc};

use bytestring::ByteString;
use futures_locks::RwLock;
use scc::HashMap;

/// [Supervisor] is the Applicative layer that allow you to interact with the
/// connections currently open in roster.
///
/// The [Supervisor] is local only for now, it's not a Global Supervisor.
///
/// What we could have is also the cluster inside the supervisor with the
/// capability to synchronise the whole cluster together for the ACL & the
/// connections data.
///
/// **Should be cheap to clone**
#[derive(Debug, Clone)]
pub struct Supervisor {
    /// The ID is monotonically incremental. If the ID of a connection is
    /// greater than the ID of another connection, it is guaranteed that the
    /// second connection was established with the server at a later time.
    ///
    /// Note: This is restarted at each restart, in a cluster
    current_id: Arc<AtomicU64>,

    // TODO(@miaxos): Think about having a Weak here, as it shouldn't matter if
    // there are no connection anymore.
    current_connections: Arc<HashMap<u64, Arc<MetadataConnection>>>,
}

impl Supervisor {
    pub fn new(init_connection: u64) -> Self {
        Supervisor {
            current_id: Arc::new(AtomicU64::new(init_connection)),
            current_connections: Default::default(),
        }
    }

    /// Assign a new connection to the [Supervisor] and return a
    /// [MetadataConnection]
    pub fn assign_new_connection(
        &self,
        addr: SocketAddr,
        laddr: SocketAddr,
        fd: i32,
    ) -> Arc<MetadataConnection> {
        let id = self
            .current_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let conn = Arc::new(MetadataConnection {
            id,
            kind: MetadataConnectionKind::Normal,
            stopped: AtomicBool::new(false),
            name: RwLock::new(None),
            addr,
            laddr,
            fd,
        });
        self.current_connections
            .insert(id, conn.clone())
            .expect("Can't fail");
        conn
    }

    /// Get the list of [MetadataConnection] for normal connection.
    pub async fn get_normal_connection(&self) -> Vec<Arc<MetadataConnection>> {
        let mut result = Vec::new();

        self.current_connections
            .scan_async(|_, conn| {
                let stopped =
                    conn.stopped.load(std::sync::atomic::Ordering::Relaxed);

                if !stopped && conn.kind == MetadataConnectionKind::Normal {
                    result.push(conn.clone());
                }
            })
            .await;

        result
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetadataConnectionKind {
    Normal,
}

/// [MetadataConnection] is where we store metadata about a Connection.
#[derive(Debug)]
pub struct MetadataConnection {
    /// The associated ID for the connection
    pub id: u64,
    /// Describe the connection kind
    pub kind: MetadataConnectionKind,
    /// the name set by the client with CLIENT SETNAME
    name: RwLock<Option<ByteString>>,
    /// Tell if the connection is stopped
    pub stopped: AtomicBool,
    /// Address/Port of the client
    pub addr: SocketAddr,
    /// address/port of local address client connected to (bind address)
    pub laddr: SocketAddr,
    /// file descriptor corresponding to the socket
    pub fd: i32,
}

impl MetadataConnection {
    /// Indicate this connection is stopped.
    pub fn stop(&self) {
        self.stopped
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Set the name of the connection
    pub async fn set_name(&self, name: ByteString) {
        let mut lock = self.name.write().await;
        *lock = Some(name);
    }

    /// Set the name of the connection
    pub async fn name(&self) -> Option<ByteString> {
        self.name.read().await.clone()
    }
}

impl MetadataConnection {
    pub fn id(&self) -> u64 {
        self.id
    }
}
