use actix::Message;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;
use kosem_webapi::phase_control_messages;

#[derive(Clone, Debug)]
pub struct MessageFromServer<T> {
    pub server_idx: usize,
    pub msg: T,
}

impl<T> Message for MessageFromServer<T> {
    type Result = ();
}

#[derive(Debug, Message)]
pub enum MessageToLoginScreen {
    AvailableProcedure(MessageFromServer<pairing_messages::AvailableProcedure>),
    UnavailableProcedure(MessageFromServer<pairing_messages::UnavailableProcedure>),
    JoinConfirmation(MessageFromServer<pairing_messages::JoinConfirmation>),
}

#[derive(Debug)]
pub enum MessageToProcedureScreen {
    PhasePushed(phase_control_messages::PhasePushed),
}

#[derive(Debug, Message)]
pub struct MessageToProcedureScreenWrapper {
    pub server_idx: usize,
    pub msg: MessageToProcedureScreen,
}

#[derive(Debug, Message)]
pub struct UserSelectedProcedure {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
}

#[derive(Message)]
pub struct ProcedureScreenSetChannel {
    pub server_idx: usize,
    pub channel: glib::Sender<MessageToProcedureScreen>,
}
