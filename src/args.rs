use structopt::StructOpt;

use std::{
    num,
    ops::{
        Deref,
        DerefMut,
    },
    str::FromStr,
};

use crate::{
    Error,
    Result
};

#[cfg(test)]
mod unit_tests;

/// Args is a structure representing the user's supplied command-line arguments supplied to the program as an
/// initialized data structure.  Types within the data structure are defined to be as restrictive as possible to
/// ensure only valid inputs are accepted.  Where types accurately reflect the game-state constraints, conditional
/// logic, validation code and tests can all be safely elminiated, as the invalid states of concern are not
/// representable.
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// Number of game board rows
    pub rows: NonZeroUsize,

    /// Number of game board columns
    pub cols: NonZeroUsize,
}

/// std::num::NonZeroUsize does not have a `FromStr` impl, which is required by `{integer}::parse()`, which `structopt`
/// uses to populate the `StructOpt` structure.
/// Rust's Orphan Rules prevent a crate from implementing a trait it does not declare on a type it does
/// not declare (i.e. `minesweeper` cannot implement `FromStr` for `NonZeroUsize` since it declares neither).
/// As a result of these two facts, we employ the NewType pattern to declare a... ahem... new type, so that we do not
/// run afoul of the Orphan Rules.  Note that the NewType pattern is zero runtime overhead (ZRO).
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct NonZeroUsize(num::NonZeroUsize);

impl FromStr for NonZeroUsize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<usize>()? {
            n if n > 0 => Ok(Self(num::NonZeroUsize::new(n)
                                                    .ok_or_else(|| Error::ArgNotConvertibleToNonZeroUsize(n))?)),
            err => Err(Error::ArgNotConvertibleToNonZeroUsize(err)),
        }
    }
}

impl Deref for NonZeroUsize {
    type Target = num::NonZeroUsize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonZeroUsize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
