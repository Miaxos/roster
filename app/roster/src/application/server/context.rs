use std::cell::Cell;
use std::sync::Arc;

use coarsetime::Instant;

use super::supervisor::{MetadataConnection, Supervisor};
use crate::domain::storage::StorageSegment;

/// [Context] is available for the whole duration of the TCP Connection.
#[derive(Clone)]
pub struct Context {
    pub storage: StorageSegment,
    pub supervisor: Supervisor,
    pub connection: Arc<MetadataConnection>,
    now: Cell<bool>,
}

impl Context {
    pub fn new(
        storage: StorageSegment,
        supervisor: Supervisor,
        meta_conn: Arc<MetadataConnection>,
    ) -> Self {
        Self {
            storage,
            supervisor,
            connection: meta_conn,
            now: Cell::new(false),
        }
    }

    #[allow(dead_code)]
    pub fn is_in_slot(&self, hash: u16) -> bool {
        self.storage.is_in_slot(hash)
    }

    pub fn slot_nb(&self, _hash: u16) -> Option<usize> {
        todo!()
        // self.storage.slot_nb(hash)
    }

    #[inline]
    pub fn now(&self) -> Instant {
        let now = self.now.get();
        if now {
            coarsetime::Instant::recent()
        } else {
            // TODO: Have each thread update the coarsetime every Xms so we
            // avoid to call it manually each time, it will goes from 5ns the
            // first call to 1-2ns
            self.now.set(true);
            coarsetime::Instant::now()
        }
    }
}
