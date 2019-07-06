use actix::Actor as _;

use kosem_webapi::handshake_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::{ProcedureActor, HumanActor};
use crate::internal_messages::SetRole;

pub struct NotYetIdentifiedActor {
    con_actor: actix::Addr<WsJrpc>,
}

impl NotYetIdentifiedActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>) -> Self {
        Self { con_actor }
    }
}

impl actix::Actor for NotYetIdentifiedActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<LoginAsProcedure> for NotYetIdentifiedActor {
    type Result = <LoginAsProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: LoginAsProcedure, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("LoginAsProcedure: {:?}", msg);
        let actor = ProcedureActor::new(self.con_actor.clone(), msg.name);
        let actor = actor.start();
        self.con_actor.do_send(SetRole::Procedure(actor));
        use actix::ActorContext;
        ctx.stop();
    }
}

impl actix::Handler<LoginAsHuman> for NotYetIdentifiedActor {
    type Result = <LoginAsProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: LoginAsHuman, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("LoginAsHuman: {:?}", msg);
        let actor = HumanActor::new(self.con_actor.clone(), msg.name);
        let actor = actor.start();
        self.con_actor.do_send(SetRole::Human(actor));
        use actix::ActorContext;
        ctx.stop();
    }
}
