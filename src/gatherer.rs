use std::{
    fs, io,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::configuration::context_config::ContextConfiguration;

pub struct FileGatherer;

impl FileGatherer {
    pub fn gather_included_files(config: &ContextConfiguration) -> io::Result<Vec<PathBuf>> {
        let config_dir = config.config_path.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "The loaded configuration does not have a parent directory.",
            )
        })?;

        let config_dir = fs::canonicalize(config_dir).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot access the directory containing `context.toml`: {error}"),
            )
        })?;

        let generated_context_file = Self::generated_context_file(config, &config_dir);

        let include_patterns = Self::compile_patterns("includes", &config.include_patterns)?;
        let exclude_patterns = Self::compile_patterns("excludes", &config.exclude_patterns)?;

        let mut files = Vec::new();

        for configured_root in &config.roots {
            let root_display = display_configured_root(configured_root);

            let requested_root = if configured_root.is_absolute() {
                configured_root.clone()
            } else {
                config_dir.join(configured_root)
            };

            let scan_root = fs::canonicalize(&requested_root).map_err(|error| {
                let message = if error.kind() == io::ErrorKind::NotFound {
                    format!("Configured root `{root_display}` does not exist.")
                } else {
                    format!("Cannot access configured root `{root_display}`: {error}")
                };

                io::Error::new(error.kind(), message)
            })?;

            if !scan_root.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("Configured root `{root_display}` is not a directory."),
                ));
            }

            Self::gather_directory(
                &config_dir,
                &scan_root,
                &generated_context_file,
                &include_patterns,
                &exclude_patterns,
                !config.include_patterns.is_empty(),
                &mut files,
            )?;
        }

        Ok(files)
    }

    fn gather_directory(
        config_root: &Path,
        current_dir: &Path,
        generated_context_file: &Path,
        include_patterns: &GlobSet,
        exclude_patterns: &GlobSet,
        has_include_patterns: bool,
        files: &mut Vec<PathBuf>,
    ) -> io::Result<()> {
        let current_dir_display = display_relative_path(config_root, current_dir);

        let read_dir = fs::read_dir(current_dir).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Cannot read directory `{current_dir_display}`: {error}"),
            )
        })?;

        let mut entries = Vec::new();

        for entry in read_dir {
            let entry = entry.map_err(|error| {
                io::Error::new(
                    error.kind(),
                    format!("Cannot read an entry in `{current_dir_display}`: {error}"),
                )
            })?;

            entries.push(entry);
        }

        entries.sort_by_key(|entry| entry.file_name());

        for entry in entries {
            let path = entry.path();

            if path == generated_context_file {
                continue;
            }

            let path_display = display_relative_path(config_root, &path);

            let file_type = entry.file_type().map_err(|error| {
                io::Error::new(
                    error.kind(),
                    format!("Cannot inspect `{path_display}`: {error}"),
                )
            })?;

            // Do not follow symlinks. This avoids loops and prevents scans
            // from unexpectedly leaving the configured project tree.
            if file_type.is_symlink() {
                continue;
            }

            if file_type.is_dir() {
                if Self::should_skip_directory(exclude_patterns, config_root, &path) {
                    continue;
                }

                Self::gather_directory(
                    config_root,
                    &path,
                    generated_context_file,
                    include_patterns,
                    exclude_patterns,
                    has_include_patterns,
                    files,
                )?;
            } else if file_type.is_file()
                && !Self::should_skip_file(
                    include_patterns,
                    exclude_patterns,
                    has_include_patterns,
                    config_root,
                    &path,
                )
            {
                files.push(path);
            }
        }

        Ok(())
    }

    fn should_skip_directory(
        exclude_patterns: &GlobSet,
        config_root: &Path,
        directory: &Path,
    ) -> bool {
        Self::matches(exclude_patterns, config_root, directory)
    }

    fn should_skip_file(
        include_patterns: &GlobSet,
        exclude_patterns: &GlobSet,
        has_include_patterns: bool,
        config_root: &Path,
        file: &Path,
    ) -> bool {
        if Self::matches(exclude_patterns, config_root, file) {
            return true;
        }

        has_include_patterns && !Self::matches(include_patterns, config_root, file)
    }

    fn matches(patterns: &GlobSet, config_root: &Path, path: &Path) -> bool {
        let relative_path = path.strip_prefix(config_root).unwrap_or(path);

        patterns.is_match(relative_path)
    }

    fn compile_patterns(section: &str, patterns: &[String]) -> io::Result<GlobSet> {
        let mut builder = GlobSetBuilder::new();

        for pattern in patterns {
            let glob = Glob::new(pattern).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid pattern `{pattern}` in [{section}].patterns: {error}"),
                )
            })?;

            builder.add(glob);
        }

        builder.build().map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Cannot compile [{section}].patterns: {error}"),
            )
        })
    }

    fn generated_context_file(config: &ContextConfiguration, config_root: &Path) -> PathBuf {
        let output_dir = if config.output.is_absolute() {
            config.output.clone()
        } else {
            config_root.join(&config.output)
        };

        let output_path = output_dir.join(format!("{}.{}", config.name, config.kind));

        fs::canonicalize(&output_path).unwrap_or(output_path)
    }
}

fn display_configured_root(root: &Path) -> String {
    let display = root.to_string_lossy().replace('\\', "/");

    if display.is_empty() {
        ".".to_owned()
    } else {
        display
    }
}

fn display_relative_path(root: &Path, path: &Path) -> String {
    let relative_path = path.strip_prefix(root).unwrap_or(path);

    let display = relative_path.to_string_lossy().replace('\\', "/");

    if display.is_empty() {
        ".".to_owned()
    } else {
        display
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::context_config::ContextConfiguration;
    use std::{
        fs, io,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    fn config(directory: &TempDir) -> ContextConfiguration {
        let mut config = ContextConfiguration::new();

        config.config_path = fs::canonicalize(directory.path())
            .unwrap()
            .join("context.toml");

        config.roots = vec![PathBuf::from(".")];

        // Tests define exclusion behavior explicitly.
        config.exclude_patterns.clear();

        config
    }

    fn write_file(path: impl AsRef<Path>) {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(path, "test file contents").unwrap();
    }

    fn relative_paths(root: &Path, files: &[PathBuf]) -> Vec<PathBuf> {
        let root = fs::canonicalize(root).unwrap();

        files
            .iter()
            .map(|file| file.strip_prefix(&root).unwrap().to_path_buf())
            .collect()
    }

    #[test]
    fn gathers_files_recursively_in_sorted_order() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("z.txt"));
        write_file(directory.path().join("b.txt"));
        write_file(directory.path().join("a").join("first.rs"));
        write_file(directory.path().join("a").join("second.md"));

        let config = config(&directory);

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![
                PathBuf::from("a").join("first.rs"),
                PathBuf::from("a").join("second.md"),
                PathBuf::from("b.txt"),
                PathBuf::from("z.txt"),
            ]
        );
    }

    #[test]
    fn skips_directories_matching_exclude_patterns() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("src").join("main.rs"));

        write_file(
            directory
                .path()
                .join("src")
                .join("target")
                .join("generated.rs"),
        );

        let mut config = config(&directory);
        config.exclude_patterns = vec!["**/target".into()];

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![PathBuf::from("src").join("main.rs")]
        );
    }

    #[test]
    fn skips_files_matching_exclude_patterns() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("src").join("main.rs"));
        write_file(directory.path().join("src").join("library.dll"));
        write_file(directory.path().join("src").join("notes.txt"));

        let mut config = config(&directory);
        config.exclude_patterns = vec!["**/*.dll".into()];

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![
                PathBuf::from("src").join("main.rs"),
                PathBuf::from("src").join("notes.txt"),
            ]
        );
    }

    #[test]
    fn only_includes_files_matching_include_patterns() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("src").join("lib.rs"));
        write_file(directory.path().join("src").join("readme.md"));
        write_file(directory.path().join("docs").join("guide.md"));

        let mut config = config(&directory);
        config.include_patterns = vec!["**/*.rs".into()];

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![PathBuf::from("src").join("lib.rs")]
        );
    }

    #[test]
    fn exclude_patterns_override_include_patterns() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("src").join("main.rs"));

        write_file(
            directory
                .path()
                .join("src")
                .join("generated")
                .join("client.rs"),
        );

        let mut config = config(&directory);
        config.include_patterns = vec!["**/*.rs".into()];
        config.exclude_patterns = vec!["**/generated".into()];

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![PathBuf::from("src").join("main.rs")]
        );
    }

    #[test]
    fn returns_error_for_invalid_glob_patterns() {
        let directory = TempDir::new().unwrap();

        let mut config = config(&directory);
        config.include_patterns = vec!["[".into()];

        let error = FileGatherer::gather_included_files(&config)
            .expect_err("Invalid glob pattern should return an error");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn returns_error_when_configured_root_does_not_exist() {
        let directory = TempDir::new().unwrap();

        let mut config = config(&directory);
        config.roots = vec![PathBuf::from("does-not-exist")];

        let error = FileGatherer::gather_included_files(&config)
            .expect_err("Missing configured root should return an error");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn returns_error_when_configured_root_is_a_file() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("not-a-directory.txt"));

        let mut config = config(&directory);
        config.roots = vec![PathBuf::from("not-a-directory.txt")];

        let error = FileGatherer::gather_included_files(&config)
            .expect_err("A configured root file should return an error");

        assert_eq!(error.kind(), io::ErrorKind::NotADirectory);
    }

    #[test]
    fn excludes_the_generated_context_file() {
        let directory = TempDir::new().unwrap();

        write_file(directory.path().join("src").join("main.rs"));
        write_file(directory.path().join("context").join("context.md"));

        let mut config = config(&directory);
        config.output = PathBuf::from("context");

        let files = FileGatherer::gather_included_files(&config).unwrap();

        assert_eq!(
            relative_paths(directory.path(), &files),
            vec![PathBuf::from("src").join("main.rs")]
        );
    }
}
