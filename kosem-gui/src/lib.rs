use gtk::prelude::*;
use actix::prelude::*;

pub mod client_config;
mod internal_messages;

mod client;
mod join_menu;
mod work_on_procedure;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Asset;

impl Asset {
    pub fn css_provider(filename: &str) -> gtk::CssProvider {
        let css_provider = gtk::CssProvider::new();
        css_provider.load_from_data(Self::get(filename).unwrap().as_ref()).unwrap();
        css_provider
    }
}

pub struct FactoriesInner {
    pub join_menu: join_menu::JoinMenuFactories,
    pub work_on_procedure: work_on_procedure::WorkOnProcedureFactories,
}

pub type Factories = std::rc::Rc<FactoriesInner>;

pub fn start_gtk(settings: client_config::ClientConfig) -> anyhow::Result<()> {
    let factories = Factories::new(FactoriesInner {
        join_menu: join_menu::JoinMenuFactories::read(&*Asset::get("join_menu.glade").unwrap())?,
        work_on_procedure: work_on_procedure::WorkOnProcedureFactories::read(&*Asset::get("work_on_procedure.glade").unwrap())?,
    });
    gtk::init()?;
    woab::run_actix_inside_gtk_event_loop("kosem.gtk-gui")?;

    let css_provider = Asset::css_provider("default.css");

    factories.join_menu.app_join_menu_window.build().actor(|ctx, widgets| {
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
            .factories(factories)
            .widgets(widgets)
            .gui_client(gui_client)
            .build()
    })?;

    gtk::main();
    Ok(())
}
