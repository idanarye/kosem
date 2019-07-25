// use actix::prelude::*;
// use actix::io::{SinkWrite, WriteHandler};
// use actix_codec::{AsyncRead, AsyncWrite, Framed};

// use futures::{lazy, Future};
// use futures::stream::{SplitSink};

// use awc::Client;
// use awc::ws::{Frame, Codec, Message};
// use awc::error::WsProtocolError;

// use kosem_base_rpc_client::{ClientHandler, RpcMessage};

use crate::client_config::ClientConfig;

pub fn start_client_actor(_config: ClientConfig) {
    // struct Handler {
    // }
    // impl ClientHandler for Handler {
        // fn started<A>(&mut self, ctx: &A::Context) where A: Actor<Context = Context<A>> {
            // ctx.address().do_send(RpcMessage::new("LoginAsHuman", kosem_webapi::handshake_messages::LoginAsHuman {
            // }));
        // }
    // }
    // let handler = Handler {
    // };
    kosem_base_rpc_client::start_client_actor("localhost", 8206);
}

/*
pub fn start_client_actor(_config: ClientConfig) {
    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws("http://127.0.0.1:8206/ws-jrpc")
            .connect()
            .map_err(|e| {
                log::error!("Error: {}", e);
            }).map(|(response, framed)| {
                log::info!("Got response {:?}", response);
                let (sink, stream) = framed.split();
                let _addr = ClientActor::create(|ctx| {
                    ClientActor::add_stream(stream, ctx);
                    ClientActor {
                        sink_write: SinkWrite::new(sink, ctx),
                    }
                });
            })
    }));
}

struct ClientActor<T: 'static + AsyncRead + AsyncWrite> {
    sink_write: SinkWrite<SplitSink<Framed<T, Codec>>>,
}

impl<T: 'static + AsyncRead + AsyncWrite> Actor for ClientActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.sink_write.write(Message::Text(r#"{
            "jsonrpc": "2.0",
            "method": "LoginAsHuman",
            "params": {
                "name": "Mister Human"
            }
        }"#.into())).unwrap();
    }
}

impl<T: 'static + AsyncRead + AsyncWrite> StreamHandler<Frame, WsProtocolError> for ClientActor<T> {
    fn handle(&mut self, _msg: Frame, _ctx: &mut Self::Context) {
    }
}

impl<T: 'static + AsyncRead + AsyncWrite> WriteHandler<WsProtocolError> for ClientActor<T> {
}
*/
