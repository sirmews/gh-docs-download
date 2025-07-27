# API Reference

This document provides detailed API and function documentation for the GitHub Documentation Download Tool.

## Core Structures

### `Args`
Command-line arguments structure parsed by `clap`.

```rust
struct Args {
    repo: String,           // GitHub tree URL (e.g., "https://github.com/owner/repo/tree/branch/path")
    output: String,         // Output directory (default: "downloads")
    list_only: bool,        // Only list files without downloading
    recursive: bool,        // Include subdirectories recursively (default: true)
}
```

**Fields:**
- `repo`: GitHub tree URL in format "https://github.com/owner/repo/tree/branch/path"
- `output`: Target directory for downloaded documentation files
- `list_only`: When true, only displays found files without downloading
- `recursive`: Controls recursive directory traversal within the specified path

### Core Domain Types

#### `RepoSpec`
Repository specification combining owner and name.

```rust
struct RepoSpec {
    owner: RepoOwner,  // Repository owner (user or organization)
    name: RepoName,    // Repository name
}
```

**Methods:**
- `new(owner: RepoOwner, name: RepoName) -> Self`: Create repository specification
- `full_name() -> String`: Get "owner/repo" format string

#### `RepoOwner`
Validated repository owner identifier.

```rust
struct RepoOwner(String);
```

**Methods:**
- `new(owner: impl AsRef<str>) -> Result<Self, RepoOwnerError>`: Create with validation
- `as_str() -> &str`: Get owner name as string reference

#### `RepoName`
Validated repository name identifier.

```rust
struct RepoName(String);
```

**Methods:**
- `new(name: impl AsRef<str>) -> Result<Self, RepoNameError>`: Create with validation
- `as_str() -> &str`: Get repository name as string reference

### File and Path Types

#### `FileName`
File name with semantic meaning.

```rust
struct FileName(String);
```

**Methods:**
- `new(name: impl Into<String>) -> Self`: Create file name
- `as_str() -> &str`: Get name as string reference

#### `FilePath`
File path with semantic meaning and path operations.

```rust
struct FilePath(PathBuf);
```

**Methods:**
- `new(path: impl Into<PathBuf>) -> Self`: Create file path
- `as_path() -> &Path`: Get path reference
- `as_string_lossy() -> Cow<str>`: Get path as string

#### `DownloadUrl`
Validated download URL.

```rust
struct DownloadUrl(Url);
```

**Methods:**
- `new(url: Url) -> Self`: Create from validated URL
- `parse(url: impl AsRef<str>) -> Result<Self, url::ParseError>`: Parse from string
- `as_str() -> &str`: Get URL as string reference

#### `FileSizeBytes`
File size with human-readable formatting.

```rust
struct FileSizeBytes(u64);
```

**Methods:**
- `new(bytes: u64) -> Self`: Create file size
- `bytes() -> u64`: Get size in bytes
- `human_readable() -> String`: Get formatted size (e.g., "1.2 MB")

#### `DocsDirectory`
Documentation directory path.

```rust
struct DocsDirectory(String);
```

**Methods:**
- `new(path: impl Into<String>) -> Self`: Create directory path
- `as_str() -> &str`: Get path as string reference

### Complete File Metadata

#### `DocumentationFile`
Complete metadata for a documentation file.

```rust
struct DocumentationFile {
    name: FileName,                // Name of the documentation file
    path: FilePath,                // Path to the file within the repository
    download_url: DownloadUrl,     // URL for downloading the file content
    size: FileSizeBytes,           // Size of the file in bytes
    docs_directory: DocsDirectory, // Documentation directory this file belongs to
}
```

## Core Configuration

### `DownloadConfig`
Configuration for the documentation downloader.

```rust
struct DownloadConfig {
    output_dir: String,     // Output directory for downloaded files
    list_only: bool,        // Whether to only list files without downloading
    recursive: bool,        // Whether to include subdirectories recursively
    target_path: String,    // Specific path within repository to download
}
```

**Methods:**
- `Default::default()`: Create with default values (output: "downloads", recursive: true, etc.)

## Main Service Classes

### `GitHubDocsDownloader`
Main service class handling git sparse checkout operations.

```rust
struct GitHubDocsDownloader {
    repo: RepoSpec,         // Repository specification
    config: DownloadConfig, // Download configuration
}
```

**Constructor:**
```rust
fn new(repo: RepoSpec, config: DownloadConfig) -> Self
```

**Core Methods:**

#### `find_docs_directories()`
Discovers documentation directories using the target path.

```rust
fn find_docs_directories(&self) -> Result<Vec<DocsDirectory>, GitHubDocsError>
```

**Returns:** List of documentation directories (typically just the target path)

#### `get_all_documentation_files()`
Retrieves all documentation files from specified directories.

```rust
fn get_all_documentation_files(
    &self,
    docs_dirs: &[DocsDirectory],
) -> Result<Vec<DocumentationFile>, GitHubDocsError>
```

**Parameters:**
- `docs_dirs`: Directories to scan for documentation files

**Returns:** List of documentation files with complete metadata

#### `download_files()`
Downloads or lists the provided documentation files.

```rust
fn download_files(&self, files: &[DocumentationFile]) -> Result<(), GitHubDocsError>
```

**Behavior:**
- If `config.list_only` is true, prints file summary without downloading
- Otherwise, files are already downloaded during the git sparse checkout process

### CLI Application

#### `CliApp`
CLI application runner that orchestrates the entire operation.

```rust
struct CliApp {
    args: Args,
}
```

**Constructor:**
```rust
fn new(args: Args) -> Self
```

**Main Method:**
```rust
async fn run(&self) -> Result<(), GitHubDocsError>
```

**Operation Flow:**
1. Validate arguments
2. Parse repository specification and extract path from tree URL
3. Create download configuration
4. Create downloader
5. Discover documentation directories
6. Collect documentation files
7. Download or list files

## URL Parsing

### Tree URL Parser
The `Args::parse_repo_spec()` method handles GitHub tree URL parsing.

```rust
impl Args {
    fn parse_repo_spec(&self) -> Result<(RepoSpec, String), GitHubDocsError>
}
```

**Input Format:** `https://github.com/owner/repo/tree/branch/path`

**Returns:** Tuple of `(RepoSpec, documentation_path)`

**Example:**
```rust
let args = Args {
    repo: "https://github.com/rust-lang/rust/tree/main/src/doc".to_string(),
    // ... other fields
};

let (repo_spec, doc_path) = args.parse_repo_spec()?;
// repo_spec.owner.as_str() == "rust-lang"
// repo_spec.name.as_str() == "rust"
// doc_path == "src/doc"
```

## Error Handling

### `GitHubDocsError`
Comprehensive error type for all operations.

```rust
enum GitHubDocsError {
    InvalidRepoFormat { input: String },
    GitOperationFailed { command: String, stderr: Cow<'static, str> },
    FileError(std::io::Error),
    WalkDirError(walkdir::Error),
    UrlParseError(url::ParseError),
    RepoOwnerValidationError(RepoOwnerError),
    RepoNameValidationError(RepoNameError),
}
```

**Helper Methods:**
- `no_documentation_found(owner: &str, name: &str) -> Self`: Create "no docs found" error
- `git_operation_failed(command: String, stderr: impl Into<Cow<'static, str>>) -> Self`: Create git error

### Validation Errors

#### `RepoOwnerError`
Repository owner validation errors.

```rust
enum RepoOwnerError {
    Empty,
    TooLong { len: usize },
    InvalidCharacters { owner: String },
}
```

#### `RepoNameError`
Repository name validation errors.

```rust
enum RepoNameError {
    Empty,
    TooLong { len: usize },
    InvalidCharacters { name: String },
}
```

## Git Operations

The tool exclusively uses git sparse checkout for efficient downloads:

### Sparse Checkout Process
1. **Clone with no checkout**: `git clone --no-checkout --depth 1 <url>`
2. **Enable sparse checkout**: `git config core.sparseCheckout true`
3. **Set sparse paths**: Write target path to `.git/info/sparse-checkout`
4. **Selective checkout**: `git checkout` (only checks out specified paths)

### File Discovery and Classification

#### `is_documentation_file()`
Determines if a file is considered documentation.

```rust
fn is_documentation_file(filename: &str) -> bool
```

**Detection Criteria:**

**File Extensions:**
- Markdown: `.md`, `.mdx`, `.markdown`
- Text: `.txt`
- reStructuredText: `.rst`
- AsciiDoc: `.adoc`, `.asciidoc`
- Org-mode: `.org`
- LaTeX: `.tex`
- PDF: `.pdf`
- HTML: `.html`, `.htm`
- XML: `.xml`

**Common Documentation Names:**
- `readme`, `changelog`, `changes`, `news`, `history`
- `license`, `copying`, `authors`, `contributors`, `todo`
- `install`, `installation`, `usage`, `guide`, `tutorial`
- `faq`, `api`, `reference`, `manual`, `docs`, `documentation`

**Pattern Matching:**
- Exact match: `README`
- With extension: `README.md`
- With underscore: `README_FIRST`
- With hyphen: `README-IMPORTANT`

## Usage Examples

### Basic Library Usage
```rust
use gh_docs_download::{
    downloader::{DownloadConfig, GitHubDocsDownloader},
    types::{RepoOwner, RepoName, RepoSpec},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse repository specification
    let owner = RepoOwner::new("rust-lang")?;
    let name = RepoName::new("rust")?;
    let repo = RepoSpec::new(owner, name);

    // Configure the downloader
    let config = DownloadConfig {
        output_dir: "docs".to_string(),
        list_only: false,
        recursive: true,
        target_path: "src/doc".to_string(),
    };

    // Create downloader
    let downloader = GitHubDocsDownloader::new(repo, config);

    // Discover documentation directories
    let docs_dirs = downloader.find_docs_directories()?;
    println!("Found {} documentation directories", docs_dirs.len());

    // Get all documentation files
    let files = downloader.get_all_documentation_files(&docs_dirs)?;
    println!("Found {} documentation files", files.len());

    // Download the files
    downloader.download_files(&files)?;
    
    Ok(())
}
```

### CLI Integration
```rust
use gh_docs_download::cli::{Args, CliApp};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let app = CliApp::new(args);
    app.run().await?;
    Ok(())
}
```

## Performance Characteristics

### Memory Usage
- Minimal memory footprint due to streaming operations
- Temporary directories automatically cleaned up
- Efficient file operations during sparse checkout

### Network Usage
- Only downloads the specific documentation path content
- Uses shallow clone (`--depth 1`) to minimize data transfer
- No API rate limits since git operations are used exclusively

### Disk Usage
- Creates temporary directory for git operations
- Files are copied immediately during checkout process
- Temporary directory is cleaned up automatically

## Dependencies

### Core Dependencies
- `clap`: CLI argument parsing
- `url`: URL parsing and validation for GitHub tree URLs
- `thiserror`: Semantic error type definitions
- `serde`: Serialization support for types

### Utility Dependencies
- `walkdir`: Filesystem traversal for file discovery
- `tempfile`: Temporary directory management for git operations

### System Dependencies
- `git`: Required system dependency for sparse checkout operations

## Future API Extensions

The current API is designed for extensibility:

### Planned Enhancements
- Support for multiple tree URLs in a single operation
- Custom file pattern definitions
- Progress tracking for large documentation directories
- Configurable output formats (ZIP, TAR, etc.)

### Extension Points
- Custom `DocumentationFile` processors
- Pluggable file detection rules
- Alternative output destinations (S3, databases, etc.)
- Support for other version control systems with tree-like URLs