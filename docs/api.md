# API Reference

This document provides detailed API and function documentation for the GitHub Documentation Download Tool.

## Core Structures

### `Args`
Command-line arguments structure parsed by `clap`.

```rust
struct Args {
    repo: String,           // GitHub repository URL or slug
    output: String,         // Output directory (default: "downloads")
    list_only: bool,        // Only list files without downloading
    recursive: bool,        // Include subdirectories recursively (default: true)
    token: Option<String>,  // GitHub API token for authentication
    use_git: bool,          // Force use of git clone instead of GitHub API
}
```

**Fields:**
- `repo`: Repository identifier in format "owner/repo" or full GitHub URL
- `output`: Target directory for downloaded documentation files
- `list_only`: When true, only displays found files without downloading
- `recursive`: Controls recursive directory traversal (currently always true)
- `token`: Optional GitHub personal access token for API authentication
- `use_git`: Forces git clone approach instead of GitHub API

### `GitHubFile`
Represents a file or directory from GitHub API response.

```rust
struct GitHubFile {
    name: String,                    // File or directory name
    path: String,                    // Full path from repository root
    download_url: Option<String>,    // Direct download URL (files only)
    file_type: String,              // "file" or "dir"
    size: u64,                      // File size in bytes
}
```

**Usage:** Internal structure for deserializing GitHub API responses.

### `DocumentationFile`
Represents a discovered documentation file ready for download.

```rust
struct DocumentationFile {
    name: String,           // File name
    path: String,           // Relative path from repository root
    download_url: String,   // URL for downloading the file
    size: u64,             // File size in bytes
    docs_directory: String, // Parent documentation directory
}
```

**Usage:** Processed documentation files with all necessary metadata for downloading.

### `GitHubDocsDownloader`
Main service class handling repository operations.

```rust
struct GitHubDocsDownloader {
    client: Client,         // HTTP client for API requests
    owner: String,          // Repository owner
    repo: String,           // Repository name
    token: Option<String>,  // Authentication token
    use_git: bool,         // Access method preference
}
```

## Core Methods

### `GitHubDocsDownloader::new()`
Creates a new downloader instance.

```rust
fn new(
    repo_input: &str, 
    token: Option<String>, 
    use_git: bool
) -> Result<Self, Box<dyn std::error::Error>>
```

**Parameters:**
- `repo_input`: Repository identifier (URL or owner/repo format)
- `token`: Optional GitHub API token
- `use_git`: Whether to force git clone approach

**Returns:** Configured downloader instance or error

**Example:**
```rust
let downloader = GitHubDocsDownloader::new(
    "rust-lang/rust", 
    Some("ghp_token123".to_string()), 
    false
)?;
```

### `parse_repo_input()`
Parses repository input into owner and repository name.

```rust
fn parse_repo_input(input: &str) -> Result<(String, String), Box<dyn std::error::Error>>
```

**Supported Formats:**
- `owner/repo`
- `https://github.com/owner/repo`
- `https://github.com/owner/repo.git`

**Returns:** Tuple of (owner, repository_name)

### `find_docs_directories()`
Discovers documentation directories in the repository.

```rust
async fn find_docs_directories(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>
```

**Behavior:**
- Uses API approach by default (if token available)
- Falls back to git clone if no token or `use_git` is true
- Searches for directories containing "doc" in their name

**Returns:** List of relative paths to documentation directories

### `find_docs_directories_api()`
API-based documentation directory discovery.

```rust
async fn find_docs_directories_api(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>
```

**Features:**
- Recursive traversal using GitHub Contents API
- Rate limit aware (returns 403 errors gracefully)
- Maintains visited set to avoid infinite loops

### `find_docs_directories_git()`
Git-based documentation directory discovery.

```rust
async fn find_docs_directories_git(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>
```

**Features:**
- Performs shallow clone (`--depth 1`)
- Uses local filesystem traversal with `walkdir`
- No API rate limits
- Requires git installation

### `get_documentation_files()`
Retrieves all documentation files from a specific directory.

```rust
async fn get_documentation_files(
    &self,
    docs_dir: &str,
) -> Result<Vec<DocumentationFile>, Box<dyn std::error::Error>>
```

**Parameters:**
- `docs_dir`: Path to documentation directory

**Returns:** List of documentation files with metadata

### `is_documentation_file()`
Determines if a file is considered documentation.

```rust
fn is_documentation_file(filename: &str) -> bool
```

**Detection Criteria:**

**File Extensions:**
- Markdown: `.md`, `.markdown`
- Text: `.txt`
- reStructuredText: `.rst`
- AsciiDoc: `.adoc`, `.asciidoc`
- Org-mode: `.org`
- LaTeX: `.tex`
- PDF: `.pdf`
- HTML: `.html`, `.htm`
- XML: `.xml`

**Common Names:**
- `readme`, `changelog`, `changes`, `news`, `history`
- `license`, `copying`, `authors`, `contributors`
- `todo`, `install`, `installation`, `usage`
- `guide`, `tutorial`, `faq`, `api`, `reference`, `manual`
- `docs`, `documentation`

**Pattern Matching:**
- Exact match: `README`
- With extension: `README.md`
- With underscore: `README_FIRST`
- With hyphen: `README-IMPORTANT`

### `download_file()`
Downloads a single documentation file.

```rust
async fn download_file(
    &self,
    doc_file: &DocumentationFile,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>>
```

**Features:**
- Preserves directory structure in output
- Creates parent directories automatically
- Handles both HTTP downloads (API) and file copying (git)
- Provides download progress feedback

## Access Methods

The tool supports two distinct access methods:

### GitHub API Method
**Advantages:**
- Faster for small repositories
- No local disk usage
- Direct file access

**Limitations:**
- Subject to rate limits (60 requests/hour unauthenticated, 5000/hour authenticated)
- Requires internet connection
- May fail on very large repositories

### Git Clone Method
**Advantages:**
- No API rate limits
- Works with any repository size
- Can work offline after initial clone

**Limitations:**
- Requires git installation
- Uses temporary disk space
- Slower initial setup

## Error Handling

The API uses `Result<T, Box<dyn std::error::Error>>` for comprehensive error handling:

**Common Error Types:**
- Network errors (connection failures, timeouts)
- Authentication errors (invalid tokens, insufficient permissions)
- Repository errors (not found, private without access)
- File system errors (permission denied, disk full)
- Git errors (clone failures, missing git binary)

**Error Recovery:**
- API failures gracefully fall back to git method
- Individual file download failures don't stop the entire process
- Rate limit errors provide helpful guidance

## Usage Examples

### Basic API Usage
```rust
use gh_docs_download::GitHubDocsDownloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = GitHubDocsDownloader::new("rust-lang/rust", None, false)?;
    
    let docs_dirs = downloader.find_docs_directories().await?;
    println!("Found directories: {:?}", docs_dirs);
    
    for dir in docs_dirs {
        let files = downloader.get_documentation_files(&dir).await?;
        for file in files {
            downloader.download_file(&file, "output").await?;
        }
    }
    
    Ok(())
}
```

### With Authentication
```rust
let token = std::env::var("GITHUB_TOKEN").ok();
let downloader = GitHubDocsDownloader::new("private/repo", token, false)?;
```

### Force Git Method
```rust
let downloader = GitHubDocsDownloader::new("large/repo", None, true)?;
```

## Performance Considerations

### Memory Usage
- Files are streamed during download to minimize memory footprint
- Temporary directories are automatically cleaned up
- Directory traversal uses iterators for efficiency

### Network Usage
- API calls are made sequentially to respect rate limits
- File downloads use efficient streaming
- Git clones use shallow depth to minimize bandwidth

### Concurrency
- Currently sequential processing for simplicity
- Future versions may implement parallel downloads
- Async/await ready for concurrent operations

## Future API Extensions

The current API is designed for extensibility:

### Planned Enhancements
- Parallel download support
- Custom file filters
- Progress callbacks
- Resume capability
- Compression options

### Extension Points
- Custom `DocumentationFile` processors
- Pluggable authentication methods
- Configurable file detection rules
- Alternative output formats