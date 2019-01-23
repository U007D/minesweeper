use std::{
    io::{
        BufRead,
        Error as IoError,
        Read,
    },
};

use crate::StdResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdInMock {
    strings: Vec<String>,
    row_idx: Option<usize>,
    row_len: usize,
    col: usize,
}

impl StdInMock {
    pub fn new<T>(mut data: Vec<T>) -> Self where T: Clone + Into<String> {
        let strings = data.drain(..)
                          .map(|s| s.into())
                          .collect::<Vec<_>>();
        let row_idx = match !strings.is_empty() {
            true => Some(0),
            false => None,
        };
        // no out-of-bounds indexing ∵ `idx` is 0, iff `!strings.is_empty()`, `None` otherwise
        #[allow(clippy::indexing_slicing)]
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
    fn read(&mut self, buf: &mut [u8]) -> StdResult<usize, IoError> {
        match self.fill_buf() {
            Ok(b) => {
                buf.clone_from_slice(b);
                Ok(b.len())
            },
            Err(e) => Err(e),
        }
    }
}

impl BufRead for StdInMock {
    fn fill_buf(&mut self) -> StdResult<&[u8], IoError> {
        // *1 - no out-of-bounds indexing ∵ `self.col` is match guarded to be < `self.row_len`
        // *2 - no out-of-bounds indexing ∵ `i` is bounded by a `checked_add()` and `and_then()` bounds check
        // see *'s below; attribute non-local ∵ as of 1.32.0, attributes on expressions are experimental
        #[allow(clippy::indexing_slicing)]
            match self.row_idx {
            Some(row) => match row < self.strings.len() {
                true => match self.col < self.row_len {
                    // *1
                    true => Ok(&self.strings[row].as_bytes()[self.col..]),
                    false => {
                        self.row_idx = row.checked_add(1)
                                          .and_then(|n| match n < self.strings.len() {
                                              true => Some(n),
                                              false => None,
                                          });
                        // *2
                        self.row_len = self.row_idx
                                           .map_or(0, |i| self.strings[i].len());
                        self.col = 0;
                        self.fill_buf()
                    },
                },
                false => {
                    self.row_idx = None;
                    self.fill_buf()
                },
            },
            None => Ok(&[]),
        }
    }

    fn consume(&mut self, amt: usize) {
        self.col = match self.col.saturating_add(amt) {
            col if col >= self.row_len => self.row_len,
            col => col,
        }
    }
}
