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

impl Handler<ProcedureUnavailable> for GuiActor {
    type Result = <ProcedureUnavailable as actix::Message>::Result;

    fn handle(&mut self, msg: ProcedureUnavailable, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that procedure is no longer available: {}", msg.procedure_uid);
        self.gui_channel.send(MessageToGui::ProcedureUnavailable(msg)).unwrap();
    }
}

impl Handler<UserSelectedProcedure> for GuiActor {
    type Result = <UserSelectedProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: UserSelectedProcedure, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("User selected procedure: {}", msg.procedure_uid);
        self.client.do_send(msg);
    }
}
