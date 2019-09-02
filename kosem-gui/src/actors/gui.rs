use actix::prelude::*;

use crate::actors::client::GuiClientActor;
use crate::internal_messages::gui_control::*;

#[derive(typed_builder::TypedBuilder)]
pub struct GuiActor {
    #[allow(unused)]
    client: Addr<GuiClientActor>,
    #[allow(unused)]
    gui_channel: glib::Sender<MessageToGui>,
}

impl Actor for GuiActor {
    type Context = Context<Self>;
}

impl Handler<ProcedureAvailable> for GuiActor {
    type Result = <ProcedureAvailable as actix::Message>::Result;

    fn handle(&mut self, msg: ProcedureAvailable, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that procedure is available: {}", msg.name);
        self.gui_channel.send(MessageToGui::ProcedureAvailable(msg)).unwrap();
    }
}
