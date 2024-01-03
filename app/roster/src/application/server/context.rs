use std::cell::Cell;
use std::rc::Rc;
use std::time::SystemTime;

use crate::domain::storage::Storage;

#[derive(Clone)]
pub struct Context {
    pub storage: Rc<Storage>,
    now: Cell<Option<SystemTime>>,
}

impl Context {
    pub fn new(storage: Rc<Storage>) -> Self {
        Self {
            storage,
            now: Cell::new(None),
        }
    }

    pub fn now(&self) -> SystemTime {
        let now = self.now.get();
        if now.is_none() {
            let n = SystemTime::now();
            self.now.set(Some(n));
            return n;
        }

        now.expect("Can't fail")
    }
}
