//! File downloader using git clone approach.
//!
//! This module provides the core downloading functionality using git clone
//! to access repository contents locally.

use crate::error::{GitHubDocsError, Result};
use crate::types::{DocsDirectory, DocumentationFile, FilePath, RepoSpec};
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
    /// Specific path within repository to download
    pub target_path: String,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            output_dir: "downloads".to_string(),
            list_only: false,
            recursive: true,
            target_path: "docs".to_string(),
        }
    }
}

/// Documentation downloader using git clone approach.
///
/// This downloader fetches documentation files by cloning the repository with git.
/// It automatically discovers documentation directories and filters files based on
/// common documentation patterns.
pub struct GitHubDocsDownloader {
    repo: RepoSpec,
    config: DownloadConfig,
}

impl GitHubDocsDownloader {
    /// Create a new documentation downloader.
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository specification (owner/name)
    /// * `config` - Download configuration
    #[must_use]
    pub fn new(repo: RepoSpec, config: DownloadConfig) -> Self {
        Self { repo, config }
    }

    /// Discover all documentation directories in the repository.
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if directory discovery fails.
    pub fn find_docs_directories(&self) -> Result<Vec<DocsDirectory>> {
        Ok(self.find_docs_directories_git())
    }

    /// Find documentation directories using git clone approach.
    fn find_docs_directories_git(&self) -> Vec<DocsDirectory> {
        println!(
            "Using sparse checkout for path: {}",
            self.config.target_path
        );
        // Return the target path directly since we always have one from the tree URL
        vec![DocsDirectory::new(self.config.target_path.clone())]
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
    pub fn get_all_documentation_files(
        &self,
        docs_dirs: &[DocsDirectory],
    ) -> Result<Vec<DocumentationFile>> {
        let mut all_files = Vec::new();

        for docs_dir in docs_dirs {
            println!("Scanning {docs_dir}...");

            let files = self.get_documentation_files_git(docs_dir)?;

            println!("Found {} documentation files in {}", files.len(), docs_dir);
            for file in &files {
                println!("  - {} ({})", file.path, file.size);
            }

            all_files.extend(files);
        }

        Ok(all_files)
    }

    /// Get documentation files from a directory using git clone approach.
    fn get_documentation_files_git(
        &self,
        docs_dir: &DocsDirectory,
    ) -> Result<Vec<DocumentationFile>> {
        // Create temporary directory and clone
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().join(self.repo.name.as_str());

        let clone_url = format!(
            "https://github.com/{}/{}.git",
            self.repo.owner.as_str(),
            self.repo.name.as_str()
        );

        // Use sparse checkout for the specific documentation path
        // Clone with no checkout
        let output = Command::new("git")
            .args(["clone", "--no-checkout", "--depth", "1", &clone_url])
            .current_dir(temp_dir.path())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitHubDocsError::git_operation_failed(
                format!("git clone --no-checkout {clone_url}"),
                stderr,
            ));
        }

        // Enable sparse checkout
        let output = Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&repo_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitHubDocsError::git_operation_failed(
                "git config core.sparseCheckout true".to_string(),
                stderr,
            ));
        }

        // Set sparse checkout paths
        let sparse_info_dir = repo_path.join(".git").join("info");
        std::fs::create_dir_all(&sparse_info_dir)?;
        let sparse_checkout_file = sparse_info_dir.join("sparse-checkout");
        std::fs::write(&sparse_checkout_file, format!("{}/*\n", docs_dir.as_str()))?;

        // Checkout the specified paths
        let output = Command::new("git")
            .args(["checkout"])
            .current_dir(&repo_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitHubDocsError::git_operation_failed(
                "git checkout".to_string(),
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
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy();
            if Self::is_documentation_file(&file_name) {
                let file_size = entry
                    .metadata()
                    .map_err(GitHubDocsError::WalkDirError)?
                    .len();

                // Copy file immediately while temp directory exists
                if !self.config.list_only {
                    // Flatten structure: use only the filename, not the full path
                    let dest_path = Path::new(&self.config.output_dir).join(entry.file_name());

                    // Create output directory if it doesn't exist
                    std::fs::create_dir_all(&self.config.output_dir)?;
                    std::fs::copy(entry.path(), &dest_path)?;
                }

                // Create documentation file record (URL not needed for git approach)
                // For flattened structure, use just the filename as the path
                let flattened_path = Path::new(file_name.as_ref());
                doc_files.push(DocumentationFile {
                    name: file_name.to_string().into(),
                    path: FilePath::new(flattened_path.to_path_buf()),
                    download_url: crate::types::DownloadUrl::parse("file://downloaded")?,
                    size: file_size.into(),
                    docs_directory: docs_dir.clone(),
                });
            }
        }

        Ok(doc_files)
    }

    /// Show download summary for files.
    ///
    /// # Arguments
    ///
    /// * `files` - Documentation files that were processed
    ///
    /// # Errors
    ///
    /// This function does not return errors in the current implementation.
    pub fn download_files(&self, files: &[DocumentationFile]) -> Result<()> {
        if self.config.list_only {
            Self::print_file_summary(files);
            return Ok(());
        }

        // Files are already downloaded during get_documentation_files_git
        println!("\nDownload complete!");
        println!(
            "  Downloaded {} files to {}",
            files.len(),
            self.config.output_dir
        );

        Self::print_file_summary(files);
        Ok(())
    }

    /// Print a summary of discovered files.
    fn print_file_summary(files: &[DocumentationFile]) {
        println!("\nTotal documentation files found: {}", files.len());

        let total_size: u64 = files.iter().map(|f| f.size.bytes()).sum();
        println!("Total size: {total_size} bytes");

        // Group files by directory
        let mut dirs_summary = std::collections::HashMap::new();
        for file in files {
            let entry = dirs_summary
                .entry(file.docs_directory.as_str())
                .or_insert((0, 0u64));
            entry.0 += 1;
            entry.1 += file.size.bytes();
        }

        println!("\nFiles by directory:");
        for (dir, (count, size)) in dirs_summary {
            println!("  {dir}: {count} files ({size} bytes)");
        }
    }

    /// Check if a file appears to be documentation based on its name and extension.
    fn is_documentation_file(filename: &str) -> bool {
        let filename_lower = filename.to_lowercase();

        // Check file extensions
        let doc_extensions = [
            ".md",
            ".mdx",
            ".markdown",
            ".txt",
            ".rst",
            ".adoc",
            ".asciidoc",
            ".org",
            ".tex",
            ".pdf",
            ".html",
            ".htm",
            ".xml",
        ];

        if doc_extensions
            .iter()
            .any(|ext| filename_lower.ends_with(ext))
        {
            return true;
        }

        // Check common documentation filenames
        let doc_names = [
            "readme",
            "changelog",
            "changes",
            "news",
            "history",
            "license",
            "copying",
            "authors",
            "contributors",
            "todo",
            "install",
            "installation",
            "usage",
            "guide",
            "tutorial",
            "faq",
            "api",
            "reference",
            "manual",
            "docs",
            "documentation",
        ];

        doc_names.iter().any(|name| {
            filename_lower == *name
                || filename_lower.starts_with(&format!("{name}."))
                || filename_lower.starts_with(&format!("{name}_"))
                || filename_lower.starts_with(&format!("{name}-"))
        })
    }

    /// Get the repository specification.
    #[must_use]
    pub fn repo(&self) -> &RepoSpec {
        &self.repo
    }
}
