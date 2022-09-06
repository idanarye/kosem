use actix::Message;

use kosem_webapi::Uuid;

use crate::role_actors::{HumanActor, JoinerActor, ProcedureActor};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ConnectionClosed;

#[derive(Message)]
#[rtype(result = "()")]
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
#[rtype(result = "()")]
pub enum SetRole {
    Procedure(actix::Addr<ProcedureActor>),
    Human(actix::Addr<JoinerActor>),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddHumanActor {
    pub request_uid: Uuid,
    pub addr: actix::Addr<HumanActor>,
}
