use kosem_server::http_server::run_server;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    flexi_logger::Logger::with_env_or_str("warn")
        .start().map_err(|e| format!("{}", e))?;

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("KosemServer.toml")).map_err(|e| format!("{}", e))?;

    let config = settings.try_into().map_err(|e| e.to_string())?;
    // log::warn!("{:?}", settings.try_into::<server_config::ServerConfig>());

    run_server(config).await;

    Ok(())
}
