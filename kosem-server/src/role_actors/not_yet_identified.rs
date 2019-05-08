use actix_web::*;
use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

pub struct NotYetIdentifiedActor {
    con_actor: actix::Addr<WsJrpc>,
}

impl NotYetIdentifiedActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>) -> Self {
        Self { con_actor }
    }
}

impl actix::Actor for NotYetIdentifiedActor {
    type Context = actix::Context<Self>;
}
