use std::io::{BufRead, Seek};

use crate::{decoders::ImageSize, error::Error, Format, ImgResult};

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub size: (usize, usize),
    pub format: Format,
}

impl ImageInfo {
    pub(crate) fn from_img_reader<R: BufRead + Seek>(img: image::io::Reader<R>) -> ImgResult<Self> {
        let format = img.format().ok_or(Error::Format)?.into();
        let size = img.into_dimensions()?;

        Ok(ImageInfo {
            size: (size.0 as usize, size.1 as usize),
            format,
        })
    }

    pub(crate) fn new(format: Format, img: impl ImageSize) -> Self {
        Self {
            size: img.size(),
            format,
        }
    }
}
