use std::collections::HashMap;

use actix::prelude::*;
use gtk4::prelude::*;

use kosem_webapi::pairing_messages;
use kosem_webapi::Uuid;

use crate::internal_messages::gui_control::{
    MessageFromServer, ShowJoinMenu, UserSelectedProcedure,
};

use crate::work_on_procedure::WorkOnProcedureActor;

#[derive(woab::Factories)]
pub struct JoinMenuFactories {
    pub app_join_menu_window: woab::BuilderFactory,
    pub row_request: woab::BuilderFactory,
}

#[derive(typed_builder::TypedBuilder)]
pub struct JoinMenuActor {
    factories: crate::Factories,
    widgets: JoinMenuWidgets,
    gui_client: Addr<crate::client::GuiClientActor>,
    #[builder(default)]
    procedure_requests: HashMap<Uuid, RequestRow>,
}

impl Actor for JoinMenuActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.widgets.app_join_menu_window.set_visible(true);
    }
}

#[derive(woab::WidgetsFromBuilder)]
pub struct JoinMenuWidgets {
    pub app_join_menu_window: gtk4::ApplicationWindow,
    #[allow(dead_code)]
    lst_procedures: gtk4::ListBox,
}

impl actix::Handler<woab::Signal> for JoinMenuActor {
    type Result = woab::SignalResult;

    fn handle(&mut self, msg: woab::Signal, _ctx: &mut Self::Context) -> Self::Result {
        Ok(match msg.name() {
            "close" => {
                self.widgets
                    .app_join_menu_window
                    .application()
                    .unwrap()
                    .quit();
                Some(glib::Propagation::Stop)
            }
            _ => msg.cant_handle()?,
        })
    }
}

impl Handler<ShowJoinMenu> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, _msg: ShowJoinMenu, _ctx: &mut Self::Context) -> Self::Result {
        self.widgets.app_join_menu_window.set_visible(true);
    }
}

impl Handler<MessageFromServer<pairing_messages::AvailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: MessageFromServer<pairing_messages::AvailableProcedure>,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let procedure_uid = msg.msg.uid;
        let new_row_widgets: RequestRowWidgets = self
            .factories
            .join_menu
            .row_request
            .instantiate_route_to((procedure_uid, ctx.address()))
            .widgets()
            .unwrap();
        new_row_widgets.lbl_request_name.set_text(&msg.msg.name);
        self.widgets
            .lst_procedures
            .append(&new_row_widgets.row_request);
        self.procedure_requests.insert(
            procedure_uid,
            RequestRow {
                widgets: new_row_widgets,
                server_idx: msg.server_idx,
                procedure: msg.msg,
            },
        );
    }
}

impl Handler<MessageFromServer<pairing_messages::UnavailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: MessageFromServer<pairing_messages::UnavailableProcedure>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(row) = self.procedure_requests.remove(&msg.msg.uid) {
            self.widgets.lst_procedures.remove(&row.widgets.row_request);
        }
    }
}

impl Handler<MessageFromServer<pairing_messages::JoinConfirmation>> for JoinMenuActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: MessageFromServer<pairing_messages::JoinConfirmation>,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        let row = if let Some(row) = self.procedure_requests.remove(&msg.msg.request_uid) {
            row
        } else {
            return;
        };
        let RequestRow {
            widgets,
            procedure,
            server_idx,
        } = row;
        self.widgets.lst_procedures.remove(&widgets.row_request);
        self.widgets.app_join_menu_window.set_visible(false);

        let work_on_procedure_ctx = Context::new();
        let bld = self
            .factories
            .work_on_procedure
            .app_work_on_procedure_window
            .instantiate_route_to(work_on_procedure_ctx.address());
        bld.set_application(&self.widgets.app_join_menu_window.application().unwrap());
        work_on_procedure_ctx.run(
            WorkOnProcedureActor::builder()
                .factories(self.factories.clone())
                .widgets(bld.widgets().unwrap())
                .join_menu(ctx.address())
                .gui_client(self.gui_client.clone())
                .server_idx(server_idx)
                .procedure(procedure)
                .build(),
        );
    }
}

pub struct RequestRow {
    widgets: RequestRowWidgets,
    server_idx: usize,
    procedure: pairing_messages::AvailableProcedure,
}

#[derive(woab::WidgetsFromBuilder)]
pub struct RequestRowWidgets {
    row_request: gtk4::ListBoxRow,
    lbl_request_name: gtk4::Label,
}

impl actix::Handler<woab::Signal<Uuid>> for JoinMenuActor {
    type Result = woab::SignalResult;

    fn handle(&mut self, msg: woab::Signal<Uuid>, _ctx: &mut Self::Context) -> Self::Result {
        let uuid = msg.tag();
        Ok(match msg.name() {
            "connect_to_procedure" => {
                if let Some(row) = self.procedure_requests.get(uuid) {
                    self.gui_client.do_send(UserSelectedProcedure {
                        server_idx: row.server_idx,
                        procedure_uid: row.procedure.uid,
                    });
                }
                None
            }
            _ => msg.cant_handle()?,
        })
    }
}
