#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use super::*;

#[test]
fn new_with_1_row_and_2_cols_yields_1_x_2_board() {
    // setup
    let expected_rows = BoardDimension::new(1).unwrap();
    let expected_cols = BoardDimension::new(2).unwrap();
    let expected_prob = Probability::new(0).unwrap();

    // given a `GameBoard` constructor
    let sut = GameBoard::new;

    // when it is invoked
    let result = sut(expected_rows, expected_cols, expected_prob);

    // then it should return the expected number of rows
    assert_eq!(result.rows(), expected_rows);

    // and it should return the expected number of columns
    assert_eq!(result.columns(), expected_cols);

    // and it should return the expected probability
    assert_eq!(result.probability(), expected_prob);
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
    // `rows` and `cols` are each range-limited per `BoardDimension` and their product cannot overflow a `u32`.
    #[allow(clippy::integer_arithmetic)]
        let expected_mine_count = usize::from(rows.get()) * usize::from(cols.get());

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
