use std::{fmt::Display, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ContextConfiguration {
    pub kind: ContextOutputKind,
    pub output: PathBuf,
    pub include_file_names: bool,
}

impl ContextConfiguration {
    pub fn raw(&self) -> String {
        let kind = self.kind.display();
        let output = self.output.display();
        let include_file_names = self.include_file_names;

        format!(
            "kind = \"{kind}\"\noutput = \"{output}\"\ninclude_file_names = {include_file_names}"
        )
    }
}

impl Default for ContextConfiguration {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            output: PathBuf::from("."),
            include_file_names: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextOutputKind {
    Text,
    #[default]
    Markdown,
}

impl ContextOutputKind {
    pub fn display(&self) -> String {
        format!("{self}")
    }
}

impl Display for ContextOutputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Markdown => f.write_str("md"),
            Self::Text => f.write_str("txt"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigurationTomlDocument {
    pub kind: Option<String>,
    pub output: Option<PathBuf>,
    pub include_file_names: Option<bool>,
}
