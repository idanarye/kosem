use serde::{Deserialize, Serialize};
use actix::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestHuman {
    pub name: String,
}

impl Message for RequestHuman {
    type Result = ();
}
