use std::{
    io::{BufRead, Seek},
    path::Path,
};

mod img_info;

pub use img_info::ImageInfo;
pub struct ImageInfoLoader;
pub type InfoResult = Result<img_info::ImageInfo, image::error::ImageError>;

impl ImageInfoLoader {
    pub fn from_file<F: AsRef<Path>>(
        file: F,
    ) -> Result<img_info::ImageInfo, image::error::ImageError> {
        let img = image::io::Reader::open(file)?.with_guessed_format()?;
        ImageInfo::from_img_reader(img)
    }

    pub fn from_reader<R: BufRead + Seek>(reader: &mut R) -> InfoResult {
        let img = image::io::Reader::new(reader).with_guessed_format()?;
        ImageInfo::from_img_reader(img)
    }
}
