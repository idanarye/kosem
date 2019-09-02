use std::sync::Arc;
use std::cell::RefCell;

use actix::prelude::*;
use gio::prelude::*;

use crate::actors::gui::GuiActor;
use crate::internal_messages::gui_control::MessageToGui;

mod hierarchy;

pub fn launch_gtk_app(gui_actor: Addr<GuiActor>, receiver: glib::Receiver<MessageToGui>) {
    let hierarchy = Arc::new(RefCell::new(None));

    let application = gtk::Application::new(Some("kosem.gtk-gui"), Default::default()).unwrap();

    {
        let hierarchy = hierarchy.clone();
        application.connect_activate(move |app| {
            let gtk_gui = hierarchy::GtkGui::create(gui_actor.clone(), app);
            gtk_gui.procedure_picking_window.activate();
            hierarchy.replace(Some(gtk_gui));
        });
    }

    receiver.attach(None, move |msg| {
        let hierarchy = hierarchy.borrow();
        if let Some(gtk_gui) = hierarchy.as_ref() {
            gtk_gui.message_received(msg);
        }
        glib::Continue(true)
    });


    application.run(&[]);
}
