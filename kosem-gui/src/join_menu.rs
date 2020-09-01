use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;

use crate::internal_messages::gui_control::{
    MessageFromServer,
    UserSelectedProcedure,
    ShowJoinMenu,
};

use crate::work_on_procedure::WorkOnProcedureActor;

#[derive(woab::Factories)]
pub struct JoinMenuFactories {
    pub app_join_menu_window: woab::Factory<JoinMenuActor, JoinMenuWidgets, JoinMenuSignal>,
    pub row_request: woab::Factory<(), RequestRowWidgets, RequestRowSignal>,
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
    WindowDestroyed,
}

impl StreamHandler<JoinMenuSignal> for JoinMenuActor {
    fn handle(&mut self, signal: JoinMenuSignal, _ctx: &mut Self::Context) {
        match signal {
            JoinMenuSignal::WindowDestroyed => {
                gtk::main_quit();
            }
        }
    }
}

impl Handler<ShowJoinMenu> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, _msg: ShowJoinMenu, _ctx: &mut Self::Context) -> Self::Result {
        self.widgets.app_join_menu_window.show_all();
    }
}

impl Handler<MessageFromServer<pairing_messages::AvailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::AvailableProcedure>, ctx: &mut Self::Context) -> Self::Result {
        let procedure_uid = msg.msg.uid;
        let new_row_widgets = self.factories.join_menu.row_request.build()
            .connect_tagged_builder_signals(ctx, procedure_uid)
            .widgets().unwrap();
        new_row_widgets.lbl_request_name.set_text(&msg.msg.name);
        self.widgets.lst_procedures.add(&new_row_widgets.row_request);
        self.procedure_requests.insert(procedure_uid, RequestRow {
            widgets: new_row_widgets,
            server_idx: msg.server_idx,
            procedure: msg.msg,
        });
    }
}

impl Handler<MessageFromServer<pairing_messages::UnavailableProcedure>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::UnavailableProcedure>, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(row) = self.procedure_requests.remove(&msg.msg.uid) {
            self.widgets.lst_procedures.remove(&row.widgets.row_request);
        }
    }
}

impl Handler<MessageFromServer<pairing_messages::JoinConfirmation>> for JoinMenuActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer<pairing_messages::JoinConfirmation>, ctx: &mut Self::Context) -> Self::Result {
        let row = if let Some(row) = self.procedure_requests.remove(&msg.msg.request_uid) {
            row
        } else {
            return;
        };
        let RequestRow { widgets, procedure, server_idx } = row;
        self.widgets.lst_procedures.remove(&widgets.row_request);
        self.widgets.app_join_menu_window.hide();

        let _addr = self.factories.work_on_procedure.app_work_on_procedure_window.build().actor(|_, widgets| {
            WorkOnProcedureActor::builder()
                .factories(self.factories.clone())
                .widgets(widgets)
                .join_menu(ctx.address())
                .gui_client(self.gui_client.clone())
                .server_idx(server_idx)
                .procedure(procedure)
                .build()
        }).unwrap();
    }
}

pub struct RequestRow {
    widgets: RequestRowWidgets,
    server_idx: usize,
    procedure: pairing_messages::AvailableProcedure,
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

impl StreamHandler<(Uuid, RequestRowSignal)> for JoinMenuActor {
    fn handle(&mut self, (uuid, signal): (Uuid, RequestRowSignal), _ctx: &mut Self::Context) {
        match signal {
            RequestRowSignal::ConnectToProcedure => {
                if let Some(row) = self.procedure_requests.get(&uuid) {
                    self.gui_client.do_send(UserSelectedProcedure {
                        server_idx: row.server_idx,
                        procedure_uid: row.procedure.uid,
                    });
                }
            }
        }
    }

    fn finished(&mut self, _: &mut Self::Context) {}
}
