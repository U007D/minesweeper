#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use stdin_mock::StdInMock;

use crate::{
    BoardDimension,
    GameBoard,
    Probability,
};

use super::*;

mod stdin_mock;
# [test]
fn new_yields_view_in_expected_state() {
// setup
    let rows = BoardDimension::new(2).unwrap();
    let cols = BoardDimension::new(3).unwrap();
    let prob = Probability::new(0.34).unwrap();
    let model = GameBoard::new(rows, cols, prob);
    let mut expected_cells = Vec::with_capacity(model.rows() * model.columns());
    model.iter()
         .flat_map(|vec| vec.iter())
         .for_each(|&v| expected_cells.push(v));
    let expected_reader = StdInMock::new(vec!["hello", "world"]);

// given a constructor
    let sut = View::new;

// when it is invoked
    let result = sut(model, expected_reader.clone());

// then the resulting instance should contain the expected model
    assert_eq!(result.model
                     .iter()
                     .flat_map(|vec| vec.iter())
                     .zip(expected_cells.iter())
                     .all(|(result, expected)| result == expected),
               true);

// and the resulting instance should contain the expected reader
    assert_eq!(result.reader, expected_reader);
}
