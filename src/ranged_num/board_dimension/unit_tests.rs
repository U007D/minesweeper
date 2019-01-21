#![allow(clippy::option_unwrap_used)]
use super::*;

#[test]
fn new_with_0_yields_none() {
    // given a constructor
    let sut = BoardDimension::new;

    // when invoked with 0
    let result = sut(0);

    // then the result should be `None`
    assert_eq!(result, None);
}

#[test]
fn new_with_1_yields_a_board_dimension() {
    // setup
    let expected_result = Some(BoardDimension(NonZeroU16::new(1).unwrap()));
    // given a constructor
    let sut = BoardDimension::new;

    // when invoked with 1
    let result = sut(1);

    // then the result should be `None`
    assert_eq!(result, expected_result);
}
