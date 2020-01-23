use serde::{Deserialize, Serialize};
use actix::Message;
use uuid::Uuid;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<Uuid>")]
pub struct RequestHuman {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<()>")]
pub struct AvailableProcedure {
    pub uid: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<()>")]
pub struct UnavailableProcedure {
    pub uid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<()>")]
pub struct JoinProcedure {
    pub uid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<()>")]
pub struct JoinConfirmation {
    pub request_uid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="KosemResult<()>")]
pub struct HumanJoined {
    pub request_uid: Uuid,
    pub human_uid: Uuid,
    pub human_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message)]
#[rtype(result="()")]
pub struct ProcedureFinished {
    pub request_uid: Uuid,
}
