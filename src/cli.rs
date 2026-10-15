use std::path::PathBuf;

use clap::{Parser, Subcommand};

fn long_about() -> String {
    "Generate AI context from your source code".into()
}

#[derive(Parser)]
#[command(name = "context", version, about = "AI Context Generator", long_about = long_about())]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Initialize a new context configuration.
    Init {
        #[arg(value_name = "project-directory", default_value = ".")]
        project_directory: PathBuf,
    },

    /// Generate the project context file.
    Generate {
        #[arg(value_name = "project-directory", default_value = ".")]
        project_directory: PathBuf,
    },
}
