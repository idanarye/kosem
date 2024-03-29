use serde::Deserialize;

pub use kosem_base_rpc_client::config::ServerConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    pub display_name: String,
    pub servers: Vec<ServerConfig>,
}
