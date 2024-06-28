use {
    clap::Parser,
    log::{error, info},
    rustapp_template::{app, cli, config},
};

fn main() {
    let mut mycli: cli::Cli = cli::Cli::parse();
    if mycli.name.is_none() && mycli.command.is_none() {
        error!("Please input the command");
        cli::Cli::parse_from(&["rustapp", "--help"]);
        return;
    }
    let mut config = config::MyConfig::load(&mycli.config, true);
    app::start(
        &mut mycli,
        &mut config,
        &app::StartOptions {
            log_init: true,
            merge_env: true,
        },
    );

    match mycli.command {
        Some(cli::Command::Test { list }) => {
            info!("list value is :  {}", list);
        }
        _ => {
            error!("Invalid command");
            cli::Cli::parse_from(&["rustapp", "--help"]);
        }
    }
}
