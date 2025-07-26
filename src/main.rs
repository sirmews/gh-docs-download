//! GitHub Documentation Downloader
//!
//! A command-line tool for discovering and downloading documentation files
//! from GitHub repositories. Supports both GitHub API and git clone approaches.

use clap::Parser;
use gh_docs_download::{
    cli::{Args, CliApp},
    error::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let app = CliApp::new(args);
    app.run().await
}