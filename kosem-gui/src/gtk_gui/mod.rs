use std::sync::Arc;
use std::cell::RefCell;

use actix::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;

use crate::actors::gui::GuiActor;
use crate::internal_messages::gui_control::MessageToGui;

mod hierarchy;
mod glade_templating;

#[derive(rust_embed::RustEmbed)]
#[folder = "kosem-gui/assets"]
struct Asset;

impl Asset {
    pub fn xml_extractor(filename: &str) -> glade_templating::GladeXmlExtractor {
        glade_templating::GladeXmlExtractor::new(std::str::from_utf8(&Self::get(filename).unwrap()).unwrap())
    }

    pub fn css_provider(filename: &str) -> gtk::CssProvider {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(crate::gtk_gui::Asset::get(filename).unwrap().as_ref()).unwrap();
        css_provider
    }
}

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
        let mut hierarchy = hierarchy.borrow_mut();
        if let Some(gtk_gui) = hierarchy.as_mut() {
            gtk_gui.message_received(msg);
        }
        glib::Continue(true)
    });


    application.run(&[]);
    gtk::main();
}
