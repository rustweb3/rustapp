use {
    anyhow::{Ok, Result},
    clap::Parser,
    log::{debug, error, info},
    rustapp_template::{app, cli, jobs},
};

#[tokio::main]
async fn main() -> Result<()> {
    app::RunTime::init();
    let mut runtime = app::RunTime::new();
    runtime.do_init(app::InitOptions {
        config_merge_env: true,
        config_merge_cli: true,
    });
    if runtime.cli.name.is_none() && runtime.cli.command.is_none() {
        error!("Please input the command");
        return Err(anyhow::anyhow!("Please input the command"));
    }

    match runtime.cli.command {
        Some(cli::Command::Test { list }) => {
            debug!("this is debug message !");
            info!("list value is :  {}", list);
            jobs::hello_job(&runtime);
        }
        _ => {
            error!("Please input the command");
            cli::Cli::parse_from(&["rustapp", "--help"]);
            return Err(anyhow::anyhow!("Please input the command"));
        }
    }

    Ok(())
}
