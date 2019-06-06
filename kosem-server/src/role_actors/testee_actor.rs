use kosem_webapi::testee_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

pub struct TesteeActor {
    _con_actor: actix::Addr<WsJrpc>,
    name: String,
}

impl TesteeActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>, name: String) -> Self {
        Self { _con_actor: con_actor, name }
    }
}

impl actix::Actor for TesteeActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting TesteeActor");
    }
}

impl actix::Handler<RequestTester> for TesteeActor {
    type Result = <RequestTester as actix::Message>::Result;

    fn handle(&mut self, msg: RequestTester, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("RequestTester from {}: {:?}", self.name, msg);
    }
}
