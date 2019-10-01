use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix::Message;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PushPhase {
    #[serde(default)]
    pub limit_to_human_uids: Vec<Uuid>,
}

impl Message for PushPhase {
    type Result = KosemResult<Uuid>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhasePushed {
    pub phase_uid: Uuid,
    pub parent_uid: Option<Uuid>,
}

impl Message for PhasePushed {
    type Result = ();
}
