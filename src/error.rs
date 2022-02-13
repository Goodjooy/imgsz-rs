use std::io;

use image::ImageError;

pub enum Error {
    Io(io::Error),
    Image(image::error::ImageError),
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
