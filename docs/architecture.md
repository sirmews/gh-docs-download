# Architecture

## Overview

The GitHub Documentation Download Tool is designed as a single-binary CLI application that can discover and download documentation files from GitHub repositories using two different approaches:

1. **GitHub API approach** - Uses GitHub's REST API for authenticated access
2. **Git clone approach** - Falls back to cloning the repository locally

## Core Components

### 1. CLI Interface (`Args` struct)
- Built with `clap` for robust argument parsing
- Supports multiple input formats (URLs, owner/repo)
- Configurable output directory and behavior flags

### 2. GitHub Repository Parser
- Handles both full GitHub URLs and owner/repo format
- Extracts repository information for API calls

### 3. Documentation Discovery Engine
- **API Mode**: Recursively traverses repository structure via GitHub API
- **Git Mode**: Uses local filesystem traversal after shallow clone
- Identifies directories containing "doc" in their name

### 4. File Classification System
- **Extension-based detection**: `.md`, `.rst`, `.pdf`, `.html`, etc.
- **Name-based detection**: `README`, `CHANGELOG`, `LICENSE`, etc.
- Comprehensive pattern matching for documentation files

### 5. Download Manager
- **API downloads**: Direct HTTP downloads from GitHub's raw content URLs
- **Git downloads**: Local file copying from cloned repository
- Preserves directory structure in output

## Design Decisions

### Dual Access Strategy
The tool implements both API and git-based access to handle different scenarios:

- **API approach**: Faster, more efficient, but subject to rate limits
- **Git approach**: No rate limits, works offline after clone, but requires git installation

### Async Architecture
- Uses `tokio` for async/await patterns
- Enables concurrent operations (though currently sequential)
- Prepares for future parallel download implementation

### Error Handling Strategy
- Uses `Result<T, Box<dyn std::error::Error>>` for flexible error handling
- Graceful degradation when API access fails
- User-friendly error messages

### Memory Management
- Streams file downloads to avoid loading large files in memory
- Uses temporary directories for git operations with automatic cleanup
- Efficient directory traversal with `walkdir`

## Data Flow

```
User Input → Repository Parser → Documentation Discovery → File Classification → Download Manager → Output
```

1. **Input Processing**: Parse repository identifier and options
2. **Authentication**: Set up GitHub API client with optional token
3. **Discovery**: Find documentation directories using chosen method
4. **Scanning**: Recursively scan directories for documentation files
5. **Classification**: Filter files based on documentation patterns
6. **Download**: Retrieve files while preserving directory structure

## Dependencies

### Core Dependencies
- `clap` - CLI argument parsing
- `reqwest` - HTTP client for GitHub API
- `serde` - JSON serialization/deserialization
- `tokio` - Async runtime

### Utility Dependencies
- `url` - URL parsing and validation
- `walkdir` - Filesystem traversal
- `tempfile` - Temporary directory management
- `regex` - Pattern matching (future use)

## Future Architecture Considerations

### Scalability
- Parallel downloads for improved performance
- Connection pooling for API requests
- Progress tracking for large repositories

### Extensibility
- Plugin system for custom file filters
- Support for additional version control systems
- Configurable output formats (ZIP, TAR, etc.)

### Reliability
- Retry mechanisms with exponential backoff
- Resume capability for interrupted downloads
- Integrity checking for downloaded files