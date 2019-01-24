use std::{
    num::NonZeroU16,
    str::FromStr,
};

use crate::{
    Error,
    Result
};

#[cfg(test)]
mod unit_tests;
/// BoardDimension is the type representing the number of rows or columns of the gameboard.  By using a ranged type of
/// exactly the supported size, we make illegal states unrepresentable, and thus eliminate the need to write conditional
/// logic to handle them, write tests for them, and so on.  We also gain the compiler as an ally, ensuring at compile
/// time that necessary contracts are upheld.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct BoardDimension(NonZeroU16);

impl BoardDimension {
    /// Constructor.
    pub fn new(n: u16) -> Option<Self> {
        NonZeroU16::new(n)
            .map(Self)
    }

    /// Inner value Accessor.  As a dimension, `usize` is returned, rather than leaking information about the current
    /// implementation's inner type.
    pub fn get(self) -> usize {
        usize::from(self.0.get())
    }
}

impl FromStr for BoardDimension {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<u16>()? {
            n if n > 0 => Self::new(n)
                .ok_or_else(|| Error::ArgNotConvertibleToNonZeroUsize(n as usize)),
            err => Err(Error::ArgNotConvertibleToNonZeroUsize(err as usize)),
        }
    }
}

