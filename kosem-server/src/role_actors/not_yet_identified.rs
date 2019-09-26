use actix::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::handshake_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::{ProcedureActor, HumanActor};
use crate::internal_messages::connection::{SetRole, ConnectionClosed};

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

impl actix::Handler<ConnectionClosed> for NotYetIdentifiedActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<LoginAsProcedure> for NotYetIdentifiedActor {
    type Result = <LoginAsProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: LoginAsProcedure, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("LoginAsProcedure: {:?}", msg);
        let procedure_uid = Uuid::new_v4();
        let actor = ProcedureActor::builder().uid(procedure_uid).con_actor(self.con_actor.clone()).name(msg.name).build();
        let actor = actor.start();
        self.con_actor.do_send(SetRole::Procedure(actor));
        ctx.stop();
        Ok(procedure_uid)
    }
}

impl actix::Handler<LoginAsHuman> for NotYetIdentifiedActor {
    type Result = <LoginAsHuman as actix::Message>::Result;

    fn handle(&mut self, msg: LoginAsHuman, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("LoginAsHuman: {:?}", msg);
        let human_uid = Uuid::new_v4();
        let actor = HumanActor::builder().uid(human_uid).con_actor(self.con_actor.clone()).name(msg.name).build();
        let actor = actor.start();
        self.con_actor.do_send(SetRole::Human(actor));
        ctx.stop();
        Ok(human_uid)
    }
}
