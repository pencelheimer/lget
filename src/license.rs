use std::fmt::Display;

use clap::ValueEnum;
use ureq::http::{Uri, uri::InvalidUri};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum License {
    // TODO(pencelheimer): add more licenses
    Unlicense,
    Mit,
    GPL3,
    Apache2,
    BSD3,
    BSD2,
}

impl License {
    pub const fn items() -> [(Self, Self, &'static str); 6] {
        [
            // TODO(pencelheimer): add description for licenses
            (Self::Unlicense, Self::Unlicense, ""),
            (Self::Mit, Self::Mit, ""),
            (Self::GPL3, Self::GPL3, ""),
            (Self::Apache2, Self::Apache2, ""),
            (Self::BSD3, Self::BSD3, ""),
            (Self::BSD2, Self::BSD2, ""),
        ]
    }
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unlicense => "Unlicense",
                Self::Mit => "MIT",
                Self::GPL3 => "GPL-3.0-only",
                Self::Apache2 => "Apache-2.0",
                Self::BSD3 => "BSD-3-Clause",
                Self::BSD2 => "BSD-2-Clause",
            }
        )
    }
}

impl TryFrom<License> for Uri {
    type Error = InvalidUri;

    fn try_from(license: License) -> Result<Self, Self::Error> {
        format!("https://raw.githubusercontent.com/spdx/license-list-data/main/text/{license}.txt")
            .parse()
    }
}
