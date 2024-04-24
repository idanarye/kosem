use actix::prelude::*;
use gtk4::prelude::*;

pub mod client_config;
mod internal_messages;

mod client;
mod join_menu;
mod work_on_procedure;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Asset;

impl Asset {
    pub fn css_provider(filename: &str) -> gtk4::CssProvider {
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data(
            std::str::from_utf8(Self::get(filename).unwrap().data.as_ref()).unwrap(),
        );
        css_provider
    }
}

pub struct FactoriesInner {
    pub join_menu: join_menu::JoinMenuFactories,
    pub work_on_procedure: work_on_procedure::WorkOnProcedureFactories,
}

pub type Factories = std::rc::Rc<FactoriesInner>;

pub fn start_gtk(settings: client_config::ClientConfig) -> woab::Result<()> {
    let factories = Factories::new(FactoriesInner {
        join_menu: join_menu::JoinMenuFactories::read(
            Asset::get("join_menu.ui").unwrap().data.as_ref(),
        )?,
        work_on_procedure: work_on_procedure::WorkOnProcedureFactories::read(
            Asset::get("work_on_procedure.ui").unwrap().data.as_ref(),
        )?,
    });

    woab::main(Default::default(), move |app| {
        let ctx = Context::new();
        let bld = factories
            .join_menu
            .app_join_menu_window
            .instantiate_route_to(ctx.address());
        bld.set_application(app);
        let widgets: crate::join_menu::JoinMenuWidgets = bld.widgets().unwrap();
        gtk4::style_context_add_provider_for_display(
            &WidgetExt::display(&widgets.app_join_menu_window),
            &Asset::css_provider("default.css"),
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let gui_client = crate::client::GuiClientActor::builder()
            .join_menu(ctx.address())
            .config(settings)
            .build()
            .start();

        ctx.run(
            join_menu::JoinMenuActor::builder()
                .factories(factories)
                .widgets(widgets)
                .gui_client(gui_client)
                .build(),
        );
        Ok(())
    })
}
