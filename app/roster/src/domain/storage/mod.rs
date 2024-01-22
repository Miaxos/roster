//! Storage primitive which is used to interact with Keys

use std::hash::BuildHasherDefault;
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use bytes::Bytes;
use bytestring::ByteString;
use coarsetime::Instant;
use rustc_hash::FxHasher;
use scc::HashMap;

#[derive(Debug)]
pub struct StorageValue {
    pub expired: Option<Instant>,
    pub val: Bytes,
}

/// A [StorageSegment] is shared across multiple threads and owns a part of the
/// hashing keys.
#[derive(Default, Debug, Clone)]
pub struct StorageSegment {
    db: Arc<HashMap<ByteString, StorageValue, BuildHasherDefault<FxHasher>>>,
    slot: Range<u16>,
    slots: Vec<Range<u16>>,
    count: Rc<AtomicU32>,
}

#[derive(Default)]
pub struct SetOptions {
    pub expired: Option<Instant>,
}

impl StorageSegment {
    /// Create a new [StorageSegment] by specifying the hash slot it handles.
    pub fn new(slots: Vec<Range<u16>>, slot: Range<u16>) -> Self {
        for _ in 0..4096 {
            drop(scc::ebr::Guard::new());
        }

        Self {
            db: Arc::new(HashMap::with_capacity_and_hasher(
                4096,
                Default::default(),
            )),
            slot,
            slots,
            count: Rc::new(AtomicU32::new(0)),
        }
    }

    pub fn is_in_slot(&self, i: u16) -> bool {
        self.slot.contains(&i)
    }

    pub fn slot_nb(&self, i: u16) -> Option<usize> {
        self.slots
            .iter()
            .enumerate()
            .find(|(idx, x)| x.contains(&i))
            .map(|(idx, _)| idx)
    }

    /// Set a key
    pub async fn set_async(
        &self,
        key: ByteString,
        val: Bytes,
        opt: SetOptions,
    ) -> Result<Option<StorageValue>, (String, StorageValue)> {
        let val = StorageValue {
            expired: opt.expired,
            val,
        };

        let old = self
            .count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        /*
        // Simulate some eviction mechanisme when we have too many keys
        if old > 200_000 {
            // dbg!("remove");
            // TODO: If the RC is for the DB instead, we could have a spawn from
            // monoio for this task instead, it would save us some
            // time for the p99.9
            let count = self.count.clone();
            let db = self.db.clone();
            monoio::spawn(async move {
                db.retain_async(|_, _| false).await;
                count.swap(0, std::sync::atomic::Ordering::Relaxed);
            });
        }
        */

        if let Err((key, val)) = self.db.insert(key, val) {
            let old = self.db.update(&key, |_, _| val);
            Ok(old)
        } else {
            Ok(None)
        }
    }

    /// Get a key
    ///
    /// Return None if it doesn't exist
    pub async fn get_async(
        &self,
        key: ByteString,
        now: Instant,
    ) -> Option<Bytes> {
        match self.db.entry_async(key).await {
            scc::hash_map::Entry::Occupied(oqp) => {
                let val = oqp.get();
                let is_expired =
                    val.expired.map(|expired| now > expired).unwrap_or(false);

                // TODO: Better handle expiration
                if is_expired {
                    // oqp.remove()?;
                    None
                } else {
                    Some(val.val.clone())
                }
            }
            scc::hash_map::Entry::Vacant(_) => None,
        }
    }
}
