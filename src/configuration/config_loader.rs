use std::{fs, io, path::Path};

use crate::configuration::context_config::{
    ConfigurationTomlDocument, ContextConfiguration, ContextOutputKind,
};

/// Loads and validates context configuration file
pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load(project_path: impl AsRef<Path>) -> io::Result<ContextConfiguration> {
        let full_project_path = std::fs::canonicalize(project_path)?;
        let config_path = full_project_path.join("context.toml");

        if !config_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Could not find context configuration file: '{}'",
                    config_path.display()
                ),
            ));
        }

        let text = fs::read_to_string(&config_path)?;

        let document: ConfigurationTomlDocument = toml::from_str(&text).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse context configuration: {err}"),
            )
        })?;

        let name = match document.name {
            Some(name) => Ok(name),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Configuration must contain a name",
            )),
        }?;

        let kind = match document
            .kind
            .map(|kind| kind.to_ascii_lowercase())
            .as_deref()
        {
            Some("md") | Some("markdown") => Ok(ContextOutputKind::Markdown),
            Some("txt") | Some("text") => Ok(ContextOutputKind::Text),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Output kind must be one of the following: 'md', 'markdown', 'txt', 'text'"
                ),
            )),
        }?;

        let output = match document.output.as_deref() {
            Some(output_dir) => Ok(output_dir.to_path_buf()),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Output directory is required in order to produce your context."),
            )),
        }?;

        let include_file_names = document.include_file_names.unwrap_or(true);

        let exclusions = document.exclude.unwrap_or_default();

        let config = ContextConfiguration {
            name,
            kind,
            output,
            include_file_names,
            excluded_filetypes: exclusions.extensions,
            excluded_directories: exclusions.dirs,
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::{fs, path::PathBuf};
    use tempfile::TempDir;

    fn write_config(directory: &Path, text: &str) {
        fs::write(directory.join("context.toml"), text).unwrap();
    }

    fn load_config(text: &str) -> (TempDir, io::Result<ContextConfiguration>) {
        let dir = tempfile::tempdir().unwrap();
        write_config(dir.path(), text);

        let result = ConfigLoader::load(dir.path());
        (dir, result)
    }

    #[rstest]
    #[case("md", ContextOutputKind::Markdown)]
    #[case("MD", ContextOutputKind::Markdown)]
    #[case("markdown", ContextOutputKind::Markdown)]
    #[case("MARKDOWN", ContextOutputKind::Markdown)]
    #[case("text", ContextOutputKind::Text)]
    #[case("TEXT", ContextOutputKind::Text)]
    #[case("txt", ContextOutputKind::Text)]
    #[case("TXT", ContextOutputKind::Text)]
    fn parses_output_kind_insensitively(#[case] kind: &str, #[case] expected: ContextOutputKind) {
        let config_text = format!(
            r#"
kind = "{kind}"
output = "."
include_file_names = true
"#
        );

        let (_dir, result) = load_config(&config_text);
        let config = result.expect("Config parsing should have succeeded");
        assert_eq!(expected, config.kind);
        assert!(config.include_file_names);
        assert_eq!(PathBuf::from("."), config.output);
    }

    #[test]
    fn includes_file_names_when_omitted_from_toml() {
        let config_text = r#"
kind = "md"
output = "."
"#;
        let (_dir, result) = load_config(config_text);
        let config = result.expect("Config should parse without include_file_names option");
        assert!(config.include_file_names);
    }

    #[test]
    fn rejects_missing_configuration_file() {
        let directory = tempfile::tempdir().unwrap();

        let error = ConfigLoader::load(directory.path()).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn rejects_missing_kind() {
        let config_text = r#"output = ".""#;
        let (_dir, result) = load_config(&config_text);
        let err = result.expect_err("Expected an error for missing kind");
        assert_eq!(io::ErrorKind::InvalidData, err.kind());
    }

    #[test]
    fn rejects_missing_output_directory() {
        let config_text = r#"kind = "md""#;
        let (_dir, result) = load_config(&config_text);
        let err = result.expect_err("Expected an error for missing output");
        assert_eq!(io::ErrorKind::InvalidData, err.kind());
    }
}
