use actix::{Addr, Message};

use kosem_webapi::pairing_messages::RequestHuman;
use kosem_webapi::{KosemResult, Uuid};

use crate::role_actors::{HumanActor, JoinerActor, ProcedureActor};

#[derive(Message)]
#[rtype(result = "()")]
pub struct HumanAvailable {
    pub uid: Uuid,
    pub name: String,
    pub addr: Addr<JoinerActor>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ProcedureRequestingHuman {
    pub uid: Uuid,
    pub orig_request: RequestHuman,
    pub addr: Addr<ProcedureActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveAvailableHuman {
    pub uid: Uuid,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveRequestForHuman {
    pub uid: Uuid,
}

#[derive(Message)]
#[rtype(result = "KosemResult<()>")]
pub struct HumanJoiningProcedure {
    pub human_uid: Uuid,
    pub request_uid: Uuid,
}

#[derive(Message)]
#[rtype(result = "Addr<HumanActor>")]
pub struct CreateNewHumanActor {
    pub request_uid: Uuid,
    pub procedure_addr: Addr<ProcedureActor>,
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct PairingPerformed {
    pub human_uid: Uuid,
    pub human_addr: Addr<HumanActor>,
    pub request_uid: Uuid,
    pub procedure_addr: Addr<ProcedureActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ProcedureTerminated {
    pub procedure_uid: Uuid,
}
