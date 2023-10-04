use serde_json::Value;
use std::sync::RwLock;

pub struct InMemoryStore {
    data: RwLock<Option<Value>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            data: RwLock::new(None),
        }
    }

    pub fn get(&self) -> Option<Value> {
        self.data.read().unwrap().clone()
    }

    pub fn save(&self, value: Value) {
        *self.data.write().unwrap() = Some(value);
    }
}
