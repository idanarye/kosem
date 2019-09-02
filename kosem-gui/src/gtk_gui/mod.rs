use actix::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;

use crate::actors::gui::GuiActor;
use crate::internal_messages::gui_control::MessageToGui;

pub fn launch_gtk_app(gui_actor: Addr<GuiActor>, receiver: glib::Receiver<MessageToGui>) {
    let application = gtk::Application::new(Some("kosem.gtk-gui"), Default::default()).unwrap();

    application.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Kosem");

        let button = gtk::Button::new_with_label("TEST");
        let gui_actor = gui_actor.clone();
        button.connect_clicked(move |_| {
            log::warn!("Button clicked");

            gui_actor.do_send(crate::internal_messages::gui_control::TmpButtonClicked);
        });
        window.add(&button);

        window.show_all();
    });

    receiver.attach(None, move |msg| {
        log::warn!("Gui got {:?}", msg);
        glib::Continue(true)
    });


    application.run(&[]);
}
