use actix::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages::*;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;
use crate::role_actors::PairingActor;

use crate::internal_messages::connection::{RpcMessage, ConnectionClosed};
use crate::internal_messages::pairing::{
    HumanAvailable,
    ProcedureRequestingHuman,
    RemoveRequestForHuman,
    RemoveAvailableHuman,
    HumanJoiningProcedure,
    PairingPerformed,
};

pub struct HumanActor {
    con_actor: actix::Addr<WsJrpc>,
    uid: Uuid,
    name: String,
}

impl HumanActor {
    pub fn new(con_actor: actix::Addr<WsJrpc>, name: String) -> Self {
        let uid = Uuid::new_v4();
        Self { con_actor: con_actor, name, uid }
    }
}

impl actix::Actor for HumanActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting HumanActor {} - {}", self.uid, self.name);
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
        log::info!("Ending HumanActor {}", self.uid);
        PairingActor::from_registry().do_send(RemoveAvailableHuman {
            uid: self.uid,
        });
    }
}

impl actix::Handler<ConnectionClosed> for HumanActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<ProcedureRequestingHuman> for HumanActor {
    type Result = ();

    fn handle(&mut self, msg: ProcedureRequestingHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("AvailableProcedure", kosem_webapi::pairing_messages::AvailableProcedure {
            uid: msg.uid,
            name: msg.orig_request.name,
        }));
    }
}

impl actix::Handler<RemoveRequestForHuman> for HumanActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveRequestForHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("UnavailableProcedure", kosem_webapi::pairing_messages::UnavailableProcedure {
            uid: msg.uid,
        }));
    }
}

impl actix::Handler<JoinProcedure> for HumanActor {
    type Result = <JoinProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: JoinProcedure, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Human {} joined procedure {}", self.name, msg.uid);
        PairingActor::from_registry().do_send(HumanJoiningProcedure {
            human_uid: self.uid,
            request_uid: msg.uid,
        });
        Ok(())
    }
}

impl actix::Handler<PairingPerformed> for HumanActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Paired human {} to request {}", msg.human_uid, msg.request_uid);
        self.con_actor.do_send(RpcMessage::new("JoinConfirmation", kosem_webapi::pairing_messages::JoinConfirmation {
            human_uid: msg.human_uid,
            request_uid: msg.request_uid,
        }));
    }
}
