use std::net::SocketAddr;

pub struct Settings {
    pub bind_address: SocketAddr,
    // Add other settings as needed
}

impl Settings {
    pub fn new() -> Self {
        // Load settings from environment variables, config files or hard-code defaults
        let bind_address = env::var("VOLTDB_BIND_ADDRESS")
            .unwrap_or_else(|_| String::from("127.0.0.1:6379"))
            .parse()
            .expect("Invalid VOLTDB_BIND_ADDRESS format");

        // Initialize the Settings struct
        Settings { bind_address }
    }
}
