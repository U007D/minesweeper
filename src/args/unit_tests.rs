use super::*;

#[test]
fn from_iter_with_0_rows_fails_to_yield_args_struct() {
    // setup
    let app = "test 0 3";
    let rows = "0";
    let cols = "3";
    let args = vec![app, rows, cols];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}

#[test]
fn from_iter_with_0_cols_fails_to_yield_args_struct() {
    // setup
    let app = "test 3 0";
    let rows = "3";
    let cols = "0";
    let args = vec![app, rows, cols];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}

#[test]
fn from_iter_with_0_rows_and_0_cols_fails_to_yield_args_struct() {
    // setup
    let app = "test 0 0";
    let rows = "0";
    let cols = "0";
    let args = vec![app, rows, cols];

    // given a `StructOpt::from_iter()` constructor
    let sut = Args::from_iter_safe::<&Vec<&str>>;

    // when it is invoked
    let result = sut(&args);

    // then it should return an error
    assert_eq!(result.is_err(), true);
}
