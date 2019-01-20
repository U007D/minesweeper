use indexmap::IndexSet;
use rand::{
    Rng,
    thread_rng,
};
use std::{
    num::NonZeroUsize,
    slice::Iter,
};

use crate::{
    consts::*,
    Probability,
};

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
        // TODO: set max at min(usize_bits, f64_mantissa_bits) / 2
        // 2^26 each to prevent product overflowing f64's 52-bit mantissa
        if rows.get() > 2_usize.pow(26) || cols.get() > 2_usize.pow(26) { panic!("board overflow"); }
        Self {
            cells: Self::init_cells(rows, cols, prob),
            rows,
            cols,
            prob,
        }
    }

    fn init_cells(rows: NonZeroUsize, cols: NonZeroUsize, prob: Probability) -> CellsVec {
        let rows_inner = rows.get();
        let cols_inner = cols.get();
        // no overflow ∵ `rows_inner` & `cols_inner` max at 2^16 or 2^26 each (depending on usize).
        #[allow(clippy::integer_arithmetic)]
        let cell_count = rows_inner * cols_inner;

        let mut cells = vec![vec![false; cols_inner]; rows_inner];
        // no overflow ∵ `rows` and `cols` are each range limited to half of an `f64`'s 52-bit mantissa
        #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let mine_count = ((rows_inner * cols_inner) as f64 * *prob) as usize;
        let mut rng = thread_rng();
        let mut unmined = IndexSet::with_capacity(cell_count);
        (0..cell_count).for_each(|i| { unmined.insert(i); });

        (0..mine_count).for_each(|_i| {
            let idx = rng.gen_range(0, unmined.len());
            let val = *unmined.get_index(idx).expect(msg::ERR_INTERNAL_UNMINED_INDEX_NOT_FOUND);
            unmined.remove(&val);

            // no divide by zero ∵ `cols_inner` is derived from `cols` which is `NonZeroUsize`
            #[allow(clippy::integer_arithmetic)]
            {
                let row = idx / cols_inner;
                let col = idx % cols_inner;
                // no out of bounds ∵ `row` & `col` indices derived from game board dimensions (ie. get_unchecked() OK)
                #[allow(clippy::indexing_slicing)]
                {
                    cells[row][col] = true;
                }
            }
        });

        cells
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
