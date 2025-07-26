# GitHub Documentation Download Tool

A CLI tool to download documentation files from GitHub repositories. This tool automatically discovers documentation directories (like `docs/`, `documentation/`, etc.) and downloads all documentation-related files while preserving the directory structure.

## Features

- **Automatic Discovery**: Finds all directories containing documentation
- **Smart File Detection**: Identifies documentation files by extension and common naming patterns
- **Flexible Input**: Accepts GitHub URLs or repository slugs (owner/repo)
- **Directory Structure Preservation**: Maintains the original folder structure when downloading
- **List Mode**: Preview files without downloading
- **GitHub API Integration**: Uses GitHub's API for reliable access

## Installation

```bash
# Clone and build
git clone <repository-url>
cd gh-docs-download
cargo build --release

# Or install directly
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Download docs from a repository slug
gh-docs-download --repo rust-lang/rust

# Download docs from a full GitHub URL
gh-docs-download --repo https://github.com/microsoft/vscode

# List files without downloading
gh-docs-download --repo owner/repo --list-only

# Specify output directory
gh-docs-download --repo owner/repo --output ./my-docs
```

### Authentication

For private repositories or to avoid rate limits, provide a GitHub token:

```bash
# Via environment variable
export GITHUB_TOKEN=your_token_here
gh-docs-download --repo private/repo

# Via command line
gh-docs-download --repo private/repo --token your_token_here
```

### Examples

```bash
# Download Rust documentation
gh-docs-download --repo rust-lang/rust --output rust-docs

# Preview what would be downloaded from VS Code
gh-docs-download --repo microsoft/vscode --list-only

# Download from a specific GitHub URL
gh-docs-download --repo https://github.com/tokio-rs/tokio --output tokio-docs
```

## Supported File Types

The tool automatically detects documentation files based on:

### File Extensions
- Markdown: `.md`, `.markdown`
- Text: `.txt`
- reStructuredText: `.rst`
- AsciiDoc: `.adoc`, `.asciidoc`
- Org-mode: `.org`
- LaTeX: `.tex`
- PDF: `.pdf`
- HTML: `.html`, `.htm`
- XML: `.xml`

### Common Documentation Files
- README files
- CHANGELOG, CHANGES, NEWS, HISTORY
- LICENSE, COPYING
- AUTHORS, CONTRIBUTORS
- TODO, INSTALL, INSTALLATION
- USAGE, GUIDE, TUTORIAL
- FAQ, API, REFERENCE, MANUAL

## Command Line Options

```
Options:
  -r, --repo <REPO>        GitHub repository URL or slug (e.g., "owner/repo")
  -o, --output <OUTPUT>    Output directory for downloaded files [default: downloads]
      --list-only          Only list files without downloading
      --recursive <RECURSIVE>  Include subdirectories recursively [default: true]
      --token <TOKEN>      GitHub API token for authenticated requests [env: GITHUB_TOKEN]
  -h, --help              Print help
  -V, --version           Print version
```

## Development

```bash
# Build the project
make build

# Run tests
make test

# Format code
make format

# Run all checks
make check

# Install locally
make install
```

## License

MIT License - see LICENSE file for details.