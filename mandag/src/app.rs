use std::sync::Arc;

use http::Extensions;
use parking_lot::RwLock;

use crate::store::Store;

#[derive(Debug)]
struct AppInner {
    store: Store,
}

#[derive(Debug, Clone)]
pub struct App(Arc<AppInner>);

impl App {
    pub(crate) fn new(store: Store) -> App {
        App(Arc::new(AppInner { store }))
    }

    pub fn store(&self) -> &Store {
        &self.0.store
    }
}
