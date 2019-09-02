use actix::prelude::*;
use gtk::prelude::*;

use crate::internal_messages::gui_control::*;
use crate::actors::gui::GuiActor;

pub struct GtkGui {
    gui_actor: Addr<GuiActor>,
    application: gtk::Application,
}

impl GtkGui {
    pub fn create_and_activate(gui_actor: Addr<GuiActor>, app: &gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Kosem");
        let button = gtk::Button::new_with_label("TEST");
        {
            let gui_actor = gui_actor.clone();
            button.connect_clicked(move |_| {
                log::warn!("Button clicked");

                gui_actor.do_send(crate::internal_messages::gui_control::TmpButtonClicked);
            });
        }
        window.add(&button);

        window.show_all();

        GtkGui {
            gui_actor,
            application: app.clone(),
        }
    }

    pub fn message_received(&self, msg: MessageToGui) {
        log::warn!("Gui {:?} got {:?}", self.application, msg);
    }
}
