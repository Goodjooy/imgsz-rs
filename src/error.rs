#[cfg(feature = "use-img")]
use image::ImageError;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Other(String),
    Format,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(io) => write!(f, "IO error {}", io),
            Error::Other(msg) => write!(f, "error {}", msg),
            Error::Format => write!(f, "Format Not Support"),
        }
    }
}

impl std::error::Error for Error {}
