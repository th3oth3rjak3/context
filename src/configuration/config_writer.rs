use std::{fs, io};

use crate::configuration::context_config::ContextConfiguration;

pub struct ConfigWriter;

impl ConfigWriter {
    pub fn write_config(config: ContextConfiguration, force: bool) -> io::Result<()> {
        let raw_config = config.raw();
        let output = fs::canonicalize(&config.output)?;
        if !output.is_dir() {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Configuration output must be a directory.",
            ));
        }

        let config_output = output.join("context.toml");

        if !force && config_output.is_file() {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "context.toml already exists, use the --force flag to overwrite it",
            ));
        }

        fs::write(&config_output, raw_config)
    }
}
