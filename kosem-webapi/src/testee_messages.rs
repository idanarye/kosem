use serde::{Deserialize, Serialize};
use actix_web::actix::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTester {
    pub name: String,
}

impl Message for RequestTester {
    type Result = ();
}
