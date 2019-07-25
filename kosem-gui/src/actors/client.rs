use actix::prelude::*;
use futures::{lazy, Future};
use awc::Client;

use crate::client_config::ClientConfig;

pub fn start_client_actor(_config: ClientConfig) {
    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws("http://localhost")
            .connect()
            .map_err(|e| {
                log::error!("Error: {}", e);
            }).map(|(_response, _framed)| {
            })
    }));
}
