use std::{
    ffi::OsString as FfiOsString,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

use derive_more::From;

#[derive(Clone, Debug, Eq, From, Ord, PartialEq, PartialOrd, Hash)]
pub struct OsString(FfiOsString);

impl Display for OsString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0.to_string_lossy())
    }
}
