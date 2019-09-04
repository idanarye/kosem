use actix::Actor as _;
use serde::Deserialize;

use kosem_webapi::handshake_messages::*;
use kosem_webapi::pairing_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::role_actors::{NotYetIdentifiedActor, ProcedureActor, HumanActor};

pub enum ActorRoleState {
    Init,
    NotYetIdentifiedActor(actix::Addr<NotYetIdentifiedActor>),
    ProcedureActor(actix::Addr<ProcedureActor>),
    HumanActor(actix::Addr<HumanActor>),
}

impl ActorRoleState {
    pub fn start_not_yet_identified_actor(con_actor: actix::Addr<WsJrpc>) -> Self {
        let actor = NotYetIdentifiedActor::new(con_actor);
        let actor = actor.start();
        ActorRoleState::NotYetIdentifiedActor(actor)
    }

    pub fn send_request_from_connection<'de>(&self, method: &str, params: impl serde::Deserializer<'de>) {
        macro_rules! route {
            ($( $msg:ident => $($roles:ident),*; )*) => {
                match method {
                    $(
                        stringify!($msg) => {
                            let params = $msg::deserialize(params).unwrap();
                            match self {
                                $(
                                    ActorRoleState::$roles(actor) => {
                                        actor.do_send(params);
                                    },
                                )*
                                _ => panic!()
                            }
                        },
                    )*
                    _ => panic!(),
                }
            }
        }
        route! {
            LoginAsProcedure => NotYetIdentifiedActor;
            LoginAsHuman => NotYetIdentifiedActor;
            RequestHuman => ProcedureActor;
            JoinProcedure => HumanActor;
        }
    }

    pub fn notify_connection_is_closed(&self) {
        let msg = crate::internal_messages::connection::ConnectionClosed;

        match self {
            ActorRoleState::Init => {},
            ActorRoleState::NotYetIdentifiedActor(addr) => addr.do_send(msg),
            ActorRoleState::ProcedureActor(addr) => addr.do_send(msg),
            ActorRoleState::HumanActor(addr) => addr.do_send(msg),
        }
    }
}
