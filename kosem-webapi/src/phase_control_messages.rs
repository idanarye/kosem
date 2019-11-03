use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix::Message;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result="KosemResult<Uuid>")]
pub struct PushPhase {
    #[serde(default)]
    pub limit_to_human_uids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Message)]
pub struct PhasePushed {
    pub phase_uid: Uuid,
    pub parent_uid: Option<Uuid>,
}
