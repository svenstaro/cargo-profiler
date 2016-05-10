use std::error;
use std::fmt;
use std::io::Error as ioError;


#[derive(Debug)]
/// Represents potential errors that may occur when performing lookups
pub enum ProfError {
    /// Represents Network errors, including access violations to the GSBL PI
    /// For when greater than gsbrs::url_limit urls are queried
    CallGrindError(String),
    /// Wraps a std::io::Error
    RegexError,
    InvalidProfiler,
    InvalidBinary,
    InvalidNum,
    InvalidSortMetric,
    /// Wraps a std::io::Error
    IOError(ioError),
}

impl fmt::Display for ProfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfError::CallGrindError(ref e) => write!(f, "Callgrind error {}", e),
            ProfError::RegexError => write!(f, "Regex error. File bug."),
            ProfError::InvalidProfiler => write!(f, "Invalid profiler"),
            ProfError::InvalidBinary => {
                println!("yes");
                write!(f, "Invalid binary")
            }
            ProfError::InvalidNum => write!(f, "Invalid number"),
            ProfError::InvalidSortMetric => write!(f, "Invalid sort metric"),
            ProfError::IOError(ref err) => fmt::Display::fmt(err, f),

        }
    }
}

impl error::Error for ProfError {
    fn description(&self) -> &str {
        match *self {
            ProfError::CallGrindError(_) => "Callgrind Error. File Bug.",
            ProfError::RegexError => "Regex error. file bug.",
            ProfError::InvalidProfiler => "Invalid Profiler.",
            ProfError::InvalidBinary => "Invalid Binary.",
            ProfError::InvalidNum => "Invalid number.",
            ProfError::InvalidSortMetric => "Invalid sort metric.",
            ProfError::IOError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ProfError::CallGrindError(_) => None,
            ProfError::RegexError => None,
            ProfError::InvalidProfiler => None,
            ProfError::InvalidBinary => None,
            ProfError::InvalidNum => None,
            ProfError::InvalidSortMetric => None,
            ProfError::IOError(ref err) => Some(err),

        }
    }
}

impl From<ioError> for ProfError {
    fn from(err: ioError) -> ProfError {
        ProfError::IOError(err)
    }
}
