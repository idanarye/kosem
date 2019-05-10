use serde::{Deserialize};
use actix_web::*;
use actix_web::actix::AsyncContext;

use crate::role_actors;

pub struct WsJrpc {
    pub state: role_actors::ActorRoleState,
}

impl actix::Actor for WsJrpc {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.state = role_actors::ActorRoleState::start_not_yet_identified_actor(ctx.address());
        // let role = role_actors::not_yet_identified::NotYetIdentifiedActor::new(ctx.address());
        // let role = role.start();
        // self.state = role_actors::ActorRoleState::NotYetIdentifiedActor(role);
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
