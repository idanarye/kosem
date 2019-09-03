use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;

use kosem_webapi::Uuid;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;

pub struct GtkGui {
    gui_actor: Addr<GuiActor>,
    application: gtk::Application,
    pub procedure_picking_window: ProcedurePickingWindow,
}

impl GtkGui {
    pub fn create(gui_actor: Addr<GuiActor>, app: &gtk::Application) -> Self {
        GtkGui {
            gui_actor: gui_actor.clone(),
            application: app.clone(),
            procedure_picking_window: ProcedurePickingWindow::create(gui_actor, app),
        }
    }

    pub fn message_received(&mut self, msg: MessageToGui) {
        log::warn!("Gui {:?} got {:?}", self.application, msg);
        match msg {
            MessageToGui::ProcedureAvailable(msg) => {
                self.procedure_picking_window.on_procedure_available(msg);
            },
            MessageToGui::ProcedureUnavailable(msg) => {
                self.procedure_picking_window.on_procedure_unavailable(msg.procedure_uid);
            },
        }
    }
}

pub struct ProcedurePickingWindow {
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    procedures_list: gtk::ListBox,
    procedure_request_rows: HashMap<Uuid, gtk::ListBoxRow>,
}

impl ProcedurePickingWindow {
    fn create(gui_actor: Addr<GuiActor>, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Kosem");

        let procedures_list = gtk::ListBox::new();
        window.add(&procedures_list);

        Self {
            gui_actor,
            window,
            procedures_list,
            procedure_request_rows: HashMap::new(),
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn on_procedure_available(&mut self, msg: ProcedureAvailable) {
        let row = gtk::ListBoxRow::new();
        row.add(&gtk::Label::new(Some(&msg.name)));
        self.procedures_list.add(&row);
        self.procedures_list.show_all();
        self.procedure_request_rows.insert(msg.procedure_uid, row.clone());
        let gui_actor = self.gui_actor.clone();
        row.connect_activate(move |row| {
            log::warn!("User selecting this row");
            gui_actor.do_send(UserSelectedProcedure {
                server_idx: msg.server_idx,
                procedure_uid: msg.procedure_uid,
            });
        });
    }

    pub fn on_procedure_unavailable(&mut self, procedure_uid: Uuid) {
        let row = if let Some(row) = self.procedure_request_rows.remove(&procedure_uid) {
            row
        } else {
            return;
        };
        self.procedures_list.remove(&row);
        self.procedures_list.show_all();
    }
}
