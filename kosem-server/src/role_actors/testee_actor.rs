use kosem_webapi::Uuid;
use kosem_webapi::testee_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

pub struct TesteeActor {
    con_actor: actix::Addr<WsJrpc>,
    uid: Uuid,
    name: String,
}

impl TesteeActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>, name: String) -> Self {
        let uid = Uuid::new_v4();
        Self { con_actor: con_actor, name, uid }
    }
}

impl actix::Actor for TesteeActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting TesteeActor {}", self.uid);
        let response = kosem_webapi::handshake_messages::LoginConfirmed {
            uid: self.uid,
        };
        let message = crate::internal_messages::RpcMessage::new("LoginConfirmed", response);
        self.con_actor.do_send(message);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending TesteeActor {}", self.uid);
    }
}

impl actix::Handler<RequestTester> for TesteeActor {
    type Result = <RequestTester as actix::Message>::Result;

    fn handle(&mut self, msg: RequestTester, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("RequestTester from {}: {:?}", self.name, msg);
    }
}
