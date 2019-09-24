use std::collections::HashMap;
use std::rc::Rc;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;
use kosem_webapi::pairing_messages;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;

pub struct JoinWindow {
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    procedures_list: gtk::ListBox,
    factories: Rc<GladeFactories>,
    procedure_request_rows: HashMap<Uuid, gtk::ListBoxRow>,
}

impl JoinWindow {
    pub fn create(gui_actor: Addr<GuiActor>, factories: Rc<GladeFactories>) -> Self {
        let window_builder = factories.join_menu.window.build();

        let procedures_list: gtk::ListBox = window_builder.get_object("procedures_list");
        let window: gtk::ApplicationWindow = window_builder.get();

        let css_provider = crate::gtk_gui::Asset::css_provider("default.css");
        gtk::StyleContext::add_provider_for_screen(
            &window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        Self {
            gui_actor,
            window,
            procedures_list,
            factories,
            procedure_request_rows: HashMap::new(),
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn on_procedure_available(&mut self, msg: MessageFromServer<pairing_messages::AvailableProcedure>) {
        let gui_actor = self.gui_actor.clone();
        let procedure_uid = msg.msg.uid;
        let row = self.factories.join_menu.request_row.build()
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
        .get();
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
