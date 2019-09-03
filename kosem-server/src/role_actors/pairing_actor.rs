use std::collections::HashMap;

use kosem_webapi::Uuid;

use crate::internal_messages::pairing::*;

#[derive(Default)]
pub struct PairingActor {
    available_humans: HashMap<Uuid, HumanAvailable>,
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
        log::info!("Adding human, {} humans already exist", self.available_humans.len());
        self.available_humans.insert(msg.uid, msg);
        log::info!("Added human, {} humans already exist", self.available_humans.len());
    }
}
impl actix::Handler<ProcedureRequestingHuman> for PairingActor {

    type Result = ();

    fn handle(&mut self, msg: ProcedureRequestingHuman, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Adding message, {} humans already exist", self.available_humans.len());
        for human in self.available_humans.values() {
            human.addr.do_send(msg.clone());
        }
        self.procedures_requesting_humans.insert(msg.uid, msg);
    }
}

impl actix::Handler<RemoveRequestForHuman> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveRequestForHuman, _ctx: &mut Self::Context) -> Self::Result {
        for human in self.available_humans.values() {
            human.addr.do_send(msg.clone());
        }
        self.procedures_requesting_humans.remove(&msg.uid);
    }
}

impl actix::Handler<RemoveAvailableHuman> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: RemoveAvailableHuman, _ctx: &mut Self::Context) -> Self::Result {
        self.available_humans.remove(&msg.uid);
    }
}
