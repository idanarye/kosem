use std::collections::{HashMap, HashSet};

use actix::prelude::*;

use kosem_webapi::{Uuid, KosemError};
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
    ProcedureTerminated,
};
use crate::internal_messages::info_sharing;

#[derive(typed_builder::TypedBuilder)]
pub struct ProcedureActor {
    con_actor: actix::Addr<WsJrpc>,
    pub uid: Uuid,
    name: String,
    #[builder(default)]
    pending_requests_for_humans: HashSet<Uuid>,
    #[builder(default)]
    humans: HashMap<Uuid, Addr<HumanActor>>,  // NOTE: the key is the request UID, not the human UID
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
        for human in self.humans.values() {
            human.do_send(ProcedureTerminated {
                procedure_uid: self.uid,
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
        Ok(uid)
    }
}

impl actix::Handler<PairingPerformed> for ProcedureActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Paired request {} to human {}", msg.request_uid, msg.human_uid);
        self.pending_requests_for_humans.remove(&msg.request_uid);
        if self.pending_requests_for_humans.is_empty() {
            log::info!("Procedure {} got all the humans it needs!", self.name);
        } else {
            log::info!("Procedure {} still needs {} more humans...", self.name, self.pending_requests_for_humans.len());
        }

        self.humans.insert(msg.request_uid, msg.human_addr.clone());

        let PairingPerformed { human_uid, request_uid, .. } = msg;

        ctx.spawn(
            msg.human_addr.send(info_sharing::GetInfo::<info_sharing::HumanDetails>::default())
            .into_actor(self)
            .map_err(|e, _, _| panic!(e))
            .map(move |msg, this, _ctx| {
                this.con_actor.do_send(RpcMessage::new("HumanJoined", kosem_webapi::pairing_messages::HumanJoined {
                    human_uid,
                    request_uid,
                    human_name: msg.name,
                }));
            })
        );

        for (&phase_uid, phase) in self.phases.iter() {
            msg.human_addr.do_send(PhasePushed {
                request_uid,
                phase_uid,
                parent_uid: None,
                components: phase.components.clone(),
            });
        }
    }
}

impl actix::Handler<PushPhase> for ProcedureActor {
    type Result = <PushPhase as actix::Message>::Result;

    fn handle(&mut self, msg: PushPhase, _ctx: &mut actix::Context<Self>) -> Self::Result {
        let phase_uid = Uuid::new_v4();
        log::info!("Phase pushed: {:?}. Generated UID {}", msg, phase_uid);
        let phase = Phase::new(msg.components);
        log::info!("Phase looks like this: {:?}", phase);
        self.phase_uids.push(phase_uid);
        for (&request_uid, human) in self.humans.iter() {
            log::info!("Informing {} of {}", request_uid, phase_uid);
            human.do_send(PhasePushed {
                request_uid,
                phase_uid,
                parent_uid: None,
                components: phase.components.clone(),
            });
        }
        self.phases.insert(phase_uid, phase);
        Ok(phase_uid)
    }
}

impl actix::Handler<PopPhase> for ProcedureActor {
    type Result = <PopPhase as actix::Message>::Result;

    fn handle(&mut self, msg: PopPhase, _ctx: &mut actix::Context<Self>) -> Self::Result {
        let _phase = self.phases.remove(&msg.phase_uid)
            .ok_or_else(|| KosemError::new("Phase does not exist").with("phase_uid", msg.phase_uid))?;
        for (&request_uid, human) in self.humans.iter() {
            human.do_send(PhasePopped {
                request_uid,
                phase_uid: msg.phase_uid,
            });
        }
        Ok(())
    }
}

impl actix::Handler<ButtonClicked> for ProcedureActor {
    type Result = <ButtonClicked as actix::Message>::Result;

    fn handle(&mut self, msg: ButtonClicked, _ctx: &mut actix::Context<Self>) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new("ButtonClicked", msg));
    }
}
