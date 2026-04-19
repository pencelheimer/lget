mod args;
mod error;
mod license;
mod utils;

use std::process::ExitCode;

use anyhow::Result;
use clap::Parser as _;

use crate::{args::CliArgs, error::CliError};

fn run(args: CliArgs) -> Result<(), CliError> {
    let license = utils::get_license(args.license, args.quiet)?;
    utils::check_overwrite(&args.output, args.force, args.quiet)?;
    utils::fetch_and_write(license, &args.output, args.quiet)?;

    Ok(())
}

fn main() -> ExitCode {
    let args = CliArgs::parse();
    let quiet = args.quiet;

    // NOTE(pencelheimer): unsetting default handler for cliclack
    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    if let Err(e) = run(args) {
        if e.is_interrupted() {
            if !quiet {
                let _ = cliclack::log::error("Aborted");
            }

            return ExitCode::SUCCESS;
        }

        let _ = cliclack::log::error(e);
        return ExitCode::FAILURE;
    }

    if !quiet {
        let _ = cliclack::log::success("Done");
    }

    ExitCode::SUCCESS
}
