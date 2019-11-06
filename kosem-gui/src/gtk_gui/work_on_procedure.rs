use std::rc::Rc;
use std::collections::HashMap;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;

use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;
use crate::internal_messages::gui_control::MessageToProcedureScreen;

pub struct WorkOnProcedureWindow {
    #[allow(unused)]
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    #[allow(unused)]
    factories: Rc<GladeFactories>,
    #[allow(unused)]
    request_uid: Uuid,
    phases_list: gtk::ListBox,
    phases: HashMap<Uuid, Phase>,
}

impl WorkOnProcedureWindow {
    pub fn create(
        gui_actor: Addr<GuiActor>,
        factories: Rc<GladeFactories>,
        available_procedure_msg: kosem_webapi::pairing_messages::AvailableProcedure,
    ) -> Self {
        let window_builder = factories.work_on_procedure.window.build();

        window_builder.get_object::<gtk::Label>("title").set_text(&available_procedure_msg.name);

        let phases_list = window_builder.get_object("phases_list");

        let window: gtk::ApplicationWindow = window_builder.get();

        Self {
            gui_actor,
            window,
            factories,
            request_uid: available_procedure_msg.uid,
            phases_list,
            phases: Default::default(),
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn message_received(&mut self, msg: MessageToProcedureScreen) {
        log::info!("Procedure screen got message {:?}", msg);
        match msg {
            MessageToProcedureScreen::PhasePushed(msg) => {
                log::info!("Add phase {}", msg.phase_uid);
                let row_builder = self.factories.work_on_procedure.phase_row.build();
                let parent_uid_label = row_builder.get_object::<gtk::Label>("parent_uid");
                if let Some(uid) = msg.parent_uid {
                    parent_uid_label.set_text(&uid.to_string());
                } else {
                    parent_uid_label.set_text("");
                }
                row_builder.get_object::<gtk::Label>("phase_uid").set_text(&msg.phase_uid.to_string());
                let row = row_builder.get();
                self.phases_list.add(&row);
                let phase = Phase {
                    uid: msg.phase_uid,
                    row,
                };
                self.phases.insert(phase.uid, phase);
            },
        }
    }
}

struct Phase {
    #[allow(unused)]
    uid: Uuid,
    #[allow(unused)]
    row: gtk::ListBoxRow,
}
