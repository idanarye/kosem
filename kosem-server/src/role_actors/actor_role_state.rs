use std::collections::HashMap;

use actix::prelude::*;
use serde::Deserialize;

use kosem_webapi::handshake_messages::*;
use kosem_webapi::pairing_messages::*;
use kosem_webapi::phase_control_messages::*;
use kosem_webapi::{KosemResult, Uuid};

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::role_actors::{HumanActor, JoinerActor, NotYetIdentifiedActor, ProcedureActor};

pub enum ActorRoleState {
    Init,
    NotYetIdentifiedActor(actix::Addr<NotYetIdentifiedActor>),
    ProcedureActor(actix::Addr<ProcedureActor>),
    HumanActor {
        joiner: actix::Addr<JoinerActor>,
        procedures: HashMap<Uuid, actix::Addr<HumanActor>>,
    },
}

pub enum RoutingError<E: serde::de::Error> {
    MethodNotFound(String),
    MethodNotAllowedForRole {
        method: String,
        current_role: &'static str,
        allowed_roles: Vec<&'static str>,
    },
    DeserializationError {
        method: Option<String>,
        error: E,
    },
}

impl ActorRoleState {
    pub fn start_not_yet_identified_actor(con_actor: actix::Addr<WsJrpc>) -> Self {
        let actor = NotYetIdentifiedActor::new(con_actor);
        let actor = actor.start();
        ActorRoleState::NotYetIdentifiedActor(actor)
    }

    fn role_name(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::NotYetIdentifiedActor(_) => "not-logged-in",
            Self::ProcedureActor(_) => "procedure",
            Self::HumanActor { .. } => "human",
        }
    }

    fn variant_text_to_role_name(variant_name: &str) -> &'static str {
        match variant_name {
            "Init" => "init",
            "NotYetIdentifiedActor" => "not-logged-in",
            "ProcedureActor" => "procedure",
            "JoinerActor" => "human",
            "HumanActor" => "human",
            _ => unreachable!("Unhandled variant"),
        }
    }

    pub fn send_request_from_connection<'de, Deser: serde::Deserializer<'de>>(
        &self,
        method: &str,
        params: Deser,
        _error_classifier: impl FnOnce(&str, Deser::Error),
    ) -> Result<ResponseFuture<KosemResult<serde_value::Value>>, RoutingError<Deser::Error>> {
        macro_rules! get_actor {
            (NotYetIdentifiedActor, $msg:expr) => {
                if let Self::NotYetIdentifiedActor(actor) = self {
                    Some(actor)
                } else {
                    None
                }
            };
            (ProcedureActor, $msg:expr) => {
                if let Self::ProcedureActor(actor) = self {
                    Some(actor)
                } else {
                    None
                }
            };
            (JoinerActor, $msg:expr) => {
                if let Self::HumanActor { joiner, .. } = self {
                    Some(joiner)
                } else {
                    None
                }
            };
            (HumanActor, $msg:expr) => {
                if let Self::HumanActor { procedures, .. } = self {
                    Some(&procedures[&$msg.request_uid])
                } else {
                    None
                }
            };
        }
        macro_rules! route {
            ($( $method:ident => $role:ident; )*) => {
                match method {
                    $(
                        stringify!($method) => {
                            let params = $method::deserialize(params).map_err(|error| {
                                RoutingError::DeserializationError {
                                    method: Some(method.to_owned()),
                                    error
                                }
                            })?;
                            if let Some(actor) = get_actor!($role, params) {
                                let sent = actor.send(params);
                                Ok(Box::pin(async {
                                    let res = sent.await.unwrap()?;
                                    Ok(serde_value::to_value(res).unwrap())
                                }))
                            } else {
                                 Err(RoutingError::MethodNotAllowedForRole {
                                    method: method.to_owned(),
                                    current_role: self.role_name(),
                                    allowed_roles: vec![
                                        Self::variant_text_to_role_name(stringify!($role))
                                    ],
                                })
                            }
                        },
                    )*
                    _ => {
                        Err(RoutingError::MethodNotFound(method.to_owned()))
                    }
                }
            }
        }
        route! {
            LoginAsProcedure => NotYetIdentifiedActor;
            LoginAsHuman => NotYetIdentifiedActor;
            RequestHuman => ProcedureActor;
            JoinProcedure => JoinerActor;
            PushPhase => ProcedureActor;
            PopPhase => ProcedureActor;
            ClickButton => HumanActor;
        }
    }

    pub fn notify_connection_is_closed(&self) {
        let msg = crate::internal_messages::connection::ConnectionClosed;

        match self {
            ActorRoleState::Init => {}
            ActorRoleState::NotYetIdentifiedActor(addr) => addr.do_send(msg),
            ActorRoleState::ProcedureActor(addr) => addr.do_send(msg),
            ActorRoleState::HumanActor { joiner, procedures } => {
                for procedure in procedures.values() {
                    procedure.do_send(msg.clone());
                }
                joiner.do_send(msg);
            }
        }
    }
}
