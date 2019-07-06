use serde::{Deserialize, Serialize};
use actix::Message;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsProcedure {
    pub name: String,
}

impl Message for LoginAsProcedure {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsHuman {
    pub name: String,
}

impl Message for LoginAsHuman {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginConfirmed {
    pub uid: Uuid,
}
