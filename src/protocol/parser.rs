use crate::core::query::Query;

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<Query, String> {
        let mut parts = input.trim().split_whitespace();

        let command = match parts.next() {
            Some(command) => command.to_uppercase(),
            None => return Err(String::from("Empty input")),
        };

        match command.as_str() {
            "INSERT" => {
                let key = parts.next().ok_or("Missing key")?.to_string();
                let vector = parts
                    .map(|s| s.parse::<f32>().map_err(|_| "Invalid vector format"))
                    .collect::<Result<Vec<f32>, &str>>()?;

                Ok(Query::Insert(key, vector))
            }
            "GET" => {
                let key = parts.next().ok_or("Missing key")?.to_string();
                Ok(Query::Get(key))
            }
            "REMOVE" => {
                let key = parts.next().ok_or("Missing key")?.to_string();
                Ok(Query::Remove(key))
            }
            "NEAREST" => {
                let key = parts.next().ok_or("Missing key")?.to_string();
                let k = parts
                    .next()
                    .ok_or("Missing k")?
                    .parse::<usize>()
                    .map_err(|_| "Invalid k value")?;

                Ok(Query::NearestNeighbors(key, k))
            }
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
}
