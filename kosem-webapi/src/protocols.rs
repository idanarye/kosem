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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    #[serde(flatten, with = "InternalUsageJrpcResult")]
    pub payload: Result<serde_json::Value, JrpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "Result::<serde_json::Value, JrpcError>")]
enum InternalUsageJrpcResult {
    #[serde(rename = "result")]
    Ok(serde_json::Value),
    #[serde(rename = "error")]
    Err(JrpcError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JrpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
