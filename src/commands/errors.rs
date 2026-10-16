use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("{source}")]
    LoadConfiguration {
        #[source]
        source: io::Error,
    },

    #[error("{source}")]
    WriteConfiguration {
        #[source]
        source: io::Error,
    },

    #[error("{source}")]
    GatherFiles {
        #[source]
        source: io::Error,
    },

    #[error("{source}")]
    WriteContext {
        #[source]
        source: io::Error,
    },
}
