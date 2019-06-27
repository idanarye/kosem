use serde::{Deserialize, Serialize};
use actix::Message;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsTestee {
    pub name: String,
}

impl Message for LoginAsTestee {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsTester {
    pub name: String,
}

impl Message for LoginAsTester {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginConfirmed {
    pub uid: Uuid,
}
