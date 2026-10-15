use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Failed to load configuration for context '{path}'\nReason: {source}")]
    LoadConfiguration {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
