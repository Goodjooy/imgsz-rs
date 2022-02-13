use std::{fmt::Display, io};
#[cfg(feature="use-img")]
use image::ImageError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    #[cfg(feature="use-img")]
    Image(image::error::ImageError),
    Other(String),
    Format,
}

#[cfg(feature="use-img")]
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
            #[cfg(feature="use-img")]
            Error::Image(img) => write!(f, "Image error {}", img),
            Error::Other(msg) => write!(f, "error {}", msg),
            Error::Format => write!(f, "Format Not Support"),
        }
    }
}

impl std::error::Error for Error {}
