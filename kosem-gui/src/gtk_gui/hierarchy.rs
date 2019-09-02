use actix::prelude::*;
use gtk::prelude::*;

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
            gui_actor,
            application: app.clone(),
            procedure_picking_window: ProcedurePickingWindow::create(app),
        }
    }

    pub fn message_received(&self, msg: MessageToGui) {
        log::warn!("Gui {:?} got {:?}", self.application, msg);
        match msg {
            MessageToGui::ProcedureAvailable(msg) => {
                self.procedure_picking_window.on_procedure_available(msg);
            },
        }
    }
}

pub struct ProcedurePickingWindow {
    window: gtk::ApplicationWindow,
    procedures_list: gtk::ListBox,
}

impl ProcedurePickingWindow {
    fn create(app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Kosem");

        let procedures_list = gtk::ListBox::new();
        window.add(&procedures_list);

        Self {window, procedures_list}
    }

    pub fn activate(&self) {
        self.window.show_all();
    }

    pub fn on_procedure_available(&self, msg: ProcedureAvailable) {
        log::warn!("on_procedure_available({:?})", msg);
        let row = gtk::ListBoxRow::new();
        row.add(&gtk::Label::new(Some(&msg.name)));
        self.procedures_list.add(&row);
        self.procedures_list.show_all();
    }
}
