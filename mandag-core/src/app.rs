use johnfig::Config;

use crate::{router::SharedRouter, store::Store};
use std::sync::Arc;

#[derive(Debug)]
struct AppInner {
    store: Store,
    config: Config,
    #[allow(dead_code)]
    router: SharedRouter,
}

#[derive(Debug, Clone)]
pub struct App(Arc<AppInner>);

impl App {
    pub fn new(router: SharedRouter, store: Store, config: Config) -> App {
        App(Arc::new(AppInner {
            store,
            config,
            router,
        }))
    }

    pub fn store(&self) -> &Store {
        &self.0.store
    }

    pub fn config(&self) -> &Config {
        &self.0.config
    }
}
