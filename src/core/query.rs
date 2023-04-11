use crate::core::database::{Database, Key, Vector};
use crate::core::vector::VectorOps;

pub enum Query {
    Insert(Key, Vector),
    Get(Key),
    Remove(Key),
    NearestNeighbors(Key, usize),
    // Add other query variants as needed
}

impl Query {
    pub fn execute(self, database: &Database) -> Result<String, String> {
        match self {
            Query::Insert(key, vector) => {
                database.insert(key, vector)?;
                Ok(String::from("OK"))
            }
            Query::Get(key) => {
                let vector = database.get(&key)?;
                match vector {
                    Some(vec) => Ok(format!("{}: {:?}", key, vec)),
                    None => Err(format!("Key not found: {}", key)),
                }
            }
            Query::Remove(key) => {
                database.remove(&key)?;
                Ok(String::from("OK"))
            }
            Query::NearestNeighbors(key, k) => {
                let vector = database.get(&key)?;
                match vector {
                    Some(vec) => {
                        let neighbors = database.find_nearest_neighbors(&vec, k)?;
                        Ok(format!("{} nearest neighbors: {:?}", k, neighbors))
                    }
                    None => Err(format!("Key not found: {}", key)),
                }
            }
            // Add other query execution cases as needed
        }
    }
}
