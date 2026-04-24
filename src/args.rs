use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

    /// Optional subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    #[command(hide = true)]
    Completions {
        // TODO(pencelheimer): add nushell support via
        // https://docs.rs/clap_complete_nushell/4.6.0/clap_complete_nushell
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
