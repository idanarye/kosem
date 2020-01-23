use actix::prelude::*;
use actix::io::{SinkWrite, WriteHandler};

use futures::stream::StreamExt;

use awc::Client;
use awc::ws;
use awc::error::WsProtocolError;

use kosem_webapi::protocols::JrpcMessage;

use crate::control_messages::*;
use crate::config::ServerConfig;

pub struct ClientRouting {
    pub connect_client_actor: Recipient<ConnectClientActor>,
    pub rpc_message: Recipient<RpcMessage>,
}

#[macro_export]
macro_rules! wrap_addr_as_routing {
    ($addr:expr) => {
        $crate::ClientRouting {
            connect_client_actor: $addr.clone().recipient(),
            rpc_message: $addr.clone().recipient(),
        }
    }
}

pub struct ClientActor {
    idx: usize,
    server_config: ServerConfig,
    write_fn: Box<dyn FnMut(awc::ws::Message) -> Result<(), WsProtocolError>>,
    routing: ClientRouting,
}

impl ClientActor {
    pub fn start_actor(idx: usize, server_config: ServerConfig, routing: ClientRouting) {
        let url = format!("http://{}:{}/ws-jrpc", server_config.url, server_config.port);
        Arbiter::spawn(async move {
            let (response, framed) = Client::new()
                .ws(&url)
                .connect()
                .await
                .map_err(|e| {
                    log::error!("Error: {}", e);
                }).unwrap();
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
    }
}

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.routing.connect_client_actor.do_send(ConnectClientActor {
            idx: self.idx,
            server_config: self.server_config.clone(),
            client_actor: ctx.address(),
        }).expect("routing should be present when the GUI is created");
    }
}

impl StreamHandler<Result<ws::Frame, WsProtocolError>> for ClientActor {
    fn handle(&mut self, msg: Result<ws::Frame, WsProtocolError>, _ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Frame::Ping(msg)) => {
                (self.write_fn)(ws::Message::Pong(msg)).unwrap();
            },
            Ok(ws::Frame::Text(txt)) => {
                let txt = String::from_utf8(Vec::from(txt.as_ref())).unwrap();
                let request: JrpcMessage = serde_json::from_str(&txt)
                    .map_err(|err| format!("Unable to parse {:?} - {:?}", txt, err))
                    .unwrap();
                self.routing.rpc_message.do_send(RpcMessage {
                    idx: Some(self.idx),
                    method: request.method,
                    params: request.params,
                }).unwrap();
            },
            Ok(ws::Frame::Close(_)) => {
                (self.write_fn)(ws::Message::Close(Some(ws::CloseReason {
                    code: ws::CloseCode::Normal,
                    description: None,
                }))).unwrap();
            },
            Ok(_) => (),
            Err(e) => panic!("Protocol error {:?}", e),
        }
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
        let response_text = serde_json::to_string(&response).expect("Response must be serializable");
        (self.write_fn)(ws::Message::Text(response_text)).unwrap();
    }
}
