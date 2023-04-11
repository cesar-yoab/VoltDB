use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Key = String;
pub type Vector = Vec<f32>;

#[derive(Default)]
pub struct Database {
    data: Arc<RwLock<HashMap<Key, Vector>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: Key, vector: Vector) -> Result<(), String> {
        let mut data = self
            .data
            .write()
            .map_err(|_| String::from("Failed to acquire write lock"))?;

        data.insert(key, vector);

        Ok(())
    }

    pub fn get(&self, key: &Key) -> Result<Option<Vector>, String> {
        let data = self
            .data
            .read()
            .map_err(|_| String::from("Failed to acquire read lock"))?;

        Ok(data.get(key).cloned())
    }

    pub fn remove(&self, key: &Key) -> Result<(), String> {
        let mut data = self
            .data
            .write()
            .map_err(|_| String::from("Failed to acquire write lock"))?;

        data.remove(key);

        Ok(())
    }

    // Add other database operations as needed
}
