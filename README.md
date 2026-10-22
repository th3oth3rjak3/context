# Context

**Context** is a command-line tool that gathers text files from a project and combines them into a single, AI-friendly context file.

Use it when you want to give an AI assistant, reviewer, or collaborator a curated snapshot of your codebase without manually copying individual files.

## Features

- Creates a straightforward `context.toml` configuration file
- Recursively gathers files from one or more project directories
- Supports glob patterns for including and excluding files
- Lists selected files before generating output
- Produces Markdown (`.md`) or plain-text (`.txt`) output
- Adds source file names to the output by default
- Skips symlinks to avoid loops and unexpected traversal outside the project tree
- Excludes its own generated context file automatically
- Processes files in a consistent order

## Quick Start

From your project root, create a default configuration:

```sh
ctx init
```

Review the files that will be included:

```sh
ctx list
```

Generate the context file:

```sh
ctx generate
```

By default, Context creates:

```text
context.md
```

If that file already exists, overwrite it explicitly:

```sh
ctx generate --force
```

## Commands

### `ctx init [project-directory]`

Creates a `context.toml` configuration file.

```sh
ctx init
```

To create configuration in another directory:

```sh
ctx init ./my-project
```

To overwrite an existing configuration file:

```sh
ctx init --force
```

### `ctx list`

Shows every file that matches the current configuration without creating an output file.

```sh
ctx list
```

Use this command to verify your configuration before generating and sharing a context file.

### `ctx generate`

Generates the configured context file.

```sh
ctx generate
```

Overwrite an existing generated file:

```sh
ctx generate --force
```

## Configuration

Context reads configuration from `context.toml`.

When you run `ctx list` or `ctx generate`, Context looks for `context.toml` in the current directory. If it does not find one, it continues searching parent directories until it finds a configuration file.

All relative paths in `context.toml` are resolved from the directory containing that file.

## Example Configuration

```toml
# The generated file will be named project-context.md.
name = "project-context"

# Supported values: md, markdown, txt, or text.
kind = "md"

# Directory where the generated context file will be written.
output = "output"

# Add each source file's relative path before its contents.
include_file_names = true

# Directories to scan.
roots = ["src", "tests", "docs"]

[includes]
# Once include patterns are specified, only matching files are included.
patterns = ["**/*.rs", "**/*.toml", "**/*.md"]

[excludes]
# Exclude patterns take priority over include patterns.
patterns = [
  "**/.git",
  "**/target",
  "**/node_modules",
  "**/*.lock",
  "**/*.min.js",
]
```

This configuration produces:

```text
output/project-context.md
```

## Configuration Reference

| Setting               | Required | Description                                                                          |
| --------------------- | -------- | ------------------------------------------------------------------------------------ |
| `name`                | Yes      | Base name of the generated context file.                                             |
| `kind`                | Yes      | Output format: `md` / `markdown` or `txt` / `text`.                                  |
| `output`              | Yes      | Directory where the generated file will be written.                                  |
| `include_file_names`  | No       | Adds each source file's relative path before its contents. Defaults to `true`.       |
| `roots`               | No       | Directories to scan. Defaults to `["."]`.                                            |
| `[includes].patterns` | No       | Glob patterns for files to include. When empty, all non-excluded files are eligible. |
| `[excludes].patterns` | No       | Glob patterns for files and directories to exclude. Excludes always take precedence. |

## Default Configuration

Running `ctx init` creates a configuration equivalent to:

```toml
name = "context"
kind = "md"
output = "."
include_file_names = true

roots = ["."]

[includes]
patterns = []

[excludes]
patterns = [
  "**/.git",
  "**/.gitignore",
  "**/.env",
  "**/*.dll",
  "**/*.exe",
  "**/*.so",
  "**/*.dylib",
  "**/*.o",
  "**/*.obj",
  "**/*.a",
  "**/*.lib",
  "**/bin",
  "**/obj",
  "**/context.toml",
]
```

## Output Formats

### Markdown

Markdown output creates a section for each included file and places its contents in a fenced code block. The file extension is used as the code-block language where possible.

For example, a Rust source file is represented conceptually as:

```text
# src/main.rs

[Markdown code fence with language: rs]
fn main() {
    println!("Hello, world!");
}
[Closing Markdown code fence]
```

Context selects a safe fence length when a source file contains backticks.

### Plain Text

Plain-text output separates files with a readable header:

```text
===== src/main.rs =====
fn main() {
    println!("Hello, world!");
}
```

## Pattern Examples

Include Rust, Markdown, and TOML files:

```toml
[includes]
patterns = [
  "**/*.rs",
  "**/*.md",
  "**/*.toml",
]
```

Exclude common generated directories and sensitive files:

```toml
[excludes]
patterns = [
  "**/.git",
  "**/target",
  "**/node_modules",
  "**/*.log",
  "**/*.secret",
]
```

## Selection Rules

- If `[includes].patterns` is empty, Context considers all files under the configured roots.
- If include patterns are specified, only files matching at least one include pattern are eligible.
- Exclude patterns override include patterns.
- Directories matching an exclude pattern are not scanned.
- Symlinks are skipped.
- The generated context file is excluded automatically.
- Included files must be valid UTF-8 text. Context reports an error if it cannot read a selected file as text.

## Recommended Workflow

1. Create a configuration file:

   ```sh
   ctx init
   ```

2. Update `context.toml` for your project.

3. Check the selected files:

   ```sh
   ctx list
   ```

4. Generate the context file:

   ```sh
   ctx generate
   ```

5. Review the generated output before sharing it with an AI tool or collaborator.

## Tips

- For large repositories, start with narrow roots such as `["src", "tests", "docs"]`.
- Use include patterns to select only the source and documentation files relevant to your task.
- Add project-specific exclusions for credentials, local configuration, logs, generated files, and large fixtures.
- Run `ctx list` after changing patterns to verify exactly what will be included.
- Review generated context before sharing it outside your project. The tool combines source material into one convenient file, so it may include information you would not otherwise share.

## License

This project is released into the public domain under the [Unlicense](UNLICENSE).

You may copy, modify, publish, use, compile, sell, or distribute this software for any purpose, commercial or non-commercial, without restriction and without warranty.
