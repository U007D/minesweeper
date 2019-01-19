use std::{
    ops::{
        Deref,
        DerefMut,
    },
    str::FromStr,
};

use crate::{
    Error,
    Result,
};

/// The `Probability` type constrains a value to be between 0.0 and 1.0 (inclusive) representing a mathematical
/// probability.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Probability(f64);

impl Probability {
    /// Constructor.
    pub fn new(n: impl Into<f64>) -> Result<Self> {
        match n.into() {
            bad_val if bad_val < 0.0 || bad_val > 1.0 => Err(Error::InvalidProbabilityRange(bad_val)),
            val => Ok(Probability(val)),
        }
    }

    /// Inner value Accessor.
    pub fn get(&self) -> f64 {
        self.0
    }
}

impl FromStr for Probability {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.parse::<f64>() {
            Ok(n) => Probability::new(n),
            Err(err) => Err(Error::ParseFloat(err)),
        }
    }
}

impl Deref for Probability {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Probability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
