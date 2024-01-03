use std::rc::Rc;

use crate::domain::storage::Storage;

#[derive(Clone)]
pub struct Context {
    pub storage: Rc<Storage>,
}
