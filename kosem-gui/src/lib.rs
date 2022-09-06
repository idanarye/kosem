use actix::prelude::*;
use gtk::prelude::*;

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
        css_provider
            .load_from_data(Self::get(filename).unwrap().data.as_ref())
            .unwrap();
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
        join_menu: join_menu::JoinMenuFactories::read(
            Asset::get("join_menu.glade").unwrap().data.as_ref(),
        )?,
        work_on_procedure: work_on_procedure::WorkOnProcedureFactories::read(
            Asset::get("work_on_procedure.glade").unwrap().data.as_ref(),
        )?,
    });
    gtk::init()?;
    woab::run_actix_inside_gtk_event_loop();

    let css_provider = Asset::css_provider("default.css");

    woab::block_on(async move {
        factories
            .join_menu
            .app_join_menu_window
            .instantiate()
            .connect_with(|bld| {
                let widgets: crate::join_menu::JoinMenuWidgets = bld.widgets().unwrap();
                gtk::StyleContext::add_provider_for_screen(
                    &widgets.app_join_menu_window.screen().unwrap(),
                    &css_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );

                join_menu::JoinMenuActor::create(|ctx| {
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
                })
            });
    });

    gtk::main();
    woab::close_actix_runtime()??;
    Ok(())
}
