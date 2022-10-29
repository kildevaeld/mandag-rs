use std::sync::Arc;

use http::Extensions;
use parking_lot::{Mutex, RwLock};

#[derive(Debug)]
pub struct Store {
    i: RwLock<Extensions>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            i: RwLock::new(Extensions::default()),
        }
    }

    pub fn insert<S>(&mut self, i: S)
    where
        S: Send + Sync + 'static,
    {
        self.i.write().insert(i);
    }

    pub fn get<S>(&mut self) -> Option<S>
    where
        S: Send + Clone + Sync + 'static,
    {
        self.i.read().get::<S>().map(|m| m.clone())
    }
}
