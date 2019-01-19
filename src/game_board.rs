use std::num::NonZeroUsize;

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
    prob: Probability,
}

impl GameBoard {
    /// Game board constructor.
    pub fn new(rows: NonZeroUsize, cols: NonZeroUsize, prob: Probability) -> Self {
        Self {
            cells: Self::init_cells(rows, cols, prob),
            prob,
        }
    }

    fn init_cells(rows: NonZeroUsize, cols: NonZeroUsize, _prob: Probability) -> CellsVec {
        let dims = (rows.get(), cols.get());
        let vec = vec![vec![false; dims.1]; dims.0];
        vec
    }

    /// Returns the number of game board columns
    #[inline]
    pub fn cols(&self) -> usize {
        // Weak change insurance
        dbg_assert!(self.rows() > 0);
        // Game board constructor precludes 0-sized dimensions
        #[allow(unsafe)]
            unsafe { self.cells.get_unchecked(0).len() }
    }

    /// Returns the number of game board rows
    #[inline]
    pub fn rows(&self) -> usize {
        self.cells.len()
    }
}
