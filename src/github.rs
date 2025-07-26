//! GitHub API client for repository operations.
//!
//! This module provides a clean interface to GitHub API operations needed
//! for documentation discovery and file retrieval.

use crate::error::{GitHubDocsError, Result};
use crate::types::{
    DocsDirectory, DocumentationFile, GitHubFile, GitHubToken,
    RepoSpec,
};
use async_recursion::async_recursion;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;

/// GitHub API client for repository operations.
///
/// This client handles authentication, rate limiting, and provides methods
/// for discovering and accessing documentation files in GitHub repositories.
pub struct GitHubClient {
    client: Client,
    repo: RepoSpec,
    token: Option<GitHubToken>,
}

impl GitHubClient {
    /// Create a new GitHub API client.
    ///
    /// # Arguments
    ///
    /// * `repo` - Repository specification (owner/name)
    /// * `token` - Optional GitHub API token for authentication
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if the HTTP client cannot be configured.
    pub fn new(repo: RepoSpec, token: Option<GitHubToken>) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("gh-docs-download/0.1.0"),
        );

        if let Some(ref token) = token {
            let auth_value = format!("token {}", token.as_str());
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&auth_value)?,
            );
        }

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            repo,
            token,
        })
    }

    /// Find all documentation directories in the repository.
    ///
    /// This method searches the repository structure to identify directories
    /// that likely contain documentation files.
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if the repository cannot be accessed or
    /// if API requests fail.
    pub async fn find_docs_directories(&self) -> Result<Vec<DocsDirectory>> {
        if self.token.is_some() {
            self.find_docs_directories_api().await
        } else {
            self.find_docs_directories_tree_api().await
        }
    }

    /// Find documentation directories using the GitHub Contents API.
    ///
    /// This method recursively searches the repository structure using
    /// the Contents API, which requires authentication for private repos.
    async fn find_docs_directories_api(&self) -> Result<Vec<DocsDirectory>> {
        let mut docs_dirs = Vec::new();
        let mut visited = HashSet::new();

        self.find_docs_recursive("", &mut docs_dirs, &mut visited)
            .await?;

        Ok(docs_dirs)
    }

    /// Find documentation directories using the GitHub Tree API.
    ///
    /// This method uses the Tree API to get the complete repository structure
    /// in a single request, which is more efficient for discovery.
    async fn find_docs_directories_tree_api(&self) -> Result<Vec<DocsDirectory>> {
        println!("Using GitHub Tree API for directory discovery...");

        // Get repository information to find default branch
        let repo_url = format!(
            "https://api.github.com/repos/{}/{}",
            self.repo.owner.as_str(),
            self.repo.name.as_str()
        );
        let repo_response = self.client.get(&repo_url).send().await?;

        if !repo_response.status().is_success() {
            return Err(GitHubDocsError::from_response_status(
                repo_response.status(),
                "Failed to get repository information",
            ));
        }

        let repo_info: Value = repo_response.json().await?;
        let default_branch = repo_info["default_branch"]
            .as_str()
            .unwrap_or("main");

        // Get the complete repository tree
        let tree_url = format!(
            "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
            self.repo.owner.as_str(),
            self.repo.name.as_str(),
            default_branch
        );

        println!("Fetching repository structure...");
        let tree_response = self.client.get(&tree_url).send().await?;

        if !tree_response.status().is_success() {
            return Err(GitHubDocsError::from_response_status(
                tree_response.status(),
                "Failed to get repository tree",
            ));
        }

        let tree_data: Value = tree_response.json().await?;
        let mut candidate_dirs = HashSet::new();

        // Analyze tree structure to find documentation directories
        if let Some(tree) = tree_data["tree"].as_array() {
            for item in tree {
                if let (Some(path), Some(item_type)) = (item["path"].as_str(), item["type"].as_str()) {
                    if item_type == "tree" {
                        // Check if directory name suggests documentation
                        if Self::is_documentation_directory(path) {
                            candidate_dirs.insert(path.to_string());
                        }
                    } else if item_type == "blob" && Self::is_documentation_file(path) {
                        // Check if file is in a documentation directory
                        if let Some(parent) = std::path::Path::new(path).parent() {
                            let dir_path = parent.to_string_lossy().to_string();
                            if !dir_path.is_empty() && Self::is_documentation_directory(&dir_path) {
                                candidate_dirs.insert(dir_path);
                            }
                        }
                    }
                }
            }
        }

        // Deduplicate and validate directories
        let candidate_dirs = Self::deduplicate_docs_paths(candidate_dirs.into_iter().collect());
        println!("Validating {} candidate directories...", candidate_dirs.len());
        
        let validated_dirs = self.validate_docs_directories(&candidate_dirs, &tree_data).await?;
        
        Ok(validated_dirs.into_iter().map(DocsDirectory::new).collect())
    }

    /// Recursively search for documentation directories using the Contents API.
    #[async_recursion]
    async fn find_docs_recursive(
        &self,
        path: &str,
        docs_dirs: &mut Vec<DocsDirectory>,
        visited: &mut HashSet<String>,
    ) -> Result<()> {
        if visited.contains(path) {
            return Ok(());
        }
        visited.insert(path.to_string());

        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.repo.owner.as_str(),
            self.repo.name.as_str(),
            path
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            // Skip directories we can't access
            return Ok(());
        }

        let files: Vec<GitHubFile> = response.json().await?;

        for file in files {
            if file.file_type == "dir" {
                // Check if this is a documentation directory
                if Self::is_documentation_directory(file.name.as_str()) {
                    docs_dirs.push(DocsDirectory::new(file.path.as_string_lossy()));
                }

                // Recursively search subdirectories
                self.find_docs_recursive(
                    &file.path.as_string_lossy(),
                    docs_dirs,
                    visited,
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Get all documentation files from a specific directory.
    ///
    /// # Arguments
    ///
    /// * `docs_dir` - Documentation directory to scan
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if the directory cannot be accessed.
    pub async fn get_documentation_files(
        &self,
        docs_dir: &DocsDirectory,
    ) -> Result<Vec<DocumentationFile>> {
        let mut doc_files = Vec::new();
        self.collect_doc_files_recursive(docs_dir.as_str(), docs_dir.as_str(), &mut doc_files)
            .await?;
        Ok(doc_files)
    }

    /// Recursively collect documentation files from a directory.
    #[async_recursion]
    async fn collect_doc_files_recursive(
        &self,
        current_path: &str,
        docs_root: &str,
        doc_files: &mut Vec<DocumentationFile>,
    ) -> Result<()> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.repo.owner.as_str(),
            self.repo.name.as_str(),
            current_path
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(());
        }

        let files: Vec<GitHubFile> = response.json().await?;

        for file in files {
            if file.file_type == "file" && Self::is_documentation_file(&file.path.as_string_lossy()) {
                if let Some(download_url) = file.download_url {
                    doc_files.push(DocumentationFile {
                        name: file.name,
                        path: file.path,
                        download_url,
                        size: file.size,
                        docs_directory: DocsDirectory::new(docs_root),
                    });
                }
            } else if file.file_type == "dir" {
                // Recursively search subdirectories
                self.collect_doc_files_recursive(
                    &file.path.as_string_lossy(),
                    docs_root,
                    doc_files,
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Validate that directories actually contain documentation files.
    async fn validate_docs_directories(
        &self,
        candidate_dirs: &[String],
        tree_data: &Value,
    ) -> Result<Vec<String>> {
        let mut validated_dirs = Vec::new();

        if let Some(tree) = tree_data["tree"].as_array() {
            for dir in candidate_dirs {
                let mut doc_file_count = 0;

                // Count documentation files in this directory
                for item in tree {
                    if let (Some(path), Some(item_type)) = (item["path"].as_str(), item["type"].as_str()) {
                        if item_type == "blob" && path.starts_with(&format!("{}/", dir)) {
                            if Self::is_documentation_file(path) {
                                doc_file_count += 1;
                            }
                        }
                    }
                }

                if doc_file_count > 0 {
                    println!("✓ {} ({} doc files)", dir, doc_file_count);
                    validated_dirs.push(dir.clone());
                } else {
                    println!("✗ {} (no documentation files found)", dir);
                }
            }
        }

        Ok(validated_dirs)
    }

    /// Remove duplicate directory paths, keeping only root directories.
    fn deduplicate_docs_paths(mut paths: Vec<String>) -> Vec<String> {
        if paths.is_empty() {
            return paths;
        }

        paths.sort();
        let mut deduplicated = Vec::new();

        for path in paths {
            let should_add = deduplicated.iter().all(|existing: &String| {
                !path.starts_with(&format!("{}/", existing))
            });

            if should_add {
                deduplicated.retain(|existing| {
                    !existing.starts_with(&format!("{}/", path))
                });
                deduplicated.push(path);
            }
        }

        deduplicated
    }

    /// Check if a directory name suggests it contains documentation.
    fn is_documentation_directory(path: &str) -> bool {
        let path_lower = path.to_lowercase();
        let doc_indicators = [
            "doc", "docs", "documentation", "guide", "guides", "manual", "wiki",
            "readme", "tutorial", "tutorials", "reference", "api", "book", "books",
        ];

        doc_indicators.iter().any(|indicator| {
            path_lower.contains(indicator)
        })
    }

    /// Check if a file appears to be documentation based on its path and extension.
    fn is_documentation_file(path: &str) -> bool {
        let path_lower = path.to_lowercase();

        // Check file extensions
        let doc_extensions = [
            ".md", ".markdown", ".txt", ".rst", ".adoc", ".asciidoc",
            ".org", ".tex", ".pdf", ".html", ".htm", ".xml",
        ];
        
        if doc_extensions.iter().any(|ext| path_lower.ends_with(ext)) {
            return true;
        }

        // Check common documentation filenames
        let filename = std::path::Path::new(&path_lower)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let doc_names = [
            "readme", "changelog", "changes", "news", "history",
            "license", "copying", "authors", "contributors", "todo",
            "install", "installation", "usage", "guide", "tutorial",
            "faq", "api", "reference", "manual", "docs", "documentation",
        ];

        doc_names.iter().any(|name| {
            filename == *name ||
            filename.starts_with(&format!("{}.", name)) ||
            filename.starts_with(&format!("{}_", name)) ||
            filename.starts_with(&format!("{}-", name))
        })
    }

    /// Get the repository specification.
    pub fn repo(&self) -> &RepoSpec {
        &self.repo
    }

    /// Download a file from its download URL.
    ///
    /// # Arguments
    ///
    /// * `doc_file` - Documentation file to download
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if the download fails.
    pub async fn download_file_content(&self, doc_file: &DocumentationFile) -> Result<Vec<u8>> {
        let response = self.client.get(doc_file.download_url.as_str()).send().await?;

        if !response.status().is_success() {
            return Err(GitHubDocsError::download_failed(
                doc_file.path.as_string_lossy(),
                format!("HTTP {}", response.status()),
            ));
        }

        let content = response.bytes().await?;
        Ok(content.to_vec())
    }
}