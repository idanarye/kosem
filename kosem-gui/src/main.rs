use kosem_gui::client_config;

fn main() -> Result<(), String> {
    flexi_logger::Logger::with_env_or_str("warn")
        .start().map_err(|e| format!("{}", e))?;

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("KosemClient.toml")).map_err(|e| format!("{}", e))?;

    let settings = settings.try_into::<client_config::ClientConfig>().map_err(|e| format!("{}", e))?;
    log::warn!("{:?}", settings);

    kosem_gui::actors::launcher::start(settings);

    Ok(())
}
