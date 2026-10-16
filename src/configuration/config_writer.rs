use std::{fs, io};

use crate::configuration::context_config::ContextConfiguration;

pub struct ConfigWriter;

impl ConfigWriter {
    pub fn write_config(config: ContextConfiguration) -> io::Result<()> {
        let raw_config = config.raw();
        let output = fs::canonicalize(&config.output)?;
        if !output.is_dir() {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Configuration output must be a directory.",
            ));
        }

        let config_ouptut = output.join("context.toml");

        fs::write(&config_ouptut, raw_config)
    }
}
