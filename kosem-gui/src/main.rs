fn main() -> anyhow::Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;

    let settings = config::Config::builder()
        .add_source(config::File::with_name("KosemClient.toml"))
        .build()?;

    let config = settings.try_deserialize()?;

    kosem_gui::start_gtk(config)?;

    Ok(())
}
