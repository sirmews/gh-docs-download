# Architecture

## Overview

The GitHub Documentation Download Tool is designed as a single-binary CLI application that downloads documentation files from specific paths in GitHub repositories using git sparse checkout. The tool targets precise documentation directories specified via GitHub tree URLs.

## Core Components

### 1. CLI Interface (`Args` struct)
- Built with `clap` for robust argument parsing
- Accepts GitHub tree URLs in format: `https://github.com/owner/repo/tree/branch/path`
- Configurable output directory and behavior flags

### 2. GitHub Tree URL Parser
- Parses GitHub tree URLs to extract repository and path information
- Validates URL format and extracts owner, repository name, and target path
- Returns `(RepoSpec, String)` tuple for repository identification and target path

### 3. Git Sparse Checkout Engine
- Uses git clone with `--no-checkout` and `--depth 1` for efficiency
- Configures sparse checkout to target only the specified documentation path
- Leverages git's native sparse checkout feature for minimal data transfer

### 4. File Classification System
- **Extension-based detection**: `.md`, `.rst`, `.pdf`, `.html`, etc.
- **Name-based detection**: `README`, `CHANGELOG`, `LICENSE`, etc.
- Comprehensive pattern matching for documentation files

### 5. Local File Manager
- Copies files from sparse checkout to output directory
- Preserves directory structure in output
- Handles file operations efficiently during temporary directory cleanup

## Design Decisions

### Git-Only Approach
The tool exclusively uses git sparse checkout for several key advantages:

- **No Authentication Required**: Works with public repositories without tokens
- **No Rate Limits**: Avoids GitHub API rate limiting entirely
- **Efficient**: Only downloads the specific path content, not entire repository
- **Native Git Integration**: Leverages git's optimized sparse checkout feature

### Synchronous Architecture
- Uses synchronous operations for simplicity and reliability
- Minimal async overhead since operations are primarily local file I/O
- Clear, predictable execution flow for git commands

### Comprehensive Error Handling
- Uses `thiserror` for semantic error types with specific variants
- Detailed error context for git operations and URL parsing
- User-friendly error messages with actionable information

### Memory Management
- Uses temporary directories for git operations with automatic cleanup
- Efficient directory traversal with `walkdir` during file discovery
- Immediate file copying during sparse checkout to avoid temporary directory bloat

## Data Flow

```
Tree URL Input → URL Parser → Git Sparse Checkout → File Discovery → File Classification → Local Copy → Output
```

1. **URL Processing**: Parse GitHub tree URL to extract repository and target path
2. **Repository Setup**: Create temporary directory and clone repository with no checkout
3. **Sparse Configuration**: Configure git sparse checkout for target documentation path
4. **Selective Checkout**: Checkout only the specified path using git sparse checkout
5. **File Discovery**: Scan checked out directory for documentation files
6. **Classification**: Filter files based on documentation patterns
7. **Local Copy**: Copy classified files to output directory with structure preservation

## Dependencies

### Core Dependencies
- `clap` - CLI argument parsing
- `url` - URL parsing and validation for GitHub tree URLs
- `thiserror` - Semantic error type definitions
- `serde` - Serialization support for types

### Utility Dependencies
- `walkdir` - Filesystem traversal for file discovery
- `tempfile` - Temporary directory management for git operations

### System Dependencies
- `git` - Required system dependency for sparse checkout operations

## Future Architecture Considerations

### Scalability
- Parallel processing for multiple tree URLs
- Progress tracking for large documentation directories
- Batch operations for related documentation paths

### Extensibility
- Plugin system for custom file filters
- Support for additional version control systems (GitLab, Bitbucket tree URLs)
- Configurable output formats (ZIP, TAR, etc.)
- Custom documentation pattern definitions

### Reliability
- Retry mechanisms for git operations with network issues
- Resume capability for interrupted sparse checkouts
- Integrity checking for copied files
- Better error recovery for malformed tree URLs