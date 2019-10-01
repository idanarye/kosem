use actix::prelude::*;

use kosem_webapi::Uuid;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::internal_messages::connection::ConnectionClosed;

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
