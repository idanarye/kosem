use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JrpcMessage {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub id: Option<usize>,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JrpcResponse {
    pub jsonrpc: String,
    pub id: usize,
    #[serde(default)]
    pub result: serde_json::Value,
}
