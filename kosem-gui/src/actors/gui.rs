use actix::prelude::*;

use crate::actors::client::GuiClientActor;
use crate::internal_messages::gui_control::*;

#[derive(typed_builder::TypedBuilder)]
pub struct GuiActor {
    client: Addr<GuiClientActor>,
}

impl Actor for GuiActor {
    type Context = Context<Self>;
}

impl Handler<ProcedureAvailable> for GuiActor {
    type Result = <ProcedureAvailable as actix::Message>::Result;

    fn handle(&mut self, msg: ProcedureAvailable, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that procedure is available: {}", msg.name);
    }
}
