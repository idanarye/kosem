use crate::client_config::ClientConfig;

pub fn start(config: ClientConfig) {
    let sys = actix::System::new("kosem-gui");

    crate::actors::client::start_client_actor(config);

    sys.run().unwrap();
}
