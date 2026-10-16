use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::configuration::context_config::{ContextConfiguration, ContextOutputKind};

pub struct ContextWriter;

impl ContextWriter {
    pub fn write(
        config: &ContextConfiguration,
        included_files: &[PathBuf],
        force: bool,
    ) -> io::Result<()> {
        let config_root = config.config_path.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Configuration path must have a parent directory.",
            )
        })?;

        let config_root = fs::canonicalize(config_root)?;

        let requested_output_dir = if config.output.is_absolute() {
            config.output.clone()
        } else {
            config_root.join(&config.output)
        };

        if requested_output_dir.exists() && !requested_output_dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!(
                    "Context output path `{}` exists but is not a directory.",
                    requested_output_dir.display()
                ),
            ));
        }

        fs::create_dir_all(&requested_output_dir)?;

        let output_dir = fs::canonicalize(&requested_output_dir)?;
        let output_path = output_dir.join(format!("{}.{}", config.name, config.kind));

        let output_display_path = output_path
            .strip_prefix(&config_root)
            .unwrap_or(&output_path)
            .to_string_lossy()
            .replace('\\', "/");

        if output_path.exists() && !force {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "A context file already exists at `{output_display_path}`.\nUse --force to overwrite it."
                ),
            ));
        }

        let mut context = String::new();

        for file in included_files {
            let file_context =
                Self::render_file(config.kind, config.include_file_names, &config_root, file)?;

            context.push_str(&file_context);
        }

        fs::write(output_path, context)
    }

    fn render_file(
        kind: ContextOutputKind,
        include_file_names: bool,
        config_root: &Path,
        file: &Path,
    ) -> io::Result<String> {
        let contents = fs::read_to_string(file).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("Failed to read source file `{}`: {error}", file.display()),
            )
        })?;

        let relative_path = file.strip_prefix(config_root).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Failed to make source file `{}` relative to configuration root `{}`: {error}",
                    file.display(),
                    config_root.display(),
                ),
            )
        })?;

        let display_path = relative_path.to_string_lossy().replace('\\', "/");

        match kind {
            ContextOutputKind::Markdown => Ok(Self::render_markdown_file(
                &display_path,
                file,
                &contents,
                include_file_names,
            )),

            ContextOutputKind::Text => Ok(Self::render_text_file(
                &display_path,
                &contents,
                include_file_names,
            )),
        }
    }

    fn render_markdown_file(
        display_path: &str,
        file: &Path,
        contents: &str,
        include_file_names: bool,
    ) -> String {
        let language = file
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("");

        let fence = Self::markdown_fence(contents);

        let mut output = String::new();

        if include_file_names {
            output.push_str("# ");
            output.push_str(display_path);
            output.push('\n');
        }

        output.push_str(&fence);
        output.push_str(language);
        output.push('\n');

        output.push_str(contents);

        if !contents.ends_with('\n') {
            output.push('\n');
        }

        output.push_str(&fence);
        output.push_str("\n\n");

        output
    }

    fn render_text_file(display_path: &str, contents: &str, include_file_names: bool) -> String {
        let mut output = String::new();

        if include_file_names {
            output.push_str("===== ");
            output.push_str(display_path);
            output.push_str(" =====\n");
        }

        output.push_str(contents);

        if !contents.ends_with('\n') {
            output.push('\n');
        }

        output.push('\n');

        output
    }

    fn markdown_fence(contents: &str) -> String {
        let mut longest_run = 0;
        let mut current_run = 0;

        for character in contents.chars() {
            if character == '`' {
                current_run += 1;
                longest_run = longest_run.max(current_run);
            } else {
                current_run = 0;
            }
        }

        "`".repeat(longest_run.max(2) + 1)
    }
}
