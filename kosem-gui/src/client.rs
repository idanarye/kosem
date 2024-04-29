use std::collections::HashMap;

use actix::prelude::*;
use serde::Deserialize;

use kosem_base_rpc_client::config::ServerConfig;
use kosem_base_rpc_client::control_messages::{ConnectClientActor, RpcMessage};
use kosem_base_rpc_client::{wrap_addr_as_routing, ClientActor};

use kosem_webapi::handshake_messages::{LoginAsHuman, LoginConfirmed};
use kosem_webapi::pairing_messages::{
    AvailableProcedure, JoinConfirmation, JoinProcedure, ProcedureFinished, UnavailableProcedure,
};
use kosem_webapi::phase_control_messages::{ClickButton, PhasePopped, PhasePushed, PhaseDataReadRequest};
use kosem_webapi::Uuid;

use crate::internal_messages::gui_control::{
    MessageFromServer, ProcedureScreenAttach, UserClickedButton, UserSelectedProcedure,
};

#[derive(typed_builder::TypedBuilder)]
pub struct GuiClientActor {
    #[builder(default)]
    uid: Option<Uuid>,
    join_menu: Addr<crate::join_menu::JoinMenuActor>,
    config: crate::client_config::ClientConfig,
    #[builder(default)]
    client_actors: HashMap<usize, (ServerConfig, Addr<ClientActor>)>,
    #[builder(default)]
    procedure_screens: HashMap<Uuid, Addr<crate::work_on_procedure::WorkOnProcedureActor>>,
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
        msg.client_actor.do_send(RpcMessage::new(
            "LoginAsHuman",
            LoginAsHuman {
                name: self.config.display_name.clone(),
            },
        ));
        self.client_actors
            .insert(msg.idx, (msg.server_config, msg.client_actor));
    }
}

impl Handler<RpcMessage> for GuiClientActor {
    type Result = <RpcMessage as actix::Message>::Result;

    fn handle(&mut self, msg: RpcMessage, _ctx: &mut Self::Context) -> Self::Result {
        let server_idx = msg.idx.expect("Clients should have index");
        let _config = &self.config.servers[server_idx];

        macro_rules! redirect_to_gui {
            (
                join_screen = {
                    $($join_screen_msg:ident),* $(,)?
                }
                procedure_screen = {
                    $(
                        $procedure_screen_msg:ident
                        $( ($procedure_screen_param:ident) $procedure_screen_also_do:block )*
                    ),* $(,)?
                }
                $(
                    $pattern:pat => $expr:expr
                ),*$(,)?
            ) => {
                {
                    match msg.method.as_ref() {
                        $(
                            stringify!($join_screen_msg) => {
                                self.join_menu.do_send(MessageFromServer {
                                    server_idx,
                                    msg: $join_screen_msg::deserialize(msg.params).unwrap(),
                                });
                            }
                        ),*,
                        $(
                            stringify!($procedure_screen_msg) => {
                                let msg = $procedure_screen_msg::deserialize(msg.params).unwrap();
                                $(
                                    {
                                        let $procedure_screen_param = &msg;
                                        $procedure_screen_also_do;
                                    }
                                )*
                                if let Some(procedure_addr) = self.procedure_screens.get(&msg.request_uid) {
                                    procedure_addr.do_send(msg);
                                }
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
                PhaseDataReadRequest,
                ProcedureFinished(_msg) {
                    // self.gui.do_send(MessageToLoginScreen::ShowAgain);
                },
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
        if let Some((_, client)) = self.client_actors.get(&msg.server_idx) {
            client.do_send(RpcMessage::new(
                "JoinProcedure",
                JoinProcedure {
                    uid: msg.procedure_uid,
                },
            ));
        }
    }
}

impl Handler<ProcedureScreenAttach> for GuiClientActor {
    type Result = ();

    fn handle(&mut self, msg: ProcedureScreenAttach, _ctx: &mut Self::Context) -> Self::Result {
        self.procedure_screens.insert(msg.request_uid, msg.addr);
    }
}

impl Handler<UserClickedButton> for GuiClientActor {
    type Result = <UserClickedButton as actix::Message>::Result;

    fn handle(&mut self, msg: UserClickedButton, _ctx: &mut Self::Context) -> Self::Result {
        if let Some((_, client)) = self.client_actors.get(&msg.server_idx) {
            client.do_send(RpcMessage::new(
                "ClickButton",
                ClickButton {
                    request_uid: msg.request_uid,
                    phase_uid: msg.phase_uid,
                    button_name: msg.button_name,
                },
            ));
        }
    }
}
