use crate::core::database::Database;
use crate::network::client::Client;
use crate::config::settings::Settings;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::task::JoinHandle;

pub struct Server {
    settings: Settings,
    database: Database,
}

impl Server {
    pub fn new(settings: Settings) -> Self {
        Server {
            settings,
            database: Database::new(),
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.settings.bind_address).await?;

        println!(
            "VoltDB listening on {}...",
            self.settings.bind_address
        );

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New client connected: {}", addr);

            let database = self.database.clone();
            let client = Client::new(stream, database)?;
            let handle: JoinHandle<Result<(), std::io::Error>> = spawn(async move {
                client.run().await
            });

            if let Err(e) = handle.await? {
                eprintln!("Client connection error: {}", e);
            }
        }
    }
}
