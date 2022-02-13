use std::io::{BufRead, Seek};

use image::ImageFormat;

use crate::InfoResult;

pub struct ImageInfo {
    pub size: (usize, usize),
    pub format: Option<ImageFormat>,
}

impl ImageInfo {
    pub(crate) fn from_img_reader<R: BufRead + Seek>(img: image::io::Reader<R>) -> InfoResult {
        let format = img.format();
        let size = img.into_dimensions()?;

        Ok(ImageInfo {
            size: (size.0 as usize, size.1 as usize),
            format,
        })
    }
}
