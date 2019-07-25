use actix::prelude::*;

pub struct RpcMessage {
    pub method: String,
    pub params: serde_json::Value,
}

impl RpcMessage {
    pub fn new(method: impl Into<String>, params: impl serde::Serialize) -> Self {
        RpcMessage {
            method: method.into(),
            params: serde_json::to_value(params).unwrap(),
        }
    }
}

impl Message for RpcMessage {
    type Result = ();
}

pub struct ClientRouting {
}

impl Message for ClientRouting {
    type Result = ();
}
