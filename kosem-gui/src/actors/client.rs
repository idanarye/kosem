use std::collections::HashMap;

use actix::prelude::*;
// use actix::io::{SinkWrite, WriteHandler};
// use actix_codec::{AsyncRead, AsyncWrite, Framed};

// use futures::{lazy, Future};
// use futures::stream::{SplitSink};

// use awc::Client;
// use awc::ws::{Frame, Codec, Message};
// use awc::error::WsProtocolError;

use kosem_base_rpc_client::{ClientActor, wrap_addr_as_routing};
use kosem_base_rpc_client::control_messages::{
    RpcMessage,
    ConnectClientActor,
};
use kosem_base_rpc_client::config::ServerConfig;

use kosem_webapi::handshake_messages::{
    LoginAsHuman,
    // LoginConfirmed,
};

use crate::client_config::ClientConfig;

pub fn start_client_actor(config: ClientConfig) {
    let client = MyClient {
        config,
        client_actors: Default::default(),
    };
    client.start();
}

struct MyClient {
    config: ClientConfig,
    client_actors: HashMap<usize, (ServerConfig, Addr<ClientActor>)>,
}

impl Actor for MyClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        for (idx, server_config) in self.config.servers.iter().enumerate() {
            ClientActor::start_actor(idx, server_config.clone(), wrap_addr_as_routing!(addr));
        }
    }
}

impl Handler<ConnectClientActor> for MyClient {
    type Result = <ConnectClientActor as actix::Message>::Result;

    fn handle(&mut self, msg: ConnectClientActor, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Connecting a client actor");
        msg.client_actor.do_send(RpcMessage::new("LoginAsHuman", LoginAsHuman {
            name: self.config.display_name.clone(),
        }));
        self.client_actors.insert(msg.idx, (msg.server_config, msg.client_actor));
    }
}
