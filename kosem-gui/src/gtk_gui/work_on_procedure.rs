use std::rc::Rc;

use actix::prelude::*;
use gtk::prelude::*;

use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;

pub struct WorkOnProcedureWindow {
    gui_actor: Addr<GuiActor>,
    window: gtk::ApplicationWindow,
    factories: Rc<GladeFactories>,
}

impl WorkOnProcedureWindow {
    pub fn create(
        gui_actor: Addr<GuiActor>,
        factories: Rc<GladeFactories>,
        _available_procedure_msg: kosem_webapi::pairing_messages::AvailableProcedure,
    ) -> Self {
        let window_builder = factories.work_on_procedure.window.build();

        let window: gtk::ApplicationWindow = window_builder.get();

        Self {
            gui_actor,
            window,
            factories,
        }
    }

    pub fn activate(&self) {
        self.window.show_all();
    }
}
