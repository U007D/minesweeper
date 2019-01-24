use structopt::StructOpt;

use crate::{
    BoardDimension,
    Probability,
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
    /// Number of game board columns
    pub cols: BoardDimension,

    /// Number of game board rows
    pub rows: BoardDimension,

    /// Probability of a mine in each cell
    pub prob: Probability,
}
