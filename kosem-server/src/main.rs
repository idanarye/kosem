use kosem_server::http_server::run_server;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    flexi_logger::Logger::try_with_env_or_str("warn")
        .map_err(|e| format!("{}", e))?
        .start()
        .map_err(|e| format!("{}", e))?;

    let settings = config::Config::builder()
        .add_source(config::File::with_name("KosemServer.toml"))
        .build()
        .map_err(|e| format!("{}", e))?;

    let config = settings.try_deserialize().map_err(|e| e.to_string())?;

    run_server(config).await;

    Ok(())
}
