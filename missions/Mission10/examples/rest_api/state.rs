use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use mission10::UnionFind;

#[derive(Clone)]
pub struct AppState {
    // Map of Instance ID -> UnionFind structure
    instances: Arc<Mutex<HashMap<Uuid, UnionFind>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_instance(&self, size: usize) -> Uuid {
        let id = Uuid::new_v4();
        let uf = UnionFind::new(size);
        self.instances.lock().unwrap().insert(id, uf);
        id
    }

    pub fn get_instance<F, R>(&self, id: Uuid, f: F) -> Option<R>
    where
        F: FnOnce(&mut UnionFind) -> R,
    {
        let mut instances = self.instances.lock().unwrap();
        instances.get_mut(&id).map(f)
    }
}
