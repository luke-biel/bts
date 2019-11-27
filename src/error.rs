use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    CopyError(io::Error),
    Lookup(io::Error),
    EmptyDirectory,
    MissingFilename,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CopyError(inner) => write!(f, "error occurred during copy action: {}", inner),
            Error::Lookup(inner) => write!(f, "error occurred during lookup: {}", inner),
            Error::EmptyDirectory => write!(f, "cannot instantiate empty template"),
            Error::MissingFilename => write!(f, "unable to retrieve filename"),
        }
    }
}

impl std::error::Error for Error {}
