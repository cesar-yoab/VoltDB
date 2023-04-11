use std::env;
use voltdb::config::settings::Settings;
use voltdb::network::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load settings from the config module
    let settings = Settings::new();

    // Setup the server
    let server = Server::new(settings);

    // Start the server and await termination
    server.run().await?;

    Ok(())
}
