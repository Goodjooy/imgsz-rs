use std::{fmt::Display, io};

use image::ImageError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Image(image::error::ImageError),
    Other(String),
    Format,
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Self::Image(err)
    }
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
            Error::Image(img) => write!(f, "Image error {}", img),
            Error::Other(msg) => write!(f, "error {}", msg),
            Error::Format => write!(f, "Format Not Support"),
        }
    }
}

impl std::error::Error for Error {}
