use std::sync::Arc;
use std::cell::RefCell;

use actix::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;

use crate::actors::gui::GuiActor;
use crate::internal_messages::gui_control::MessageToLoginScreen;

mod gui_root;
mod glade_templating;
mod glade_factories;
mod join_menu;
mod work_on_procedure;

use glade_factories::GladeFactories;

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

pub fn launch_gtk_app(gui_actor: Addr<GuiActor>, receiver: glib::Receiver<MessageToLoginScreen>) {
    let gui_root = Arc::new(RefCell::new(None));

    let application = gtk::Application::new(Some("kosem.gtk-gui"), Default::default()).unwrap();

    {
        let gui_root = gui_root.clone();
        application.connect_activate(move |app| {
            let gtk_gui = gui_root::GtkGui::create(gui_actor.clone(), app);
            gtk_gui.procedure_picking_window.activate();
            gui_root.replace(Some(gtk_gui));
        });
    }

    receiver.attach(None, move |msg| {
        let mut gui_root = gui_root.borrow_mut();
        if let Some(gtk_gui) = gui_root.as_mut() {
            gtk_gui.message_received(msg);
        }
        glib::Continue(true)
    });


    application.run(&[]);
    gtk::main();
}
