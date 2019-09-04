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

impl actix::Handler<HumanJoiningProcedure> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: HumanJoiningProcedure, _ctx: &mut Self::Context) -> Self::Result {
        let human_entry = self.available_humans.entry(msg.human_uid);
        let request_entry = self.procedures_requesting_humans.entry(msg.request_uid);
        use std::collections::hash_map::Entry;
        let (human_entry, request_entry) = match (human_entry, request_entry) {
            (Entry::Occupied(human_entry), Entry::Occupied(request_entry)) => {
                (human_entry, request_entry)
            },
            (_, _) => {
                // TODO: deal with situations when human or request is missing
                panic!("Missing entries");
            },
        };

        let human = human_entry.remove();
        let request = request_entry.remove();

        let pairing_performed = PairingPerformed {
            human_uid: human.uid,
            human_addr: human.addr,
            request_uid: request.uid,
            procedure_addr: request.addr,
        };

        pairing_performed.human_addr.do_send(pairing_performed.clone());
        pairing_performed.procedure_addr.clone().do_send(pairing_performed);
    }
}
