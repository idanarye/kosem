use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    servers: Vec<ServerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    name: String,
    url: String,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 {
    8206
}
