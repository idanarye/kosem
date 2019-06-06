use actix::Actor as _;
use serde::Deserialize;

use kosem_webapi::handshake_messages::*;
use kosem_webapi::testee_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::role_actors::{NotYetIdentifiedActor, TesteeActor};

pub enum ActorRoleState {
    Init,
    NotYetIdentifiedActor(actix::Addr<NotYetIdentifiedActor>),
    TesteeActor(actix::Addr<TesteeActor>),
}

impl ActorRoleState {
    pub fn start_not_yet_identified_actor(con_actor: actix::Addr<WsJrpc>) -> Self {
        let actor = NotYetIdentifiedActor::new(con_actor);
        let actor = actor.start();
        ActorRoleState::NotYetIdentifiedActor(actor)
    }

    pub fn send_request_from_connection<'de>(&self, method: &str, params: impl serde::Deserializer<'de>) {
        macro_rules! route {
            ($( $msg:ident: $($roles:ident),*; )*) => {
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
            LoginAsTestee: NotYetIdentifiedActor;
            RequestTester: TesteeActor;
        }
    }
}
