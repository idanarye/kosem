use actix::Message;

use kosem_webapi::Uuid;

pub struct ProcedureAvailable {
    pub server_idx: usize,
    pub procedure_uid: Uuid,
    pub name: String,
}

impl Message for ProcedureAvailable {
    type Result = ();
}

pub struct TmpButtonClicked;

impl Message for TmpButtonClicked {
    type Result = ();
}

#[derive(Debug)]
pub enum MessageToGui {
    TmpSayHello(),
}
