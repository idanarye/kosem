use serde::{Deserialize, Serialize};
use actix_web::actix::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsTestee {
    pub name: String,
}

impl Message for LoginAsTestee {
    type Result = ();
}
