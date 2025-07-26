# Development Guide

This guide covers development setup, contribution guidelines, and technical details for contributors.

## Development Setup

### Prerequisites
- Rust 1.70+ (2021 edition)
- Git
- GitHub account (for testing)

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
# Build and run with example repository
make test

# Test with different repositories
./target/debug/gh-docs-download --repo rust-lang/rust --list-only
./target/debug/gh-docs-download --repo microsoft/vscode --list-only
```

## Project Structure

```
gh-docs-download/
├── src/
│   └── main.rs              # Main application code
├── docs/                    # Documentation
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
    repo: String,           // Repository identifier
    output: String,         // Output directory
    list_only: bool,        // Preview mode
    recursive: bool,        // Recursive scanning
    token: Option<String>,  // GitHub token
    use_git: bool,          // Force git method
}
```

#### 2. GitHub API Client (`GitHubDocsDownloader`)
- Handles authentication
- Manages API requests
- Implements fallback strategies

#### 3. Data Structures
```rust
struct GitHubFile {         // GitHub API response
    name: String,
    path: String,
    download_url: Option<String>,
    file_type: String,
    size: u64,
}

struct DocumentationFile {  // Internal representation
    name: String,
    path: String,
    download_url: String,
    size: u64,
    docs_directory: String,
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

2. Use in main function:
```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.new_option {
        // Handle new option
    }
    // ... rest of function
}
```

### 3. Adding New Discovery Methods

To add support for new documentation directory patterns:

```rust
async fn find_docs_recursive(&self, path: &str, docs_dirs: &mut Vec<String>, visited: &mut HashSet<String>) {
    // ... existing code
    
    for file in files {
        if file.file_type == "dir" {
            let dir_name = file.name.to_lowercase();
            // Add new patterns here
            if dir_name.contains("doc") || 
               dir_name.contains("manual") ||  // New pattern
               dir_name.contains("guide") {    // New pattern
                docs_dirs.push(file.path.clone());
            }
        }
    }
}
```

## Testing Guidelines

### Manual Testing

#### Basic Functionality
```bash
# Test repository parsing
./target/debug/gh-docs-download --repo rust-lang/rust --list-only
./target/debug/gh-docs-download --repo https://github.com/microsoft/vscode --list-only

# Test authentication
export GITHUB_TOKEN=your_token
./target/debug/gh-docs-download --repo private/repo --list-only

# Test git fallback
./target/debug/gh-docs-download --repo rust-lang/rust --use-git --list-only
```

#### Edge Cases
```bash
# Invalid repository
./target/debug/gh-docs-download --repo invalid/repo --list-only

# Repository with no docs
./target/debug/gh-docs-download --repo user/empty-repo --list-only

# Large repository
./target/debug/gh-docs-download --repo kubernetes/kubernetes --use-git --list-only
```

### Adding Unit Tests

Currently, the project lacks unit tests. Here's how to add them:

1. Create `src/lib.rs`:
```rust
pub mod downloader;
pub mod cli;
pub mod utils;
```

2. Move code to modules and add tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_input() {
        assert_eq!(
            GitHubDocsDownloader::parse_repo_input("owner/repo").unwrap(),
            ("owner".to_string(), "repo".to_string())
        );
    }

    #[test]
    fn test_is_documentation_file() {
        assert!(GitHubDocsDownloader::is_documentation_file("README.md"));
        assert!(GitHubDocsDownloader::is_documentation_file("docs.txt"));
        assert!(!GitHubDocsDownloader::is_documentation_file("main.rs"));
    }
}
```

3. Run tests:
```bash
cargo test
```

## Performance Optimization

### Current Bottlenecks
1. Sequential API requests
2. Sequential file downloads
3. No caching mechanism

### Potential Improvements

#### 1. Parallel Downloads
```rust
use futures::future::join_all;

async fn download_files_parallel(&self, files: &[DocumentationFile], output_dir: &str) {
    let downloads = files.iter().map(|file| {
        self.download_file(file, output_dir)
    });
    
    let results = join_all(downloads).await;
    // Handle results
}
```

#### 2. Connection Pooling
```rust
let client = Client::builder()
    .pool_max_idle_per_host(10)
    .pool_idle_timeout(Duration::from_secs(30))
    .build()?;
```

#### 3. Progress Tracking
```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(total_files as u64);
pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
    .progress_chars("#>-"));
```

## Error Handling Best Practices

### Current Pattern
```rust
fn operation() -> Result<T, Box<dyn std::error::Error>> {
    // Implementation
}
```

### Recommended Improvements
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Repository not found: {repo}")]
    RepoNotFound { repo: String },
    
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
```

## Contributing Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable names
- Add documentation for public functions
- Handle errors appropriately

### Commit Messages
```
feat: add support for new file types
fix: handle rate limiting gracefully
docs: update API documentation
refactor: extract file detection logic
test: add unit tests for parser
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
RUST_LOG=debug ./target/debug/gh-docs-download --repo owner/repo
```

### Common Debug Scenarios
```bash
# Debug API requests
RUST_LOG=reqwest=debug ./target/debug/gh-docs-download --repo owner/repo

# Debug file operations
RUST_LOG=gh_docs_download=debug ./target/debug/gh-docs-download --repo owner/repo
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