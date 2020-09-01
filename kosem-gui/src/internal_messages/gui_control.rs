use actix::Message;

use kosem_webapi::Uuid;

#[derive(Clone, Debug)]
pub struct MessageFromServer<T> {
    pub server_idx: usize,
    pub msg: T,
}

impl<T> Message for MessageFromServer<T> {
    type Result = ();
}

#[derive(Debug, Message)]
#[rtype(result="()")]
pub struct UserSelectedProcedure {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct ProcedureScreenAttach {
    pub server_idx: usize,
    pub request_uid: Uuid,
    pub addr: actix::Addr<crate::work_on_procedure::WorkOnProcedureActor>,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct UserClickedButton {
    pub server_idx: usize,
    pub request_uid: Uuid,
    pub phase_uid: Uuid,
    pub button_name: Option<String>,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct ShowJoinMenu;
