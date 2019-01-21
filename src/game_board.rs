use std::slice::Iter;

use rand::{
    Rng,
    thread_rng,
};

use crate::{
    BoardDimension,
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
    rows: BoardDimension,
    cols: BoardDimension,
    prob: Probability,
}

impl GameBoard {
    /// Game board constructor.
    pub fn new(rows: BoardDimension, cols: BoardDimension, prob: Probability) -> Self {
        let n_rows = usize::from((*rows).get());
        let n_cols = usize::from((*cols).get());
        Self {
            cells: Self::init_cells(vec![vec![false; n_cols]; n_rows], prob),
            rows,
            cols,
            prob,
        }
    }

    fn init_cells(mut cells: CellsVec, prob: Probability) -> CellsVec {
        let n_rows = cells.len();
        // no out-of-bounds indexing ∵ `n_rows` must be > 0 as per `NonZeroUsize`.
        #[allow(clippy::indexing_slicing)]
            let n_cols = cells[0].len();
        let mut rng = thread_rng();

        // no precision loss or overflow ∵ `n_rows` & `n_cols` are limited to 16 bits per `BoardDimension` and prob (0..=1).
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
            let mine_count = (n_rows as f64 * n_cols as f64 * *prob) as usize;
        (0..mine_count).for_each(|_| {
            let mut row_idx: usize;
            let mut col_idx: usize;
            // place a mine in a randomly selected cell which must not already contain a mine.
            // If that cell does already contain a mine, reassign this mine to another randomly selected cell, until
            // one is found which does not already contain a mine.
            while {
                row_idx = rng.gen_range(0, n_rows);
                col_idx = rng.gen_range(0, n_cols);

                // no out-of-bounds indexing  ∵ `row_idx` & `col_idx` are bound by `n_rows` & `n_cols.
                #[allow(clippy::indexing_slicing)]
                {
                    match &mut cells[row_idx][col_idx] {
                        // already has a mine; select another cell for this mine
                        true => true,
                        // cell contains no pre-existing mine: assign the mine to this cell and
                        // exit the reassignment while loop
                        cell => {
                            *cell = true;
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
    pub fn columns(&self) -> BoardDimension { self.cols }

    /// Returns the probability of a mine in a given cell setting used to initialize the game board
    #[inline]
    pub fn probability(&self) -> Probability { self.prob }

    /// Returns the number of game board rows
    #[inline]
    pub fn rows(&self) -> BoardDimension { self.rows }
}
