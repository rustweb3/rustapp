use {
    crate::cli,
    ansi_term::Colour::{Blue, Green, Purple, Red, Yellow},
    chrono::{FixedOffset, Utc},
    env_logger::Env,
    log::info,
    std::io::Write,
};

pub struct StartOptions {
    pub log_init: bool,
    pub merge_env: bool,
}

fn log_init(debug: bool) {
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

pub fn start(mycli: &mut cli::Cli, config: &mut crate::config::MyConfig, options: &StartOptions) {
    if mycli.debug {
        config.main.debug = true;
    }
    if options.log_init {
        log_init(config.main.debug);
    }
    if options.merge_env {
        config.merge_env();
    }
    info!("rust app started ....");
}
