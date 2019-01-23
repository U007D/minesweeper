#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use super::*;

#[test]
fn new_with_1_row_and_2_cols_yields_1_x_2_board() {
    // setup
    let rows = BoardDimension::new(1).unwrap();
    let cols = BoardDimension::new(2).unwrap();
    let prob = Probability::new(0).unwrap();

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of rows
    assert_eq!(result.rows(), rows.get());

    // and it should return the expected number of columns
    assert_eq!(result.cols(), cols.get());

    // and it should return the expected probability
    // no rounding error ∵ rounding not permitted on the probability set by the caller
    #[allow(clippy::float_cmp)]
        {
            assert_eq!(result.probability(), prob.get());
        }
}

#[test]
fn new_with_mine_probability_0_yields_board_with_no_mines() {
    // setup
    let n_rows = 100;
    let n_cols = 100;
    let rows = BoardDimension::new(n_rows).unwrap();
    let cols = BoardDimension::new(n_cols).unwrap();
    let prob = Probability::new(0).unwrap();
    let expected_mine_count = 0;

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of mines
    assert_eq!(result.iter()
                     .flat_map(|col| col.iter())
                     .filter(|&&cell| cell)
                     .count(),
               expected_mine_count);
}

#[test]
fn new_with_mine_probability_1_yields_board_with_all_mines() {
    // setup
    let n_rows = 3;
    let n_cols = 3;
    let rows = BoardDimension::new(n_rows).unwrap();
    let cols = BoardDimension::new(n_cols).unwrap();
    let prob = Probability::new(1).unwrap();
    // no overflow ∵ `rows` and `cols` are each range-limited per `BoardDimension` such that their product fits a u32.
    #[allow(clippy::integer_arithmetic)]
        let expected_mine_count = rows.get() * cols.get();

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of mines
    assert_eq!(result.iter()
                     .flat_map(|col| col.iter())
                     .filter(|&&cell| cell)
                     .count(),
               expected_mine_count);
}

#[test]
fn new_3_x_3_with_mine_probability_0_point_5_yields_board_with_4_mines() {
    // setup
    let n_rows = 3;
    let n_cols = 3;
    let rows = BoardDimension::new(n_rows).unwrap();
    let cols = BoardDimension::new(n_cols).unwrap();
    let prob = Probability::new(0.5).unwrap();
    let expected_mine_count = 4;

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(rows, cols, prob);

    // then it should return the expected number of mines
    assert_eq!(result.iter()
                     .flat_map(|col| col.iter())
                     .filter(|&&cell| cell)
                     .count(),
               expected_mine_count);
}
