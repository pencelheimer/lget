use std::path::PathBuf;

use clap::Parser;

use crate::license::License;

#[derive(Parser)]
pub struct CliArgs {
    /// Output file name
    #[arg(short, default_value = "LICENSE")]
    pub output: PathBuf,

    /// License list to choose from
    #[arg(short)]
    pub license: Option<License>,

    /// Overwrite existing file without prompting
    #[arg(short, long)]
    pub force: bool,

    /// Suppress all output (requires -l, fails if prompt is needed)
    #[arg(short, long)]
    pub quiet: bool,
}
