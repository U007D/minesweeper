use std::io::Error as StdIoError;

#[derive(Debug)]
pub struct IoError(StdIoError);

impl PartialEq for IoError {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.kind() == rhs.0.kind()
    }
}
