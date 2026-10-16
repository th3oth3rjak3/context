use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ContextConfiguration {
    pub name: String,
    pub kind: ContextOutputKind,
    pub output: PathBuf,
    pub roots: Vec<PathBuf>,
    pub include_file_names: bool,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub config_path: PathBuf,
}

impl Default for ContextConfiguration {
    fn default() -> Self {
        Self {
            name: "context".into(),
            kind: ContextOutputKind::Markdown,
            output: PathBuf::from("."),
            roots: vec![PathBuf::from(".")],
            include_file_names: true,
            include_patterns: Vec::new(),
            exclude_patterns: vec![
                "**/.git".into(),
                "**/.gitignore".into(),
                "**/.env".into(),
                "**/*.dll".into(),
                "**/*.exe".into(),
                "**/*.so".into(),
                "**/*.dylib".into(),
                "**/*.o".into(),
                "**/*.obj".into(),
                "**/*.a".into(),
                "**/*.lib".into(),
                "**/context.toml".into(),
            ],
            config_path: PathBuf::from(".").join("context.toml"),
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

impl From<&ContextConfiguration> for ConfigurationTomlDocument {
    fn from(config: &ContextConfiguration) -> Self {
        Self {
            name: Some(config.name.clone()),
            kind: Some(config.kind.to_string()),
            output: Some(config.output.clone()),
            include_file_names: Some(config.include_file_names),

            roots: config.roots.clone(),

            includes: ConfigurationTomlPatterns {
                patterns: config.include_patterns.clone(),
            },

            excludes: ConfigurationTomlPatterns {
                patterns: config.exclude_patterns.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationTomlDocument {
    pub name: Option<String>,
    pub kind: Option<String>,
    pub output: Option<PathBuf>,
    pub include_file_names: Option<bool>,

    #[serde(default = "default_roots")]
    pub roots: Vec<PathBuf>,

    #[serde(default)]
    pub includes: ConfigurationTomlPatterns,

    #[serde(default)]
    pub excludes: ConfigurationTomlPatterns,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationTomlPatterns {
    #[serde(default)]
    pub patterns: Vec<String>,
}

fn default_roots() -> Vec<PathBuf> {
    vec![PathBuf::from(".")]
}
