use {
    crate::config::APP_NAME,
    ansi_term::Colour::{Blue, Green, Purple, Red, Yellow},
    chrono::{FixedOffset, Utc},
    env_logger::Env,
    std::io::Write,
    std::path::PathBuf,
};

pub fn app_dir(sub_path: &str) -> Option<PathBuf> {
    let sub_path = format!(".config/{}{}", APP_NAME, sub_path);
    let user_home = dirs::home_dir();
    if user_home.is_none() {
        None
    } else {
        Some(user_home.unwrap().join(sub_path))
    }
}

pub fn default_app_config_path() -> Option<PathBuf> {
    app_dir("/config/config.toml")
}

pub fn log_init(debug: bool) {
    let default_level = if debug { "debug" } else { "info" };
    let bj_time_zone = FixedOffset::east_opt(8 * 3600).unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or(default_level))
        .format_timestamp_millis()
        .format(move |buf, record: &log::Record| {
            let level = match record.level() {
                log::Level::Error => Red.paint("ERROR"),
                log::Level::Warn => Yellow.paint("WARN"),
                log::Level::Info => Green.paint("INFO"),
                log::Level::Debug => Blue.paint("DEBUG"),
                log::Level::Trace => Purple.paint("TRACE"),
            };

            writeln!(
                buf,
                "{}",
                format!(
                    "[{} {}]  {}",
                    Utc::now().with_timezone(&bj_time_zone).to_rfc3339(),
                    level,
                    record.args()
                )
            )
        })
        .init();
}
