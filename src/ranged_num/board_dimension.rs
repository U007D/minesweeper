use std::{
    num::NonZeroU16,
    ops::Deref,
    u16,
};

use crate::consts::*;

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
        match n {
            1..=u16::MAX => Some(Self(NonZeroU16::new(n)
                .expect(msg::ERR_ARG_NOT_CONVERTIBLE_TO_NON_ZERO_USIZE))),
            _ => None,
        }
    }
}

impl Deref for BoardDimension {
    type Target = NonZeroU16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

