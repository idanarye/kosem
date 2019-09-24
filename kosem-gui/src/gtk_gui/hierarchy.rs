use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;
use crate::gtk_gui::glade_templating::GladeFactory;

pub struct GtkGui {
    #[allow(dead_code)]
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
            MessageToGui::AvailableProcedure(msg) => {
                self.procedure_picking_window.on_procedure_available(msg);
            },
            MessageToGui::UnavailableProcedure(msg) => {
                self.procedure_picking_window.on_procedure_unavailable(msg.msg.uid);
            },
            MessageToGui::JoinConfirmation(msg) => {
                self.procedure_picking_window.on_procedure_unavailable(msg.msg.request_uid);
            },
        }
    }
}

pub struct ProcedurePickingWindow {
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    procedures_list: gtk::ListBox,
    request_row_factory: GladeFactory<gtk::ListBoxRow>,
    procedure_request_rows: HashMap<Uuid, gtk::ListBoxRow>,
}

impl ProcedurePickingWindow {
    fn create(gui_actor: Addr<GuiActor>, _app: &gtk::Application) -> Self {
        let mut xml_extractor = crate::gtk_gui::Asset::xml_extractor("main_menu.glade");
        let request_row_factory = xml_extractor.extract::<gtk::ListBoxRow>("request_row");

        let builder = xml_extractor.build_rest();

        let window: gtk::ApplicationWindow = builder.get_object("procedure_picking_window").unwrap();
        let css_provider = crate::gtk_gui::Asset::css_provider("default.css");
        gtk::StyleContext::add_provider_for_screen(
            &window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let procedures_list: gtk::ListBox = builder.get_object("procedures_list").unwrap();

        Self {
            gui_actor,
            window,
            procedures_list,
            request_row_factory,
            procedure_request_rows: HashMap::new(),
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn on_procedure_available(&mut self, msg: MessageFromServer<pairing_messages::AvailableProcedure>) {
        let gui_actor = self.gui_actor.clone();
        let procedure_uid = msg.msg.uid;
        let row = self.request_row_factory.build()
            .modify_child("request_name", |label: gtk::Label| label.set_text(&msg.msg.name))
            .modify_child("join_request", move |button: gtk::Button| {
                button.connect_clicked(move |_| {
                    log::warn!("User selecting this request");
                    gui_actor.do_send(UserSelectedProcedure {
                        server_idx: msg.server_idx,
                        procedure_uid: msg.msg.uid,
                    });
                });
            })
        .build();
        self.procedures_list.add(&row);
        self.procedure_request_rows.insert(procedure_uid, row);
    }

    pub fn on_procedure_unavailable(&mut self, procedure_uid: Uuid) {
        let row = if let Some(row) = self.procedure_request_rows.remove(&procedure_uid) {
            row
        } else {
            return;
        };
        self.procedures_list.remove(&row);
    }
}
