use std::fmt::{self};

use std::io::Error;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
// Resource: https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileError {
    message: String,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in handling the file.")
    }
}

impl From<Error> for FileError {
    fn from(error: Error) -> Self {
        FileError {
            message: format!("{}", error),
        }
    }
}
