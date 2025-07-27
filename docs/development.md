# Development Guide

This guide covers development setup, contribution guidelines, and technical details for contributors.

## Development Setup

### Prerequisites
- Rust 1.70+ (2021 edition)
- Git command line tool
- Access to public GitHub repositories (for testing)

### Initial Setup
```bash
# Clone the repository
git clone <repository-url>
cd gh-docs-download

# Build the project
make build

# Run basic test
make test

# Install locally for development
make install
```

### Development Workflow

#### Code Quality Checks
```bash
# Format code
make format

# Check formatting
make check-format

# Run linting
make lint

# Run all checks
make check
```

#### Testing
```bash
# Run unit tests
make test-unit

# Run documentation tests
make test-doc

# Test with real repositories using tree URLs
./target/debug/gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --list-only
./target/debug/gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --list-only
```

## Project Structure

```
gh-docs-download/
├── src/
│   ├── main.rs             # Main application entry point
│   ├── lib.rs              # Library crate root
│   ├── cli.rs              # CLI argument parsing and application runner
│   ├── downloader.rs       # Git sparse checkout implementation
│   ├── types.rs            # Semantic types and domain models
│   └── error.rs            # Error handling types
├── docs/                   # Documentation
├── Cargo.toml              # Project configuration
├── Cargo.lock              # Dependency lock file
├── Makefile                # Build automation
├── README.md               # Project overview
└── .gitignore              # Git ignore rules
```

## Code Architecture

### Main Components

#### 1. CLI Interface (`Args`)
```rust
#[derive(Parser, Debug)]
struct Args {
    repo: String,           // GitHub tree URL
    output: String,         // Output directory
    list_only: bool,        // Preview mode
    recursive: bool,        // Recursive scanning
}
```

#### 2. Git Sparse Checkout Downloader (`GitHubDocsDownloader`)
- Performs efficient git sparse checkout operations
- Manages temporary directories and file operations
- Implements documentation file discovery

#### 3. Semantic Types
```rust
struct RepoSpec {           // Repository specification
    owner: RepoOwner,
    name: RepoName,
}

struct DocumentationFile {  // Documentation file metadata
    name: FileName,
    path: FilePath,
    download_url: DownloadUrl,
    size: FileSizeBytes,
    docs_directory: DocsDirectory,
}
```

## Adding New Features

### 1. Adding New File Types

To add support for new documentation file types:

```rust
fn is_documentation_file(filename: &str) -> bool {
    let doc_extensions = [
        ".md", ".markdown", ".txt", ".rst", 
        ".adoc", ".asciidoc", ".org", ".tex", 
        ".pdf", ".html", ".htm", ".xml",
        ".new_extension",  // Add new extension here
    ];
    // ... rest of function
}
```

### 2. Adding New CLI Options

1. Add to `Args` struct:
```rust
#[derive(Parser, Debug)]
struct Args {
    // ... existing fields
    
    /// New option description
    #[arg(long)]
    new_option: bool,
}
```

2. Use in CLI application:
```rust
impl CliApp {
    async fn run(&self) -> Result<(), GitHubDocsError> {
        if self.args.new_option {
            // Handle new option
        }
        // ... rest of function
    }
}
```

### 3. Adding New Documentation Patterns

To add support for new documentation file patterns, update the file detection logic:

```rust
fn is_documentation_file(filename: &str) -> bool {
    let doc_extensions = [
        ".md", ".markdown", ".txt", ".rst", 
        ".adoc", ".asciidoc", ".org", ".tex", 
        ".pdf", ".html", ".htm", ".xml",
        ".new_extension",  // Add new extension here
    ];
    
    let common_doc_names = [
        "readme", "changelog", "license", "guide",
        "tutorial", "manual",  // Add new patterns here
    ];
    // ... rest of function
}
```

## Testing Guidelines

### Manual Testing

#### Basic Functionality
```bash
# Test tree URL parsing and git operations
./target/debug/gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --list-only
./target/debug/gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/src/doc" --list-only

# Test different branches
./target/debug/gh-docs-download --repo "https://github.com/microsoft/vscode/tree/main/docs" --list-only

# Test actual download
./target/debug/gh-docs-download --repo "https://github.com/TanStack/router/tree/main/docs/router/eslint" --output test-docs
```

#### Edge Cases
```bash
# Invalid tree URL format
./target/debug/gh-docs-download --repo "invalid-url-format" --list-only

# Non-existent repository
./target/debug/gh-docs-download --repo "https://github.com/nonexistent/repo/tree/main/docs" --list-only

# Non-existent branch
./target/debug/gh-docs-download --repo "https://github.com/rust-lang/rust/tree/nonexistent-branch/docs" --list-only

# Non-existent path
./target/debug/gh-docs-download --repo "https://github.com/rust-lang/rust/tree/main/nonexistent-path" --list-only
```

### Unit Tests

The project includes comprehensive unit tests covering core functionality:

#### Existing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_spec_tree_url() {
        let args = Args {
            repo: "https://github.com/rust-lang/rust/tree/main/docs".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };
        let (repo_spec, path) = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "rust-lang");
        assert_eq!(repo_spec.name.as_str(), "rust");
        assert_eq!(path, "docs");
    }

    #[test]
    fn test_parse_repo_spec_invalid_url() {
        let args = Args {
            repo: "invalid-url".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };
        assert!(args.parse_repo_spec().is_err());
    }
}
```

#### Running Tests
```bash
# Run all tests including comprehensive clippy checks
make check

# Run unit tests only
make test-unit

# Run documentation tests
make test-doc
```

## Performance Optimization

### Current Performance Characteristics
1. Efficient git sparse checkout minimizes network transfer
2. Single operation downloads all targeted files
3. Temporary directories are automatically cleaned up

### Potential Improvements

#### 1. Progress Tracking for Large Repositories
```rust
use indicatif::{ProgressBar, ProgressStyle};

impl GitHubDocsDownloader {
    fn download_with_progress(&self, files: &[DocumentationFile]) -> Result<(), GitHubDocsError> {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .progress_chars("#>-"));
        
        // Update progress during file discovery
        for file in files {
            pb.inc(1);
            pb.set_message(format!("Processing {}", file.name.as_str()));
        }
        pb.finish();
        Ok(())
    }
}
```

#### 2. Batch Operations for Multiple URLs
```rust
struct BatchDownloadConfig {
    tree_urls: Vec<String>,
    output_dir: String,
    organize_by_repo: bool,
}

impl GitHubDocsDownloader {
    fn download_batch(&self, config: BatchDownloadConfig) -> Result<(), GitHubDocsError> {
        for url in config.tree_urls {
            // Process each tree URL independently
        }
    }
}
```

#### 3. Caching for Repeated Operations
```rust
use std::collections::HashMap;

struct RepositoryCache {
    file_listings: HashMap<String, Vec<DocumentationFile>>,
    max_age: Duration,
}

impl RepositoryCache {
    fn get_or_fetch(&mut self, tree_url: &str) -> Result<Vec<DocumentationFile>, GitHubDocsError> {
        if let Some(cached) = self.file_listings.get(tree_url) {
            return Ok(cached.clone());
        }
        
        // Fetch and cache
        let files = self.fetch_fresh(tree_url)?;
        self.file_listings.insert(tree_url.to_string(), files.clone());
        Ok(files)
    }
}
```

## Error Handling Best Practices

### Current Error Architecture
The project uses comprehensive, semantic error types with `thiserror`:

```rust
#[derive(Error, Debug)]
pub enum GitHubDocsError {
    #[error("Invalid repository format: Expected GitHub tree URL format like 'https://github.com/owner/repo/tree/branch/path', got: {input}")]
    InvalidRepoFormat { input: String },
    
    #[error("Git operation failed: {command}\nError: {stderr}")]
    GitOperationFailed { command: String, stderr: Cow<'static, str> },
    
    #[error("File system error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Directory traversal error: {0}")]
    WalkDirError(#[from] walkdir::Error),
    
    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),
}
```

### Error Handling Best Practices
- Use semantic error types that provide clear context
- Include the failing input in error messages for debugging
- Chain errors using `#[from]` for automatic conversion
- Provide user-friendly error messages that suggest solutions

## Contributing Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable names
- Add documentation for public functions
- Handle errors appropriately

### Commit Messages
```
feat: add support for new file types
fix: handle git sparse checkout edge cases
docs: update API documentation
refactor: extract file detection logic
test: add unit tests for tree URL parser
```

### Pull Request Process
1. Fork the repository
2. Create feature branch
3. Make changes with tests
4. Run `make check`
5. Submit pull request with description

### Documentation
- Update relevant documentation files
- Add examples for new features
- Update README if needed

## Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug ./target/debug/gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"
```

### Common Debug Scenarios
```bash
# Debug git operations
RUST_LOG=gh_docs_download=debug ./target/debug/gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"

# Debug with verbose git output
GIT_TRACE=1 ./target/debug/gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"

# Debug file operations
RUST_LOG=gh_docs_download::downloader=debug ./target/debug/gh-docs-download --repo "https://github.com/owner/repo/tree/main/docs"
```

## Release Process

### Version Bumping
1. Update version in `Cargo.toml`
2. Update version in documentation
3. Create git tag
4. Build release binary

### Release Build
```bash
make release
strip target/release/gh-docs-download  # Optional: reduce binary size
```

### Distribution
```bash
# Create release archive
tar -czf gh-docs-download-v0.1.0-linux-x86_64.tar.gz -C target/release gh-docs-download

# Create checksums
sha256sum gh-docs-download-v0.1.0-linux-x86_64.tar.gz > checksums.txt
```