use std::cell::Cell;
use std::rc::Rc;

use coarsetime::Instant;

use crate::domain::storage::Storage;

#[derive(Clone)]
pub struct Context {
    pub storage: Rc<Storage>,
    now: Cell<bool>,
}

impl Context {
    pub fn new(storage: Rc<Storage>) -> Self {
        Self {
            storage,
            now: Cell::new(false),
        }
    }

    pub fn now(&self) -> Instant {
        let now = self.now.get();
        if now {
            coarsetime::Instant::recent()
        } else {
            // TODO: Have each thread update the coarsetime every 50ms so we
            // avoid to call it manually each time, it will gain us some ns
            self.now.set(true);
            coarsetime::Instant::now()
        }
    }
}
