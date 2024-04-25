use kosem_gui::client_config;
use kosem_gui::start_gui;
use kosem_server::http_server::run_server;
use kosem_server::server_config;

fn main() -> anyhow::Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;

    woab::main(Default::default(), |app| {
        actix::spawn(run_server(server_config::ServerConfig {
            server: server_config::ServerSection {
                name: "Local Kosem Server".to_owned(),
                port: 8206,
            },
        }));
        start_gui(
            app,
            client_config::ClientConfig {
                display_name: "Local Kosem".to_owned(),
                servers: [client_config::ServerConfig {
                    name: "Local Kosem Server".to_owned(),
                    url: "127.0.0.1".to_owned(),
                    port: 8206,
                }]
                .into(),
            },
        )
    })?;

    Ok(())
}
