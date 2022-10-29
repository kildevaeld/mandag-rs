use crate::store::Store;
use std::sync::Arc;

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
