use actix::Message;

use crate::role_actors::TesteeActor;

pub enum SetRole {
    Testee(actix::Addr<TesteeActor>),
}

impl Message for SetRole {
    type Result = ();
}