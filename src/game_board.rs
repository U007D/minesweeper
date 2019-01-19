use std::{
    num::NonZeroUsize,
    slice::Iter,
};

use crate::Probability;

// Alias game board storage type to facilitate possible future optimizations
// (e.g. [bit-vec](https://crates.io/crates/bit-vec))
type CellsVec = Vec<Vec<bool>>;

#[cfg(test)]
mod unit_tests;
/// This struct represents the game board.  It contains the board dimensions and the locations of all mines.
#[derive(Debug)]
pub struct GameBoard {
    cells: CellsVec,
    rows: NonZeroUsize,
    cols: NonZeroUsize,
    prob: Probability,
}

impl GameBoard {
    /// Game board constructor.
    pub fn new(rows: NonZeroUsize, cols: NonZeroUsize, prob: Probability) -> Self {
        // TODO: Replace range-check with ranged type
        if rows.get() > 2_usize.pow(26) || cols.get() > 2_usize.pow(26) { panic!("board overflow"); }
        Self {
            cells: Self::init_cells(rows, cols, prob),
            rows,
            cols,
            prob,
        }
    }

    fn init_cells(rows: NonZeroUsize, cols: NonZeroUsize, _prob: Probability) -> CellsVec {
        let dims = (rows.get(), cols.get());
        let vec = vec![vec![false; dims.1]; dims.0];
        vec
    }

    /// Create ref iterator over game board rows
    pub fn iter(&self) -> Iter<Vec<bool>> {
        self.cells.iter()
    }

    /// Returns the number of game board columns
    #[inline]
    pub fn columns(&self) -> NonZeroUsize { self.cols }

    /// Returns the probability of a mine in a given cell setting used to initialize the game board
    #[inline]
    pub fn probability(&self) -> Probability { self.prob }

    /// Returns the number of game board rows
    #[inline]
    pub fn rows(&self) -> NonZeroUsize {
        self.rows
    }
}
