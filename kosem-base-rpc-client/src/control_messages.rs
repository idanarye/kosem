use actix::prelude::*;

use crate::ClientActor;
use crate::config::ServerConfig;

pub struct RpcMessage {
    pub idx: Option<usize>,
    pub method: String,
    pub params: serde_json::Value,
}

impl RpcMessage {
    pub fn new(method: impl Into<String>, params: impl serde::Serialize) -> Self {
        RpcMessage {
            idx: None,
            method: method.into(),
            params: serde_json::to_value(params).expect("Generated RpcMessage must be serializable"),
        }
    }
}

impl Message for RpcMessage {
    type Result = ();
}

pub struct ConnectClientActor {
    pub idx: usize,
    pub server_config: ServerConfig,
    pub client_actor: Addr<ClientActor>,
}

impl Message for ConnectClientActor {
    type Result = ();
}
