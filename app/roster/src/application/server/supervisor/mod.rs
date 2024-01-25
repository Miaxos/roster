
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use scc::HashMap;





/// [Supervisor] is the Applicative layer that allow you to interact with the
/// connections currently open in roster.
///
/// The [Supervisor] is local only, it's not a Global Supervisor.
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
    pub fn assign_new_connection(&self) -> Arc<MetadataConnection> {
        let id = self
            .current_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let conn = Arc::new(MetadataConnection { id });
        self.current_connections
            .insert(id, conn.clone())
            .expect("Can't fail");
        conn
    }
}

/// [MetadataConnection] is where we store metadata about a Connection.
#[derive(Debug)]
pub struct MetadataConnection {
    /// The associated ID for the connection
    pub id: u64,
}
