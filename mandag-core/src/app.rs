use johnfig::Config;

use crate::store::Store;
use std::sync::Arc;

#[derive(Debug)]
struct AppInner {
    store: Store,
    config: Config,
}

#[derive(Debug, Clone)]
pub struct App(Arc<AppInner>);

impl App {
    pub fn new(store: Store, config: Config) -> App {
        App(Arc::new(AppInner { store, config }))
    }

    pub fn store(&self) -> &Store {
        &self.0.store
    }

    pub fn config(&self) -> &Config {
        &self.0.config
    }
}
