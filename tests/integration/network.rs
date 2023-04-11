#[cfg(test)]
mod tests {
    use crate::core::database::Database;
    use crate::core::query::Query;
    use crate::network::client::Client;
    use crate::network::server::Server;
    use crate::config::settings::Settings;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_client_server_interaction() {
        let settings = Settings {
            bind_address: "127.0.0.1:3344".to_string(),
            memory_limit: 10_000_000,
        };

        let server = Server::new(settings.clone());
        let server_handle = thread::spawn(move || {
            tokio_test::block_on(server.run()).unwrap();
        });

        // Wait for the server to start
        thread::sleep(Duration::from_secs(1));

        let mut database = Database::new();
        let client = Client::new(&settings.bind_address, database.clone()).unwrap();

        let test_key = "test_key".to_string();
        let test_vector = vec![1.0, 2.0, 3.0];
        let insert_query = Query::Insert(test_key.clone(), test_vector.clone());
        let response = insert_query.execute(&mut database);
        assert!(response.is_ok());

        let get_query = Query::Get(test_key.clone());
        let response = get_query.execute(&mut database);
        assert_eq!(response, Ok(format!("{:?}", test_vector)));

        let remove_query = Query::Remove(test_key.clone());
        let response = remove_query.execute(&mut database);
        assert!(response.is_ok());

        let get_query = Query::Get(test_key);
        let response = get_query.execute(&mut database);
        assert_eq!(response, Err(format!("Key not found")));

        // Close the server
        drop(TcpStream::connect(settings.bind_address).unwrap());
        server_handle.join().unwrap();
    }
}
