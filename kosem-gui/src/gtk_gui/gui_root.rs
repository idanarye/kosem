use std::rc::Rc;

use actix::prelude::*;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;
use crate::gtk_gui::GladeFactories;
use crate::gtk_gui::join_menu::JoinMenuWindow;
use crate::gtk_gui::work_on_procedure::WorkOnProcedureWindow;

pub struct GtkGui {
    gui_actor: Addr<GuiActor>,
    application: gtk::Application,
    factories: Rc<GladeFactories>,
    pub procedure_picking_window: JoinMenuWindow,
}

impl GtkGui {
    pub fn create(gui_actor: Addr<GuiActor>, app: &gtk::Application) -> Self {
        let factories = Rc::new(GladeFactories::new());
        GtkGui {
            gui_actor: gui_actor.clone(),
            application: app.clone(),
            factories: factories.clone(),
            procedure_picking_window: JoinMenuWindow::create(gui_actor, factories),
        }
    }

    pub fn message_received(&mut self, msg: MessageToLoginScreen) {
        log::warn!("Gui {:?} got {:?}", self.application, msg);
        match msg {
            MessageToLoginScreen::AvailableProcedure(msg) => {
                self.procedure_picking_window.on_procedure_available(msg);
            },
            MessageToLoginScreen::UnavailableProcedure(msg) => {
                self.procedure_picking_window.on_procedure_unavailable(msg.msg.uid);
            },
            MessageToLoginScreen::JoinConfirmation(msg) => {
                log::info!("Got join confirmation {:?}", msg);
                let procedure_request = if let Some(procedure_request) = self.procedure_picking_window.get_procedure_request(msg.msg.request_uid) {
                    procedure_request
                } else {
                    log::error!("Procedure request {} is not in the list", msg.msg.request_uid);
                    return;
                };
                let procedure_request = procedure_request.msg.clone();
                log::info!("Existing procedure {:?}", procedure_request);
                self.procedure_picking_window.on_procedure_unavailable(msg.msg.request_uid);
                self.procedure_picking_window.deactivate();

                let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                self.gui_actor.do_send(ProcedureScreenSetChannel {
                    server_idx: msg.server_idx,
                    channel: sender,
                });

                let mut work_on_procedure = WorkOnProcedureWindow::create(
                    self.gui_actor.clone(),
                    self.factories.clone(),
                    procedure_request,
                    msg.server_idx,
                );
                work_on_procedure.activate();

                receiver.attach(None, move |msg| {
                    work_on_procedure.message_received(msg);
                    glib::Continue(true)
                });
            },
            MessageToLoginScreen::ShowAgain => {
                self.procedure_picking_window.activate();
            }
        }
    }
}
