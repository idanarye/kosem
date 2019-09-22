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

impl Handler<MessageToGui> for GuiActor {
    type Result = <MessageToGui as actix::Message>::Result;

    fn handle(&mut self, msg: MessageToGui, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that {:?}", msg);
        self.gui_channel.send(msg).unwrap();
    }
}

impl Handler<UserSelectedProcedure> for GuiActor {
    type Result = <UserSelectedProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: UserSelectedProcedure, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("User selected procedure: {}", msg.procedure_uid);
        self.client.do_send(msg);
    }
}
