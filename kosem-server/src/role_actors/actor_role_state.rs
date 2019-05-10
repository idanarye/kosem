use actix_web::actix;
use actix_web::actix::Actor as _;
use serde::Deserialize;

use kosem_webapi::handshake_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::role_actors::not_yet_identified::NotYetIdentifiedActor;

pub enum ActorRoleState {
    Init,
    NotYetIdentifiedActor(actix::Addr<NotYetIdentifiedActor>),
}

impl ActorRoleState {
    pub fn start_not_yet_identified_actor(con_actor: actix::Addr<WsJrpc>) -> Self {
        let actor = NotYetIdentifiedActor::new(con_actor);
        let actor = actor.start();
        ActorRoleState::NotYetIdentifiedActor(actor)
    }

    pub fn send_request_from_connection<'de>(&self, method: &str, params: impl serde::Deserializer<'de>) {
        match method {
            "LoginAsTestee" => {
                let params = LoginAsTestee::deserialize(params).unwrap();
                log::info!("LoginAsTestee: {:?}", params);
            },
            _ => {},
        }
    }
}
