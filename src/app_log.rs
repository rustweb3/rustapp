use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use chrono::{FixedOffset, Utc};
use env_logger::Env;
use std::io::Write;

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
