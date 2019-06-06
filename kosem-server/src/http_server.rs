use crate::server_config::*;

use crate::protocol_handlers;
use crate::role_actors;

pub fn run_server(config: ServerConfig) {
    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(actix_web::web::resource("/ws-jrpc").route(actix_web::web::get().to(|request, stream: actix_web::web::Payload| {
                let handler = protocol_handlers::websocket_jsonrpc::WsJrpc {
                    state: role_actors::ActorRoleState::Init,
                };
                let res = actix_web_actors::ws::start(handler, &request, stream);
                res.unwrap()
            })))
    });
    let bind_address = format!("localhost:{}", config.server.port);
    log::info!("Starting server on {}", bind_address);
    let server = server.bind(bind_address).unwrap();
    server.run().unwrap();
}
