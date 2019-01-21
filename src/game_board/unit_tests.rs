#![allow(clippy::option_unwrap_used, clippy::result_unwrap_used)]
use super::*;

#[test]
fn new_with_1_row_and_2_cols_yields_1_x_2_board() {
    // setup
    let expected_rows = NonZeroUsize::new(1).unwrap();
    let expected_cols = NonZeroUsize::new(2).unwrap();
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
    let rows = NonZeroUsize::new(n_rows).unwrap();
    let cols = NonZeroUsize::new(n_cols).unwrap();
    let prob = Probability::new(0).unwrap();
    // `rows` and `cols` are each range limited to half of an `f64`'s 52-bit mantissa; `prob` is range limited (0..=1)
    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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
    let rows = NonZeroUsize::new(n_rows).unwrap();
    let cols = NonZeroUsize::new(n_cols).unwrap();
    let prob = Probability::new(1).unwrap();
    // `rows` and `cols` are each range limited to half of an `f64`'s 52-bit mantissa; `prob` is range limited (0..=1)
    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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
    let rows = NonZeroUsize::new(n_rows).unwrap();
    let cols = NonZeroUsize::new(n_cols).unwrap();
    let prob = Probability::new(0.5).unwrap();
    // `rows` and `cols` are each range limited to half of an `f64`'s 52-bit mantissa; `prob` is range limited (0..=1)
    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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
