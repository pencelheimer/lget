use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Net(#[from] ureq::Error),

    #[error("Cannot prompt for input in quiet mode. Please provide '--license' explicitly.")]
    QuietNoLicense,

    #[error(
        "File exists. Cannot ask for overwrite permission in quiet mode. Use '--force' to overwrite."
    )]
    QuietNeedsForce,
}

impl CliError {
    pub fn is_interrupted(&self) -> bool {
        matches!(self, Self::Io(err) if err.kind() == IoErrorKind::Interrupted)
    }

    pub fn interrupt() -> Self {
        Self::Io(IoError::from(IoErrorKind::Interrupted))
    }
}
