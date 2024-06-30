use {
    crate::app::RunTime,
    log::{info, warn},
};

pub fn hello_job(r: &RunTime) {
    info!("config form hello.debug Job : {}", r.config.main.debug);
    warn!("param from cli {}", r.cli.name.as_ref().unwrap());
}
