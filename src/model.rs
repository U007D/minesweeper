use std::num::NonZeroUsize;

#[cfg(test)]
mod unit_tests;
/// This struct represents the game board.  It contains the board dimensions and the locations of all mines.
#[derive(Debug)]
pub struct Model {
    rows: NonZeroUsize,
    cols: NonZeroUsize,
}

impl Model {
    /// Game board constructor.
    pub fn new(rows: NonZeroUsize, cols: NonZeroUsize) -> Self {
        Self {
            rows,
            cols,
        }
    }
}
