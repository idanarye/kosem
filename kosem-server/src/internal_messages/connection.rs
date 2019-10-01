use actix::Message;

use crate::role_actors::{ProcedureActor, JoinerActor, HumanActor};

pub struct ConnectionClosed;

impl Message for ConnectionClosed {
    type Result = ();
}

pub struct RpcMessage {
    pub method: String,
    pub params: serde_value::Value,
}

impl Message for RpcMessage {
    type Result = ();
}

impl RpcMessage {
    pub fn new(method: impl Into<String>, params: impl serde::Serialize) -> Self {
        RpcMessage {
            method: method.into(),
            params: serde_value::to_value(params).unwrap(),
        }
    }
}

pub enum SetRole {
    Procedure(actix::Addr<ProcedureActor>),
    Joiner(actix::Addr<JoinerActor>),
    Human(actix::Addr<HumanActor>),
}

impl Message for SetRole {
    type Result = ();
}
