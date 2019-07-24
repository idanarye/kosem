use actix::prelude::*;

use kosem_webapi::Uuid;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::PairingActor;

use crate::internal_messages::{RpcMessage, ConnectionClosed};
use crate::internal_messages::{HumanAvailable, ProcedureRequestingHuman, RemoveAvailableHuman};

pub struct HumanActor {
    con_actor: actix::Addr<WsJrpc>,
    uid: Uuid,
    name: String,
}

impl HumanActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>, name: String) -> Self {
        let uid = Uuid::new_v4();
        Self { con_actor: con_actor, name, uid }
    }
}

impl actix::Actor for HumanActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting HumanActor {} - {}", self.uid, self.name);
        let response = kosem_webapi::handshake_messages::LoginConfirmed {
            uid: self.uid,
        };
        let message = RpcMessage::new("LoginConfirmed", response);
        self.con_actor.do_send(message);

        PairingActor::from_registry().do_send(HumanAvailable {
            uid: self.uid,
            addr: ctx.address(),
            name: self.name.clone(),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending HumanActor {}", self.uid);
        PairingActor::from_registry().do_send(RemoveAvailableHuman {
            uid: self.uid,
        });
    }
}

impl actix::Handler<ConnectionClosed> for HumanActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<ProcedureRequestingHuman> for HumanActor {
    type Result = ();

    fn handle(&mut self, msg: ProcedureRequestingHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("AvailableProcedure", kosem_webapi::pairing_messages::AvailableProcedure {
            uid: msg.uid,
            name: msg.orig_request.name,
        }));
    }
}
