mod img_info;
mod decoders;
pub mod error;
use std::{
    io::{BufRead, Seek},
    path::Path,
};



pub use decoders::Format;
pub use img_info::ImageInfo;
pub struct ImageInfoLoader;
pub type ImgResult<T> = Result<T, error::Error>;

impl ImageInfoLoader {
    pub fn from_file<F: AsRef<Path>>(
        file: F,
    ) -> ImgResult<ImageInfo> {
        let img = image::io::Reader::open(file)?.with_guessed_format()?;
        ImageInfo::from_img_reader(img)
    }

    pub fn from_reader<R: BufRead + Seek>(reader: &mut R) -> ImgResult<ImageInfo> {
        let img = image::io::Reader::new(reader).with_guessed_format()?;
        ImageInfo::from_img_reader(img)
    }
}
