use std::io::{BufRead, Seek};

use crate::{error::Error, Format, ImgResult};

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
}
