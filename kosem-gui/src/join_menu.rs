use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::{Uuid, KosemResult};
use kosem_webapi::pairing_messages;

use crate::internal_messages::gui_control::{
    MessageFromServer,
    UserSelectedProcedure,
};

#[derive(woab::Factories)]
pub struct JoinMenuFactories {
    pub app_join_menu_window: woab::Factory<JoinMenuActor, JoinMenuWidgets, JoinMenuSignal>,
    pub row_request: woab::Factory<RequestRowActor, RequestRowWidgets, RequestRowSignal>,
}

#[derive(typed_builder::TypedBuilder)]
pub struct JoinMenuActor {
    factories: crate::Factories,
    widgets: JoinMenuWidgets,
    gui_client: Addr<crate::client::GuiClientActor>,
    #[builder(default)]
    procedure_requests: HashMap<Uuid, Addr<RequestRowActor>>,
}

impl Actor for JoinMenuActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.widgets.app_join_menu_window.show_all();
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct JoinMenuWidgets {
    pub app_join_menu_window: gtk::ApplicationWindow,
    #[allow(dead_code)]
    lst_procedures: gtk::ListBox,
}

#[derive(woab::BuilderSignal)]
pub enum JoinMenuSignal {
}

impl StreamHandler<JoinMenuSignal> for JoinMenuActor {
    fn handle(&mut self, signal: JoinMenuSignal, _ctx: &mut Self::Context) {
        match signal {
        }
    }
}

impl Handler<MessageFromServer<pairing_messages::AvailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::AvailableProcedure>, _ctx: &mut Self::Context) -> Self::Result {
        let procedure_uid = msg.msg.uid;
        let new_row = self.factories.join_menu.row_request.build().actor(|_ctx, widgets| {
            self.widgets.lst_procedures.add(&widgets.row_request);
            RequestRowActor::builder()
                .factories(self.factories.clone())
                .widgets(widgets)
                .procedure(msg.msg)
                .gui_client(self.gui_client.clone())
                .server_idx(msg.server_idx)
                .build()
        }).unwrap();
        self.procedure_requests.insert(procedure_uid, new_row);
    }
}

impl Handler<MessageFromServer<pairing_messages::UnavailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::UnavailableProcedure>, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(row) = self.procedure_requests.remove(&msg.msg.uid) {
            row.do_send(woab::Remove);
            // row.do_send(msg.msg);
        }
    }
}

impl Handler<MessageFromServer<pairing_messages::JoinConfirmation>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::JoinConfirmation>, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(row) = self.procedure_requests.get(&msg.msg.request_uid) {
            row.do_send(msg.msg);
        }
    }
}

#[derive(typed_builder::TypedBuilder, woab::Removable)]
#[removable(self.widgets.row_request)]
pub struct RequestRowActor {
    #[allow(dead_code)]
    factories: crate::Factories,
    widgets: RequestRowWidgets,
    gui_client: Addr<crate::client::GuiClientActor>,
    server_idx: usize,
    procedure: pairing_messages::AvailableProcedure,
}

impl Actor for RequestRowActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.widgets.lbl_request_name.set_text(&self.procedure.name);
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct RequestRowWidgets {
    row_request: gtk::ListBoxRow,
    lbl_request_name: gtk::Label,
}

#[derive(woab::BuilderSignal)]
pub enum RequestRowSignal {
    ConnectToProcedure,
}

impl StreamHandler<RequestRowSignal> for RequestRowActor {
    fn handle(&mut self, signal: RequestRowSignal, _ctx: &mut Self::Context) {
        match signal {
            RequestRowSignal::ConnectToProcedure => {
                self.gui_client.do_send(UserSelectedProcedure {
                    server_idx: self.server_idx,
                    procedure_uid: self.procedure.uid,
                });
            }
        }
    }
}

impl Handler<pairing_messages::JoinConfirmation> for RequestRowActor {
    type Result = KosemResult<()>;

    fn handle(&mut self, msg: pairing_messages::JoinConfirmation, _ctx: &mut Self::Context) -> Self::Result {
        /*
        let procedure_request = self.procedure.clone();
        log::info!("Existing procedure {:?}", procedure_request);
        self.procedure_picking_window.on_procedure_unavailable(msg.msg.request_uid);
        self.procedure_picking_window.deactivate();

        let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        self.gui_actor.do_send(ProcedureScreenSetChannel {
            server_idx: msg.server_idx,
            request_uid: msg.msg.request_uid,
            channel: sender,
        });

        let mut work_on_procedure = WorkOnProcedureWindow::create(
            self.gui_actor.clone(),
            self.factories.clone(),
            procedure_request,
            msg.server_idx,
        );
        work_on_procedure.activate();

        receiver.attach(None, move |msg| {
            work_on_procedure.message_received(msg);
            glib::Continue(true)
        });
        */
        Ok(())
    }
}
