use actix::prelude::*;

use crate::client_config::ClientConfig;

use crate::gtk_gui::launch_gtk_app;

use crate::actors::client::GuiClientActor;
use crate::actors::gui::GuiActor;

pub fn start(config: ClientConfig) {
    let sys = actix::System::new("kosem-gui");

    GuiClientActor::create(|client_ctx| {

        let gui = GuiActor::builder().client(client_ctx.address()).build().start();

        let gui_actor_address = gui.clone();
        std::thread::spawn(move || {
            launch_gtk_app(gui_actor_address);
        });

        GuiClientActor::builder().gui(gui).config(config).build()
    });

    sys.run().unwrap();
}
