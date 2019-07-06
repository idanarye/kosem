use actix::Message;

use kosem_webapi::Uuid;

use crate::role_actors::{ProcedureActor, HumanActor};

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
    Procedure(actix::Addr<ProcedureActor>),
    Human(actix::Addr<HumanActor>),
}

impl Message for SetRole {
    type Result = ();
}

pub struct HumanAvailable {
    pub uid: Uuid,
    pub name: String,
    pub addr: actix::Addr<HumanActor>,
}

impl Message for HumanAvailable {
    type Result = ();
}
