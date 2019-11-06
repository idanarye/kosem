use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix::Message;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result="KosemResult<Uuid>")]
pub struct PushPhase {
    #[serde(default)]
    pub limit_to_human_uids: Vec<Uuid>,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize, Message)]
pub struct PhasePushed {
    pub phase_uid: Uuid,
    pub parent_uid: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub name: Option<String>,
    #[serde(flatten)]
    pub params: ComponentParams,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum ComponentParams {
    Caption {
        text: String,
    },
    Button {
        text: String,
    },
}
