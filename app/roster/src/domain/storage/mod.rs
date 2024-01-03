//! Storage primitive which is used to interact with Keys

use bytes::Bytes;
use scc::HashMap;

// We disallow Send just to be sure
impl !Send for Storage {}

/// Storage
#[derive(Default, Debug)]
pub struct Storage {
    db: HashMap<String, Bytes>,
}

impl Storage {
    /// Set a key without any expiration, return old value
    pub async fn set_async(
        &self,
        key: String,
        val: Bytes,
    ) -> Result<Option<Bytes>, (String, Bytes)> {
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
