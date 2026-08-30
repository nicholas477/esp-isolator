

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ParsingError,
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParsingError => write!(f, "Parsing error"),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

/// Helper type alias for easier Result handling throughout your library.
pub type Result<T> = std::result::Result<T, Error>;