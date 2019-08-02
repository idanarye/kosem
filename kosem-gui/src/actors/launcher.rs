use actix::prelude::*;

use crate::client_config::ClientConfig;

use crate::actors::client::GuiClientActor;
use crate::actors::gui::GuiActor;

pub fn start(config: ClientConfig) {
    let sys = actix::System::new("kosem-gui");

    GuiClientActor::create(|client_ctx| {
        let gui = GuiActor::builder().client(client_ctx.address()).build().start();
        GuiClientActor::builder().gui(gui).config(config).build()
    });

    sys.run().unwrap();
}
