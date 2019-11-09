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
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: Option<String>,
    #[serde(flatten)]
    pub params: ComponentParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum ComponentParams {
    Caption {
        text: String,
    },
    Button {
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result="KosemResult<()>")]
pub struct ClickButton {
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
pub struct ButtonClicked {
    pub human_uid: Uuid,
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}
