use crate::cli::Cli;
use clap::Parser;

pub mod cli;

fn main() {
    let _cli = Cli::parse();
}
