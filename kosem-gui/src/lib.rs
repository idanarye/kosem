use gtk::prelude::*;
use actix::prelude::*;

pub mod client_config;
// pub mod actors;
mod internal_messages;
// pub mod gtk_gui;

mod client;
mod join_menu;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Asset;

impl Asset {
    // pub fn xml_extractor(filename: &str) -> glade_templating::GladeXmlExtractor {
        // glade_templating::GladeXmlExtractor::new(std::str::from_utf8(&Self::get(filename).unwrap()).unwrap())
    // }

    pub fn css_provider(filename: &str) -> gtk::CssProvider {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(Self::get(filename).unwrap().as_ref()).unwrap();
        css_provider
    }
}

#[derive(woab::Factories)]
pub struct WorkOnProcedureFactories {
}

pub struct FactoriesInner {
    pub join_menu: join_menu::JoinMenuFactories,
    pub work_on_procedure: WorkOnProcedureFactories,
}

pub type Factories = std::rc::Rc<FactoriesInner>;

pub fn start_gtk(settings: client_config::ClientConfig) -> anyhow::Result<()> {
    let factories = Factories::new(FactoriesInner {
        join_menu: join_menu::JoinMenuFactories::read(&*Asset::get("join_menu.glade").unwrap())?,
        work_on_procedure: WorkOnProcedureFactories::read(&*Asset::get("work_on_procedure.glade").unwrap())?,
    });
    gtk::init()?;
    woab::run_actix_inside_gtk_event_loop("kosem.gtk-gui")?;

    let css_provider = Asset::css_provider("default.css");

    factories.join_menu.app_join_menu_window.create(|ctx, widgets| {
        gtk::StyleContext::add_provider_for_screen(
            &widgets.app_join_menu_window.get_screen().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let gui_client = crate::client::GuiClientActor::builder()
            .join_menu(ctx.address())
            .config(settings)
            .build()
            .start();

        join_menu::JoinMenuActor::builder()
            .factories(factories.clone())
            .widgets(widgets)
            .gui_client(gui_client)
            .build()
    })?;

    gtk::main();
    Ok(())
}
