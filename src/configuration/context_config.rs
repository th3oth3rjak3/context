use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ContextConfiguration {
    pub kind: ContextOutputKind,
    pub output: PathBuf,
    pub include_file_names: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextOutputKind {
    Text,
    Markdown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigurationTomlDocument {
    pub kind: Option<String>,
    pub output: Option<PathBuf>,
    pub include_file_names: Option<bool>,
}
