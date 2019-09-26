use actix::{Message, Addr};

use kosem_webapi::{Uuid, KosemResult};
use kosem_webapi::pairing_messages::RequestHuman;

use crate::role_actors::{ProcedureActor, HumanActor};

pub struct HumanAvailable {
    pub uid: Uuid,
    pub name: String,
    pub addr: Addr<HumanActor>,
}

impl Message for HumanAvailable {
    type Result = ();
}

#[derive(Clone)]
pub struct ProcedureRequestingHuman {
    pub uid: Uuid,
    pub orig_request: RequestHuman,
    pub addr: Addr<ProcedureActor>,
}

impl Message for ProcedureRequestingHuman {
    type Result = ();
}

pub struct RemoveAvailableHuman {
    pub uid: Uuid,
}

impl Message for RemoveAvailableHuman {
    type Result = ();
}

#[derive(Clone)]
pub struct RemoveRequestForHuman {
    pub uid: Uuid,
}

impl Message for RemoveRequestForHuman {
    type Result = ();
}

pub struct HumanJoiningProcedure {
    pub human_uid: Uuid,
    pub request_uid: Uuid,
}

impl Message for HumanJoiningProcedure {
    type Result = KosemResult<()>;
}

#[derive(Clone)]
pub struct PairingPerformed {
    pub human_uid: Uuid,
    pub human_addr: Addr<HumanActor>,
    pub request_uid: Uuid,
    pub procedure_addr: Addr<ProcedureActor>,
}

impl Message for PairingPerformed {
    type Result = ();
}
