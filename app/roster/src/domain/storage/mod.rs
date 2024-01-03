//! Storage primitive which is used to interact with Keys

use std::time::SystemTime;

use bytes::Bytes;
use scc::HashMap;

// We disallow Send just to be sure
impl !Send for Storage {}

#[derive(Debug)]
pub struct StorageValue {
    pub expired: Option<SystemTime>,
    pub val: Bytes,
}

/// Storage
#[derive(Default, Debug)]
pub struct Storage {
    db: HashMap<String, StorageValue>,
}

#[derive(Default)]
pub struct SetOptions {
    pub expired: Option<SystemTime>,
}

impl Storage {
    /// Set a key
    pub async fn set_async(
        &self,
        key: String,
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
}
