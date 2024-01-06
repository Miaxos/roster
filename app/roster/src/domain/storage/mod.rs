//! Storage primitive which is used to interact with Keys

use std::time::SystemTime;

use bytes::Bytes;
use bytestring::ByteString;
use coarsetime::Instant;
use scc::HashMap;

// We disallow Send just to be sure
impl !Send for Storage {}

#[derive(Debug)]
pub struct StorageValue {
    pub expired: Option<Instant>,
    pub val: Bytes,
}

/// Storage
#[derive(Default, Debug)]
pub struct Storage {
    db: HashMap<ByteString, StorageValue>,
}

#[derive(Default)]
pub struct SetOptions {
    pub expired: Option<Instant>,
}

impl Storage {
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

        match self.db.entry_async(key).await {
            scc::hash_map::Entry::Occupied(mut oqp) => {
                let old = oqp.insert(val);
                Ok(Some(old))
            }
            scc::hash_map::Entry::Vacant(vac) => {
                vac.insert_entry(val);
                Ok(None)
            }
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
