use actix::Message;

use kosem_webapi::Uuid;

pub struct HumanPushPhase {
    pub phase_uid: Uuid,
}

impl Message for HumanPushPhase {
    type Result = ();
}
