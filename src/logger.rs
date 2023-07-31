

//TODO! This succks i dont like it needs to be changed.
pub enum LoggerType {
    Off,
    Error,
    Info,
    Warning,
    Trace,
    Debug,
}

pub fn log(log_type: LoggerType) {
    if std::env::var_os("RUST_LOG").is_none() {
        match log_type {
            LoggerType::Off => std::env::set_var("RUST_LOG", "off"),
            LoggerType::Error => std::env::set_var("RUST_LOG", "error"),
            LoggerType::Info => std::env::set_var("RUST_LOG", "info"),
            LoggerType::Warning => std::env::set_var("RUST_LOG", "warn"),
            LoggerType::Trace => std::env::set_var("RUST_LOG", "trace"),
            LoggerType::Debug => std::env::set_var("RUST_LOG", "debug"),
        }
        env_logger::init();
    }
}
