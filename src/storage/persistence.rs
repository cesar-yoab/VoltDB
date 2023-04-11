use crate::core::database::{Database, Key, Vector};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct Persistence;

impl Persistence {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Database, String> {
        let file = File::open(path).map_err(|_| "Unable to open file")?;
        let reader = BufReader::new(file);
        let mut database = Database::new();

        for line in reader.lines() {
            let line = line.map_err(|_| "Unable to read line")?;
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() != 2 {
                return Err(String::from("Invalid file format"));
            }

            let key = parts[0].to_string();
            let vector = parts[1]
                .split_whitespace()
                .map(|s| s.parse::<f32>().map_err(|_| "Invalid vector format"))
                .collect::<Result<Vector, &str>>()?;

            database.insert(key, vector)?;
        }

        Ok(database)
    }

    pub fn save_to_file<P: AsRef<Path>>(database: &Database, path: P) -> Result<(), String> {
        let file = File::create(path).map_err(|_| "Unable to create file")?;
        let mut writer = std::io::BufWriter::new(file);

        for (key, vector) in database.iter() {
            let line = format!(
                "{},{}\n",
                key,
                vector
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );

            writer.write_all(line.as_bytes()).map_err(|_| "Unable to write to file")?;
        }

        Ok(())
    }
}
