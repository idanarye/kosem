use actix::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::KosemResult;

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<Uuid>")]
pub struct LoginAsProcedure {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "KosemResult<Uuid>")]
pub struct LoginAsHuman {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginConfirmed {
    pub uid: Uuid,
}
