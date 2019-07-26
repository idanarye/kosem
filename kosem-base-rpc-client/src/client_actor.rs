use actix::prelude::*;
use actix::io::{SinkWrite, WriteHandler};

use futures::Future;

use awc::Client;
use awc::ws::Frame;
use awc::error::WsProtocolError;

use kosem_webapi::protocols::JrpcMessage;

use crate::control_messages::*;
use crate::config::ServerConfig;

pub struct ClientRouting {
    pub connect_client_actor: Recipient<ConnectClientActor>,
}

#[macro_export]
macro_rules! wrap_addr_as_routing {
    ($addr:expr) => {
        $crate::ClientRouting {
            connect_client_actor: $addr.clone().recipient(),
        }
    }
}

// impl ClientRouting {
    // pub fn routing_to<A>(addr: Addr<A>) -> Self
        // where A: Actor,
              // A: Handler<ConnectClientActor>
    // {
        // ClientRouting {
            // connect_client_actor: addr.recipient(),
        // }
    // }
// }

pub struct ClientActor {
    idx: usize,
    server_config: ServerConfig,
    write_fn: Box<dyn FnMut(awc::ws::Message) -> Result<(), WsProtocolError>>,
    routing: ClientRouting,
}

impl ClientActor {
    pub fn start_actor(idx: usize, server_config: ServerConfig, routing: ClientRouting) {
        let url = format!("http://{}:{}/ws-jrpc", server_config.url, server_config.port);
        Arbiter::spawn_fn(move || {
            Client::new()
                .ws(&url)
                .connect()
                .map_err(|e| {
                    log::error!("Error: {}", e);
                }).map(move |(response, framed)| {
                    log::info!("hello {:?}", response);
                    let (sink, stream) = framed.split();
                    let _addr = ClientActor::create(move |ctx| {
                        ClientActor::add_stream(stream, ctx);
                        let mut sink_write = SinkWrite::new(sink, ctx);
                        ClientActor {
                            idx,
                            server_config,
                            write_fn: Box::new(move |msg| {
                                sink_write.write(msg).map(|_| ())
                            }),
                            routing,
                        }
                    });
                })
        })
    }
}

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.routing.connect_client_actor.do_send(ConnectClientActor {
            idx: self.idx,
            server_config: self.server_config.clone(),
            client_actor: ctx.address(),
        }).unwrap();
    }
}

impl StreamHandler<Frame, WsProtocolError> for ClientActor {
    fn handle(&mut self, _msg: Frame, _ctx: &mut Self::Context) {
    }
}

impl WriteHandler<WsProtocolError> for ClientActor {
}

// Message Handling

impl Handler<RpcMessage> for ClientActor {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, _ctx: &mut Self::Context) -> Self::Result {
        let response = JrpcMessage {
            jsonrpc: "2.0".into(),
            method: msg.method,
            id: None,
            params: msg.params.into(),
        };
        (self.write_fn)(awc::ws::Message::Text(serde_json::to_string(&response).unwrap())).unwrap();
    }
}
