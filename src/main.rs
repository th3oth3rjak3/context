use crate::{
    cli::{Cli, CliCommand},
    commands::{CommandError, generate_command, init_command, list_command},
};
use clap::Parser;

pub mod cli;
pub mod commands;
pub mod configuration;
pub mod context_writer;
pub mod gatherer;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.command) {
        eprintln!("Unexpected Error: {e}");
        std::process::exit(1);
    }
}

fn run(command: CliCommand) -> Result<(), CommandError> {
    match command {
        CliCommand::Init {
            project_directory,
            force,
        } => init_command::execute(project_directory, force),
        CliCommand::Generate { force } => generate_command::execute(force),
        CliCommand::List => list_command::execute(),
    }
}
