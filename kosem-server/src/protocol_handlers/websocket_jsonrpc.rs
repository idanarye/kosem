use serde::{Deserialize};
use actix_web::*;
use actix_web::actix;
use actix_web::actix::AsyncContext;

use crate::role_actors;
use crate::internal_messages::SetRole;

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
                let request: Request = serde_json::from_str(&txt).unwrap();
                log::info!("got {:?}", request);
                self.state.send_request_from_connection(&request.method, request.params);
            },
            _ => (),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Request {
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

impl actix::Handler<SetRole> for WsJrpc {
    type Result = <SetRole as actix::Message>::Result;

    fn handle(&mut self, msg: SetRole, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            SetRole::Testee(addr) => {
                self.state = role_actors::ActorRoleState::TesteeActor(addr);
            }
        }
    }
}
