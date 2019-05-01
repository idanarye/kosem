fn main() -> Result<(), String> {
    flexi_logger::Logger::with_env_or_str("warn")
        .start().map_err(|e| format!("{}", e))?;

    Ok(())
}
