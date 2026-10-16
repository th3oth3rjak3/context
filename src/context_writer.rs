use std::{io, path::PathBuf};

use crate::configuration::context_config::ContextConfiguration;

pub struct ContextWriter;

impl ContextWriter {
    pub fn write(
        _config: &ContextConfiguration,
        _included_files: &[PathBuf],
        _force: bool,
    ) -> io::Result<()> {
        todo!("Context output generation has not been implemented yet")
    }
}
