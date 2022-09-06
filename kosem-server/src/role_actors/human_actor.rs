use actix::prelude::*;

use kosem_webapi::Uuid;

use crate::protocol_handlers::websocket_jsonrpc::WsJrpc;

use crate::internal_messages::connection::{AddHumanActor, ConnectionClosed, RpcMessage};
use crate::internal_messages::info_sharing;
use crate::internal_messages::pairing::{PairingPerformed, ProcedureTerminated};
use kosem_webapi::phase_control_messages::*;

use crate::role_actors::ProcedureActor;

#[derive(typed_builder::TypedBuilder)]
pub struct HumanActor {
    con_actor: actix::Addr<WsJrpc>,
    procedure_actor: actix::Addr<ProcedureActor>,
    uid: Uuid,
    request_uid: Uuid,
    name: String,
}

impl actix::Actor for HumanActor {
    type Context = actix::Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Starting HumanActor {} - {}", self.uid, self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Ending HumanActor {}", self.uid);
    }
}

impl actix::Handler<ConnectionClosed> for HumanActor {
    type Result = ();

    fn handle(&mut self, _msg: ConnectionClosed, ctx: &mut actix::Context<Self>) -> Self::Result {
        ctx.stop();
    }
}

impl actix::Handler<PairingPerformed> for HumanActor {
    type Result = <PairingPerformed as actix::Message>::Result;

    fn handle(&mut self, msg: PairingPerformed, ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!(
            "Paired human {} to request {}",
            msg.human_uid,
            msg.request_uid
        );
        self.con_actor.do_send(AddHumanActor {
            request_uid: msg.request_uid,
            addr: ctx.address(),
        });
        self.con_actor.do_send(RpcMessage::new(
            "JoinConfirmation",
            kosem_webapi::pairing_messages::JoinConfirmation {
                request_uid: msg.request_uid,
            },
        ));
    }
}

impl actix::Handler<ProcedureTerminated> for HumanActor {
    type Result = <ProcedureTerminated as actix::Message>::Result;

    fn handle(
        &mut self,
        _msg: ProcedureTerminated,
        _ctx: &mut actix::Context<Self>,
    ) -> Self::Result {
        self.con_actor.do_send(RpcMessage::new(
            "ProcedureFinished",
            kosem_webapi::pairing_messages::ProcedureFinished {
                request_uid: self.request_uid,
            },
        ));
    }
}

impl actix::Handler<info_sharing::GetInfo<info_sharing::HumanDetails>> for HumanActor {
    type Result = <info_sharing::GetInfo<info_sharing::HumanDetails> as actix::Message>::Result;

    fn handle(
        &mut self,
        _msg: info_sharing::GetInfo<info_sharing::HumanDetails>,
        _ctx: &mut actix::Context<Self>,
    ) -> Self::Result {
        info_sharing::HumanDetails {
            name: self.name.clone(),
        }
    }
}

impl actix::Handler<PhasePushed> for HumanActor {
    type Result = <PhasePushed as actix::Message>::Result;

    fn handle(&mut self, msg: PhasePushed, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Human {} got phase {}", self.uid, msg.phase_uid);
        self.con_actor.do_send(RpcMessage::new("PhasePushed", msg));
    }
}

impl actix::Handler<PhasePopped> for HumanActor {
    type Result = <PhasePopped as actix::Message>::Result;

    fn handle(&mut self, msg: PhasePopped, _ctx: &mut actix::Context<Self>) -> Self::Result {
        log::info!("Human {} got phase {}", self.uid, msg.phase_uid);
        self.con_actor.do_send(RpcMessage::new("PhasePopped", msg));
    }
}

impl actix::Handler<ClickButton> for HumanActor {
    type Result = <ClickButton as actix::Message>::Result;

    fn handle(&mut self, msg: ClickButton, _ctx: &mut actix::Context<Self>) -> Self::Result {
        self.procedure_actor.do_send(ButtonClicked {
            human_uid: self.uid,
            phase_uid: msg.phase_uid,
            button_name: msg.button_name,
        });
        Ok(())
    }
}
