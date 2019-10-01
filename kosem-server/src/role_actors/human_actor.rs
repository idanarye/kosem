use actix::prelude::*;

use kosem_webapi::Uuid;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::internal_messages::connection::{RpcMessage, ConnectionClosed, SetRole};
use crate::internal_messages::pairing::PairingPerformed;

use crate::role_actors::ProcedureActor;

#[derive(typed_builder::TypedBuilder)]
pub struct HumanActor {
    con_actor: actix::Addr<WsJrpc>,
    procedure_actor: actix::Addr<ProcedureActor>,
    uid: Uuid,
    name: String,
}

impl actix::Actor for HumanActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting HumanActor {} - {}", self.uid, self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending HumanActor {}", self.uid);
    }
}

impl actix::Handler<ConnectionClosed> for HumanActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<PairingPerformed> for HumanActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Paired human {} to request {}", msg.human_uid, msg.request_uid);
        self.con_actor.do_send(SetRole::Human(ctx.address()));
        self.con_actor.do_send(RpcMessage::new("JoinConfirmation", kosem_webapi::pairing_messages::JoinConfirmation {
            human_uid: self.uid,
            request_uid: msg.request_uid,
        }));
    }
}
