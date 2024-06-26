use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix::Message;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<Uuid>")]
pub struct PushPhase {
    #[serde(default)]
    pub limit_to_human_uids: Vec<Uuid>,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct PhasePushed {
    pub request_uid: Uuid,
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
    Caption { text: String },
    Button { text: String },
    Textbox { text: String },
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<()>")]
pub struct PopPhase {
    pub phase_uid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct PhasePopped {
    pub request_uid: Uuid,
    pub phase_uid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<()>")]
pub struct ClickButton {
    pub request_uid: Uuid,
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct ButtonClicked {
    pub human_uid: Uuid,
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<()>")]
pub struct ReadPhaseData {
    #[serde(default)]
    pub limit_to_human_uids: Vec<Uuid>,
    pub phase_uid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct PhaseDataReadRequest {
    pub request_uid: Uuid,
    pub phase_uid: Uuid,
}
