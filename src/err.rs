use std::error;
use std::fmt;
use std::io::Error as ioError;

#[derive(Debug)]
/// Represents potential errors that may occur when profiling
pub enum ProfError {
    RegexError,
    InvalidProfiler,
    InvalidBinary,
    InvalidNum,
    InvalidSortMetric,
    /// Wraps a std::io::Error
    IOError(ioError),
    UTF8Error,
    MisalignedData,
    CompilationError(String, String),
    TomlError,
    ReadManifestError,
    NoNameError,
    NoTargetDirectory,
    OutOfMemoryError,
    CliError,
}

impl fmt::Display for ProfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfError::RegexError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mRegex error -- please file a bug. In bug report, \
                 please include the original output file from profiler, e.g. from \
                 valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.txt"
            ),
            ProfError::InvalidProfiler => write!(
                f,
                "\x1b[1;31merror: \x1b[0mInvalid profiler. cargo profiler currently \
                 supports callgrind and cachegrind."
            ),
            ProfError::InvalidBinary => write!(
                f,
                "\x1b[1;31merror: \x1b[0mInvalid binary. make sure binary exists."
            ),
            ProfError::InvalidNum => write!(
                f,
                "\x1b[1;31merror: \x1b[0mInvalid number. make sure number is a positive \
                 integer."
            ),
            ProfError::InvalidSortMetric => write!(
                f,
                "\x1b[1;31merror: \x1b[0mInvalid metric to sort on. available cachegrind \
                 metrics are \nir, i1mr, ilmr, dr, d1mr, dlmr, dw, d1mw, and dlmw. Check \
                 README for details on these metrics."
            ),
            ProfError::IOError(ref err) => write!(
                f,
                "\x1b[1;31merror: \x1b[0mIO error: {} -- please file a bug.",
                err
            ),
            ProfError::UTF8Error => write!(
                f,
                "\x1b[1;31merror: \x1b[0mCLI Utf8 error -- please file a bug."
            ),
            ProfError::MisalignedData => write!(
                f,
                "\x1b[1;31merror: \x1b[0mMisaligned data arrays due to regex error -- \
                 please file a bug."
            ),
            ProfError::CompilationError(ref package_name, ref stderr) => write!(
                f,
                "\x1b[1;31merror: \x1b[0mFailed to compile {}.\n\n{}",
                package_name, stderr
            ),
            ProfError::TomlError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mError in parsing Cargo.toml to derive package \
                 name. Make sure package name is directly under [package] tag."
            ),
            ProfError::ReadManifestError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mCargo.toml missing. Are you sure you're in a Rust \
                 project?"
            ),

            ProfError::NoNameError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mNo package name found in Cargo.toml. Run \
                 cargo read-manifest to make sure everything looks okay. Otherwise please \
                 submit bug."
            ),

            ProfError::NoTargetDirectory => write!(
                f,
                "\x1b[1;31merror: \x1b[0mNo target output directory found in project. \
                 Binary must be in target/debug/ or target/release/, or specify binary \
                 path explicitly with --bin argument."
            ),
            ProfError::OutOfMemoryError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mValgrind's memory management: out of memory. \
                 Valgrind cannot continue. Sorry. "
            ),
            ProfError::CliError => write!(
                f,
                "\x1b[1;31merror: \x1b[0mError in valgrind cli call. Make sure valgrind is \
                 installed properly."
            ),
        }
    }
}

impl error::Error for ProfError {
    fn description(&self) -> &str {
        match *self {
            ProfError::RegexError => "Regex error. file bug.",
            ProfError::InvalidProfiler => "Invalid Profiler.",
            ProfError::InvalidBinary => "Invalid Binary.",
            ProfError::InvalidNum => "Invalid number.",
            ProfError::InvalidSortMetric => "Invalid sort metric.",
            ProfError::MisalignedData => "Misaligned Data. File bug.",
            ProfError::CompilationError(_, _) => {
                "Failed to compile. Run cargo build to get compilation error."
            }
            ProfError::TomlError => "Error in parsing Cargo.toml.",
            ProfError::ReadManifestError => "Error in reading the manifest of this crate.",
            ProfError::NoNameError => "No package name found in Cargo.toml",
            ProfError::NoTargetDirectory => "No target output directory found in project.",
            ProfError::IOError(ref err) => err.description(),
            ProfError::OutOfMemoryError => "out of memory.",
            ProfError::CliError => "make sure valgrind is installed properly.",
            ProfError::UTF8Error => "utf8 error. file bug.",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ProfError::RegexError => None,
            ProfError::InvalidProfiler => None,
            ProfError::InvalidBinary => None,
            ProfError::InvalidNum => None,
            ProfError::InvalidSortMetric => None,
            ProfError::MisalignedData => None,
            ProfError::TomlError => None,
            ProfError::IOError(ref err) => Some(err),
            ProfError::CompilationError(_, _) => None,
            ProfError::ReadManifestError => None,
            ProfError::NoNameError => None,
            ProfError::NoTargetDirectory => None,
            ProfError::OutOfMemoryError => None,
            ProfError::CliError => None,
            ProfError::UTF8Error => None,
        }
    }
}

impl From<ioError> for ProfError {
    fn from(err: ioError) -> ProfError {
        ProfError::IOError(err)
    }
}
