use actix::prelude::*;
use actix::io::{SinkWrite, WriteHandler};
use actix_codec::{AsyncRead, AsyncWrite, Framed};

use futures::{lazy, Future};
use futures::stream::{SplitSink};

use awc::Client;
use awc::ws::{Frame, Codec};
use awc::error::WsProtocolError;

use kosem_webapi::protocols::JrpcMessage;

use crate::control_messages::*;

pub fn start_client_actor(url: &str, port: u16) {
    let url = format!("http://{}:{}/ws-jrpc", url, port);
    Arbiter::spawn(lazy(move || {
        Client::new()
            .ws(&url)
            .connect()
            .map_err(|e| {
                log::error!("Error: {}", e);
            }).map(|(response, framed)| {
                log::info!("hello {:?}", response);
                let (sink, stream) = framed.split();
                let _addr = ClientActor::create(|ctx| {
                    ClientActor::add_stream(stream, ctx);
                    ClientActor {
                        sink_write: SinkWrite::new(sink, ctx),
                        routing: None,
                    }
                });
            })
    }))
}

pub struct ClientActor<T: 'static + AsyncRead + AsyncWrite> {
    sink_write: SinkWrite<SplitSink<Framed<T, Codec>>>,
    routing: Option<ClientRouting>,
}

impl<T: 'static + AsyncRead + AsyncWrite> Actor for ClientActor<T> {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
    }
}

impl<T: 'static + AsyncRead + AsyncWrite> StreamHandler<Frame, WsProtocolError> for ClientActor<T> {
    fn handle(&mut self, _msg: Frame, _ctx: &mut Self::Context) {
    }
}

impl<T: 'static + AsyncRead + AsyncWrite> WriteHandler<WsProtocolError> for ClientActor<T> {
}

// Message Handling

impl<T: 'static + AsyncRead + AsyncWrite> Handler<ClientRouting> for ClientActor<T> {
    type Result = <ClientRouting as actix::Message>::Result;

    fn handle(&mut self, msg: ClientRouting, _ctx: &mut Self::Context) -> Self::Result {
        assert!(self.routing.is_none(), "Routing set twice");
        self.routing = Some(msg);
    }
}

impl<T: 'static + AsyncRead + AsyncWrite> Handler<RpcMessage> for ClientActor<T> {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, _ctx: &mut Self::Context) -> Self::Result {
        assert!(self.routing.is_some(), "Client was used without setting the routing");
        let response = JrpcMessage {
            jsonrpc: "2.0".into(),
            method: msg.method,
            id: None,
            params: msg.params.into(),
        };
        self.sink_write.write(awc::ws::Message::Text(serde_json::to_string(&response).unwrap())).unwrap();
    }
}
