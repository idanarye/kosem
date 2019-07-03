use actix::Message;

use kosem_webapi::Uuid;

use crate::role_actors::{TesteeActor, TesterActor};

pub struct RpcMessage {
    pub method: String,
    pub params: serde_json::Value,
}

impl Message for RpcMessage {
    type Result = ();
}

impl RpcMessage {
    pub fn new(method: impl Into<String>, params: impl serde::Serialize) -> Self {
        RpcMessage {
            method: method.into(),
            params: serde_json::to_value(params).unwrap(),
        }
    }
}

pub enum SetRole {
    Testee(actix::Addr<TesteeActor>),
    Tester(actix::Addr<TesterActor>),
}

impl Message for SetRole {
    type Result = ();
}

pub struct TesterAvailable {
    pub uid: Uuid,
    pub name: String,
    pub addr: actix::Addr<TesterActor>,
}

impl Message for TesterAvailable {
    type Result = ();
}
