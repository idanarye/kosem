use actix::Message;

use crate::role_actors::{ProcedureActor, JoinerActor, HumanActor};

#[derive(Message)]
pub struct ConnectionClosed;

#[derive(Message)]
pub struct RpcMessage {
    pub method: String,
    pub params: serde_value::Value,
}

impl RpcMessage {
    pub fn new(method: impl Into<String>, params: impl serde::Serialize) -> Self {
        RpcMessage {
            method: method.into(),
            params: serde_value::to_value(params).unwrap(),
        }
    }
}

#[derive(Message)]
pub enum SetRole {
    Procedure(actix::Addr<ProcedureActor>),
    Joiner(actix::Addr<JoinerActor>),
    Human(actix::Addr<HumanActor>),
}
