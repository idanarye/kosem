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
    ShowAgain,
}

#[derive(Debug)]
pub enum MessageToProcedureScreen {
    PhasePushed(phase_control_messages::PhasePushed),
    PhasePopped(phase_control_messages::PhasePopped),
    ProcedureFinished(pairing_messages::ProcedureFinished),
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

#[derive(Debug, Message)]
pub enum WindowClosed {
    JoinScreen,
    ProcedureScreen {
        server_idx: usize,
    },
}

#[derive(Message)]
pub struct UserClickedButton {
    pub server_idx: usize,
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}
