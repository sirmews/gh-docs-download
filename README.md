# GitHub Documentation Download Tool

A CLI tool to download documentation files from specific paths in GitHub repositories using tree URLs. This tool uses git sparse checkout to efficiently download only the documentation files you need from a specific directory path.

## Features

- **Direct Path Targeting**: Downloads from specific GitHub tree URLs (e.g., `/tree/main/docs`)
- **Git Sparse Checkout**: Efficient downloads without cloning entire repositories
- **Smart File Detection**: Identifies documentation files by extension and common naming patterns
- **Directory Structure Preservation**: Maintains the original folder structure when downloading
- **List Mode**: Preview files without downloading
- **No Authentication Required**: Works with public repositories without tokens

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
# Download docs from a specific path using GitHub tree URL
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc"

# Download from documentation directory
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs"

# List files without downloading
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --list-only

# Specify output directory
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --output ./my-docs
```

### Advanced Usage

```bash
# Download from nested documentation paths
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint"

# Control recursion
gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs" --recursive false
```

### Examples

```bash
# Download Rust documentation from specific path
gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --output rust-docs

# Preview TanStack Router documentation
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs" --list-only

# Download nested documentation structure
gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/framework/react/guide" --output react-guide-docs
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
  -r, --repo <REPO>           GitHub tree URL (e.g., "https://github.com/owner/repo/tree/branch/path")
  -o, --output <OUTPUT>       Output directory for downloaded files [default: downloads]
      --list-only             Only list files without downloading
      --recursive <RECURSIVE> Include subdirectories recursively [default: true]
  -h, --help                  Print help
  -V, --version               Print version
```

## URL Format

The tool expects GitHub tree URLs in this format:
```
https://github.com/owner/repo/tree/branch/path
```

Examples:
- `https://github.com/rust-lang/rust/tree/main/src/doc`
- `https://github.com/TanStack/router/tree/main/docs`
- `https://github.com/owner/repo/tree/feature-branch/documentation/api`

## Development

```bash
# Build the project
make build

# Run tests
make test-unit
make test-doc

# Format code
make format

# Run all checks
make check

# Install locally
make install
```

## License

MIT License - see LICENSE file for details.