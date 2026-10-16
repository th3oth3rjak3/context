use std::{fmt::Display, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ContextConfiguration {
    pub name: String,
    pub kind: ContextOutputKind,
    pub output: PathBuf,
    pub include_file_names: bool,
    pub excluded_filetypes: Vec<String>,
    pub excluded_directories: Vec<PathBuf>,
}

impl ContextConfiguration {
    pub fn raw(&self) -> String {
        let kind = self.kind.display();
        let output = self.output.display();
        let include_file_names = self.include_file_names;

        let mut raw_config = String::new();

        let header_row = format!(
            "kind = \"{kind}\"\noutput = \"{output}\"\ninclude_file_names = {include_file_names}"
        );

        let exclusions = "\n\n[excludes]\nextensions = []\ndirs = []";

        raw_config.push_str(&header_row);
        raw_config.push_str(exclusions);

        raw_config
    }
}

impl Default for ContextConfiguration {
    fn default() -> Self {
        Self {
            name: "context".into(),
            kind: Default::default(),
            output: PathBuf::from("."),
            include_file_names: true,
            excluded_filetypes: Vec::new(),
            excluded_directories: Vec::new(),
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
    pub name: Option<String>,
    pub kind: Option<String>,
    pub output: Option<PathBuf>,
    pub include_file_names: Option<bool>,
    pub exclude: Option<ConfigurationTomlExclusion>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConfigurationTomlExclusion {
    pub extensions: Vec<String>,
    pub dirs: Vec<PathBuf>,
}
