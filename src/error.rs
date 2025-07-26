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

    /// GitHub API request failed
    #[error("GitHub API request failed: {status} - {message}")]
    ApiRequestFailed {
        /// HTTP status code of the failed request
        status: u16,
        /// Error message from the API response
        message: String
    },

    /// GitHub API rate limit exceeded
    #[error("GitHub API rate limit exceeded. Consider using --token with a GitHub token")]
    RateLimitExceeded,

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

    /// Network request failed
    #[error("Network request failed")]
    NetworkError(#[from] reqwest::Error),

    /// URL parsing failed
    #[error("URL parsing failed")]
    UrlParseError(#[from] url::ParseError),

    /// HTTP header parsing failed
    #[error("HTTP header parsing failed")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    /// JSON parsing failed
    #[error("JSON parsing failed")]
    JsonError(#[from] serde_json::Error),

    /// Path manipulation failed
    #[error("Path manipulation failed")]
    PathError(#[from] std::path::StripPrefixError),

    /// WalkDir error
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
    /// Create a new API request failed error from a response.
    pub fn from_response_status(status: reqwest::StatusCode, message: impl Into<String>) -> Self {
        if status == reqwest::StatusCode::FORBIDDEN {
            Self::RateLimitExceeded
        } else {
            Self::ApiRequestFailed {
                status: status.as_u16(),
                message: message.into(),
            }
        }
    }

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

    /// Check if this error indicates a rate limit was exceeded.
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimitExceeded)
    }

    /// Check if this error indicates a network-related issue.
    pub fn is_network_error(&self) -> bool {
        matches!(
            self,
            Self::NetworkError(_) | Self::ApiRequestFailed { .. } | Self::RateLimitExceeded
        )
    }

    /// Check if this error indicates a repository access issue.
    pub fn is_repository_access_error(&self) -> bool {
        matches!(
            self,
            Self::RepositoryNotFound { .. } | Self::RateLimitExceeded
        )
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