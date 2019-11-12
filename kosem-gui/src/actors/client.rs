use std::collections::HashMap;

use serde::Deserialize;
use actix::prelude::*;
// use actix::io::{SinkWrite, WriteHandler};
// use actix_codec::{AsyncRead, AsyncWrite, Framed};

// use futures::{lazy, Future};
// use futures::stream::{SplitSink};

// use awc::Client;
// use awc::ws::{Frame, Codec, Message};
// use awc::error::WsProtocolError;

use kosem_base_rpc_client::{ClientActor, wrap_addr_as_routing};
use kosem_base_rpc_client::control_messages::{
    RpcMessage,
    ConnectClientActor,
};
use kosem_base_rpc_client::config::ServerConfig;

use kosem_webapi::Uuid;
use kosem_webapi::handshake_messages::{
    LoginAsHuman,
    LoginConfirmed,
};
use kosem_webapi::pairing_messages::{
    AvailableProcedure,
    UnavailableProcedure,
    JoinProcedure,
    JoinConfirmation,
    ProcedureFinished,
};
use kosem_webapi::phase_control_messages::{
    PhasePushed,
    PhasePopped,
    ClickButton,
};

use crate::client_config::ClientConfig;
use crate::actors::gui::GuiActor;
use crate::internal_messages::gui_control::*;

#[derive(typed_builder::TypedBuilder)]
pub struct GuiClientActor {
    #[builder(default)]
    uid: Option<Uuid>,
    gui: Addr<GuiActor>,
    config: ClientConfig,
    #[builder(default)]
    client_actors: HashMap<usize, (ServerConfig, Addr<ClientActor>)>,
}

impl Actor for GuiClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        for (idx, server_config) in self.config.servers.iter().enumerate() {
            ClientActor::start_actor(idx, server_config.clone(), wrap_addr_as_routing!(addr));
        }
    }
}

impl Handler<ConnectClientActor> for GuiClientActor {
    type Result = <ConnectClientActor as actix::Message>::Result;

    fn handle(&mut self, msg: ConnectClientActor, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Connecting a client actor");
        msg.client_actor.do_send(RpcMessage::new("LoginAsHuman", LoginAsHuman {
            name: self.config.display_name.clone(),
        }));
        self.client_actors.insert(msg.idx, (msg.server_config, msg.client_actor));
    }
}

impl Handler<RpcMessage> for GuiClientActor {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, _ctx: &mut Self::Context) -> Self::Result {
        let server_idx = msg.idx.expect("Clients should have index");
        let config = &self.config.servers[server_idx];
        log::info!("GuiClientActor got {}: {:?} from server {:?}", msg.method, msg.params, config);

        macro_rules! redirect_to_gui {
            (
                join_screen = {
                    $($join_screen_msg:ident),
                    * $(,)?
                }
                procedure_screen = {
                    $($procedure_screen_msg:ident),
                    * $(,)?
                }
                $(
                    $pattern:pat => $expr:expr
                ),*$(,)?
            ) => {
                {
                    if false {
                        // NOTE: this should fail on "non-exhaustive patterns" if we forget to
                        // redirect some of the messages supported by `MessageToLoginScreen` or
                        // `MessageToProcedureScreen`
                        match unreachable!() {
                            $(
                                #[allow(unreachable_code)]
                                MessageToLoginScreen::$join_screen_msg(_) => unreachable!()
                            ),*,
                            MessageToLoginScreen::ShowAgain => unreachable!(),
                        }
                        match unreachable!() {
                            $(
                                #[allow(unreachable_code)]
                                MessageToProcedureScreen::$procedure_screen_msg(_) => unreachable!()
                            ),*
                        }
                    }
                    match msg.method.as_ref() {
                        $(
                            stringify!($join_screen_msg) => {
                                self.gui.do_send(MessageToLoginScreen::$join_screen_msg(MessageFromServer {
                                    server_idx,
                                    msg: $join_screen_msg::deserialize(msg.params).unwrap(),
                                }));
                            }
                        ),*,
                        $(
                            stringify!($procedure_screen_msg) => {
                                let msg = MessageToProcedureScreen::$procedure_screen_msg($procedure_screen_msg::deserialize(msg.params).unwrap());
                                self.gui.do_send(MessageToProcedureScreenWrapper { server_idx, msg });
                        }
                        ),*,
                        $(
                            $pattern => $expr
                        ),*
                    }
                }
            }
        }

        redirect_to_gui!(
            join_screen = {
                AvailableProcedure,
                UnavailableProcedure,
                JoinConfirmation,
            }
            procedure_screen = {
                PhasePushed,
                PhasePopped,
                ProcedureFinished,
            }
            "LoginConfirmed" => {
                let params = LoginConfirmed::deserialize(msg.params).unwrap();
                log::info!("Setting uid to {}", params.uid);
                self.uid = Some(params.uid);
            },
            unknown_method => {
                log::warn!("Unknown method {}", unknown_method);
            }
        );
    }
}

impl Handler<UserSelectedProcedure> for GuiClientActor {
    type Result = <UserSelectedProcedure as actix::Message>::Result;

    fn handle(&mut self, msg: UserSelectedProcedure, _ctx: &mut Self::Context) -> Self::Result {
        log::warn!("User selected procedure: {}", msg.procedure_uid);
        self.client_actors.get(&msg.server_idx).map(|(_, client)| {
            client.do_send(RpcMessage::new("JoinProcedure", JoinProcedure {
                uid: msg.procedure_uid,
            }));
        });
    }
}

impl Handler<UserClickedButton> for GuiClientActor {
    type Result = <UserClickedButton as actix::Message>::Result;

    fn handle(&mut self, msg: UserClickedButton, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("GuiClientActor UserClickedButton");
        if let Some((_, client)) = self.client_actors.get(&msg.server_idx) {
            client.do_send(RpcMessage::new("ClickButton", ClickButton {
                phase_uid: msg.phase_uid,
                button_name: msg.button_name,
            }));
        }
    }
}
