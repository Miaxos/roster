//! Storage primitive which is used to interact with Keys

use std::hash::BuildHasherDefault;
use std::mem::size_of_val;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use bytes::Bytes;
use bytestring::ByteString;
use coarsetime::Instant;
use rand::random;
use rustc_hash::FxHasher;
use scc::HashMap;

use super::dialer::Slot;
use crate::infrastructure::hash::HASH_SLOT_MAX;

#[derive(Debug)]
pub struct StorageValue {
    pub expired: Option<Instant>,
    pub val: Vec<u8>,
}

/// A [StorageSegment] is shared across multiple threads and owns a part of the
/// hashing keys.
#[derive(Debug, Clone)]
pub struct StorageSegment {
    db: Arc<HashMap<Vec<u8>, StorageValue, BuildHasherDefault<FxHasher>>>,
    slot: Slot,
    count: Arc<AtomicU32>,
}

#[derive(Default)]
pub struct SetOptions {
    pub expired: Option<Instant>,
}

impl StorageSegment {
    /// Create a new [StorageSegment] by specifying the hash slot it handles.
    pub fn new(slot: Slot) -> Self {
        let h = HashMap::with_capacity_and_hasher(
            2usize.pow(20),
            Default::default(),
        );

        for _ in 0..(2usize.pow(20)) {
            drop(scc::ebr::Guard::new());
        }

        Self {
            db: Arc::new(h),
            slot,
            count: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn is_in_slot(&self, i: u16) -> bool {
        self.slot.contains(&i)
    }

    /// Set a key into the storage
    pub fn set_async(
        &self,
        key: ByteString,
        val: Bytes,
        opt: SetOptions,
    ) -> Result<Option<StorageValue>, (String, StorageValue)> {
        let mut val = val.to_vec();
        val.shrink_to_fit();

        let val = StorageValue {
            expired: opt.expired,
            val,
        };

        let mut key = key.into_bytes().to_vec();
        key.shrink_to_fit();

        let old = self
            .count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        if old % 1_000_000 == 0 {
            dbg!(old);
        }

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
        let mut key = key.into_bytes().to_vec();
        key.shrink_to_fit();

        match self.db.entry_async(key).await {
            scc::hash_map::Entry::Occupied(oqp) => {
                let val = oqp.get();
                let is_expired =
                    val.expired.map(|expired| now > expired).unwrap_or(false);

                // TODO: Better handle expiration
                if is_expired {
                    let _ = oqp.remove();
                    None
                } else {
                    Some(Bytes::from(val.val.clone()))
                }
            }
            scc::hash_map::Entry::Vacant(_) => None,
        }
    }
}

/// A [Storage] is composed of multipe [StorageSegment] shared in threads.
#[derive(Debug, Clone)]
pub struct Storage {
    global_slot: Slot,
    internal_vec: Vec<(Slot, StorageSegment)>,
}

impl Storage {
    /// Create a new [Storage] by specifying the number of slot wanted and the
    /// whole [Slot] this [Storage] should handle.
    pub fn new(nb_slot: u16, slot: Slot) -> Self {
        assert!(nb_slot != 0);
        assert!(nb_slot <= HASH_SLOT_MAX);

        let global_slot = slot.clone();

        // We generate the Slot where we need to create a StorageSegment.
        let mut slots: Vec<(Slot, StorageSegment)> = Vec::new();
        for slot in 0..nb_slot {
            let part_size: u16 = HASH_SLOT_MAX / nb_slot;
            let remainder: u16 = HASH_SLOT_MAX % nb_slot;

            let start = slot * part_size;
            let end = if slot == nb_slot - 1 {
                (slot + 1) * part_size + remainder
            } else {
                (slot + 1) * part_size
            };

            let slot = Slot::from(start..end);
            let store = StorageSegment::new(slot.clone());
            slots.push((slot, store));
        }

        Self {
            internal_vec: slots,
            global_slot,
        }
    }

    /// Give every [Slot] associated to a part, where the part is the index.
    pub fn slots(&self) -> Vec<Slot> {
        self.internal_vec.iter().map(|(x, _)| x.clone()).collect()
    }

    /// Give the global [Slot] of the storage, which is the whole [Slot] handled
    /// by the current server.
    pub fn global_slot(&self) -> &Slot {
        &self.global_slot
    }

    /// Based on the number of [StorageSegment] we have, we can take the
    /// corresponding part based on the modulo of the nb_part.
    ///
    /// If we have n part, we'll assign the part based on (part % n).
    pub fn part(&self, part: u16) -> (Slot, StorageSegment) {
        let remainder: u16 = part % self.internal_vec.len() as u16;
        self.internal_vec
            .get(remainder as usize)
            .cloned()
            .expect("WTF")
    }
}
