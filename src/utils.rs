use std::path::Path;

use clap::CommandFactory as _;

use crate::{args::CliArgs, error::CliError, license::License};

pub fn get_license(license_opt: Option<License>, quiet: bool) -> Result<License, CliError> {
    match license_opt {
        Some(license) => Ok(license),
        None if quiet => Err(CliError::QuietNoLicense),
        None => cliclack::select("Pick a License")
            .initial_value(License::Unlicense)
            .items(License::items().as_ref())
            .interact()
            .map_err(Into::into),
    }
}

pub fn check_overwrite(path: impl AsRef<Path>, force: bool, quiet: bool) -> Result<(), CliError> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(());
    }

    if force {
        return Ok(());
    } else if quiet {
        return Err(CliError::QuietNeedsForce);
    }

    let overwrite = cliclack::confirm(format!(
        "File '{}' already exists. Overwrite?",
        path.display()
    ))
    .interact()?;

    if !overwrite {
        return Err(CliError::interrupt());
    }

    Ok(())
}

pub fn fetch_and_write(
    license: License,
    path: impl AsRef<Path>,
    quiet: bool,
) -> Result<(), CliError> {
    let path = path.as_ref();

    let spinner = if quiet {
        None
    } else {
        Some(cliclack::spinner())
    };

    if let Some(s) = &spinner {
        s.start("Fetching license from SPDX...");
    }

    let text = ureq::get(license).call()?.body_mut().read_to_string()?;

    if let Some(s) = &spinner {
        s.set_message("Writing the license into a file");
    }

    std::fs::write(path, text)?;

    if let Some(s) = &spinner {
        s.stop(format!("Successfully wrote to '{}'", path.display()));
    }

    Ok(())
}

pub fn gen_shell_completions(shell: impl clap_complete::Generator) {
    let mut cmd = CliArgs::command();
    let bin_name = env!("CARGO_PKG_NAME");
    let mut buf = std::io::stdout();

    clap_complete::generate(shell, &mut cmd, bin_name, &mut buf);
}
