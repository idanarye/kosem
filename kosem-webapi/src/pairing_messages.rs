use serde::{Deserialize, Serialize};
use actix::Message;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestHuman {
    pub name: String,
}

impl Message for RequestHuman {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailableProcedure {
    pub uid: Uuid,
    pub name: String,
}

impl Message for AvailableProcedure {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnavailableProcedure {
    pub uid: Uuid,
}

impl Message for UnavailableProcedure {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinProcedure {
    pub uid: Uuid,
}

impl Message for JoinProcedure {
    type Result = ();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinConfirmation {
    pub human_uid: Uuid,
    pub request_uid: Uuid,
}

impl Message for JoinConfirmation {
    type Result = ();
}
