use actix::Message;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages::RequestHuman;

use crate::role_actors::{ProcedureActor, HumanActor};

pub struct HumanAvailable {
    pub uid: Uuid,
    pub name: String,
    pub addr: actix::Addr<HumanActor>,
}

impl Message for HumanAvailable {
    type Result = ();
}

#[derive(Clone)]
pub struct ProcedureRequestingHuman {
    pub uid: Uuid,
    pub orig_request: RequestHuman,
    pub addr: actix::Addr<ProcedureActor>,
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
