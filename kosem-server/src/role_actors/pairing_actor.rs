use std::collections::HashMap;

use actix::prelude::*;

use kosem_webapi::{Uuid, KosemError};

use crate::internal_messages::pairing::*;

#[derive(Default)]
pub struct PairingActor {
    available_joiners: HashMap<Uuid, HumanAvailable>,
    procedures_requesting_humans: HashMap<Uuid, ProcedureRequestingHuman>,
}

impl actix::Supervised for PairingActor {
}

impl actix::SystemService for PairingActor {
}

impl actix::Actor for PairingActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<HumanAvailable> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: HumanAvailable, _ctx: &mut Self::Context) -> Self::Result {
        for procedure in self.procedures_requesting_humans.values() {
            msg.addr.do_send(procedure.clone());
        }
        log::info!("Adding joiner, {} joiners already exist", self.available_joiners.len());
        self.available_joiners.insert(msg.uid, msg);
        log::info!("Added joiner, {} joiners already exist", self.available_joiners.len());
    }
}
impl actix::Handler<ProcedureRequestingHuman> for PairingActor {

    type Result = ();

    fn handle(&mut self, msg: ProcedureRequestingHuman, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Adding message, {} joiners already exist", self.available_joiners.len());
        for joiner in self.available_joiners.values() {
            joiner.addr.do_send(msg.clone());
        }
        self.procedures_requesting_humans.insert(msg.uid, msg);
    }
}

impl actix::Handler<RemoveRequestForHuman> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveRequestForHuman, _ctx: &mut Self::Context) -> Self::Result {
        for joiner in self.available_joiners.values() {
            joiner.addr.do_send(msg.clone());
        }
        self.procedures_requesting_humans.remove(&msg.uid);
    }
}

impl actix::Handler<RemoveAvailableHuman> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveAvailableHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.available_joiners.remove(&msg.uid);
    }
}

impl actix::Handler<HumanJoiningProcedure> for PairingActor {
    type Result = ResponseActFuture<Self, (), KosemError>;

    fn handle(&mut self, msg: HumanJoiningProcedure, _ctx: &mut Self::Context) -> Self::Result {
        let joiner_entry = self.available_joiners.entry(msg.human_uid);
        let request_entry = self.procedures_requesting_humans.entry(msg.request_uid);
        use std::collections::hash_map::Entry;
        let (joiner_entry, request_entry) = match (joiner_entry, request_entry) {
            (Entry::Occupied(joiner_entry), Entry::Occupied(request_entry)) => {
                (joiner_entry, request_entry)
            },
            (Entry::Vacant(_), _) => {
                return Box::new(fut::err(KosemError::new("Human is not available for handling procedures")));
            },
            (_, Entry::Vacant(_)) => {
                return Box::new(fut::err(
                    KosemError::new("Request does not exist in pending requests")
                    .with("request_uid", msg.request_uid)
                ));
            },
        };

        let joiner = joiner_entry.get();
        let joiner_addr = joiner.addr.clone();
        let joiner_uid = joiner.uid;
        let request = request_entry.remove();

        Box::new(
            joiner_addr.send(CreateNewHumanActor {
                request_uid: request.uid,
                procedure_addr: request.addr.clone(),
            })
            .into_actor(self)
            .map_err(|e, _, _| {
                panic!(e);
            })
            .and_then(move |human_addr, actor, _ctx| {
                let pairing_performed = PairingPerformed {
                    human_uid: joiner_uid,
                    human_addr,
                    request_uid: request.uid,
                    procedure_addr: request.addr,
                };

                joiner_addr.do_send(pairing_performed.clone());
                pairing_performed.procedure_addr.clone().do_send(pairing_performed);

                // NOTE: this does not include the joiner that accepted the request, because they were just
                // removed.
                for joiner in actor.available_joiners.values() {
                    if joiner.uid != joiner_uid {
                        joiner.addr.do_send(RemoveRequestForHuman {
                            uid: request.uid,
                        });
                    }
                }
                fut::ok(())
            }))
    }
}
