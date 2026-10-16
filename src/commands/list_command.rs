use std::path::{Path, PathBuf};

use crate::{CommandError, configuration::config_loader::ConfigLoader, gatherer::FileGatherer};

pub fn execute() -> Result<(), CommandError> {
    let dir = PathBuf::from(".");

    let config =
        ConfigLoader::load(&dir).map_err(|source| CommandError::LoadConfiguration { source })?;

    let included_files = FileGatherer::gather_included_files(&config)
        .map_err(|source| CommandError::GatherFiles { source })?;

    if included_files.is_empty() {
        println!("No files matched the current configuration.");
        return Ok(());
    }

    let config_dir = config
        .config_path
        .parent()
        .unwrap_or_else(|| Path::new("."));

    println!();
    println!("Included files ({})", included_files.len());
    println!("------------------");

    for file in included_files {
        println!("{}", display_relative_path(config_dir, &file));
    }

    println!();

    Ok(())
}

fn display_relative_path(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}
