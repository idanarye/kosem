use actix::prelude::*;
use serde::{Serialize, Deserialize};
use actix_web_actors::ws;

use crate::role_actors;
use crate::internal_messages::connection::{RpcMessage, SetRole};

pub struct WsJrpc {
    pub state: role_actors::ActorRoleState,
}

impl actix::Actor for WsJrpc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.state = role_actors::ActorRoleState::start_not_yet_identified_actor(ctx.address());
    }
}

impl actix::StreamHandler<ws::Message, ws::ProtocolError> for WsJrpc {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(txt) => {
                let request: JrpcMessage = serde_json::from_str(&txt).unwrap();
                log::info!("got {:?}", request);
                self.state.send_request_from_connection(&request.method, request.params);
            },
            ws::Message::Close(_) => {
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Normal,
                    description: None,
                }));
            },
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        self.state.notify_connection_is_closed();
        ctx.stop()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JrpcMessage {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    id: Option<usize>,
    params: serde_json::Value,
}

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum RequestParams {
    // Positional(Vec<serde_json::value::Value>),
    // Named(serde_json::value::Map<String, serde_json::value::Value>),
// }

impl actix::Handler<RpcMessage> for WsJrpc {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, ctx: &mut Self::Context) -> Self::Result {
        let response = JrpcMessage {
            jsonrpc: "2.0".into(),
            method: msg.method,
            id: None,
            params: msg.params.into(),
        };
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}

impl actix::Handler<SetRole> for WsJrpc {
    type Result = <SetRole as actix::Message>::Result;

    fn handle(&mut self, msg: SetRole, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SetRole::Procedure(addr) => {
                self.state = role_actors::ActorRoleState::ProcedureActor(addr);
            }
            SetRole::Human(addr) => {
                self.state = role_actors::ActorRoleState::HumanActor(addr);
            }
        }
    }
}
