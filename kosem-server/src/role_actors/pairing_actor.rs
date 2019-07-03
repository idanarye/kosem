use std::collections::HashMap;

use kosem_webapi::Uuid;

use crate::internal_messages::TesterAvailable;

#[derive(Default)]
pub struct PairingActor {
    available_testers: HashMap<Uuid, TesterAvailable>,
}

impl actix::Supervised for PairingActor {
}

impl actix::SystemService for PairingActor {
}

impl actix::Actor for PairingActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TesterAvailable> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: TesterAvailable, _ctx: &mut Self::Context) -> Self::Result {
        self.available_testers.insert(msg.uid, msg);
    }
}
