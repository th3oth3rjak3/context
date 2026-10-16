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
        /// The directory where the config file should be placed.
        #[arg(value_name = "project-directory", default_value = ".")]
        project_directory: PathBuf,

        /// Overwrite an existing config file if one already exists.
        #[arg(long, short)]
        force: bool,
    },

    /// List the files that would be included in the output context.
    List,

    /// Generate the project context file.
    Generate {
        /// Overwrite the generated context file if one already exists.
        #[arg(long, short)]
        force: bool,
    },
}
