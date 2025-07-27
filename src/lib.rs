//! # GitHub Documentation Downloader
//!
//! A comprehensive library for discovering and downloading documentation files
//! from GitHub repositories using git sparse checkout. This crate provides
//! efficient git-based access to repository contents, with automatic discovery of
//! documentation directories and intelligent file filtering.
//!
//! ## Features
//!
//! - **Git Sparse Checkout**: Efficient downloading using git sparse checkout for targeted paths
//! - **Tree URL Support**: Direct support for GitHub tree URLs (e.g., github.com/owner/repo/tree/branch/path)
//! - **Documentation Detection**: Smart identification of documentation files by extension and name
//! - **Type Safety**: Comprehensive use of semantic types to prevent category errors
//! - **Error Handling**: Detailed error types with actionable information
//! - **Performance**: Fast downloads without rate limiting concerns
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gh_docs_download::{
//!     downloader::{DownloadConfig, GitHubDocsDownloader},
//!     types::{RepoOwner, RepoName, RepoSpec},
//! };
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a repository specification
//! let owner = RepoOwner::new("rust-lang")?;
//! let name = RepoName::new("rust")?;
//! let repo = RepoSpec::new(owner, name);
//!
//! // Configure the downloader with target path
//! let config = DownloadConfig {
//!     output_dir: "docs".to_string(),
//!     list_only: false,
//!     recursive: true,
//!     target_path: "src/doc".to_string(), // Specific documentation path
//! };
//!
//! // Create downloader (git-only, no authentication needed)
//! let downloader = GitHubDocsDownloader::new(repo, config);
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
//! - [`downloader`] - Git-based downloading logic and configuration
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
//!     Err(GitHubDocsError::GitOperationFailed { command, stderr }) => {
//!         println!("Git command '{}' failed: {}", command, stderr);
//!     }
//!     Err(GitHubDocsError::RepositoryNotFound { owner, repo }) => {
//!         println!("Repository {}/{} not found or inaccessible", owner, repo);
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
pub mod types;

// Re-export the most commonly used types for convenience
pub use error::{GitHubDocsError, Result};
pub use types::{
    DocumentationFile, DocsDirectory, DownloadUrl, FileName, FilePath, FileSizeBytes,
    RepoName, RepoOwner, RepoSpec,
};

/// Library version information.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

