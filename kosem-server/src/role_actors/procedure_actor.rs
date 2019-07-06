// use actix::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

// use crate::role_actors::PairingActor;

pub struct ProcedureActor {
    con_actor: actix::Addr<WsJrpc>,
    uid: Uuid,
    name: String,
}

impl ProcedureActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>, name: String) -> Self {
        let uid = Uuid::new_v4();
        Self { con_actor: con_actor, name, uid }
    }
}

impl actix::Actor for ProcedureActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting ProcedureActor {} - {}", self.uid, self.name);
        let response = kosem_webapi::handshake_messages::LoginConfirmed {
            uid: self.uid,
        };
        let message = crate::internal_messages::RpcMessage::new("LoginConfirmed", response);
        self.con_actor.do_send(message);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending ProcedureActor {}", self.uid);
    }
}

impl actix::Handler<RequestHuman> for ProcedureActor {
    type Result = <RequestHuman as actix::Message>::Result;

    fn handle(&mut self, msg: RequestHuman, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("RequestHuman from {}: {:?}", self.name, msg);
        // log::info!("Pairing actor be {:?}", PairingActor::from_registry());
    }
}
