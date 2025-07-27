//! GitHub Documentation Downloader
//!
//! A command-line tool for efficiently downloading documentation files
//! from GitHub repositories using git sparse checkout.

use clap::Parser;
use gh_docs_download::{
    cli::{Args, CliApp},
    error::Result,
};

fn main() -> Result<()> {
    let args = Args::parse();
    let app = CliApp::new(args);
    app.run()
}
