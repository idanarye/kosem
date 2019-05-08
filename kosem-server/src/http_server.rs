// use actix_web::actix::*;
// use actix_web::*;

use crate::server_config::*;
use crate::protocol_handlers;
use crate::role_actors;

pub fn run_server(config: ServerConfig) {
    let server = actix_web::server::new(|| {
        actix_web::App::new()
            .resource("/ws-jrpc", |r| {
                r.f(|r| actix_web::ws::start(r, protocol_handlers::websocket_jsonrpc::WsJrpc {
                    state: role_actors::ActorRoleState::Init,
                }))
            })
    });
    let bind_address = format!("localhost:{}", config.server.port);
    log::info!("Starting server on {}", bind_address);
    let server = server.bind(bind_address).unwrap();
    server.run();
}
