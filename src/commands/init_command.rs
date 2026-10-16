use std::path::PathBuf;

use crate::{
    CommandError,
    configuration::{config_writer::ConfigWriter, context_config::ContextConfiguration},
};

pub fn execute(project_directory: PathBuf) -> Result<(), CommandError> {
    let default_config = ContextConfiguration::default();

    ConfigWriter::write_config(default_config).map_err(|e| CommandError::WriteConfiguration {
        path: project_directory,
        source: e,
    })
}
