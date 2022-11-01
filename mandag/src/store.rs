use http::Extensions;
use parking_lot::RwLock;

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
        S: Send + Sync + Clone + 'static,
    {
        self.i.write().insert(i);
    }

    pub fn get<S>(&self) -> Option<S>
    where
        S: Send + Clone + Sync + 'static,
    {
        self.i.read().get::<S>().map(|m| m.clone())
    }
}
