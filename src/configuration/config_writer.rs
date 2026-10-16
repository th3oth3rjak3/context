use std::{fs, io, path::Path};

use crate::configuration::context_config::{ConfigurationTomlDocument, ContextConfiguration};

pub struct ConfigWriter;

impl ConfigWriter {
    pub fn write_config(
        config: &ContextConfiguration,
        config_directory: impl AsRef<Path>,
        force: bool,
    ) -> io::Result<()> {
        let requested_directory = config_directory.as_ref();
        let requested_directory_display = display_path(requested_directory);

        let config_directory = fs::canonicalize(requested_directory).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "Cannot access configuration directory `{requested_directory_display}`: {error}"
                ),
            )
        })?;

        if !config_directory.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("Configuration path `{requested_directory_display}` is not a directory."),
            ));
        }

        let config_path = config_directory.join("context.toml");

        if config_path.exists() && !force {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "A `context.toml` file already exists in `{requested_directory_display}`.\nUse --force to overwrite it."
                ),
            ));
        }

        let document = ConfigurationTomlDocument::from(config);

        let toml_content = toml::to_string_pretty(&document).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot serialize default configuration: {error}"),
            )
        })?;

        fs::write(&config_path, toml_content).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot write `context.toml` in `{requested_directory_display}`: {error}"),
            )
        })
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
