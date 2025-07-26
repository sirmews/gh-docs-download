//! # GitHub Documentation Downloader
//!
//! A comprehensive library for discovering and downloading documentation files
//! from GitHub repositories. This crate provides both API-based and git-based
//! approaches to access repository contents, with automatic discovery of
//! documentation directories and intelligent file filtering.
//!
//! ## Features
//!
//! - **Multiple Access Methods**: Support for both GitHub API and git clone approaches
//! - **Automatic Discovery**: Intelligent detection of documentation directories
//! - **File Filtering**: Smart identification of documentation files by extension and name
//! - **Type Safety**: Comprehensive use of semantic types to prevent category errors
//! - **Error Handling**: Detailed error types with actionable information
//! - **Authentication**: Optional GitHub API token support for private repositories
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gh_docs_download::{
//!     cli::{Args, CliApp},
//!     downloader::{DownloadConfig, GitHubDocsDownloader},
//!     types::{RepoOwner, RepoName, RepoSpec, GitHubToken},
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a repository specification
//! let owner = RepoOwner::new("rust-lang")?;
//! let name = RepoName::new("rust")?;
//! let repo = RepoSpec::new(owner, name);
//!
//! // Configure the downloader
//! let config = DownloadConfig {
//!     output_dir: "docs".to_string(),
//!     list_only: false,
//!     recursive: true,
//!     use_git: false,
//! };
//!
//! // Create downloader (no authentication)
//! let downloader = GitHubDocsDownloader::new(repo, None, config)?;
//!
//! // Discover documentation directories
//! let docs_dirs = downloader.find_docs_directories().await?;
//! println!("Found {} documentation directories", docs_dirs.len());
//!
//! // Get all documentation files
//! let files = downloader.get_all_documentation_files(&docs_dirs).await?;
//! println!("Found {} documentation files", files.len());
//!
//! // Download the files
//! downloader.download_files(&files).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several focused modules:
//!
//! - [`error`] - Comprehensive error types with semantic meaning
//! - [`types`] - Domain types and newtypes for type safety
//! - [`github`] - GitHub API client for repository operations
//! - [`downloader`] - Core downloading logic and configuration
//! - [`cli`] - Command-line interface and argument parsing
//!
//! ## Error Handling
//!
//! All operations return a [`Result`](error::Result) type with detailed
//! [`GitHubDocsError`](error::GitHubDocsError) variants that provide
//! actionable information about what went wrong.
//!
//! ```rust,no_run
//! use gh_docs_download::error::GitHubDocsError;
//!
//! # async fn example() -> Result<(), GitHubDocsError> {
//! match some_operation().await {
//!     Err(GitHubDocsError::RateLimitExceeded) => {
//!         println!("Rate limited! Consider using a GitHub token.");
//!     }
//!     Err(GitHubDocsError::RepositoryNotFound { owner, repo }) => {
//!         println!("Repository {}/{} not found or private", owner, repo);
//!     }
//!     Err(e) => {
//!         println!("Operation failed: {}", e);
//!     }
//!     Ok(result) => {
//!         // Handle success
//!     }
//! }
//! # Ok(())
//! # }
//! # async fn some_operation() -> Result<(), GitHubDocsError> { Ok(()) }
//! ```

#![warn(missing_docs)]

pub mod cli;
pub mod downloader;
pub mod error;
pub mod github;
pub mod types;

// Re-export the most commonly used types for convenience
pub use error::{GitHubDocsError, Result};
pub use types::{
    DocumentationFile, DocsDirectory, DownloadUrl, FileName, FilePath, FileSizeBytes,
    GitHubFile, GitHubToken, RepoName, RepoOwner, RepoSpec,
};

/// Library version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string used for HTTP requests.
pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION")
);