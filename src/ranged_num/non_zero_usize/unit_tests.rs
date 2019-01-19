use super::*;

#[test]
fn new_with_0_value_fails() {
    // setup
    let invalid_value = "0";
    // given a constructor
    let sut = NonZeroUsize::from_str;

    // when invoked with an invalid value
    let result = sut(invalid_value);

    // then the result should be an error
    assert_eq!(result, Err(Error::ArgNotConvertibleToNonZeroUsize(invalid_value.parse::<usize>().unwrap())));
}

#[test]
fn new_with_1_value_succeeds() {
    // setup
    let value = "1";
    // given a constructor
    let sut = NonZeroUsize::from_str;

    // when invoked with a valid value
    let result = sut(value);

    // then the result should succeed
    assert_eq!(result, Ok(NonZeroUsize(num::NonZeroUsize::new(value.parse::<usize>().unwrap()).unwrap())));
}

