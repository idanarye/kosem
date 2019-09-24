use actix::Message;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;

#[derive(Clone, Debug)]
pub struct MessageFromServer<T> {
    pub server_idx: usize,
    pub msg: T,
}

impl<T> Message for MessageFromServer<T> {
    type Result = ();
}

#[derive(Debug)]
pub enum MessageToGui {
    AvailableProcedure(MessageFromServer<pairing_messages::AvailableProcedure>),
    UnavailableProcedure(MessageFromServer<pairing_messages::UnavailableProcedure>),
    JoinConfirmation(MessageFromServer<pairing_messages::JoinConfirmation>),
}

impl Message for MessageToGui {
    type Result = ();
}

#[derive(Debug)]
pub struct UserSelectedProcedure {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
}

impl Message for UserSelectedProcedure {
    type Result = ();
}
