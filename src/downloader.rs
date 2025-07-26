//! File downloader with support for both API and git-based approaches.
//!
//! This module provides the core downloading functionality with support
//! for different data sources and output formats.

use crate::error::{GitHubDocsError, Result};
use crate::github::GitHubClient;
use crate::types::{DocumentationFile, DocsDirectory, FilePath, GitHubToken, RepoSpec};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;

/// Configuration for the documentation downloader.
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    /// Output directory for downloaded files
    pub output_dir: String,
    /// Whether to only list files without downloading
    pub list_only: bool,
    /// Whether to include subdirectories recursively
    pub recursive: bool,
    /// Whether to use git clone instead of GitHub API
    pub use_git: bool,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            output_dir: "downloads".to_string(),
            list_only: false,
            recursive: true,
            use_git: false,
        }
    }
}

/// Documentation downloader that supports multiple data sources.
///
/// This downloader can fetch documentation files using either the GitHub API
/// or by cloning the repository with git. It automatically discovers documentation
/// directories and filters files based on common documentation patterns.
pub struct GitHubDocsDownloader {
    github_client: GitHubClient,
    config: DownloadConfig,
}

impl GitHubDocsDownloader {
    /// Create a new documentation downloader.
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository specification (owner/name)
    /// * `token` - Optional GitHub API token for authentication
    /// * `config` - Download configuration
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if the GitHub client cannot be created.
    pub fn new(
        repo: RepoSpec,
        token: Option<GitHubToken>,
        config: DownloadConfig,
    ) -> Result<Self> {
        let github_client = GitHubClient::new(repo, token)?;

        Ok(Self {
            github_client,
            config,
        })
    }

    /// Discover all documentation directories in the repository.
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if directory discovery fails.
    pub async fn find_docs_directories(&self) -> Result<Vec<DocsDirectory>> {
        if self.config.use_git {
            self.find_docs_directories_git().await
        } else {
            self.github_client.find_docs_directories().await
        }
    }

    /// Find documentation directories using git clone approach.
    async fn find_docs_directories_git(&self) -> Result<Vec<DocsDirectory>> {
        println!("Using git clone for directory discovery...");

        // Create temporary directory and clone
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().join(self.github_client.repo().name.as_str());

        let clone_url = format!(
            "https://github.com/{}/{}.git",
            self.github_client.repo().owner.as_str(),
            self.github_client.repo().name.as_str()
        );

        let output = Command::new("git")
            .args(&["clone", "--depth", "1", &clone_url])
            .current_dir(temp_dir.path())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitHubDocsError::git_operation_failed(
                format!("git clone {}", clone_url),
                stderr,
            ));
        }

        // Find documentation directories in the cloned repository
        let mut docs_dirs = Vec::new();
        
        for entry in WalkDir::new(&repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let dir_name = entry.file_name().to_string_lossy();
            if Self::is_documentation_directory(&dir_name) {
                let relative_path = entry.path().strip_prefix(&repo_path)?;
                docs_dirs.push(DocsDirectory::new(relative_path.to_string_lossy()));
            }
        }

        Ok(docs_dirs)
    }

    /// Get all documentation files from the specified directories.
    ///
    /// # Arguments
    ///
    /// * `docs_dirs` - Directories to scan for documentation files
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if file discovery fails.
    pub async fn get_all_documentation_files(
        &self,
        docs_dirs: &[DocsDirectory],
    ) -> Result<Vec<DocumentationFile>> {
        let mut all_files = Vec::new();

        for docs_dir in docs_dirs {
            println!("Scanning {}...", docs_dir);
            
            let files = if self.config.use_git {
                self.get_documentation_files_git(docs_dir).await?
            } else {
                self.github_client.get_documentation_files(docs_dir).await?
            };

            println!("Found {} documentation files in {}", files.len(), docs_dir);
            for file in &files {
                println!("  - {} ({})", file.path, file.size);
            }

            all_files.extend(files);
        }

        Ok(all_files)
    }

    /// Get documentation files from a directory using git clone approach.
    async fn get_documentation_files_git(
        &self,
        docs_dir: &DocsDirectory,
    ) -> Result<Vec<DocumentationFile>> {
        // Create temporary directory and clone
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().join(self.github_client.repo().name.as_str());

        let clone_url = format!(
            "https://github.com/{}/{}.git",
            self.github_client.repo().owner.as_str(),
            self.github_client.repo().name.as_str()
        );

        let output = Command::new("git")
            .args(&["clone", "--depth", "1", &clone_url])
            .current_dir(temp_dir.path())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitHubDocsError::git_operation_failed(
                format!("git clone {}", clone_url),
                stderr,
            ));
        }

        let docs_path = repo_path.join(docs_dir.as_str());
        if !docs_path.exists() {
            return Ok(Vec::new());
        }

        let mut doc_files = Vec::new();

        for entry in WalkDir::new(&docs_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy();
            if Self::is_documentation_file(&file_name) {
                let relative_path = entry.path().strip_prefix(&repo_path)?;
                let file_size = entry.metadata().map_err(|e| GitHubDocsError::WalkDirError(e))?.len();

                // For git approach, we'll use file:// URLs for local access
                doc_files.push(DocumentationFile {
                    name: file_name.to_string().into(),
                    path: FilePath::new(relative_path.to_path_buf()),
                    download_url: crate::types::DownloadUrl::parse(&format!("file://{}", entry.path().display()))?,
                    size: file_size.into(),
                    docs_directory: docs_dir.clone(),
                });
            }
        }

        Ok(doc_files)
    }

    /// Download all files to the configured output directory.
    ///
    /// # Arguments
    ///
    /// * `files` - Documentation files to download
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if downloading fails.
    pub async fn download_files(&self, files: &[DocumentationFile]) -> Result<()> {
        if self.config.list_only {
            self.print_file_summary(files);
            return Ok(());
        }

        println!("Downloading {} files to {}...", files.len(), self.config.output_dir);
        std::fs::create_dir_all(&self.config.output_dir)?;

        let mut success_count = 0;
        let mut error_count = 0;

        for doc_file in files {
            match self.download_single_file(doc_file).await {
                Ok(()) => {
                    success_count += 1;
                    println!("Downloaded: {}", doc_file.path);
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!("Error downloading {}: {}", doc_file.path, e);
                }
            }
        }

        println!("\nDownload complete!");
        println!("  Successful: {}", success_count);
        if error_count > 0 {
            println!("  Failed: {}", error_count);
        }

        Ok(())
    }

    /// Download a single file to the output directory.
    async fn download_single_file(&self, doc_file: &DocumentationFile) -> Result<()> {
        // Create the full path maintaining directory structure
        let file_path = Path::new(&self.config.output_dir).join(doc_file.path.as_path());

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if doc_file.download_url.as_str().starts_with("file://") {
            // Local file from git clone - copy directly
            let source_path = doc_file.download_url.as_str().strip_prefix("file://").unwrap();
            std::fs::copy(source_path, &file_path)?;
        } else {
            // Remote file from API - download via HTTP
            let content = self.github_client.download_file_content(doc_file).await?;
            std::fs::write(&file_path, content)?;
        }

        Ok(())
    }

    /// Print a summary of discovered files.
    fn print_file_summary(&self, files: &[DocumentationFile]) {
        println!("\nTotal documentation files found: {}", files.len());
        
        let total_size: u64 = files.iter().map(|f| f.size.bytes()).sum();
        println!("Total size: {} bytes", total_size);

        // Group files by directory
        let mut dirs_summary = std::collections::HashMap::new();
        for file in files {
            let entry = dirs_summary.entry(file.docs_directory.as_str()).or_insert((0, 0u64));
            entry.0 += 1;
            entry.1 += file.size.bytes();
        }

        println!("\nFiles by directory:");
        for (dir, (count, size)) in dirs_summary {
            println!("  {}: {} files ({} bytes)", dir, count, size);
        }
    }

    /// Check if a directory name suggests it contains documentation.
    fn is_documentation_directory(name: &str) -> bool {
        let name_lower = name.to_lowercase();
        let doc_indicators = [
            "doc", "docs", "documentation", "guide", "guides", "manual", "wiki",
            "readme", "tutorial", "tutorials", "reference", "api", "book", "books",
        ];

        doc_indicators.iter().any(|indicator| {
            name_lower.contains(indicator)
        })
    }

    /// Check if a file appears to be documentation based on its name and extension.
    fn is_documentation_file(filename: &str) -> bool {
        let filename_lower = filename.to_lowercase();

        // Check file extensions
        let doc_extensions = [
            ".md", ".markdown", ".txt", ".rst", ".adoc", ".asciidoc",
            ".org", ".tex", ".pdf", ".html", ".htm", ".xml",
        ];

        if doc_extensions.iter().any(|ext| filename_lower.ends_with(ext)) {
            return true;
        }

        // Check common documentation filenames
        let doc_names = [
            "readme", "changelog", "changes", "news", "history",
            "license", "copying", "authors", "contributors", "todo",
            "install", "installation", "usage", "guide", "tutorial",
            "faq", "api", "reference", "manual", "docs", "documentation",
        ];

        doc_names.iter().any(|name| {
            filename_lower == *name ||
            filename_lower.starts_with(&format!("{}.", name)) ||
            filename_lower.starts_with(&format!("{}_", name)) ||
            filename_lower.starts_with(&format!("{}-", name))
        })
    }

    /// Get the repository specification.
    pub fn repo(&self) -> &RepoSpec {
        self.github_client.repo()
    }
}