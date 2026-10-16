use std::{fs, io};

use crate::configuration::context_config::{ConfigurationTomlDocument, ContextConfiguration};

pub struct ConfigWriter;

impl ConfigWriter {
    pub fn write_config(config: &ContextConfiguration, force: bool) -> io::Result<()> {
        let output = fs::canonicalize(&config.output)?;

        if !output.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Configuration output must be a directory.",
            ));
        }

        let config_output = output.join("context.toml");

        if !force && config_output.exists() {
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

        fs::write(config_output, toml_content)
    }
}
