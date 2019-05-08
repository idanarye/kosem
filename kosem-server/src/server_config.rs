use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub server: ServerSection,
}

#[derive(Debug, Deserialize)]
pub struct ServerSection {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_name() -> String {
    "Kosem Server".to_owned()
}

fn default_port() -> u16 {
    8206
}
