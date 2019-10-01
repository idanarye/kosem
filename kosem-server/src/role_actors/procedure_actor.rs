use std::collections::{HashMap, HashSet};

use actix::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages::*;
use kosem_webapi::phase_control_messages::*;

use crate::common_types::Phase;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::role_actors::{PairingActor, HumanActor};
use crate::internal_messages::connection::{RpcMessage, ConnectionClosed};
use crate::internal_messages::pairing::{
    RemoveRequestForHuman,
    ProcedureRequestingHuman,
    PairingPerformed,
};

#[derive(typed_builder::TypedBuilder)]
pub struct ProcedureActor {
    con_actor: actix::Addr<WsJrpc>,
    pub uid: Uuid,
    name: String,
    #[builder(default)]
    pending_requests_for_humans: HashSet<Uuid>,
    #[builder(default)]
    humans: HashMap<Uuid, Addr<HumanActor>>,
    #[builder(default)]
    phase_uids: Vec<Uuid>,
    #[builder(default)]
    phases: HashMap<Uuid, Phase>,
}

impl actix::Actor for ProcedureActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting ProcedureActor {} - {}", self.uid, self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending ProcedureActor {}", self.uid);
        for pending_request in self.pending_requests_for_humans.iter() {
            PairingActor::from_registry().do_send(RemoveRequestForHuman {
                uid: *pending_request,
            });
        }
    }
}

impl actix::Handler<ConnectionClosed> for ProcedureActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<RequestHuman> for ProcedureActor {
    type Result = <RequestHuman as actix::Message>::Result;

    fn handle(&mut self, msg: RequestHuman, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("RequestHuman from {}: {:?}", self.name, msg);
        let uid = Uuid::new_v4();
        self.pending_requests_for_humans.insert(uid);
        PairingActor::from_registry().do_send(ProcedureRequestingHuman {
            uid: uid,
            orig_request: msg,
            addr: ctx.address(),
        });
        Ok(())
    }
}

impl actix::Handler<PairingPerformed> for ProcedureActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Paired request {} to human {}", msg.request_uid, msg.human_uid);
        self.pending_requests_for_humans.remove(&msg.request_uid);
        if self.pending_requests_for_humans.is_empty() {
            log::info!("Procedure {} got all the humans it needs!", self.name);
        } else {
            log::info!("Procedure {} still needs {} more humans...", self.name, self.pending_requests_for_humans.len());
        }

        self.humans.insert(msg.human_uid, msg.human_addr);

        self.con_actor.do_send(RpcMessage::new("JoinConfirmation", kosem_webapi::pairing_messages::JoinConfirmation {
            human_uid: msg.human_uid,
            request_uid: msg.request_uid,
        }));
    }
}

impl actix::Handler<PushPhase> for ProcedureActor {
    type Result = <PushPhase as actix::Message>::Result;

    fn handle(&mut self, msg: PushPhase, _ctx: &mut actix::Context<Self>) -> Self::Result {
        let phase_uid = Uuid::new_v4();
        log::info!("Phase pushed: {:?}. Generated UID {}", msg, phase_uid);
        let phase = Phase {
        };
        self.phase_uids.push(phase_uid);
        self.phases.insert(phase_uid, phase);
        Ok(phase_uid)
    }
}
