use std::path::PathBuf;

use crate::{CommandError, configuration::config_loader::ConfigLoader};

pub fn execute(project_directory: PathBuf) -> Result<(), CommandError> {
    let config =
        ConfigLoader::load(&project_directory).map_err(|err| CommandError::LoadConfiguration {
            path: project_directory.clone(),
            source: err,
        })?;

    println!("{:?}", config);

    Ok(())
}
