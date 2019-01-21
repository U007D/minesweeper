use std::{
    num::NonZeroUsize,
    slice::Iter,
};

use rand::{
    Rng,
    thread_rng,
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
    rows_nz: NonZeroUsize,
    cols_nz: NonZeroUsize,
    prob: Probability,
}

impl GameBoard {
    /// Game board constructor.
    pub fn new(rows_nz: NonZeroUsize, cols_nz: NonZeroUsize, prob: Probability) -> Self {
        let rows = rows_nz.get();
        let cols = cols_nz.get();
        // TODO: Replace range-check with ranged type
        // TODO: set max at min(usize_bits, 0_u32.count_zeros()) / 2
        // 2^16 each to prevent product overflowing usize == u32 (and f64's 52-bit mantissa)
        if rows > 2_usize.pow(16) || cols > 2_usize.pow(16) { panic!("board overflow"); }
        Self {
            cells: Self::init_cells(vec![vec![false; cols]; rows], prob),
            rows_nz,
            cols_nz,
            prob,
        }
    }

    fn init_cells(mut cells: CellsVec, prob: Probability) -> CellsVec {
        let rows = cells.len();
        // no out-of-bounds indexing ∵ `rows` must be > 0 as per `NonZeroUsize`.
        #[allow(clippy::indexing_slicing)]
            let cols = cells[0].len();
        let mut rng = thread_rng();

        // no precision loss or overflow ∵ `rows` & `cols` are limited to 16 bits and prob (0..=1).
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
            let mine_count = (rows as f64 * cols as f64 * *prob) as usize;
        (0..mine_count).for_each(|_| {
            let mut row: usize;
            let mut col: usize;
            // place a mine in a randomly selected cell which must not already contain a mine.
            // If that cell does already contain a mine, reassign this mine to another randomly selected cell, until
            // one is found which does not already contain a mine.
            while {
                row = rng.gen_range(0, rows);
                col = rng.gen_range(0, cols);

                // no out-of-bounds indexing  ∵ `row` & `col` are bound by `rows` & `cols.
                #[allow(clippy::indexing_slicing)]
                {
                    match &mut cells[row][col] {
                        // already has a mine; select another cell for this mine
                        true => true,
                        // cell contains no pre-existing mine: assign the mine to this cell and
                        // exit the reassignment while loop
                        val => {
                            *val = true;
                            false
                        }
                    }
                }
            } {}
        });
        cells
    }

    /// Create ref iterator over game board rows
    pub fn iter(&self) -> Iter<Vec<bool>> {
        self.cells.iter()
    }

    /// Returns the number of game board columns
    #[inline]
    pub fn columns(&self) -> NonZeroUsize { self.cols_nz }

    /// Returns the probability of a mine in a given cell setting used to initialize the game board
    #[inline]
    pub fn probability(&self) -> Probability { self.prob }

    /// Returns the number of game board rows
    #[inline]
    pub fn rows(&self) -> NonZeroUsize {
        self.rows_nz
    }
}
