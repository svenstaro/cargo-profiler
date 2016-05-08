use std::error;
use std::fmt;


#[derive(Debug)]
/// Represents potential errors that may occur when performing lookups
pub enum ProfError {
    /// Represents Network errors, including access violations to the GSBL PI
    /// For when greater than gsbrs::url_limit urls are queried
    CallGrindError(String),
    /// Wraps a std::io::Error
    RegexError(String),


}

impl fmt::Display for ProfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfError::CallGrindError(ref e) => write!(f, "Callgrind error {}",e),
            ProfError::RegexError(ref line) => write!(f, "Regex error: {}", line),
        }
    }
}

impl error::Error for ProfError {
    fn description(&self) -> &str {
        match *self {
            ProfError::CallGrindError(_) => "Callgrind Error. File Bug.",
            ProfError::RegexError(_) => "Regex error. file bug.",

        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ProfError::CallGrindError(_)=> None,
            ProfError::RegexError(_) => None,
        }
    }
}
