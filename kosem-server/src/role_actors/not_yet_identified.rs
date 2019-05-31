use actix_web::*;
use actix_web::actix::Actor as _;

use kosem_webapi::handshake_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::TesteeActor;
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

impl actix::Handler<LoginAsTestee> for NotYetIdentifiedActor {
    type Result = <LoginAsTestee as actix::Message>::Result;

    fn handle(&mut self, msg: LoginAsTestee, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("LoginAsTestee: {:?}", msg);
        let actor = TesteeActor::new(self.con_actor.clone(), msg.name);
        let actor = actor.start();
        self.con_actor.do_send(SetRole::Testee(actor));
        use actix::ActorContext;
        ctx.stop();
    }
}
