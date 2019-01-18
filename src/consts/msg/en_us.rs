/// Supplied argument (as `OsStr`) cannot be converted to a `String` (ie. `UTF-8`).
pub const ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8: &str = "Error: supplied command-line argument not convertible to UTF-8";
/// Supplied argument cannot be converted to a NonZeroUsize.  Examples include floating point numbers, negative
/// numbers or 0.
pub const ERR_ARG_NOT_CONVERTIBLE_TO_NON_ZERO_USIZE: &str = "Error: supplied command-line argument not convertible to `NonZeroUsize`";
/// Supplied argument could not be recognized as an integer.  Examples include "hello" and "12j5."
pub const ERR_PARSE_INT: &str = "Error parsing integer value";
