use actix::prelude::*;

use kosem_webapi::Uuid;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::PairingActor;

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
        let message = crate::internal_messages::RpcMessage::new("LoginConfirmed", response);
        self.con_actor.do_send(message);

        PairingActor::from_registry().do_send(crate::internal_messages::HumanAvailable {
            uid: self.uid,
            addr: ctx.address(),
            name: self.name.clone(),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending HumanActor {}", self.uid);
    }
}
