use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::configuration::context_config::{
    ConfigurationTomlDocument, ContextConfiguration, ContextOutputKind,
};

/// Loads and validates a context configuration file.
pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load(project_path: impl AsRef<Path>) -> io::Result<ContextConfiguration> {
        let full_project_path = fs::canonicalize(project_path)?;

        let config_path = Self::get_config_path(&full_project_path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not find a context.toml file in the directory provided or any of its ancestors",
            )
        })?;

        let config_path = fs::canonicalize(config_path)?;
        let text = fs::read_to_string(&config_path)?;

        let document: ConfigurationTomlDocument = toml::from_str(&text).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse context configuration: {error}"),
            )
        })?;

        let ConfigurationTomlDocument {
            name,
            kind,
            output,
            include_file_names,
            roots,
            includes,
            excludes,
        } = document;

        let name = name.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Configuration must contain a name",
            )
        })?;

        let kind = match kind.as_deref() {
            Some(kind)
                if kind.eq_ignore_ascii_case("md") || kind.eq_ignore_ascii_case("markdown") =>
            {
                ContextOutputKind::Markdown
            }

            Some(kind) if kind.eq_ignore_ascii_case("txt") || kind.eq_ignore_ascii_case("text") => {
                ContextOutputKind::Text
            }

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Output kind must be one of: 'md', 'markdown', 'txt', or 'text'",
                ));
            }
        };

        let output = output.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Output directory is required in order to produce your context.",
            )
        })?;

        Ok(ContextConfiguration {
            name,
            kind,
            output,
            roots,
            include_file_names: include_file_names.unwrap_or(true),
            include_patterns: includes.patterns,
            exclude_patterns: excludes.patterns,
            config_path,
        })
    }

    fn get_config_path(starting_path: impl AsRef<Path>) -> Option<PathBuf> {
        let mut current_dir = starting_path.as_ref().to_path_buf();

        loop {
            let potential_file = current_dir.join("context.toml");

            if potential_file.is_file() {
                return Some(potential_file);
            }

            current_dir = current_dir.parent()?.to_path_buf();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    fn write_config(directory: &Path, text: &str) {
        fs::write(directory.join("context.toml"), text).unwrap();
    }

    fn load_config(text: &str) -> (TempDir, io::Result<ContextConfiguration>) {
        let directory = tempfile::tempdir().unwrap();

        write_config(directory.path(), text);

        let result = ConfigLoader::load(directory.path());

        (directory, result)
    }

    #[rstest]
    #[case("md", ContextOutputKind::Markdown)]
    #[case("MD", ContextOutputKind::Markdown)]
    #[case("markdown", ContextOutputKind::Markdown)]
    #[case("MARKDOWN", ContextOutputKind::Markdown)]
    #[case("txt", ContextOutputKind::Text)]
    #[case("TXT", ContextOutputKind::Text)]
    #[case("text", ContextOutputKind::Text)]
    #[case("TEXT", ContextOutputKind::Text)]
    fn parses_output_kind_insensitively(#[case] kind: &str, #[case] expected: ContextOutputKind) {
        let config_text = format!(
            r#"
name = "context"
kind = "{kind}"
output = "."
include_file_names = true
"#
        );

        let (_directory, result) = load_config(&config_text);
        let config = result.expect("Configuration should parse successfully");

        assert_eq!(config.kind, expected);
        assert!(config.include_file_names);
        assert_eq!(config.output, PathBuf::from("."));
    }

    #[test]
    fn uses_defaults_for_omitted_optional_values() {
        let config_text = r#"
name = "context"
kind = "md"
output = "."
"#;

        let (_directory, result) = load_config(config_text);
        let config = result.expect("Configuration should parse successfully");

        assert!(config.include_file_names);
        assert_eq!(config.roots, vec![PathBuf::from(".")]);
        assert!(config.include_patterns.is_empty());
        assert!(config.exclude_patterns.is_empty());
    }

    #[test]
    fn loads_roots_include_patterns_and_exclude_patterns() {
        let config_text = r#"
name = "project-context"
kind = "markdown"
output = "output"
include_file_names = false

roots = ["src", "tests"]

[includes]
patterns = ["**/*.rs", "**/*.toml", "**/*.md"]

[excludes]
patterns = ["**/.git", "**/target", "**/node_modules", "**/*.dll"]
"#;

        let (_directory, result) = load_config(config_text);
        let config = result.expect("Configuration should parse successfully");

        assert_eq!(config.name, "project-context");
        assert_eq!(config.kind, ContextOutputKind::Markdown);
        assert_eq!(config.output, PathBuf::from("output"));
        assert!(!config.include_file_names);

        assert_eq!(
            config.roots,
            vec![PathBuf::from("src"), PathBuf::from("tests")]
        );

        assert_eq!(
            config.include_patterns,
            vec!["**/*.rs", "**/*.toml", "**/*.md"]
        );

        assert_eq!(
            config.exclude_patterns,
            vec!["**/.git", "**/target", "**/node_modules", "**/*.dll"]
        );
    }

    #[test]
    fn rejects_missing_configuration_file() {
        let directory = tempfile::tempdir().unwrap();

        let error = ConfigLoader::load(directory.path())
            .expect_err("Expected missing configuration file error");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn rejects_missing_name() {
        let config_text = r#"
kind = "md"
output = "."
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected missing name error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "Configuration must contain a name");
    }

    #[test]
    fn rejects_missing_kind() {
        let config_text = r#"
name = "context"
output = "."
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected missing kind error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn rejects_invalid_kind() {
        let config_text = r#"
name = "context"
kind = "html"
output = "."
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected invalid kind error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            error.to_string(),
            "Output kind must be one of: 'md', 'markdown', 'txt', or 'text'"
        );
    }

    #[test]
    fn rejects_missing_output_directory() {
        let config_text = r#"
name = "context"
kind = "md"
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected missing output error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            error.to_string(),
            "Output directory is required in order to produce your context."
        );
    }

    #[test]
    fn rejects_unknown_configuration_fields() {
        let config_text = r#"
name = "context"
kind = "md"
output = "."
unknown_setting = true
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected unknown configuration field error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn rejects_unknown_include_fields() {
        let config_text = r#"
name = "context"
kind = "md"
output = "."

[includes]
invalid_field = ["**/*.rs"]
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected unknown include field error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn rejects_unknown_exclude_fields() {
        let config_text = r#"
name = "context"
kind = "md"
output = "."

[excludes]
directories = ["target"]
"#;

        let (_directory, result) = load_config(config_text);

        let error = result.expect_err("Expected unknown exclude field error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn finds_configuration_in_an_ancestor_directory() {
        let root = tempfile::tempdir().unwrap();

        write_config(
            root.path(),
            r#"
name = "context"
kind = "md"
output = "."
"#,
        );

        let nested_project_path = root.path().join("src").join("nested");
        fs::create_dir_all(&nested_project_path).unwrap();

        let config = ConfigLoader::load(&nested_project_path)
            .expect("Loader should find context.toml in an ancestor directory");

        assert_eq!(config.name, "context");

        assert_eq!(
            config.config_path,
            fs::canonicalize(root.path().join("context.toml")).unwrap()
        );
    }
}
