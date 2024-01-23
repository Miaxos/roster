use std::ops::{Deref, Range};
use std::rc::Rc;
use std::sync::Arc;

use sharded_thread::mesh::MeshBuilder;
use sharded_thread::shard::Shard;

use super::cluster::Cluster;
use super::storage::Storage;
use crate::application::server::handle::ConnectionMsg;

/// [RootDialer] is the layer which enable communication between Roster shards.
///
/// It should know only his local slots and also the slots initiated by other
/// CPU core locally. For every other slots, it should be delegated to the
/// [Cluster].
///
/// Should be cheap to clone
#[derive(Debug)]
pub struct RootDialer {
    mesh: MeshBuilder<ConnectionMsg>,
    cluster: Cluster,
    /// The whole slice of the current server
    global_slot: Slot,
    /// Slot -> part based on index
    inner_slots: Vec<Slot>,
}

impl RootDialer {
    pub fn new(mesh: MeshBuilder<ConnectionMsg>, storage: &Storage) -> Self {
        let inner_slots = storage.slots();
        let global_slot = storage.global_slot().clone();

        let cluster = Cluster {};

        Self {
            global_slot,
            inner_slots,
            cluster,
            mesh,
        }
    }
}

/// [Dialer] is the layer which enable communication between Roster shards.
///
/// It should know only his local slots and also the slots initiated by other
/// CPU core locally. For every other slots, it should be delegated to the
/// [Cluster].
///
/// Should be cheap to clone
#[derive(Debug, Clone)]
pub struct Dialer {
    shard: Rc<Shard<ConnectionMsg>>,
    cluster: Cluster,
    /// The current local slot of the Dialer
    local_slot: Slot,
    /// The whole slice of the current server
    global_slot: Slot,
    /// Slot -> part based on index
    inner_slots: Vec<Slot>,
}

impl Dialer {
    pub fn new(mesh: &MeshBuilder<ConnectionMsg>, storage: &Storage) -> Self {
        let inner_slots = storage.slots();
        let global_slot = storage.global_slot().clone();

        let cluster = Cluster {};

        Self {
            global_slot,
            inner_slots,
            cluster,
            shard: 
        }
    }

    /// We return a [Dialer] specific to the part we use which knows to where it
    /// should communicate based on the hash key of other threads.
    pub fn part(&self, part: u16) -> Self {
        let remainder: u16 = part % self.inner_slots.len() as u16;

        let local_slot =
            self.inner_slots.get(remainder as usize).cloned().unwrap();

        Self {
            local_slot,
            ..Clone::clone(self)
        }
    }
}

/// A Hash [Slot] as defined in the Redis Cluster Specification[^1].
///
/// ### References
///
/// [^1]: https://redis.io/docs/reference/cluster-spec/#key-distribution-model
#[derive(Debug, Clone)]
pub struct Slot(Range<u16>);

impl From<Range<u16>> for Slot {
    fn from(value: Range<u16>) -> Self {
        Self(value)
    }
}

impl Deref for Slot {
    type Target = Range<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
