use std::{fs, io, path::Path};

use crate::configuration::context_config::{ConfigurationTomlDocument, ContextConfiguration};

pub struct ConfigWriter;

impl ConfigWriter {
    pub fn write_config(
        config: &ContextConfiguration,
        config_directory: impl AsRef<Path>,
        force: bool,
    ) -> io::Result<()> {
        let config_directory = fs::canonicalize(config_directory)?;

        if !config_directory.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Configuration directory must be a directory.",
            ));
        }

        let config_path = config_directory.join("context.toml");

        if !force && config_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "context.toml already exists; use the --force flag to overwrite it.",
            ));
        }

        let document = ConfigurationTomlDocument::from(config);

        let toml_content = toml::to_string_pretty(&document).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize context configuration: {error}"),
            )
        })?;

        fs::write(config_path, toml_content)
    }
}
