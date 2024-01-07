use std::cell::Cell;

use coarsetime::Instant;

use crate::domain::storage::Storage;

#[derive(Clone)]
pub struct Context {
    pub storage: Storage,
    now: Cell<bool>,
}

impl Context {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            now: Cell::new(false),
        }
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
