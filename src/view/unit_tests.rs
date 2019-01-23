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
fn new_yields_view_which_display_only_hidden_cells() {
// setup
    let rows = BoardDimension::new(2).unwrap();
    let cols = BoardDimension::new(3).unwrap();
    let prob = Probability::new(0.34).unwrap();
    let model = GameBoard::new(rows, cols, prob);
    let reader = StdInMock::new(Vec::<&str>::new());
    let mut expected_cells = Vec::with_capacity(model.rows() * model.cols());
    model.iter()
         .flat_map(|vec| vec.iter())
         .for_each(|&v| expected_cells.push(v));
    let mut expected_string = String::from(" |123\n-+---\n");
    (1..=model.rows()).for_each(|r| {
        expected_string += &(r.to_string() + "|");
        (1..=model.cols()).for_each(|_c| expected_string += " ");
        expected_string += "\n";
    });

// given a constructor
    let sut = View::new;

// when it is invoked
    let result = sut(model, reader);

// then the resulting instance should contain the expected model
    assert_eq!(format!("{}", result), expected_string);
}
