#![allow(unused_imports)]
use actix::prelude::*;

use kosem_webapi::{Uuid, KosemResult};
use kosem_webapi::pairing_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::{PairingActor, HumanActor};

use crate::internal_messages::connection::{RpcMessage, ConnectionClosed};
use crate::internal_messages::pairing::{
    HumanAvailable,
    ProcedureRequestingHuman,
    RemoveRequestForHuman,
    RemoveAvailableHuman,
    HumanJoiningProcedure,
    CreateNewHumanActor,
    PairingPerformed,
};

#[derive(typed_builder::TypedBuilder)]
pub struct JoinerActor {
    con_actor: actix::Addr<WsJrpc>,
    uid: Uuid,
    name: String,
}

impl actix::Actor for JoinerActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting JoinerActor {} - {}", self.uid, self.name);
        let response = kosem_webapi::handshake_messages::LoginConfirmed {
            uid: self.uid,
        };
        let message = RpcMessage::new("LoginConfirmed", response);
        self.con_actor.do_send(message);

        PairingActor::from_registry().do_send(HumanAvailable {
            uid: self.uid,
            addr: ctx.address(),
            name: self.name.clone(),
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending JoinerActor {}", self.uid);
        PairingActor::from_registry().do_send(RemoveAvailableHuman {
            uid: self.uid,
        });
    }
}

impl actix::Handler<ConnectionClosed> for JoinerActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<ProcedureRequestingHuman> for JoinerActor {
    type Result = ();

    fn handle(&mut self, msg: ProcedureRequestingHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("AvailableProcedure", kosem_webapi::pairing_messages::AvailableProcedure {
            uid: msg.uid,
            name: msg.orig_request.name,
        }));
    }
}

impl actix::Handler<RemoveRequestForHuman> for JoinerActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveRequestForHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("UnavailableProcedure", kosem_webapi::pairing_messages::UnavailableProcedure {
            uid: msg.uid,
        }));
    }
}

impl actix::Handler<JoinProcedure> for JoinerActor {
    type Result = ResponseActFuture<Self, KosemResult<()>>;

    fn handle(&mut self, msg: JoinProcedure, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Human {} joined procedure {}", self.name, msg.uid);

        Box::new(
            PairingActor::from_registry().send(HumanJoiningProcedure {
                human_uid: self.uid,
                request_uid: msg.uid,
            })
            .into_actor(self)
            .then(|result, _actor, _ctx| {
                let result = result.unwrap();
                log::warn!("Join result is {:?}", result);
                fut::result(result)
            })
        )
    }
}

impl actix::Handler<CreateNewHumanActor> for JoinerActor {
    type Result = <CreateNewHumanActor as actix::Message>::Result;

    fn handle(&mut self, msg: CreateNewHumanActor, _ctx: &mut actix::Context<Self>) -> Self::Result {
        HumanActor::builder()
            .con_actor(self.con_actor.clone())
            .procedure_actor(msg.procedure_addr)
            .uid(self.uid)
            .request_uid(msg.request_uid)
            .name(self.name.clone())
            .build().start()
    }
}

impl actix::Handler<PairingPerformed> for JoinerActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, _ctx: &mut actix::Context<Self>) -> Self::Result {
        msg.human_addr.clone().do_send(msg);
    }
}
