use std::path::PathBuf;

use crate::{
    CommandError, configuration::config_loader::ConfigLoader, context_writer::ContextWriter,
    gatherer::FileGatherer,
};

pub fn execute(force: bool) -> Result<(), CommandError> {
    let dir = PathBuf::from(".");

    let config =
        ConfigLoader::load(&dir).map_err(|source| CommandError::LoadConfiguration { source })?;

    let included_files = FileGatherer::gather_included_files(&config)
        .map_err(|source| CommandError::GatherFiles { source })?;

    if included_files.is_empty() {
        println!("No files matched the current configuration.");
        return Ok(());
    }

    ContextWriter::write(&config, &included_files, force)
        .map_err(|source| CommandError::WriteContext { source })?;

    Ok(())
}
