use kosem_server::server_config;

fn main() -> Result<(), String> {
    flexi_logger::Logger::with_env_or_str("warn")
        .start().map_err(|e| format!("{}", e))?;

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("KosemServer.toml")).map_err(|e| format!("{}", e))?;

    log::warn!("{:?}", settings.try_into::<server_config::ServerConfig>());

    Ok(())
}
