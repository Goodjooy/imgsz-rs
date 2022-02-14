mod decoders;
pub mod error;
mod img_info;
use std::{
    io::{Read, Seek},
    path::Path,
};

pub use decoders::Format;
pub use img_info::ImageInfo;
pub struct ImageInfoLoader;
pub type ImgResult<T> = Result<T, error::Error>;


impl ImageInfoLoader {
    pub fn from_file<F: AsRef<Path>>(file: F) -> ImgResult<ImageInfo> {
        let file = std::fs::File::open(file)?;
        let mut file = file;
        Self::from_reader(&mut file)
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> ImgResult<ImageInfo> {
        decoders::imgsz(reader)
    }
}

