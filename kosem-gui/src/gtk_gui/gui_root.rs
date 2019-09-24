use std::rc::Rc;

use actix::prelude::*;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;
use crate::gtk_gui::join_screen::JoinWindow;

pub struct GtkGui {
    #[allow(dead_code)]
    gui_actor: Addr<GuiActor>,
    application: gtk::Application,
    #[allow(dead_code)]
    factories: Rc<GladeFactories>,
    pub procedure_picking_window: JoinWindow,
}

impl GtkGui {
    pub fn create(gui_actor: Addr<GuiActor>, app: &gtk::Application) -> Self {
        let factories = Rc::new(GladeFactories::new());
        GtkGui {
            gui_actor: gui_actor.clone(),
            application: app.clone(),
            factories: factories.clone(),
            procedure_picking_window: JoinWindow::create(gui_actor, factories),
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
