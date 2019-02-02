/// Supplied argument (as `OsStr`) cannot be converted to a `String` (ie. `UTF-8`).
pub const ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8: &str = "Error: Supplied command-line argument not convertible to UTF-8";
/// Supplied argument cannot be converted to a NonZeroUsize.  Examples include floating point numbers, negative
/// numbers or 0.
pub const ERR_ARG_NOT_CONVERTIBLE_TO_NON_ZERO_USIZE: &str = "Error: Supplied command-line argument not convertible to `NonZeroUsize`";
/// Supplied argument could not be recognized as an integer.  Examples include "hello" and "12j5."
pub const ERR_PARSE_INT: &str = "Error parsing integer value";
/// IO error encountered"
pub const ERR_IO: &str = "IO error";
/// Supplied argument could not be recognized as an floating point value.  Examples include "hello" and "12j5."
pub const ERR_PARSE_FLOAT: &str = "Error parsing floating point value";
/// Supplied value could not be converted into a probability (0.0 <= n <= 1.0).
pub const ERR_INVALID_PROBABILITY_RANGE: &str = "Error: Supplied value not convertible into a proability (a value from 0.0 to 1.0, inclusive)";
/// Index of unmined cells cannot be found
pub const ERR_INTERNAL_UNMINED_INDEX_NOT_FOUND: &str = "Internal error: Index of unmined cells not found";
/// Base-10 digit (0..=9) required, but encountered value outside of expected range
pub const ERR_INTERNAL_DIGIT_NOT_BASE_10: &str = "Internal error: Non-base-10 digit encountered";
