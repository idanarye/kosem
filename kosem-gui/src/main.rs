use kosem_gui::client_config;

fn main() -> anyhow::Result<()> {
    flexi_logger::Logger::with_env_or_str("warn").start()?;

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("KosemClient.toml"))?;

    let settings = settings.try_into::<client_config::ClientConfig>()?;
    log::warn!("{:?}", settings);

    kosem_gui::start_gtk(settings)?;

    Ok(())
}
