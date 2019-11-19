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

impl Handler<WindowClosed> for GuiActor {
    type Result = <WindowClosed as actix::Message>::Result;

    fn handle(&mut self, msg: WindowClosed, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Got {:?}", msg);
        match msg {
            WindowClosed::JoinScreen => {
                if self.procedure_screen_channels.is_empty() {
                    log::info!("Join screen closed - exiting");
                    System::current().stop();
                }
            }
            WindowClosed::ProcedureScreen { server_idx, by_user } => {
                self.procedure_screen_channels.remove(&server_idx);
                // TODO: send disconnect message?
                if by_user && self.procedure_screen_channels.is_empty() {
                    log::info!("Last procedure screen closed - exiting");
                    System::current().stop();
                }
            }
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

impl Handler<UserClickedButton> for GuiActor {
    type Result = <UserClickedButton as actix::Message>::Result;

    fn handle(&mut self, msg: UserClickedButton, _ctx: &mut Self::Context) -> Self::Result {
        self.client.do_send(msg);
    }
}
