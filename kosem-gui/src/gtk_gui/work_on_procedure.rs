use std::rc::Rc;

use actix::prelude::*;
use gtk::prelude::*;

use kosem_webapi::Uuid;

use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;

pub struct WorkOnProcedureWindow {
    #[allow(unused)]
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    #[allow(unused)]
    factories: Rc<GladeFactories>,
    #[allow(unused)]
    request_uid: Uuid,
}

impl WorkOnProcedureWindow {
    pub fn create(
        gui_actor: Addr<GuiActor>,
        factories: Rc<GladeFactories>,
        available_procedure_msg: kosem_webapi::pairing_messages::AvailableProcedure,
    ) -> Self {
        let window_builder = factories.work_on_procedure.window.build();

        window_builder.get_object::<gtk::Label>("title").set_text(&available_procedure_msg.name);

        let window: gtk::ApplicationWindow = window_builder.get();

        Self {
            gui_actor,
            window,
            factories,
            request_uid: available_procedure_msg.uid,
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }
}
