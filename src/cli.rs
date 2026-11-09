use clap::{Args, Parser};

#[derive(Args)]
pub struct BuildArgs {
    file: String,
}

#[derive(Args)]
pub struct RunArgs {
    file: String,
}

/// The frg programming language
#[derive(Parser)]
pub enum FrgArgs {
    Build(BuildArgs),
    Run(RunArgs),
    Test,
}
