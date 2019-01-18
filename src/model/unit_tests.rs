#![allow(clippy::option_unwrap_used)]
use super::*;

#[test]
fn new_with_1_row_and_2_cols_yields_expected_board() {
    // setup
    let rows = NonZeroUsize::new(1).unwrap();
    let cols = NonZeroUsize::new(2).unwrap();

    // given a `Model` constructor
    let sut = Model::new;

    // when it is invoked
    let result = sut(rows, cols);

    // then it should return the expected number of rows
    assert_eq!(result.rows, rows);

    // and it should return the expected number of columns
    assert_eq!(result.cols, cols);
}
