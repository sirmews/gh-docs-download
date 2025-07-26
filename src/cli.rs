//! Command-line interface for the GitHub documentation downloader.
//!
//! This module provides the CLI argument parsing and main application logic.

use crate::error::{GitHubDocsError, Result};
use crate::types::{GitHubToken, RepoName, RepoOwner, RepoSpec};
use clap::Parser;
use url::Url;

/// A CLI tool to download documentation files from GitHub repositories.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GitHub repository URL or slug (e.g., "owner/repo" or "<https://github.com/owner/repo>")
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

    /// GitHub API token for authenticated requests
    #[arg(long)]
    pub token: Option<String>,

    /// Force use of git clone instead of GitHub API
    #[arg(long)]
    pub use_git: bool,
}

impl Args {
    /// Parse the repository input into a validated `RepoSpec`.
    ///
    /// # Panics
    ///
    /// This method may panic if string prefix operations fail unexpectedly.
    ///
    /// Supports various input formats:
    /// - `owner/repo` format
    /// - Full GitHub URLs: `https://github.com/owner/repo`
    /// - SSH URLs: `git@github.com:owner/repo.git`
    ///
    /// # Errors
    ///
    /// Returns `GitHubDocsError::InvalidRepoFormat` if the input cannot be parsed.
    pub fn parse_repo_spec(&self) -> Result<RepoSpec> {
        // Handle full GitHub URLs
        if self.repo.starts_with("http") {
            let url = Url::parse(&self.repo)?;
            let path_segments: Vec<&str> = url
                .path_segments()
                .ok_or_else(|| GitHubDocsError::InvalidRepoFormat {
                    input: self.repo.clone(),
                })?
                .collect();

            if path_segments.len() >= 2 {
                let owner = RepoOwner::new(path_segments[0])?;
                let repo_name = path_segments[1].trim_end_matches(".git");
                let name = RepoName::new(repo_name)?;
                return Ok(RepoSpec::new(owner, name));
            }
        }

        // Handle SSH URLs (git@github.com:owner/repo.git)
        if self.repo.starts_with("git@github.com:") {
            let repo_part = self.repo.strip_prefix("git@github.com:").unwrap();
            if let Some((owner, repo)) = repo_part.split_once('/') {
                let owner = RepoOwner::new(owner)?;
                let repo_name = repo.trim_end_matches(".git");
                let name = RepoName::new(repo_name)?;
                return Ok(RepoSpec::new(owner, name));
            }
        }

        // Handle owner/repo format
        if let Some((owner, repo)) = self.repo.split_once('/') {
            let owner = RepoOwner::new(owner)?;
            let repo_name = repo.trim_end_matches(".git");
            let name = RepoName::new(repo_name)?;
            return Ok(RepoSpec::new(owner, name));
        }

        Err(GitHubDocsError::InvalidRepoFormat {
            input: self.repo.clone(),
        })
    }

    /// Get the GitHub token if provided.
    #[must_use]
    pub fn github_token(&self) -> Option<GitHubToken> {
        self.token.as_ref().map(|t| GitHubToken::new(t.clone()))
    }

    /// Validate the arguments and return any validation errors.
    pub fn validate(&self) -> Result<()> {
        // Validate repository format
        self.parse_repo_spec()?;

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
    pub fn new(args: Args) -> Self {
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
    pub async fn run(&self) -> Result<()> {
        // Validate arguments
        self.args.validate()?;

        // Parse repository specification
        let repo_spec = self.args.parse_repo_spec()?;
        let token = self.args.github_token();

        // Create download configuration
        let config = crate::downloader::DownloadConfig {
            output_dir: self.args.output.clone(),
            list_only: self.args.list_only,
            recursive: self.args.recursive,
            use_git: self.args.use_git,
        };

        // Create downloader
        let downloader = crate::downloader::GitHubDocsDownloader::new(repo_spec.clone(), token, config)?;

        println!(
            "Searching for documentation directories in {}...",
            repo_spec.full_name()
        );

        // Discover documentation directories
        let docs_dirs = downloader.find_docs_directories().await?;

        if docs_dirs.is_empty() {
            return Err(GitHubDocsError::no_documentation_found(
                repo_spec.owner.as_str(),
                repo_spec.name.as_str(),
            ));
        }

        println!("Found {} documentation directories:", docs_dirs.len());
        for dir in &docs_dirs {
            println!("  - {}", dir);
        }

        // Collect all documentation files
        let all_doc_files = downloader.get_all_documentation_files(&docs_dirs).await?;

        if all_doc_files.is_empty() {
            println!("No documentation files found in the discovered directories.");
            return Ok(());
        }

        // Download or list files
        downloader.download_files(&all_doc_files).await?;

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
    fn test_parse_repo_spec_owner_slash_repo() {
        let args = Args {
            repo: "rust-lang/rust".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: None,
            use_git: false,
        };

        let repo_spec = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "rust-lang");
        assert_eq!(repo_spec.name.as_str(), "rust");
    }

    #[test]
    fn test_parse_repo_spec_https_url() {
        let args = Args {
            repo: "https://github.com/rust-lang/rust".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: None,
            use_git: false,
        };

        let repo_spec = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "rust-lang");
        assert_eq!(repo_spec.name.as_str(), "rust");
    }

    #[test]
    fn test_parse_repo_spec_ssh_url() {
        let args = Args {
            repo: "git@github.com:rust-lang/rust.git".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: None,
            use_git: false,
        };

        let repo_spec = args.parse_repo_spec().unwrap();
        assert_eq!(repo_spec.owner.as_str(), "rust-lang");
        assert_eq!(repo_spec.name.as_str(), "rust");
    }

    #[test]
    fn test_parse_repo_spec_invalid_format() {
        let args = Args {
            repo: "invalid-repo-format".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: None,
            use_git: false,
        };

        assert!(args.parse_repo_spec().is_err());
    }

    #[test]
    fn test_github_token() {
        let args = Args {
            repo: "owner/repo".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: Some("ghp_test_token".to_string()),
            use_git: false,
        };

        let token = args.github_token().unwrap();
        assert_eq!(token.as_str(), "ghp_test_token");
    }

    #[test]
    fn test_github_token_none() {
        let args = Args {
            repo: "owner/repo".to_string(),
            output: "test".to_string(),
            list_only: false,
            recursive: true,
            token: None,
            use_git: false,
        };

        assert!(args.github_token().is_none());
    }
}