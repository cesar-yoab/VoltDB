use crate::core::query::Query;
use crate::core::database::Database;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
    database: Database,
}

impl Client {
    pub fn new(addr: &str, database: Database) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Client { stream, database })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut reader = BufReader::new(&self.stream);

        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;

            if line.is_empty() {
                break;
            }

            let query = self.parse_query(&line);
            match query {
                Ok(query) => {
                    let response = query.execute(&self.database);
                    match response {
                        Ok(response) => {
                            writeln!(&mut self.stream, "{}", response)?;
                        }
                        Err(error) => {
                            writeln!(&mut self.stream, "Error: {}", error)?;
                        }
                    }
                }
                Err(error) => {
                    writeln!(&mut self.stream, "Error: {}", error)?;
                }
            }
        }

        Ok(())
    }

    fn parse_query(&self, input: &str) -> Result<Query, String> {
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
