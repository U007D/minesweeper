#![allow(clippy::option_unwrap_used)]
use super::*;

#[test]
fn new_with_1_row_and_2_cols_yields_1_x_2_board() {
    // setup
    let rows = NonZeroUsize::new(1).unwrap();
    let cols = NonZeroUsize::new(2).unwrap();
    let prob = Probability::new(0).unwrap();

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of rows
    assert_eq!(result.rows(), rows);

    // and it should return the expected number of columns
    assert_eq!(result.cols(), cols);
}

#[test]
fn new_with_mine_probability_0_yields_board_with_no_mines() {
    // setup
    let n_rows = 100;
    let n_cols = 100;
    let rows = NonZeroUsize::new(n_rows).unwrap();
    let cols = NonZeroUsize::new(n_cols).unwrap();
    let prob = Probability::new(0).unwrap();
    let expected_mine_count = ((rows.get() * cols.get()) as f64 * *prob) as usize;

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of mines
    assert_eq!(result.iter()
                     .flat_map(|col| col.iter_mut())
                     .filter(|cell| cell)
                     .count(),
               expected_mine_count);
}
