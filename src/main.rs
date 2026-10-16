use crate::{
    cli::{Cli, CliCommand},
    commands::{CommandError, generate_command, init_command},
};
use clap::Parser;

pub mod cli;
pub mod commands;
pub mod configuration;

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
        CliCommand::Generate { project_directory } => generate_command::execute(project_directory),
    }
}
