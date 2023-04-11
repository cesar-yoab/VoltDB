use crate::client::Client;
use crate::core::settings::Settings;
use crate::server::Server;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread;

pub fn run_benchmark() {
    let settings = Settings::default();
    let server = Arc::new(Server::new(settings.clone()));
    let server_clone = server.clone();

    let server_thread = thread::spawn(move || {
        server_clone.start();
    });

    let mut clients = vec![];
    for _ in 0..10 {
        let client = Client::new(settings.bind_address.clone());
        clients.push(client);
    }

    let handles: Vec<_> = clients
        .into_iter()
        .map(|client| {
            thread::spawn(move || {
                for _ in 0..1000 {
                    let key: String = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect();
                    let vec: Vec<f32> = (0..128)
                        .map(|_| thread_rng().gen_range(-1.0..1.0))
                        .collect();
                    client.insert(&key, &vec).unwrap();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    server.stop();
    server_thread.join().unwrap();
}
