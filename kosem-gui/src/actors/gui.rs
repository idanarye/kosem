use std::collections::{HashMap, VecDeque};

use actix::prelude::*;

use crate::actors::client::GuiClientActor;
use crate::internal_messages::gui_control::*;

#[derive(typed_builder::TypedBuilder)]
pub struct GuiActor {
    client: Addr<GuiClientActor>,
    login_screen_channel: glib::Sender<MessageToLoginScreen>,
    #[builder(default)]
    procedure_screen_channels: HashMap<usize, glib::Sender<MessageToProcedureScreen>>,
    #[builder(default)]
    pending_messages_to_procedures: HashMap<usize, VecDeque<MessageToProcedureScreen>>,
}

impl Actor for GuiActor {
    type Context = Context<Self>;
}

impl Handler<MessageToLoginScreen> for GuiActor {
    type Result = <MessageToLoginScreen as actix::Message>::Result;

    fn handle(&mut self, msg: MessageToLoginScreen, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that {:?}", msg);
        self.login_screen_channel.send(msg).unwrap();
    }
}

impl Handler<ProcedureScreenSetChannel> for GuiActor {
    type Result = <ProcedureScreenSetChannel as actix::Message>::Result;

    fn handle(&mut self, msg: ProcedureScreenSetChannel, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(pending) = self.pending_messages_to_procedures.remove(&msg.server_idx) {
            for pending_message in pending {
                msg.channel.send(pending_message).unwrap();
            }
        }
        self.procedure_screen_channels.insert(msg.server_idx, msg.channel);
    }
}

impl Handler<MessageToProcedureScreenWrapper> for GuiActor {
    type Result = <MessageToProcedureScreenWrapper as actix::Message>::Result;

    fn handle(&mut self, msg: MessageToProcedureScreenWrapper, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Gui knows that {:?}", msg);
        if let Some(channel) = self.procedure_screen_channels.get(&msg.server_idx) {
            channel.send(msg.msg).unwrap();
        } else {
            self.pending_messages_to_procedures.entry(msg.server_idx).or_default().push_back(msg.msg);
        }
    }
}

impl Handler<UserSelectedProcedure> for GuiActor {
    type Result = <UserSelectedProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: UserSelectedProcedure, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("User selected procedure: {}", msg.procedure_uid);
        self.client.do_send(msg);
    }
}
