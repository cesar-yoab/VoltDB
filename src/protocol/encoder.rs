use crate::core::query::Query;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EncodedQuery {
    command: String,
    key: Option<String>,
    vector: Option<Vec<f32>>,
    k: Option<usize>,
}

impl EncodedQuery {
    pub fn from_query(query: &Query) -> Self {
        match query {
            Query::Insert(key, vector) => EncodedQuery {
                command: "INSERT".to_string(),
                key: Some(key.clone()),
                vector: Some(vector.clone()),
                k: None,
            },
            Query::Get(key) => EncodedQuery {
                command: "GET".to_string(),
                key: Some(key.clone()),
                vector: None,
                k: None,
            },
            Query::Remove(key) => EncodedQuery {
                command: "REMOVE".to_string(),
                key: Some(key.clone()),
                vector: None,
                k: None,
            },
            Query::NearestNeighbors(key, k) => EncodedQuery {
                command: "NEAREST".to_string(),
                key: Some(key.clone()),
                vector: None,
                k: Some(*k),
            },
            // Add other query variants as needed
        }
    }

    pub fn to_query(&self) -> Result<Query, String> {
        match self.command.as_str() {
            "INSERT" => {
                let key = self
                    .key
                    .as_ref()
                    .ok_or_else(|| String::from("Missing key"))?
                    .clone();
                let vector = self
                    .vector
                    .as_ref()
                    .ok_or_else(|| String::from("Missing vector"))?
                    .clone();
                Ok(Query::Insert(key, vector))
            }
            "GET" => {
                let key = self
                    .key
                    .as_ref()
                    .ok_or_else(|| String::from("Missing key"))?
                    .clone();
                Ok(Query::Get(key))
            }
            "REMOVE" => {
                let key = self
                    .key
                    .as_ref()
                    .ok_or_else(|| String::from("Missing key"))?
                    .clone();
                Ok(Query::Remove(key))
            }
            "NEAREST" => {
                let key = self
                    .key
                    .as_ref()
                    .ok_or_else(|| String::from("Missing key"))?
                    .clone();
                let k = self.k.ok_or_else(|| String::from("Missing k"))?;
                Ok(Query::NearestNeighbors(key, k))
            }
            _ => Err(format!("Unknown command: {}", self.command)),
        }
    }
}
