#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use std::{
    io::{
        BufRead,
        Error as IoError,
        ErrorKind as IoErrorKind,
        Read,
    },
};

use crate::{
    BoardDimension,
    GameBoard,
    Probability,
    StdResult,
};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct StdInMock {
    strings: Vec<String>,
    row_idx: Option<usize>,
    row_len: usize,
    col: usize,
}

impl StdInMock {
    fn new(strings: Vec<String>) -> Self {
        let row_idx = match strings.len() > 0 {
            true => Some(0),
            false => None,
        };
        let row_len = match row_idx {
            Some(idx) => strings[idx].len(),
            None => 0,
        };
        Self {
            strings,
            row_idx,
            row_len,
            col: 0,
        }
    }
}

impl Read for StdInMock {
    fn read(&mut self, _buf: &mut [u8]) -> StdResult<usize, IoError> {
        unimplemented!()
    }
}

impl BufRead for StdInMock {
    fn fill_buf(&mut self) -> StdResult<&[u8], IoError> {
        match self.row_idx {
            Some(row) => match row < self.strings.len() {
                true => match col < self.row_len {
                    // no out-of-bounds indexing ∵ `self.col` is match guarded to be < `self.row_len`
                    #[allow(clippy::indexing_slicing)]
                    true => Ok(&self.strings[row].as_bytes()[self.col..]),
                    false => {
                        self.row_idx = row.checked_add(1)
                                          .and_then(|n| match n < self.strings.len() {
                                              true => Some(n),
                                              false => None,
                                          });
                        // no out-of-bounds indexing ∵ `i` is bounded by a `checked_add()` and `and_then()` bounds check
                        #[allow(clippy::indexing_slicing)]
                            self.row_len = self.row_idx
                                               .map_or(0, |i| self.strings[i].len());
                        self.col = 0;
                        fill_buf()
                    },
                },
                false => {
                    self.row_idx = None;
                    fill_buf()
                },
            },
            None => Ok(&[]),
        }
    }

    fn consume(&mut self, amt: usize) {
        // no out-of-bounds indexing ∵ `StdInMock::fill_buf()` and `amt` are match guarded
        // no arithmetic overflow ∵ `
        # [allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
//        match amt >= self.strings[self.row_idx].len() - self.col {
//            true => match self.row_idx == 0 {
//                true => self.col = 0,
//                false => self.col = self.strings[self.col - 1].len(),
//            },
//            false => self.col += amt,
    }
}
}

# [test]
fn new_yields_view_in_expected_state() {
// setup
let rows = BoardDimension::new(2).unwrap();
let cols = BoardDimension::new(3).unwrap();
let prob = Probability::new(0.34).unwrap();
let model = GameBoard::new(rows, cols, prob);
let mut expected_cells = Vec::with_capacity(model.rows() * model.columns());
model.iter()
.flat_map( | vec | vec.iter())
.for_each( | & v| expected_cells.push(v));
let expected_reader = StdInMock::new(vec ! [String::from("hello"), String::from("world!")]);

// given a constructor
let sut = View::new;

// when it is invoked
let result = sut(model, expected_reader.clone());

// then the resulting instance should contain the expected model
assert_eq ! (result.model
.iter()
.flat_map( |vec | vec.iter())
.zip(expected_cells.iter())
.all(| (result, expected) | result == expected),
true);

// and the resulting instance should contain the expected reader
assert_eq ! (result.reader, expected_reader);
}
