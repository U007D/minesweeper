use super::*;

#[test]
fn new_with_negative_zero_point_five_value_fails() {
    // setup
    let value = -0.5;

    // given a constructor
    let sut = Probability::new;

    // when invoked with an invalid value
    let result = sut(value);

    // then the result should be an error
    assert_eq!(result, Error::OutsideProbabilityRange(value));
}

#[test]
fn new_with_one_point_one_value_fails() {
    // setup
    let value = 1.1;

    // given a constructor
    let sut = Probability::new;

    // when invoked with an invalid value
    let result = sut(value);

    // then the result should be an error
    assert_eq!(result, Error::OutsideProbabilityRange(value));
}

#[test]
fn new_with_zero_value_succeeds() {
    // setup
    let value = 0;

    // given a constructor
    let sut = Probability::new;

    // when invoked with a valid value
    let result = sut(value);

    // then the result should be a `Probability`
    assert_eq!(result, Ok(Probability(value)));
}

#[test]
fn new_with_one_value_succeeds() {
    // setup
    let value = 1;

    // given a constructor
    let sut = Probability::new;

    // when invoked with a valid value
    let result = sut(value);

    // then the result should be a `Probability`
    assert_eq!(result, Ok(Probability(value)));
}

#[test]
fn get_returns_expected_value() {
    // setup
    let expected_value = 0.42;

    // given a `Probability` initialized to `expected_value`
    let prob = Probability::new(expected_value).unwrap();

    // when `get()` is queried
    let result = prob.get();

    // then the result should be the expected value
    assert_eq!(result, expected_value);
}

