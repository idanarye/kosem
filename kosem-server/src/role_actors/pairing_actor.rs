use std::collections::HashMap;

use kosem_webapi::Uuid;

use crate::internal_messages::HumanAvailable;

#[derive(Default)]
pub struct PairingActor {
    available_humans: HashMap<Uuid, HumanAvailable>,
}

impl actix::Supervised for PairingActor {
}

impl actix::SystemService for PairingActor {
}

impl actix::Actor for PairingActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<HumanAvailable> for PairingActor {
    type Result = ();

    fn handle(&mut self, msg: HumanAvailable, _ctx: &mut Self::Context) -> Self::Result {
        self.available_humans.insert(msg.uid, msg);
    }
}
