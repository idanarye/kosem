use actix::Message;

use kosem_webapi::Uuid;

#[derive(Debug)]
pub struct ProcedureAvailable {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
    pub name: String,
}

impl Message for ProcedureAvailable {
    type Result = ();
}

#[derive(Debug)]
pub struct ProcedureUnavailable {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
}

impl Message for ProcedureUnavailable {
    type Result = ();
}

#[derive(Debug)]
pub enum MessageToGui {
    ProcedureAvailable(ProcedureAvailable),
    ProcedureUnavailable(ProcedureUnavailable),
}
