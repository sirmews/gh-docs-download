//! Error types for the GitHub documentation downloader.
//!
//! This module provides comprehensive error handling with semantic error types
//! that clearly indicate what went wrong and provide actionable information.

use thiserror::Error;

/// Result type alias for GitHub documentation operations.
pub type Result<T> = std::result::Result<T, GitHubDocsError>;

/// Comprehensive error type for GitHub documentation operations.
///
/// This error type provides specific variants for different failure modes,
/// making it easier to handle errors appropriately and provide useful
/// feedback to users.
#[derive(Debug, Error)]
pub enum GitHubDocsError {
    /// Invalid repository format provided
    #[error("Invalid repository format: {input}. Expected format: 'owner/repo' or 'https://github.com/owner/repo'")]
    InvalidRepoFormat {
        /// The invalid input that was provided
        input: String
    },

    /// Repository not found or access denied
    #[error("Repository '{owner}/{repo}' not found or access denied")]
    RepositoryNotFound {
        /// Repository owner name
        owner: String,
        /// Repository name
        repo: String
    },

    /// No documentation directories found
    #[error("No documentation directories found in repository '{owner}/{repo}'")]
    NoDocumentationFound {
        /// Repository owner name
        owner: String,
        /// Repository name
        repo: String
    },

    /// File download failed
    #[error("Failed to download file '{path}': {reason}")]
    DownloadFailed {
        /// Path of the file that failed to download
        path: String,
        /// Reason for the download failure
        reason: String
    },

    /// Git operation failed
    #[error("Git operation failed: {command} - {stderr}")]
    GitOperationFailed {
        /// Git command that failed
        command: String,
        /// Standard error output from the git command
        stderr: String
    },

    /// Invalid URL provided
    #[error("Invalid URL: {url}")]
    InvalidUrl {
        /// The invalid URL that was provided
        url: String
    },

    /// File system operation failed
    #[error("File system operation failed")]
    FileSystemError(#[from] std::io::Error),

    /// URL parsing failed
    #[error("URL parsing failed")]
    UrlParseError(#[from] url::ParseError),

    /// Path manipulation failed
    #[error("Path manipulation failed")]
    PathError(#[from] std::path::StripPrefixError),

    /// `WalkDir` error
    #[error("Directory traversal failed")]
    WalkDirError(#[from] walkdir::Error),

    /// Repository owner validation error
    #[error("Repository owner validation failed")]
    RepoOwnerValidationError(#[from] RepoOwnerError),

    /// Repository name validation error
    #[error("Repository name validation failed")]
    RepoNameValidationError(#[from] RepoNameError),
}

impl GitHubDocsError {

    /// Create a repository not found error.
    pub fn repository_not_found(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self::RepositoryNotFound {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// Create a no documentation found error.
    pub fn no_documentation_found(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self::NoDocumentationFound {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// Create a download failed error.
    pub fn download_failed(path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::DownloadFailed {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Create a git operation failed error.
    pub fn git_operation_failed(command: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self::GitOperationFailed {
            command: command.into(),
            stderr: stderr.into(),
        }
    }

}

/// Error type for repository owner validation.
#[derive(Debug, Error)]
pub enum RepoOwnerError {
    /// Repository owner name is empty
    #[error("Repository owner cannot be empty")]
    Empty,
    /// Repository owner contains invalid characters
    #[error("Repository owner contains invalid characters: {owner}")]
    InvalidCharacters {
        /// The invalid owner name that was provided
        owner: String
    },
    /// Repository owner name exceeds maximum length
    #[error("Repository owner is too long: {len} characters (max 39)")]
    TooLong {
        /// The actual length of the provided owner name
        len: usize
    },
}

/// Error type for repository name validation.
#[derive(Debug, Error)]
pub enum RepoNameError {
    /// Repository name is empty
    #[error("Repository name cannot be empty")]
    Empty,
    /// Repository name contains invalid characters
    #[error("Repository name contains invalid characters: {name}")]
    InvalidCharacters {
        /// The invalid repository name that was provided
        name: String
    },
    /// Repository name exceeds maximum length
    #[error("Repository name is too long: {len} characters (max 100)")]
    TooLong {
        /// The actual length of the provided repository name
        len: usize
    },
}