//! Domain types for GitHub documentation handling.
//!
//! This module provides semantic newtypes that prevent category errors and
//! make the domain model more explicit and understandable.

use crate::error::{RepoNameError, RepoOwnerError};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use url::Url;

/// Repository owner identifier (e.g., "rust-lang").
///
/// This type ensures that repository owners are validated and prevents
/// confusion with other string types in the API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoOwner(String);

impl RepoOwner {
    /// Create a new repository owner after validation.
    ///
    /// # Errors
    ///
    /// Returns `RepoOwnerError` if the owner name is invalid.
    pub fn new(owner: impl AsRef<str>) -> Result<Self, RepoOwnerError> {
        let owner = owner.as_ref();
        
        if owner.is_empty() {
            return Err(RepoOwnerError::Empty);
        }
        
        if owner.len() > 39 {
            return Err(RepoOwnerError::TooLong { len: owner.len() });
        }
        
        // GitHub username/organization validation
        if !owner.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(RepoOwnerError::InvalidCharacters {
                owner: owner.to_string(),
            });
        }
        
        Ok(Self(owner.to_string()))
    }
    
    /// Get the owner name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RepoOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RepoOwner {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Repository name identifier (e.g., "rust").
///
/// This type ensures that repository names are validated and prevents
/// confusion with other string types in the API.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoName(String);

impl RepoName {
    /// Create a new repository name after validation.
    ///
    /// # Errors
    ///
    /// Returns `RepoNameError` if the repository name is invalid.
    pub fn new(name: impl AsRef<str>) -> Result<Self, RepoNameError> {
        let name = name.as_ref();
        
        if name.is_empty() {
            return Err(RepoNameError::Empty);
        }
        
        if name.len() > 100 {
            return Err(RepoNameError::TooLong { len: name.len() });
        }
        
        // GitHub repository name validation (simplified)
        if !name.chars().all(|c| c.is_alphanumeric() || "-_.".contains(c)) {
            return Err(RepoNameError::InvalidCharacters {
                name: name.to_string(),
            });
        }
        
        Ok(Self(name.to_string()))
    }
    
    /// Get the repository name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RepoName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RepoName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// File name with validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileName(String);

impl FileName {
    /// Create a new file name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
    
    /// Get the file name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FileName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

/// File path with semantic meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilePath(PathBuf);

impl FilePath {
    /// Create a new file path.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
    
    /// Get the path as a `PathBuf` reference.
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
    
    /// Convert into the underlying `PathBuf`.
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
    
    /// Get the path as a string, using lossy conversion if needed.
    pub fn as_string_lossy(&self) -> std::borrow::Cow<str> {
        self.0.to_string_lossy()
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<PathBuf> for FilePath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<&str> for FilePath {
    fn from(path: &str) -> Self {
        Self(PathBuf::from(path))
    }
}

/// Download URL with validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadUrl(Url);

impl DownloadUrl {
    /// Create a new download URL from a validated URL.
    pub fn new(url: Url) -> Self {
        Self(url)
    }
    
    /// Parse a download URL from a string.
    ///
    /// # Errors
    ///
    /// Returns `url::ParseError` if the URL is invalid.
    pub fn parse(url: impl AsRef<str>) -> Result<Self, url::ParseError> {
        Ok(Self(Url::parse(url.as_ref())?))
    }
    
    /// Get the URL as a reference.
    pub fn as_url(&self) -> &Url {
        &self.0
    }
    
    /// Convert into the underlying URL.
    pub fn into_url(self) -> Url {
        self.0
    }
    
    /// Get the URL as a string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for DownloadUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Url> for DownloadUrl {
    fn from(url: Url) -> Self {
        Self(url)
    }
}

/// File size in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileSizeBytes(u64);

impl FileSizeBytes {
    /// Create a new file size.
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }
    
    /// Get the size in bytes.
    pub fn bytes(&self) -> u64 {
        self.0
    }
    
    /// Get the size as a human-readable string.
    pub fn human_readable(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = self.0 as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", self.0, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }
}

impl fmt::Display for FileSizeBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.human_readable())
    }
}

impl From<u64> for FileSizeBytes {
    fn from(bytes: u64) -> Self {
        Self(bytes)
    }
}


/// Directory path for documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocsDirectory(String);

impl DocsDirectory {
    /// Create a new documentation directory path.
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
    
    /// Get the directory path as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    /// Convert into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for DocsDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for DocsDirectory {
    fn from(path: String) -> Self {
        Self(path)
    }
}

impl AsRef<str> for DocsDirectory {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Documentation file with complete metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentationFile {
    /// Name of the documentation file
    pub name: FileName,
    /// Path to the file within the repository
    pub path: FilePath,
    /// URL for downloading the file content
    pub download_url: DownloadUrl,
    /// Size of the file in bytes
    pub size: FileSizeBytes,
    /// Documentation directory this file belongs to
    pub docs_directory: DocsDirectory,
}

/// Repository specification combining owner and name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepoSpec {
    /// Repository owner (user or organization)
    pub owner: RepoOwner,
    /// Repository name
    pub name: RepoName,
}

impl RepoSpec {
    /// Create a new repository specification.
    pub fn new(owner: RepoOwner, name: RepoName) -> Self {
        Self { owner, name }
    }
    
    /// Get the full repository identifier as "owner/repo".
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

impl fmt::Display for RepoSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}