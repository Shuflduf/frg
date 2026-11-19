use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Args)]
pub struct BuildArgs {
    file: String,
}

#[derive(Args)]
pub struct RunArgs {
    pub file: PathBuf,
}

/// The frg programming language
#[derive(Parser)]
pub enum FrgArgs {
    /// [TODO] Build a file into a static binary
    Build(BuildArgs),
    /// Build and run a file
    Run(RunArgs),
    /// [DEBUG] Used internally for testing
    Test,
}
