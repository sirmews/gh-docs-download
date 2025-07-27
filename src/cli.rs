//! Command-line interface for the GitHub documentation downloader.
//!
//! This module provides the CLI argument parsing and main application logic.

use crate::error::{GitHubDocsError, Result};
use crate::types::{RepoName, RepoOwner, RepoSpec};
use clap::Parser;
use url::Url;

/// A CLI tool to download documentation files from GitHub repositories.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GitHub tree URL (e.g., "<https://github.com/owner/repo/tree/branch/path>")
    #[arg(short = 'r', long)]
    pub repo: String,

    /// Output directory for downloaded files
    #[arg(short = 'o', long, default_value = "downloads")]
    pub output: String,

    /// Only list files without downloading
    #[arg(long)]
    pub list_only: bool,

    /// Include subdirectories recursively
    #[arg(long, default_value = "true")]
    pub recursive: bool,

}

impl Args {
    /// Parse GitHub tree URL into repository spec and documentation path.
    ///
    /// Expected format: `https://github.com/owner/repo/tree/branch/path`
    ///
    /// # Returns
    ///
    /// Returns `(RepoSpec, String)` where the second element is the documentation path.
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError::InvalidRepoFormat` if the URL is not a valid GitHub tree URL.
    pub fn parse_repo_spec(&self) -> Result<(RepoSpec, String)> {
        let url = Url::parse(&self.repo)?;
        
        // Verify it's a GitHub URL
        if url.host_str() != Some("github.com") {
            return Err(GitHubDocsError::InvalidRepoFormat {
                input: self.repo.clone(),
            });
        }
        
        let path_segments: Vec<&str> = url
            .path_segments()
            .ok_or_else(|| GitHubDocsError::InvalidRepoFormat {
                input: self.repo.clone(),
            })?
            .collect();

        // Must be: /owner/repo/tree/branch/path...
        if path_segments.len() < 5 || path_segments[2] != "tree" {
            return Err(GitHubDocsError::InvalidRepoFormat {
                input: format!("Expected GitHub tree URL format: https://github.com/owner/repo/tree/branch/path, got: {}", self.repo),
            });
        }

        let owner = RepoOwner::new(path_segments[0])?;
        let repo_name = RepoName::new(path_segments[1])?;
        let repo_spec = RepoSpec::new(owner, repo_name);
        
        // Extract path after /tree/branch/
        let doc_path = path_segments[4..].join("/");
        
        Ok((repo_spec, doc_path))
    }

    /// Validate the arguments and return any validation errors.
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError::InvalidRepoFormat` if the repository URL format is invalid.
    pub fn validate(&self) -> Result<()> {
        // Validate repository format
        let _ = self.parse_repo_spec()?;

        // Validate output directory path
        if self.output.is_empty() {
            return Err(GitHubDocsError::InvalidRepoFormat {
                input: "Output directory cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}

/// CLI application runner.
pub struct CliApp {
    args: Args,
}

impl CliApp {
    /// Create a new CLI application with the given arguments.
    #[must_use] pub fn new(args: Args) -> Self {
        Self { args }
    }

    /// Run the CLI application.
    ///
    /// This is the main entry point that orchestrates the entire operation:
    /// 1. Validate arguments
    /// 2. Create downloader
    /// 3. Discover documentation directories
    /// 4. Collect documentation files
    /// 5. Download or list files
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError` if any step of the process fails.
    #[allow(clippy::unused_async)]
    pub async fn run(&self) -> Result<()> {
        // Validate arguments
        self.args.validate()?;

        // Parse repository specification and extract path from tree URL
        let (repo_spec, doc_path) = self.args.parse_repo_spec()?;

        // Create download configuration
        let config = crate::downloader::DownloadConfig {
            output_dir: self.args.output.clone(),
            list_only: self.args.list_only,
            recursive: self.args.recursive,
            target_path: doc_path,
        };

        // Create downloader
        let downloader = crate::downloader::GitHubDocsDownloader::new(repo_spec.clone(), config);

        println!(
            "Searching for documentation directories in {}...",
            repo_spec.full_name()
        );

        // Discover documentation directories
        let docs_dirs = downloader.find_docs_directories()?;

        if docs_dirs.is_empty() {
            return Err(GitHubDocsError::no_documentation_found(
                repo_spec.owner.as_str(),
                repo_spec.name.as_str(),
            ));
        }

        println!("Found {} documentation directories:", docs_dirs.len());
        for dir in &docs_dirs {
            println!("  - {dir}");
        }

        // Collect all documentation files
        let all_doc_files = downloader.get_all_documentation_files(&docs_dirs)?;

        if all_doc_files.is_empty() {
            println!("No documentation files found in the discovered directories.");
            return Ok(());
        }

        // Download or list files
        downloader.download_files(&all_doc_files)?;

        Ok(())
    }

    /// Get the parsed arguments.
    #[must_use]
    pub fn args(&self) -> &Args {
        &self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_spec_tree_url() {
        let args = Args {
            repo: "https://github.com/rust-lang/rust/tree/main/docs".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };

        let (repo_spec, path) = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "rust-lang");
        assert_eq!(repo_spec.name.as_str(), "rust");
        assert_eq!(path, "docs");
    }

    #[test]
    fn test_parse_repo_spec_tree_url_nested_path() {
        let args = Args {
            repo: "https://github.com/TanStack/router/tree/main/docs/router/eslint".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };

        let (repo_spec, path) = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "TanStack");
        assert_eq!(repo_spec.name.as_str(), "router");
        assert_eq!(path, "docs/router/eslint");
    }

    #[test]
    fn test_parse_repo_spec_invalid_url() {
        let args = Args {
            repo: "https://notgithub.com/owner/repo/tree/main/docs".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };

        assert!(args.parse_repo_spec().is_err());
    }

    #[test]
    fn test_parse_repo_spec_missing_tree_structure() {
        let args = Args {
            repo: "https://github.com/owner/repo".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };

        assert!(args.parse_repo_spec().is_err());
    }

    #[test]
    fn test_parse_repo_spec_invalid_format() {
        let args = Args {
            repo: "invalid-repo-format".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
        };

        assert!(args.parse_repo_spec().is_err());
    }
}